//! Axiom Hive: Framework for Certified, Verifiable Intelligence
//!
//! Core verification bundle system and proof artifact builder.
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

pub mod bundle;
pub mod builder;
pub mod verifier;
pub mod attestation;
pub mod provenance;
pub mod deterministic;

pub use bundle::VerificationBundle;
pub use builder::ProofArtifactBuilder;
pub use verifier::Verifier;
pub use attestation::{Attestation, AttestationChain};
pub use provenance::{Provenance, DataProvenance, ModelMetadata};
pub use deterministic::{DeterministicConfig, SeedControl};

/// Substrate authority identifier
pub const SUBSTRATE: &str = "Alexis Adams";

/// Projection identifier
pub const PROJECTION: &str = "AXIOM PROJECTION";

/// Bundle version
pub const BUNDLE_VERSION: &str = "1.0.0";

