#![allow(clippy::module_name_repetitions)]

use cucumber::{given, then, when, writer::Basic, World};
use std::path::PathBuf;

const BDD_ENV_FLAG: &str = "NRV_RUN_BDD";

#[derive(Debug, Default, World)]
#[world(init = Self::default)]
struct OrchWorld {
    capabilities: Option<nrv_orch_client::Capabilities>,
    task_requests: Vec<nrv_orch_client::TaskRequest>,
    accepted_tasks: Vec<nrv_orch_client::TaskAccepted>,
    stream_events: Vec<nrv_orch_client::StreamEvent>,
    cancelled_tasks: Vec<nrv_orch_client::TaskId>,
    errors: Vec<nrv_orch_client::NrvServerError>,
}

#[given("an orchestrator binding contract draft")]
async fn orchestrator_contract(world: &mut OrchWorld) {
    *world = OrchWorld::default();
}

#[when("capabilities are requested")]
async fn request_capabilities(world: &mut OrchWorld) {
    world.capabilities = Some(nrv_orch_client::Capabilities {
        metadata: nrv_orch_client::OrchestratorMetadata {
            engine: "nrv-sse".into(),
            version: "2024.09".into(),
            build: Some("2024-09-18".into()),
            commit: Some("abcdef123456".into()),
        },
        limits: nrv_orch_client::Limits {
            ctx_max: 32_768,
            max_tokens_out: Some(4_096),
            max_concurrent_requests: Some(8),
            queue_depth_limit: Some(512),
        },
        workloads: vec![nrv_orch_client::WorkloadCapability {
            workload: nrv_orch_client::WorkloadKind::Chat,
            supported_models: vec!["meta-llama/llama-3-70b".into()],
            default_model: Some("meta-llama/llama-3-70b".into()),
            supports_guardrails: true,
        }],
        models: vec![nrv_orch_client::ModelCapability {
            id: "meta-llama/llama-3-70b".into(),
            display_name: Some("Llama 3 70B".into()),
            family: Some("meta-llama".into()),
            modality: nrv_orch_client::ModelModality::Text,
            ctx_max: 32_768,
            max_tokens_out: Some(4_096),
            supports_tool_calls: true,
            supports_parallel_functions: false,
            inference_units_per_ms: Some(0.42),
        }],
        hardware: nrv_orch_client::HardwareInventory {
            gpus: vec![nrv_orch_client::GpuInfo {
                id: "H100-PCIE-80GB-0".into(),
                vendor: "nvidia".into(),
                name: "NVIDIA H100 80GB".into(),
                memory_gb: 80,
                driver: Some("550.54".into()),
                arch: Some("sm90".into()),
            }],
            cpus: Some(vec![nrv_orch_client::CpuInfo {
                model: "AMD EPYC".into(),
                cores: 64,
                threads: 128,
            }]),
        },
        tools: Some(vec![nrv_orch_client::ToolCapability {
            name: "vector-search".into(),
            description: Some("Fetches documents from the project index".into()),
            input_schema: Some("{\"type\":\"object\"}".into()),
            returns_schema: Some(
                "{\"type\":\"object\",\"properties\":{\"accepted\":{\"type\":\"boolean\"}}}".into(),
            ),
            timeout_ms: Some(5_000),
        }]),
        captured_at: "2024-09-18T12:34:56Z".into(),
    });
}

#[then("the client tracks capability constraints for later assertions")]
async fn track_capability_constraints(world: &mut OrchWorld) {
    let caps = world
        .capabilities
        .as_ref()
        .expect("capabilities were never requested in this scenario");
    assert!(caps.limits.ctx_max > 0);
    assert!(caps.limits.max_tokens_out.is_some_and(|tokens| tokens > 0));
    assert!(caps
        .workloads
        .iter()
        .any(|workload| !workload.supported_models.is_empty()));
    assert!(!caps.models.is_empty());
}

