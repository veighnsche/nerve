#![forbid(unsafe_code)]
//! UI applet primitives for Nerve (ADR-013, ADR-018).
//! These helpers expose deterministic narration mechanics and collect
//! events so callers can wire them into proofs or custom renderers.

use std::sync::Arc;

/// Marker event captured by a `Step` applet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepEvent {
    Info(String),
    Ok(Option<String>),
    Fail(Option<String>),
}

/// A minimal narration step that records events.
#[derive(Debug, Clone)]
pub struct Step {
    label: Arc<str>,
    events: Arc<[StepEvent]>,
}

impl Step {
    /// Construct a new `Step` with the provided label.
    #[must_use]
    pub fn new(label: impl Into<Arc<str>>) -> Self {
        Self {
            label: label.into(),
            events: Arc::new([]),
        }
    }

    /// Internal helper to append events immutably.
    fn with_event(&self, event: StepEvent) -> Self {
        let mut events = self.events.to_vec();
        events.push(event);
        Self {
            label: Arc::clone(&self.label),
            events: events.into(),
        }
    }

    /// Record an informational message.
    #[must_use]
    pub fn info(&self, msg: impl Into<String>) -> Self {
        self.with_event(StepEvent::Info(msg.into()))
    }

    /// Record a success marker with optional message.
    #[must_use]
    pub fn ok(&self, msg: Option<impl Into<String>>) -> Self {
        self.with_event(StepEvent::Ok(msg.map(Into::into)))
    }

    /// Record a failure marker with optional message.
    #[must_use]
    pub fn fail(&self, msg: Option<impl Into<String>>) -> Self {
        self.with_event(StepEvent::Fail(msg.map(Into::into)))
    }

    /// Label accessor to keep narration auditable.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Expose collected events for downstream consumers.
    #[must_use]
    pub fn events(&self) -> &[StepEvent] {
        &self.events
    }
}

/// Unique identifier for the UI module, used by BDD scaffolding.
#[must_use]
pub const fn module_name() -> &'static str {
    "ui"
}

/// Create a new narration step applet.
#[must_use]
pub fn step(label: impl Into<Arc<str>>) -> Step {
    Step::new(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_records_events() {
        let step = step("scaffold").info("starting").ok(Some("done"));
        assert_eq!(step.label(), "scaffold");
        assert_eq!(
            step.events(),
            [
                StepEvent::Info("starting".into()),
                StepEvent::Ok(Some("done".into())),
            ]
        );
    }
}
