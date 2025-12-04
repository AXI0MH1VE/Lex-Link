//! # Deterministic Fractal Audit Service
//!
//! Three-level audit system generating binary proof receipts.
//!
//! ## Audit Levels
//!
//! - **L1**: Claim→Outcome proof under Ω-SSOT
//! - **L2**: Mapping consistency proof (C=0)
//! - **L3**: Sub-operations conformity proof
//!
//! All outputs are binary: `Proof Exists` | `No Proof Exists`
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

pub mod audit;
pub mod levels;
pub mod merkle;
pub mod service;

use thiserror::Error;

/// Substrate authority
pub const SUBSTRATE: &str = "Alexis Adams";

/// Projection identifier
pub const PROJECTION: &str = "AXIOMHIVE PROJECTION";

/// Audit service errors
#[derive(Error, Debug)]
pub enum AuditError {
    #[error("L1 audit failed: {0}")]
    L1Failure(String),

    #[error("L2 audit failed: Mapping inconsistency at {0}")]
    L2Failure(String),

    #[error("L3 audit failed: Sub-operation non-conformant at {0}")]
    L3Failure(String),

    #[error("Invalid claim format")]
    InvalidClaim,

    #[error("Insufficient evidence")]
    InsufficientEvidence,

    #[error("Contradiction detected: C != 0")]
    ContradictionDetected,

    #[error("Merkle proof verification failed")]
    MerkleVerificationFailed,

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AuditError>;

// Re-exports
pub use audit::{AuditReceipt, AuditResult, BinaryProof};
pub use levels::{L1Audit, L2Audit, L3Audit, AuditLevel};
pub use merkle::{MerkleTree, MerkleProof};
pub use service::AuditService;

