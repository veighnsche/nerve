#![allow(clippy::module_name_repetitions)]

use std::sync::Arc;

use nrv_orch_client::{
    Capabilities, NrvServerError, OrchestratorClient, StreamEvent, TaskAccepted, TaskId,
    TaskRequest, WorkloadKind,
};

/// Name of the `nrv.llm` module for scaffolding checks.
#[must_use]
pub const fn module_name() -> &'static str {
    "llm"
}

/// Configuration supplied when constructing an [`LlmClient`].
#[derive(Debug, Clone)]
pub struct LlmClientConfig<O>
where
    O: OrchestratorClient + Send + Sync + 'static,
{
    pub orchestrator: Arc<O>,
}

impl<O> LlmClientConfig<O>
where
    O: OrchestratorClient + Send + Sync + 'static,
{
    #[must_use]
    pub fn new(orchestrator: O) -> Self {
        Self {
            orchestrator: Arc::new(orchestrator),
        }
    }
}

/// Entry point for building an [`LlmClient`].
pub fn client<O>(config: LlmClientConfig<O>) -> Result<LlmClient<O>, LlmError>
where
    O: OrchestratorClient + Send + Sync + 'static,
{
    Ok(LlmClient::new(config.orchestrator))
}

/// Primary Rust entry point for interacting with the orchestrator-backed LLM surface.
pub struct LlmClient<O>
where
    O: OrchestratorClient + Send + Sync + 'static,
{
    orchestrator: Arc<O>,
}

impl<O> Clone for LlmClient<O>
where
    O: OrchestratorClient + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            orchestrator: Arc::clone(&self.orchestrator),
        }
    }
}

impl<O> LlmClient<O>
where
    O: OrchestratorClient + Send + Sync + 'static,
{
    #[must_use]
    pub fn new(orchestrator: Arc<O>) -> Self {
        Self { orchestrator }
    }

    /// Retrieves the current capability snapshot from the orchestrator.
    pub fn capabilities(&self) -> Result<Capabilities, LlmError> {
        self.orchestrator
            .capabilities()
            .map_err(LlmError::from_orchestrator)
    }

    /// Submits an [`LlmRequest`] to the orchestrator queue and returns a handle for follow-up calls.
    pub fn enqueue(&self, request: LlmRequest) -> Result<LlmTaskHandle, LlmError> {
        let accepted = self
            .orchestrator
            .enqueue(request.into_task_request())
            .map_err(LlmError::from_orchestrator)?;
        Ok(LlmTaskHandle::from(accepted))
    }

    /// Subscribes to streaming events for a previously enqueued task.
    pub fn stream(&self, handle: &LlmTaskHandle) -> Result<LlmStream<O::Stream>, LlmError> {
        let stream = self
            .orchestrator
            .stream(handle.task_id())
            .map_err(LlmError::from_orchestrator)?;
        Ok(LlmStream::new(stream))
    }

    /// Attempts to cancel an in-flight task.
    pub fn cancel(&self, handle: &LlmTaskHandle) -> Result<CancelOutcome, LlmError> {
        self.orchestrator
            .cancel(handle.task_id())
            .map_err(LlmError::from_orchestrator)?;
        Ok(CancelOutcome)
    }
}

/// Simplified request model passed to [`LlmClient::enqueue`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmRequest {
    pub model: String,
    pub workload: WorkloadKind,
    pub max_tokens: Option<u32>,
}

impl LlmRequest {
    #[must_use]
    pub fn builder(capabilities: Capabilities) -> LlmRequestBuilder {
        LlmRequestBuilder::new(capabilities)
    }

    fn into_task_request(self) -> TaskRequest {
        TaskRequest {
            model: self.model,
            workload: Some(self.workload),
            max_tokens: self.max_tokens,
        }
    }
}

/// Builder used to validate requests against a capability snapshot before enqueueing.
#[derive(Debug, Clone)]
pub struct LlmRequestBuilder {
    capabilities: Capabilities,
    model: Option<String>,
    workload: Option<WorkloadKind>,
    max_tokens: Option<u32>,
}

impl LlmRequestBuilder {
    #[must_use]
    pub fn new(capabilities: Capabilities) -> Self {
        Self {
            capabilities,
            model: None,
            workload: None,
            max_tokens: None,
        }
    }

