#![forbid(unsafe_code)]
//! Nerve core library (stub)
//!
//! This crate exists to hold the minimal, explicit primitives per ADRs:
//! - ADR-006: file/dir/apply guardrails (overrideable)
//! - ADR-007: context mechanics (no policy)
//! - ADR-009: matcher helpers (compile/validate/route)
//! - ADR-012: proof bundle helpers
//! - ADR-013: UI applets (TTY + JSON mode)
//! - ADR-008: Anti-Insanity Clause (no hidden behavior)

pub mod llm;

pub mod file {
    /// Placeholder for file guardrail primitives (ADR-006).
    #[must_use]
    pub const fn module_name() -> &'static str {
        "file"
    }
}

pub mod dir {
    /// Placeholder for directory guardrail primitives (ADR-006).
    #[must_use]
    pub const fn module_name() -> &'static str {
        "dir"
    }
}

pub mod apply;

pub mod ctx {
    /// Placeholder for context budgeting primitives (ADR-007).
    #[must_use]
    pub const fn module_name() -> &'static str {
        "ctx"
    }
}

pub mod r#match {
    /// Placeholder for semantic matcher helpers (ADR-009).
    #[must_use]
    pub const fn module_name() -> &'static str {
        "match"
    }
}

pub mod proof {
    /// Placeholder for proof bundle helpers (ADR-012).
    #[must_use]
    pub const fn module_name() -> &'static str {
        "proof"
    }
}

pub use nrv_ui as ui;
