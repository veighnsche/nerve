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

pub mod file {}
pub mod dir {}
pub mod apply {}
pub mod ctx {}
pub mod r#match {}
pub mod proof {}
pub mod ui {}