    #[must_use]
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    #[must_use]
    pub fn workload(mut self, workload: WorkloadKind) -> Self {
        self.workload = Some(workload);
        self
    }

    #[must_use]
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn build(self) -> Result<LlmRequest, LlmRequestBuildError> {
        let model = self.model.ok_or(LlmRequestBuildError::MissingModel)?;

        let model_capability = self
            .capabilities
            .models
            .iter()
            .find(|entry| entry.id == model)
            .ok_or_else(|| LlmRequestBuildError::UnknownModel {
                model: model.clone(),
            })?;

        let workload = match self.workload {
            Some(workload) => {
                let workload_cap = self
                    .capabilities
                    .workloads
                    .iter()
                    .find(|entry| entry.workload == workload)
                    .ok_or_else(|| LlmRequestBuildError::UnknownWorkload { workload })?;
                if !workload_cap
                    .supported_models
                    .iter()
                    .any(|entry| entry == &model)
                {
                    return Err(LlmRequestBuildError::WorkloadUnsupported { workload, model });
                }
                workload
            }
            None => self
                .capabilities
                .workloads
                .iter()
                .find(|entry| {
                    entry
                        .supported_models
                        .iter()
                        .any(|candidate| candidate == &model)
                })
                .map(|entry| entry.workload)
                .ok_or_else(|| LlmRequestBuildError::WorkloadUnavailable {
                    model: model.clone(),
                })?,
        };

        if let Some(requested) = self.max_tokens {
            if requested == 0 {
                return Err(LlmRequestBuildError::InvalidTokenLimit { requested });
            }

            if let Some(limit) = model_capability.max_tokens_out {
                if requested > limit {
                    return Err(LlmRequestBuildError::TokenLimitExceededForModel {
                        requested,
                        limit,
                        model: model.clone(),
                    });
                }
            }

            if let Some(limit) = self.capabilities.limits.max_tokens_out {
                if requested > limit {
                    return Err(LlmRequestBuildError::TokenLimitExceededGlobal {
                        requested,
                        limit,
                    });
                }
            }
        }

        Ok(LlmRequest {
            model,
            workload,
            max_tokens: self.max_tokens,
        })
    }
}

/// Errors emitted while validating an LLM request against capabilities.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum LlmRequestBuildError {
    #[error("nrv.llm: request builder requires a model identifier")]
    MissingModel,
    #[error("nrv.llm: model {model} not present in capability snapshot")]
    UnknownModel { model: String },
    #[error("nrv.llm: workload {workload:?} not present in capability snapshot")]
    UnknownWorkload { workload: WorkloadKind },
    #[error("nrv.llm: workload {workload:?} does not support model {model}")]
    WorkloadUnsupported {
        workload: WorkloadKind,
        model: String,
    },
    #[error("nrv.llm: no workload supports model {model}")]
    WorkloadUnavailable { model: String },
    #[error("nrv.llm: max_tokens must be greater than zero (got {requested})")]
    InvalidTokenLimit { requested: u32 },
    #[error("nrv.llm: requested max_tokens {requested} exceeds model {model} limit {limit}")]
    TokenLimitExceededForModel {
        requested: u32,
        limit: u32,
        model: String,
    },
    #[error("nrv.llm: requested max_tokens {requested} exceeds global limit {limit}")]
    TokenLimitExceededGlobal { requested: u32, limit: u32 },
}

/// Handle returned after enqueueing a request.
#[derive(Debug, Clone)]
pub struct LlmTaskHandle {
    task_id: TaskId,
    queue_position: Option<u32>,
}

impl LlmTaskHandle {
    #[must_use]
    pub fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    #[must_use]
    pub fn queue_position(&self) -> Option<u32> {
        self.queue_position
    }
}

impl From<TaskAccepted> for LlmTaskHandle {
    fn from(accepted: TaskAccepted) -> Self {
        Self {
            task_id: accepted.task_id,
            queue_position: accepted.queue_position,
        }
    }
}

/// Convenience wrapper around orchestrator stream events with additional bookkeeping.
pub struct LlmStream<S>
where
    S: Iterator<Item = StreamEvent> + Send,
{
    inner: S,
    next_token_index: usize,
    finished: bool,
}

impl<S> LlmStream<S>
where
    S: Iterator<Item = StreamEvent> + Send,
{
    #[must_use]
    pub fn new(stream: S) -> Self {
        Self {
            inner: stream,
            next_token_index: 0,
            finished: false,
        }
    }
}

