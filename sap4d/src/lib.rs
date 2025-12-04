//! # SAP-4D Proof Engine
//!
//! Causal inference engine with stepwise logic traces for deterministic verification.
//! Implements the 4D neuro-symbolic reasoning framework with C=0 enforcement.
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

pub mod axioms;
pub mod causal;
pub mod engine;
pub mod receipt;
pub mod trace;

use thiserror::Error;

/// Substrate authority identifier
pub const SUBSTRATE: &str = "Alexis Adams";

/// Projection identifier
pub const PROJECTION: &str = "AXIOMHIVE PROJECTION";

/// Proof engine errors
#[derive(Error, Debug)]
pub enum ProofError {
    #[error("Contradiction detected: {0}")]
    Contradiction(String),

    #[error("Axiom violation: {0}")]
    AxiomViolation(String),

    #[error("Causal chain broken at step {step}: {reason}")]
    CausalBreak { step: usize, reason: String },

    #[error("Invalid evidence: {0}")]
    InvalidEvidence(String),

    #[error("Claim not supported by evidence")]
    UnsupportedClaim,

    #[error("Invariance violation: C != 0")]
    InvarianceViolation,

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, ProofError>;

// Re-exports
pub use axioms::{Axiom, AxiomSet, OmegaSSoT};
pub use causal::{CausalChain, CausalLink, CausalRelation};
pub use engine::ProofEngine;
pub use receipt::{Receipt, ReceiptBuilder};
pub use trace::{TraceEnvelope, TraceStep};

