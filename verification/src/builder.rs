//! Proof Artifact Builder - Constructs verification bundles
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use chrono::Utc;
use crate::{
    bundle::{VerificationBundle, ExecutionTrace, ExecutionStep, TraceArtifact, VerificationTest, TestType, Tolerance, OutputArtifact},
    provenance::{Provenance, DataProvenance, ModelMetadata, EnvironmentManifest},
    attestation::Attestation,
    deterministic::DeterministicConfig,
    BUNDLE_VERSION,
};

/// Builder for constructing verification bundles
pub struct ProofArtifactBuilder {
    inputs: Vec<DataProvenance>,
    model: Option<ModelMetadata>,
    environment: Option<EnvironmentManifest>,
    config: Option<DeterministicConfig>,
    execution_steps: Vec<ExecutionStep>,
    trace_artifacts: Vec<TraceArtifact>,
    tests: Vec<VerificationTest>,
    outputs: Vec<OutputArtifact>,
    signatures: Vec<Attestation>,
}

impl ProofArtifactBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            model: None,
            environment: None,
            config: None,
            execution_steps: Vec::new(),
            trace_artifacts: Vec::new(),
            tests: Vec::new(),
            outputs: Vec::new(),
            signatures: Vec::new(),
        }
    }
    
    /// Add input data provenance
    pub fn with_input(mut self, input: DataProvenance) -> Self {
        self.inputs.push(input);
        self
    }
    
    /// Set model metadata
    pub fn with_model(mut self, model: ModelMetadata) -> Self {
        self.model = Some(model);
        self
    }
    
    /// Set environment manifest
    pub fn with_environment(mut self, env: EnvironmentManifest) -> Self {
        self.environment = Some(env);
        self
    }
    
    /// Set deterministic configuration
    pub fn with_config(mut self, config: DeterministicConfig) -> Self {
        self.config = Some(config);
        self
    }
    
    /// Add execution step
    pub fn add_execution_step(mut self, name: impl Into<String>, hash: impl Into<String>) -> Self {
        self.execution_steps.push(ExecutionStep {
            name: name.into(),
            hash: hash.into(),
            timestamp: Some(Utc::now()),
        });
        self
    }
    
    /// Add trace artifact
    pub fn add_trace_artifact(
        mut self,
        name: impl Into<String>,
        hash: impl Into<String>,
        optional: bool,
    ) -> Self {
        self.trace_artifacts.push(TraceArtifact {
            name: name.into(),
            hash: hash.into(),
            uri: None,
            optional,
        });
        self
    }
    
    /// Add verification test
    pub fn add_test(
        mut self,
        name: impl Into<String>,
        test_type: TestType,
        expected_hash: impl Into<String>,
        tolerance: Tolerance,
    ) -> Self {
        self.tests.push(VerificationTest {
            name: name.into(),
            test_type,
            expected_output_hash: expected_hash.into(),
            tolerance,
        });
        self
    }
    
    /// Add output artifact
    pub fn add_output(
        mut self,
        name: impl Into<String>,
        hash: impl Into<String>,
        uri: impl Into<String>,
    ) -> Self {
        self.outputs.push(OutputArtifact {
            name: name.into(),
            hash: hash.into(),
            uri: uri.into(),
            mime_type: None,
        });
        self
    }
    
    /// Add attestation/signature
    pub fn add_signature(mut self, signature: Attestation) -> Self {
        self.signatures.push(signature);
        self
    }
    
    /// Build the verification bundle
    pub fn build(self) -> Result<VerificationBundle, BuilderError> {
        // Validate required fields
        let model = self.model.ok_or(BuilderError::MissingModel)?;
        let environment = self.environment.ok_or(BuilderError::MissingEnvironment)?;
        let config = self.config.ok_or(BuilderError::MissingConfig)?;
        
        let provenance = Provenance {
            inputs: self.inputs,
            model,
            environment,
            config,
        };
        
        let execution_trace = if self.execution_steps.is_empty() && self.trace_artifacts.is_empty() {
            None
        } else {
            Some(ExecutionTrace {
                steps: self.execution_steps,
                artifacts: self.trace_artifacts,
            })
        };
        
        let created_at = Utc::now();
        
        // Create bundle
        let mut bundle = VerificationBundle {
            bundle_version: BUNDLE_VERSION.to_string(),
            content_address: String::new(), // Will be computed
            created_at,
            signatures: self.signatures,
            provenance,
            execution_trace,
            tests: self.tests,
            outputs: self.outputs,
        };
        
        // Compute content address
        bundle.content_address = bundle.compute_content_address();
        
        Ok(bundle)
    }
}

impl Default for ProofArtifactBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder errors
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("Model metadata is required")]
    MissingModel,
    
    #[error("Environment manifest is required")]
    MissingEnvironment,
    
    #[error("Deterministic configuration is required")]
    MissingConfig,
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::{ModelMetadata, EnvironmentManifest, Dependency};
    
    #[test]
    fn test_builder() {
        let model = ModelMetadata {
            name: "test-model".to_string(),
            version: "1.0.0".to_string(),
            weights_hash: "sha256:abc".to_string(),
            tokenizer_hash: "sha256:def".to_string(),
            card_uri: None,
        };
        
        let env = EnvironmentManifest {
            container_image_hash: "sha256:xyz".to_string(),
            os: "ubuntu:22.04".to_string(),
            deps: vec![Dependency {
                name: "torch".to_string(),
                version: "2.4.0".to_string(),
                hash: "sha256:torch".to_string(),
            }],
            hardware: None,
        };
        
        let config = DeterministicConfig {
            seed: 42,
            parameters: Default::default(),
        };
        
        let bundle = ProofArtifactBuilder::new()
            .with_model(model)
            .with_environment(env)
            .with_config(config)
            .add_test(
                "determinism_check",
                TestType::Determinism,
                "sha256:expected",
                Tolerance::Exact,
            )
            .add_output("result", "sha256:result", "hash://sha256/result")
            .build()
            .unwrap();
        
        assert_eq!(bundle.bundle_version, BUNDLE_VERSION);
        assert!(!bundle.content_address.is_empty());
    }
}