impl<S> Iterator for LlmStream<S>
where
    S: Iterator<Item = StreamEvent> + Send,
{
    type Item = Result<LlmStreamEvent, LlmError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let event = self.inner.next()?;
        let mapped = match event {
            StreamEvent::Started => Ok(LlmStreamEvent::Started),
            StreamEvent::Token(text) => {
                let index = self.next_token_index;
                self.next_token_index += 1;
                Ok(LlmStreamEvent::Token { text, index })
            }
            StreamEvent::Metrics(payload) => Ok(LlmStreamEvent::Metrics { payload }),
            StreamEvent::End => {
                self.finished = true;
                Ok(LlmStreamEvent::Completed)
            }
            StreamEvent::Error(message) => {
                self.finished = true;
                Err(LlmError::stream_error("orch.stream", message))
            }
        };
        Some(mapped)
    }
}

/// High-level stream events surfaced to LLM callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmStreamEvent {
    Started,
    Token { text: String, index: usize },
    Metrics { payload: String },
    Completed,
}

/// Outcome returned after a successful cancellation request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancelOutcome;

/// Errors emitted while interacting with the LLM client surface.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum LlmError {
    #[error("nrv.llm: orchestrator error {code}: {message}")]
    Orchestrator {
        code: String,
        message: String,
        retriable: Option<bool>,
        retry_after_ms: Option<u64>,
    },
    #[error("nrv.llm: stream terminated with error {code}: {message}")]
    Stream {
        code: String,
        message: String,
        retriable: Option<bool>,
        retry_after_ms: Option<u64>,
    },
}

impl LlmError {
    fn from_orchestrator(err: NrvServerError) -> Self {
        Self::Orchestrator {
            code: err.code,
            message: err.message,
            retriable: err.retriable,
            retry_after_ms: err.retry_after_ms,
        }
    }

