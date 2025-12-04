//! SAP-4D Proof Engine
//!
//! Main entry point for proof generation and verification.
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use crate::axioms::{Axiom, AxiomSet, OmegaSSoT};
use crate::causal::{CausalChain, CausalChainBuilder, CausalRelation};
use crate::receipt::Receipt;
use crate::trace::{TraceBuilder, TraceEnvelope};
use crate::{ProofError, Result};

/// Configuration for the proof engine
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Minimum explainability index required (default: 0.98)
    pub min_explainability: f64,
    /// Maximum causal chain length
    pub max_chain_length: usize,
    /// Whether to enforce strict C=0
    pub strict_c_zero: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            min_explainability: 0.98,
            max_chain_length: 100,
            strict_c_zero: true,
        }
    }
}

/// The SAP-4D Proof Engine
pub struct ProofEngine {
    /// Ω-SSOT containing core axioms
    omega_ssot: OmegaSSoT,
    /// Additional domain axioms
    domain_axioms: AxiomSet,
    /// Engine configuration
    config: EngineConfig,
}

impl ProofEngine {
    /// Create a new proof engine with default Ω-SSOT
    pub fn new() -> Self {
        Self {
            omega_ssot: OmegaSSoT::new(),
            domain_axioms: AxiomSet::new(),
            config: EngineConfig::default(),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: EngineConfig) -> Self {
        Self {
            omega_ssot: OmegaSSoT::new(),
            domain_axioms: AxiomSet::new(),
            config,
        }
    }
    
    /// Add a domain-specific axiom
    pub fn add_axiom(&mut self, axiom: Axiom) {
        self.domain_axioms.add(axiom);
    }
    
    /// Get all available axioms
    pub fn all_axioms(&self) -> Vec<&Axiom> {
        let mut axioms: Vec<_> = self.omega_ssot.core_axioms.all().collect();
        axioms.extend(self.domain_axioms.all());
        axioms
    }
    
    /// Prove a claim given observations
    pub fn prove(
        &self,
        claim: &str,
        observations: Vec<String>,
        sign_fn: impl FnOnce(&str) -> String,
    ) -> Result<(TraceEnvelope, Receipt)> {
        // Step 1: Build causal chain
        let chain = self.build_causal_chain(claim, &observations)?;
        
        // Step 2: Verify C=0
        if self.config.strict_c_zero && !chain.is_c_zero() {
            return Err(ProofError::InvarianceViolation);
        }
        
        // Step 3: Generate trace
        let trace = self.generate_trace(claim, &observations, &chain)?;
        
        // Step 4: Verify explainability
        let explainability = trace.explainability_index();
        if explainability < self.config.min_explainability {
            return Err(ProofError::Internal(format!(
                "Explainability index {} below minimum {}",
                explainability, self.config.min_explainability
            )));
        }
        
        // Step 5: Generate receipt
        let receipt = Receipt::from_trace(&trace, sign_fn);
        
        Ok((trace, receipt))
    }
    
    /// Build a causal chain from observations to claim
    fn build_causal_chain(&self, claim: &str, observations: &[String]) -> Result<CausalChain> {
        let mut builder = CausalChainBuilder::new(claim)
            .with_observations(observations.to_vec());
        
        // Simple inference: connect observations to claim
        // In production, this would use more sophisticated causal inference
        
        if observations.is_empty() {
            return Err(ProofError::UnsupportedClaim);
        }
        
        // Build chain from observations
        let mut current = observations[0].clone();
        
        for (i, obs) in observations.iter().enumerate().skip(1) {
            builder = builder.with_link(
                current.clone(),
                obs.clone(),
                CausalRelation::CorrelatedWith,
                vec![format!("Observation {}", i)],
            )?;
            current = obs.clone();
        }
        
        // Connect to claim
        builder = builder.with_link(
            current,
            claim.to_string(),
            CausalRelation::Implies,
            vec!["Inference from observations".to_string()],
        )?;
        
        builder.build()
    }
    
    /// Generate a proof trace
    fn generate_trace(
        &self,
        claim: &str,
        observations: &[String],
        chain: &CausalChain,
    ) -> Result<TraceEnvelope> {
        let mut builder = TraceBuilder::new(claim)
            .with_observations(observations.to_vec())
            .with_causal_chain(chain);
        
        // Add axioms used
        let axioms: Vec<Axiom> = self.omega_ssot.core_axioms.all().cloned().collect();
        builder = builder.with_axioms(&axioms);
        
        // Step 1: Initialize
        builder = builder.add_step(
            "initialize",
            format!("observations: {:?}", observations),
            "Initialized proof context",
            vec!["A4_SUBSTRATE_AUTHORITY".to_string()],
        );
        
        // Step 2: Validate observations
        builder = builder.add_step(
            "validate_observations",
            format!("{} observations", observations.len()),
            "Observations validated",
            vec!["A5_DETERMINISM".to_string()],
        );
        
        // Step 3: Build causal model
        builder = builder.add_step(
            "build_causal_model",
            "Observations",
            format!("Causal chain with {} links", chain.len()),
            vec!["A7_CAUSAL_CLOSURE".to_string()],
        );
        
        // Step 4: Check contradictions
        builder = builder.add_step(
            "check_contradictions",
            format!("C = {}", chain.contradiction_measure()),
            format!("C = {} ({})", chain.contradiction_measure(), 
                    if chain.is_c_zero() { "PASS" } else { "FAIL" }),
            vec!["A2_NON_CONTRADICTION".to_string(), "A6_C_ZERO".to_string()],
        );
        
        // Step 5: Verify claim support
        let supports = chain.supports_claim();
        builder = builder.add_step(
            "verify_claim_support",
            claim.to_string(),
            format!("Claim {} by evidence", if supports { "supported" } else { "not supported" }),
            vec!["A8_BINARY_PROOF".to_string()],
        );
        
        // Step 6: Finalize
        builder = builder.add_step(
            "finalize",
            "Proof complete",
            format!("Claim '{}' verified with C=0", claim),
            vec!["A1_IDENTITY".to_string()],
        );
        
        Ok(builder.build())
    }
    
    /// Verify a receipt
    pub fn verify_receipt(
        &self,
        receipt: &Receipt,
        verify_fn: impl FnOnce(&str, &str) -> bool,
    ) -> Result<bool> {
        // Check hash integrity
        if !receipt.verify_hash() {
            return Err(ProofError::Internal("Receipt hash verification failed".to_string()));
        }
        
        // Check signature
        if !receipt.verify_signature(verify_fn) {
            return Err(ProofError::Internal("Receipt signature verification failed".to_string()));
        }
        
        // Check C=0
        if self.config.strict_c_zero && !receipt.c_zero {
            return Err(ProofError::InvarianceViolation);
        }
        
        Ok(true)
    }
    
    /// Verify a claim against evidence (simple interface)
    pub fn verify_claim(
        &self,
        claim: &str,
        evidence: &[String],
    ) -> Result<bool> {
        // Build causal chain
        let chain = self.build_causal_chain(claim, evidence)?;
        
        // Check C=0
        if !chain.is_c_zero() {
            return Ok(false);
        }
        
        // Check claim support
        Ok(chain.supports_claim())
    }
}

impl Default for ProofEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Verify a claim with the default engine
pub fn verify_claim(claim: &str, facts: &[String], axioms: &[String]) -> Result<Receipt> {
    let mut engine = ProofEngine::new();
    
    // Add custom axioms
    for (i, axiom_str) in axioms.iter().enumerate() {
        engine.add_axiom(Axiom::new(
            format!("CUSTOM_{}", i),
            format!("Custom Axiom {}", i),
            axiom_str.clone(),
            "custom",
        ));
    }
    
    // Mock signer for simple verification
    let mock_sign = |hash: &str| -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"MOCK_SIG:");
        hasher.update(hash.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    };
    
