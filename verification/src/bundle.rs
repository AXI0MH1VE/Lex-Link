//! Verification Bundle - Core artifact for verifiable intelligence
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::provenance::Provenance;
use crate::attestation::Attestation;

/// Verification Bundle - Enables independent replay and validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationBundle {
    /// Bundle schema version
    #[serde(rename = "bundle_version")]
    pub bundle_version: String,
    
    /// Content address (hash of bundle contents)
    #[serde(rename = "content_address")]
    pub content_address: String,
    
    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    
    /// Cryptographic signatures from signers
    pub signatures: Vec<Attestation>,
    
    /// Provenance information
    pub provenance: Provenance,
    
    /// Execution trace (optional intermediates)
    #[serde(rename = "execution_trace")]
    pub execution_trace: Option<ExecutionTrace>,
    
    /// Verification tests
    pub tests: Vec<VerificationTest>,
    
    /// Output artifacts
    pub outputs: Vec<OutputArtifact>,
}

/// Execution trace with intermediate steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrace {
    /// Execution steps
    pub steps: Vec<ExecutionStep>,
    
    /// Intermediate artifacts (optional)
    pub artifacts: Vec<TraceArtifact>,
}

/// Single execution step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// Step name
    pub name: String,
    
    /// Hash of step output
    pub hash: String,
    
    /// Timestamp
    pub timestamp: Option<DateTime<Utc>>,
}

/// Trace artifact (intermediate computation result)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceArtifact {
    /// Artifact name
    pub name: String,
    
    /// Content hash
    pub hash: String,
    
    /// Content address URI
    pub uri: Option<String>,
    
    /// Whether this artifact is optional for verification
    pub optional: bool,
}

/// Verification test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTest {
    /// Test name
    pub name: String,
    
    /// Test type
    #[serde(rename = "type")]
    pub test_type: TestType,
    
    /// Expected output hash
    #[serde(rename = "expected_output_hash")]
    pub expected_output_hash: String,
    
    /// Tolerance specification
    pub tolerance: Tolerance,
}

/// Test type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestType {
    /// Exact replay test
    Replay,
    /// Determinism check
    Determinism,
    /// Invariant check
    Invariant,
    /// Numerical stability
    Stability,
}

/// Tolerance specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Tolerance {
    /// Exact match required
    #[serde(rename = "exact")]
    Exact,
    
    /// Floating point tolerance
    #[serde(rename = "float")]
    Float {
        /// Relative tolerance
        relative: f64,
        /// Absolute tolerance
        absolute: f64,
    },
    
    /// Hash-based (content address match)
    #[serde(rename = "hash")]
    Hash,
}

/// Output artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputArtifact {
    /// Artifact name
    pub name: String,
    
    /// Content hash
    pub hash: String,
    
    /// Content address URI
    pub uri: String,
    
    /// MIME type
    #[serde(rename = "mime_type")]
    pub mime_type: Option<String>,
}

impl VerificationBundle {
    /// Compute content address from bundle contents
    pub fn compute_content_address(&self) -> String {
        use sha2::{Digest, Sha256};
        use serde_json;
        
        let mut hasher = Sha256::new();
        
        // Hash critical fields
        hasher.update(self.bundle_version.as_bytes());
        hasher.update(self.created_at.to_rfc3339().as_bytes());
        
        // Hash provenance
        if let Ok(prov_json) = serde_json::to_string(&self.provenance) {
            hasher.update(prov_json.as_bytes());
        }
        
        // Hash outputs
        for output in &self.outputs {
            hasher.update(output.hash.as_bytes());
        }
        
        // Hash signatures (without signature values themselves)
        for sig in &self.signatures {
            hasher.update(sig.signer_id.as_bytes());
            hasher.update(sig.timestamp.to_rfc3339().as_bytes());
        }
        
        format!("hash://sha256/{}", hex::encode(hasher.finalize()))
    }
    
    /// Verify bundle integrity
    pub fn verify_integrity(&self) -> bool {
        self.content_address == self.compute_content_address()
    }
    
    /// Get bundle as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Parse bundle from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::{ModelMetadata, EnvironmentManifest};
    
    #[test]
    fn test_bundle_content_address() {
        let bundle = VerificationBundle {
            bundle_version: "1.0.0".to_string(),
            content_address: String::new(), // Will be computed
            created_at: Utc::now(),
            signatures: vec![],
            provenance: Provenance {
                inputs: vec![],
                model: ModelMetadata {
                    name: "test-model".to_string(),
                    version: "1.0.0".to_string(),
                    weights_hash: "sha256:abc".to_string(),
                    tokenizer_hash: "sha256:def".to_string(),
                    card_uri: None,
                },
                environment: EnvironmentManifest {
                    container_image_hash: "sha256:xyz".to_string(),
                    os: "ubuntu:22.04".to_string(),
                    deps: vec![],
                    hardware: None,
                },
                config: crate::deterministic::DeterministicConfig {
                    seed: 42,
                    parameters: Default::default(),
                },
            },
            execution_trace: None,
            tests: vec![],
            outputs: vec![],
        };
        
        let addr = bundle.compute_content_address();
        assert!(!addr.is_empty());
        assert!(addr.starts_with("hash://sha256/"));
    }
}