    fn stream_error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Stream {
            code: code.into(),
            message: message.into(),
            retriable: None,
            retry_after_ms: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nrv_orch_client::Cancelled;
    use std::collections::{HashMap, VecDeque};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    fn sample_capabilities() -> Capabilities {
        Capabilities {
            metadata: nrv_orch_client::OrchestratorMetadata {
                engine: "stub".into(),
                version: "0.0.0".into(),
                build: None,
                commit: None,
            },
            limits: nrv_orch_client::Limits {
                ctx_max: 32_768,
                max_tokens_out: Some(2_048),
                max_concurrent_requests: Some(4),
                queue_depth_limit: Some(128),
            },
            workloads: vec![nrv_orch_client::WorkloadCapability {
                workload: nrv_orch_client::WorkloadKind::Chat,
                supported_models: vec!["stub-model".into()],
                default_model: Some("stub-model".into()),
                supports_guardrails: true,
            }],
            models: vec![nrv_orch_client::ModelCapability {
                id: "stub-model".into(),
                display_name: Some("Stub Model".into()),
                family: None,
                modality: nrv_orch_client::ModelModality::Text,
                ctx_max: 32_768,
                max_tokens_out: Some(2_048),
                supports_tool_calls: false,
                supports_parallel_functions: false,
                inference_units_per_ms: None,
            }],
            hardware: nrv_orch_client::HardwareInventory {
                gpus: vec![],
                cpus: None,
            },
            tools: None,
            captured_at: "1970-01-01T00:00:00Z".into(),
        }
    }

    #[derive(Debug)]
    struct StubOrchestrator {
        capabilities: Capabilities,
        enqueue_log: Mutex<Vec<TaskRequest>>,
        cancel_log: Mutex<Vec<String>>,
        stream_queue: Mutex<VecDeque<Vec<StreamEvent>>>,
        assigned_streams: Mutex<HashMap<String, Vec<StreamEvent>>>,
        next_id: AtomicUsize,
    }

    impl StubOrchestrator {
        fn new(streams: Vec<Vec<StreamEvent>>) -> Self {
            Self {
                capabilities: sample_capabilities(),
                enqueue_log: Mutex::new(Vec::new()),
                cancel_log: Mutex::new(Vec::new()),
                stream_queue: Mutex::new(streams.into_iter().collect()),
                assigned_streams: Mutex::new(HashMap::new()),
                next_id: AtomicUsize::new(0),
            }
        }

        fn recorded_requests(&self) -> Vec<TaskRequest> {
            self.enqueue_log.lock().expect("log lock").clone()
        }

        fn cancelled_tasks(&self) -> Vec<String> {
            self.cancel_log.lock().expect("cancel log lock").clone()
        }

        fn default_stream_plan() -> Vec<StreamEvent> {
            vec![StreamEvent::Started, StreamEvent::End]
        }
    }

    impl OrchestratorClient for StubOrchestrator {
        type Stream = std::vec::IntoIter<StreamEvent>;

        fn capabilities(&self) -> Result<Capabilities, NrvServerError> {
            Ok(self.capabilities.clone())
        }

        fn enqueue(&self, req: TaskRequest) -> Result<TaskAccepted, NrvServerError> {
            self.enqueue_log.lock().expect("log lock").push(req.clone());
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let task_id = format!("task-{id}");

            let stream_plan = self
                .stream_queue
                .lock()
                .expect("stream queue lock")
                .pop_front()
                .unwrap_or_else(Self::default_stream_plan);
            self.assigned_streams
                .lock()
                .expect("assigned streams lock")
                .insert(task_id.clone(), stream_plan);

            Ok(TaskAccepted {
                task_id: TaskId(task_id),
                queue_position: Some(0),
            })
        }

        fn stream(&self, task_id: &TaskId) -> Result<Self::Stream, NrvServerError> {
            let maybe_stream = self
                .assigned_streams
                .lock()
                .expect("assigned streams lock")
                .remove(&task_id.0);
            match maybe_stream {
                Some(events) => Ok(events.into_iter()),
                None => Err(NrvServerError {
                    code: "missing_stream".into(),
                    message: format!("no stream registered for {}", task_id.0),
                    retriable: Some(false),
                    retry_after_ms: None,
                }),
            }
        }

        fn cancel(&self, task_id: &TaskId) -> Result<Cancelled, NrvServerError> {
            self.cancel_log
                .lock()
                .expect("cancel log lock")
                .push(task_id.0.clone());
            Ok(Cancelled)
        }
    }

    #[test]
    fn builder_produces_request_with_defaults() {
        let request = LlmRequest::builder(sample_capabilities())
            .model("stub-model")
            .build()
            .expect("build");

        assert_eq!(request.model, "stub-model");
        assert_eq!(request.workload, WorkloadKind::Chat);
        assert_eq!(request.max_tokens, None);
    }

    #[test]
    fn builder_rejects_unknown_model() {
        let err = LlmRequest::builder(sample_capabilities())
            .model("does-not-exist")
            .build()
            .expect_err("unknown model");

        assert!(
            matches!(err, LlmRequestBuildError::UnknownModel { model } if model == "does-not-exist")
        );
    }

    #[test]
    fn builder_rejects_unknown_workload() {
        let err = LlmRequest::builder(sample_capabilities())
            .model("stub-model")
            .workload(WorkloadKind::Tool)
            .build()
            .expect_err("unknown workload");

        assert!(matches!(
            err,
            LlmRequestBuildError::UnknownWorkload {
                workload: WorkloadKind::Tool
            }
        ));
    }

    #[test]
    fn builder_enforces_token_limits() {
        let err = LlmRequest::builder(sample_capabilities())
            .model("stub-model")
            .max_tokens(10_000)
            .build()
            .expect_err("token limit");

        assert!(
            matches!(err, LlmRequestBuildError::TokenLimitExceededForModel { limit, .. } if limit == 2_048)
        );
    }

    #[test]
    fn client_wires_enqueue_stream_and_cancel() {
        let orchestrator = Arc::new(StubOrchestrator::new(vec![vec![
            StreamEvent::Started,
            StreamEvent::Token("hi".into()),
            StreamEvent::Metrics("latency:10".into()),
            StreamEvent::End,
        ]]));

        let config = LlmClientConfig {
            orchestrator: Arc::clone(&orchestrator),
        };
        let client = client(config).expect("client");

        let caps = client.capabilities().expect("capabilities");
        assert_eq!(caps.metadata.engine, "stub");

        let request = LlmRequest::builder(caps.clone())
            .model("stub-model")
            .max_tokens(128)
            .build()
            .expect("request");

        let handle = client.enqueue(request).expect("enqueue");
        assert_eq!(handle.queue_position(), Some(0));

        let events: Vec<LlmStreamEvent> = client
            .stream(&handle)
            .expect("stream")
            .collect::<Result<_, _>>()
            .expect("events");

        assert!(matches!(events[0], LlmStreamEvent::Started));
        assert!(
            matches!(events[1], LlmStreamEvent::Token { ref text, index } if text == "hi" && index == 0)
        );
        assert!(
            matches!(events[2], LlmStreamEvent::Metrics { ref payload } if payload == "latency:10")
        );
        assert!(matches!(events[3], LlmStreamEvent::Completed));

        client.cancel(&handle).expect("cancel");

        let recorded = orchestrator.recorded_requests();
        assert_eq!(recorded.len(), 1);
        assert_eq!(recorded[0].model, "stub-model");

        let cancelled = orchestrator.cancelled_tasks();
        assert_eq!(cancelled, vec![handle.task_id().0.clone()]);
    }

    #[test]
    fn stream_errors_propagate() {
        let orchestrator = Arc::new(StubOrchestrator::new(vec![vec![
            StreamEvent::Started,
            StreamEvent::Error("boom".into()),
        ]]));

        let client = client(LlmClientConfig {
            orchestrator: Arc::clone(&orchestrator),
        })
        .expect("client");

        let request = LlmRequest::builder(client.capabilities().expect("capabilities"))
            .model("stub-model")
            .build()
            .expect("request");

        let handle = client.enqueue(request).expect("enqueue");

        let mut stream = client.stream(&handle).expect("stream");
        assert!(matches!(stream.next(), Some(Ok(LlmStreamEvent::Started))));

        match stream.next() {
            Some(Err(LlmError::Stream { message, .. })) => {
                assert_eq!(message, "boom");
            }
            other => panic!("unexpected stream outcome: {other:?}"),
        }

        assert!(stream.next().is_none());
    }

    #[derive(Debug)]
    struct RejectingOrchestrator;

    impl OrchestratorClient for RejectingOrchestrator {
        type Stream = std::vec::IntoIter<StreamEvent>;

        fn capabilities(&self) -> Result<Capabilities, NrvServerError> {
            Err(NrvServerError {
                code: "capabilities_denied".into(),
                message: "no capabilities".into(),
                retriable: Some(false),
                retry_after_ms: None,
            })
        }

        fn enqueue(&self, _req: TaskRequest) -> Result<TaskAccepted, NrvServerError> {
            Err(NrvServerError {
                code: "enqueue_denied".into(),
                message: "nope".into(),
                retriable: Some(false),
                retry_after_ms: None,
            })
        }

        fn stream(&self, _task_id: &TaskId) -> Result<Self::Stream, NrvServerError> {
            Err(NrvServerError {
                code: "stream_denied".into(),
                message: "no stream".into(),
                retriable: None,
                retry_after_ms: None,
            })
        }

        fn cancel(&self, _task_id: &TaskId) -> Result<Cancelled, NrvServerError> {
            Err(NrvServerError {
                code: "cancel_denied".into(),
                message: "cannot cancel".into(),
                retriable: Some(true),
                retry_after_ms: Some(1_000),
            })
        }
    }

    #[test]
    fn orchestrator_errors_are_wrapped() {
        let client = client(LlmClientConfig {
            orchestrator: Arc::new(RejectingOrchestrator),
        })
        .expect("client");

        let err = client.capabilities().expect_err("expected error");
        assert!(
            matches!(err, LlmError::Orchestrator { ref code, .. } if code == "capabilities_denied")
        );

        let err = client
            .enqueue(LlmRequest {
                model: "stub-model".into(),
                workload: WorkloadKind::Chat,
                max_tokens: None,
            })
            .expect_err("expected enqueue error");
        assert!(matches!(err, LlmError::Orchestrator { ref code, .. } if code == "enqueue_denied"));

        let err = client
            .stream(&LlmTaskHandle::from(TaskAccepted {
                task_id: TaskId("task-unknown".into()),
                queue_position: None,
            }))
            .expect_err("expected stream error");
        assert!(matches!(err, LlmError::Orchestrator { ref code, .. } if code == "stream_denied"));

        let err = client
            .cancel(&LlmTaskHandle::from(TaskAccepted {
                task_id: TaskId("task-unknown".into()),
                queue_position: None,
            }))
            .expect_err("expected cancel error");
        assert!(matches!(err, LlmError::Orchestrator { ref code, .. } if code == "cancel_denied"));
    }
}