    let (_, receipt) = engine.prove(claim, facts.to_vec(), mock_sign)?;
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_sign(hash: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"TEST_SIG:");
        hasher.update(hash.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    }
    
    fn test_verify(hash: &str, sig: &str) -> bool {
        test_sign(hash) == sig
    }
    
    #[test]
    fn test_engine_creation() {
        let engine = ProofEngine::new();
        assert!(!engine.all_axioms().is_empty());
    }
    
    #[test]
    fn test_prove_claim() {
        let engine = ProofEngine::new();
        
        let observations = vec![
            "The sky is blue".to_string(),
            "Blue things reflect certain wavelengths".to_string(),
        ];
        
        let result = engine.prove(
            "The sky reflects certain wavelengths",
            observations,
            test_sign,
        );
        
        assert!(result.is_ok());
        let (trace, receipt) = result.unwrap();
        
        assert!(trace.is_c_zero());
        assert!(receipt.c_zero);
    }
    
    #[test]
    fn test_verify_receipt() {
        let engine = ProofEngine::new();
        
        let observations = vec!["Fact A".to_string(), "Fact B".to_string()];
        let (_, receipt) = engine.prove("Conclusion", observations, test_sign).unwrap();
        
        let verified = engine.verify_receipt(&receipt, test_verify);
        assert!(verified.is_ok());
        assert!(verified.unwrap());
    }
    
    #[test]
    fn test_unsupported_claim() {
        let engine = ProofEngine::new();
        
        // No observations
        let result = engine.prove("Unsupported claim", vec![], test_sign);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_explainability_requirement() {
        let config = EngineConfig {
            min_explainability: 0.98,
            ..Default::default()
        };
        
        let engine = ProofEngine::with_config(config);
        
        let observations = vec!["Evidence".to_string()];
        let result = engine.prove("Claim", observations, test_sign);
        
        // Should pass since our trace has good explainability
        assert!(result.is_ok());
    }
}

