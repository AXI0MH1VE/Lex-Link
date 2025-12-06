//! Trace generation for proof steps
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};

use crate::axioms::Axiom;
use crate::causal::CausalChain;

/// A single step in a proof trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceStep {
    /// Step index (0-based)
    pub index: usize,
    /// Operation performed
    pub operation: String,
    /// Input to this step
    pub input: String,
    /// Output from this step
    pub output: String,
    /// Axioms applied
    pub axioms_applied: Vec<String>,
    /// Hash of this step
    pub step_hash: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl TraceStep {
    /// Create a new trace step
    pub fn new(
        index: usize,
        operation: impl Into<String>,
        input: impl Into<String>,
        output: impl Into<String>,
        axioms_applied: Vec<String>,
    ) -> Self {
        let operation = operation.into();
        let input = input.into();
        let output = output.into();
        let timestamp = Utc::now();
        
        let step_hash = Self::compute_hash(index, &operation, &input, &output, &axioms_applied);
        
        Self {
            index,
            operation,
            input,
            output,
            axioms_applied,
            step_hash,
            timestamp,
        }
    }
    
    fn compute_hash(
        index: usize,
        operation: &str,
        input: &str,
        output: &str,
        axioms: &[String],
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_le_bytes());
        hasher.update(operation.as_bytes());
        hasher.update(input.as_bytes());
        hasher.update(output.as_bytes());
        for axiom in axioms {
            hasher.update(axiom.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
    
    /// Verify the step's integrity
    pub fn verify_integrity(&self) -> bool {
        let computed = Self::compute_hash(
            self.index,
            &self.operation,
            &self.input,
            &self.output,
            &self.axioms_applied,
        );
        computed == self.step_hash
    }
}

/// Complete trace envelope containing all proof steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEnvelope {
    /// The claim being proven
    pub claim: String,
    /// Initial observations/facts
    pub observations: Vec<String>,
    /// The causal chain
    pub causal_chain: Vec<String>,
    /// All axioms referenced
    pub axioms: Vec<String>,
    /// Ordered proof steps
    pub steps: Vec<TraceStep>,
    /// Whether contradiction check passed (C=0)
    pub contradiction_check: bool,
    /// Hash of the entire trace
    pub receipt_hash: String,
    /// Timestamp of trace creation
    pub created_at: DateTime<Utc>,
    /// Substrate authority
    pub substrate: String,
    /// Projection identifier
    pub projection: String,
}

impl TraceEnvelope {
    /// Create a new trace envelope
    pub fn new(claim: impl Into<String>, observations: Vec<String>) -> Self {
        let claim = claim.into();
        let created_at = Utc::now();
        
        Self {
            claim: claim.clone(),
            observations,
            causal_chain: Vec::new(),
            axioms: Vec::new(),
            steps: Vec::new(),
            contradiction_check: true,
            receipt_hash: String::new(),
            created_at,
            substrate: crate::SUBSTRATE.to_string(),
            projection: crate::PROJECTION.to_string(),
        }
    }
    
    /// Add a trace step
    pub fn add_step(&mut self, step: TraceStep) {
        self.steps.push(step);
    }
    
    /// Add a causal chain
    pub fn set_causal_chain(&mut self, chain: &CausalChain) {
        self.causal_chain = chain.to_string_chain();
        self.contradiction_check = chain.is_c_zero();
    }
    
    /// Add axioms
    pub fn add_axioms(&mut self, axioms: &[Axiom]) {
        self.axioms = axioms.iter().map(|a| a.id.clone()).collect();
    }
    
    /// Finalize the trace and compute hash
    pub fn finalize(&mut self) {
        let mut hasher = Sha256::new();
        
        hasher.update(self.claim.as_bytes());
        
        for obs in &self.observations {
            hasher.update(obs.as_bytes());
        }
        
        for link in &self.causal_chain {
            hasher.update(link.as_bytes());
        }
        
        for axiom in &self.axioms {
            hasher.update(axiom.as_bytes());
        }
        
        for step in &self.steps {
            hasher.update(step.step_hash.as_bytes());
        }
        
        hasher.update([self.contradiction_check as u8]);
        hasher.update(self.created_at.to_rfc3339().as_bytes());
        hasher.update(self.substrate.as_bytes());
        hasher.update(self.projection.as_bytes());
        
        self.receipt_hash = hex::encode(hasher.finalize());
    }
    
    /// Verify the trace's integrity
    pub fn verify_integrity(&self) -> bool {
        // Verify all steps
        if !self.steps.iter().all(|s| s.verify_integrity()) {
            return false;
        }
        
        // Recompute and verify hash
        let mut hasher = Sha256::new();
        
        hasher.update(self.claim.as_bytes());
        
        for obs in &self.observations {
            hasher.update(obs.as_bytes());
        }
        
        for link in &self.causal_chain {
            hasher.update(link.as_bytes());
        }
        
        for axiom in &self.axioms {
            hasher.update(axiom.as_bytes());
        }
        
        for step in &self.steps {
            hasher.update(step.step_hash.as_bytes());
        }
        
        hasher.update([self.contradiction_check as u8]);
        hasher.update(self.created_at.to_rfc3339().as_bytes());
        hasher.update(self.substrate.as_bytes());
        hasher.update(self.projection.as_bytes());
        
        let computed = hex::encode(hasher.finalize());
        computed == self.receipt_hash
    }
    
    /// Check if trace is C=0 compliant
    pub fn is_c_zero(&self) -> bool {
        self.contradiction_check
    }
    
    /// Get the explainability index (ratio of explained steps)
    pub fn explainability_index(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        
        let explained = self.steps.iter()
            .filter(|s| !s.axioms_applied.is_empty())
            .count();
        
        explained as f64 / self.steps.len() as f64
    }
    
    /// Convert to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Builder for constructing trace envelopes
pub struct TraceBuilder {
    envelope: TraceEnvelope,
    step_counter: usize,
}

impl TraceBuilder {
    /// Create a new builder
    pub fn new(claim: impl Into<String>) -> Self {
        Self {
            envelope: TraceEnvelope::new(claim, Vec::new()),
            step_counter: 0,
        }
    }
    
    /// Add an observation
    pub fn with_observation(mut self, obs: impl Into<String>) -> Self {
        self.envelope.observations.push(obs.into());
        self
    }
    
    /// Add multiple observations
    pub fn with_observations(mut self, obs: Vec<String>) -> Self {
        self.envelope.observations.extend(obs);
        self
    }
    
    /// Add a step
    pub fn add_step(
        mut self,
        operation: impl Into<String>,
        input: impl Into<String>,
        output: impl Into<String>,
        axioms: Vec<String>,
    ) -> Self {
        let step = TraceStep::new(
            self.step_counter,
            operation,
            input,
            output,
            axioms,
        );
        self.envelope.add_step(step);
        self.step_counter += 1;
        self
    }
    
    /// Set the causal chain
    pub fn with_causal_chain(mut self, chain: &CausalChain) -> Self {
        self.envelope.set_causal_chain(chain);
        self
    }
    
    /// Add axioms
    pub fn with_axioms(mut self, axioms: &[Axiom]) -> Self {
        self.envelope.add_axioms(axioms);
        self
    }
    
    /// Build and finalize the trace
    pub fn build(mut self) -> TraceEnvelope {
        self.envelope.finalize();
        self.envelope
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trace_step_creation() {
        let step = TraceStep::new(
            0,
            "inference",
            "premise A",
            "conclusion B",
            vec!["A1_IDENTITY".to_string()],
        );
        
        assert_eq!(step.index, 0);
        assert!(step.verify_integrity());
    }
    
    #[test]
    fn test_trace_envelope_creation() {
        let envelope = TraceEnvelope::new(
            "test claim",
            vec!["observation 1".to_string()],
        );
        
        assert!(envelope.is_c_zero());
        assert_eq!(envelope.substrate, crate::SUBSTRATE);
    }
    
    #[test]
    fn test_trace_builder() {
        let trace = TraceBuilder::new("conclusion")
            .with_observation("fact A")
            .with_observation("fact B")
            .add_step(
                "analyze",
                "fact A",
                "intermediate",
                vec!["A1_IDENTITY".to_string()],
            )
            .add_step(
                "deduce",
                "intermediate",
                "conclusion",
                vec!["A2_NON_CONTRADICTION".to_string()],
            )
            .build();
        
        assert!(trace.verify_integrity());
        assert_eq!(trace.steps.len(), 2);
        assert!(trace.explainability_index() > 0.0);
    }
    
    #[test]
    fn test_explainability_index() {
        let trace = TraceBuilder::new("claim")
            .add_step("op1", "in", "out", vec!["axiom".to_string()])
            .add_step("op2", "in", "out", vec![]) // No axiom
            .build();
        
        assert_eq!(trace.explainability_index(), 0.5);
    }
}