#[then("capability metadata includes optional fields")]
async fn capability_metadata(world: &mut OrchWorld) {
    let caps = world
        .capabilities
        .as_ref()
        .expect("capabilities should exist for metadata assertions");
    assert!(caps.metadata.engine.as_str().contains("nrv"));
    assert!(caps.metadata.version.as_str().contains('.'));
    assert!(caps.metadata.build.is_some());
    assert!(caps.metadata.commit.is_some());
    assert!(caps.hardware.gpus.iter().any(|gpu| gpu.memory_gb >= 16));
}

#[when(regex = r#"a task is enqueued with model \"([^\"]+)\""#)]
async fn enqueue_task(world: &mut OrchWorld, model: String) {
    let request = nrv_orch_client::TaskRequest {
        model: model.clone(),
    };
    let accepted = nrv_orch_client::TaskAccepted {
        task_id: nrv_orch_client::TaskId(format!("queued-{}", model)),
        queue_position: Some(0),
    };

    world.task_requests.push(request);
    world.accepted_tasks.push(accepted);
}

#[when(regex = r#"task \"([^\"]+)\" is cancelled"#)]
async fn cancel_task(world: &mut OrchWorld, task_id: String) {
    let task_id = nrv_orch_client::TaskId(task_id);
    if !world
        .cancelled_tasks
        .iter()
        .any(|existing| existing.0 == task_id.0)
    {
        world.cancelled_tasks.push(task_id);
    }
}

#[then("task cancellation remains a first-class contract")]
async fn ensure_cancel_contract(world: &mut OrchWorld) {
    assert!(
        !world.cancelled_tasks.is_empty(),
        "cancellation placeholders missing from orchestrator contract"
    );
}

#[when("the orchestrator begins streaming events")]
async fn orchestrator_begins_stream(world: &mut OrchWorld) {
    world.stream_events = vec![
        nrv_orch_client::StreamEvent::Started,
        nrv_orch_client::StreamEvent::Token("partial".into()),
        nrv_orch_client::StreamEvent::Metrics("latency_ms=12".into()),
        nrv_orch_client::StreamEvent::End,
    ];
}

#[then("the BDD scaffold records pending stream and cancel flows")]
async fn record_pending_stream(world: &mut OrchWorld) {
    assert!(
        !world.task_requests.is_empty(),
        "no models enqueued; stream/cancel placeholders would be empty"
    );
    assert!(
        matches!(
            world.stream_events.first(),
            Some(nrv_orch_client::StreamEvent::Started)
        ),
        "stream must start with StreamEvent::Started"
    );
    assert!(
        matches!(
            world.stream_events.last(),
            Some(nrv_orch_client::StreamEvent::End)
        ),
        "stream must end with StreamEvent::End"
    );
    assert_eq!(world.accepted_tasks.len(), world.task_requests.len());
}

#[when(regex = r#"the orchestrator reports error code \"([^\"]+)\" with message \"([^\"]+)\""#)]
async fn orchestrator_reports_error(world: &mut OrchWorld, code: String, message: String) {
    world.errors.push(nrv_orch_client::NrvServerError {
        code,
        message,
        retriable: None,
        retry_after_ms: None,
    });
}

#[when(regex = r#"the error is flagged retriable after (\d+) ms"#)]
async fn flag_error_retriable(world: &mut OrchWorld, delay_ms: u64) {
    let Some(last) = world.errors.last_mut() else {
        panic!("no orchestrator error recorded to update retriable status");
    };
    last.retriable = Some(true);
    last.retry_after_ms = Some(delay_ms);
}

#[then("the structured error is exposed to callers")]
async fn structured_error_exposed(world: &mut OrchWorld) {
    let Some(err) = world.errors.last() else {
        panic!("expected an orchestrator error to be recorded");
    };
    assert_eq!(err.code, "E429");
    assert_eq!(err.message, "Too many requests");
    assert!(err.retriable == Some(true));
    assert_eq!(err.retry_after_ms, Some(1_000));
}

#[tokio::test]
async fn run_bdd() {
    if std::env::var_os(BDD_ENV_FLAG).is_none() {
        eprintln!(
            "skipping nrv-orch-client BDD scaffolding; set {BDD_ENV_FLAG}=1 to execute the feature suite"
        );
        return;
    }

    let features = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/features");

    OrchWorld::cucumber()
        .with_writer(Basic::stdout())
        .run(features)
        .await;
}
