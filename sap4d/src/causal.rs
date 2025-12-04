//! Causal inference and chain construction
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{ProofError, Result};

/// Types of causal relationships
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalRelation {
    /// A causes B (A → B)
    Causes,
    /// A is caused by B (A ← B)
    CausedBy,
    /// A is correlated with B (A ~ B)
    CorrelatedWith,
    /// A implies B (A ⟹ B)
    Implies,
    /// A is equivalent to B (A ⟺ B)
    Equivalent,
    /// A contradicts B (A ⊥ B)
    Contradicts,
}

/// A single link in a causal chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    /// Source fact/observation
    pub source: String,
    /// Target fact/claim
    pub target: String,
    /// Type of causal relationship
    pub relation: CausalRelation,
    /// Confidence level (must be 1.0 for production)
    pub confidence: f64,
    /// Supporting evidence
    pub evidence: Vec<String>,
    /// Hash of this link
    pub hash: String,
}

impl CausalLink {
    /// Create a new causal link
    pub fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        relation: CausalRelation,
        evidence: Vec<String>,
    ) -> Self {
        let source = source.into();
        let target = target.into();
        
        let hash = Self::compute_hash(&source, &target, &relation, &evidence);
        
        Self {
            source,
            target,
            relation,
            confidence: 1.0, // Production requires 1.0
            evidence,
            hash,
        }
    }
    
    fn compute_hash(
        source: &str,
        target: &str,
        relation: &CausalRelation,
        evidence: &[String],
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(source.as_bytes());
        hasher.update(target.as_bytes());
        hasher.update(format!("{:?}", relation).as_bytes());
        for e in evidence {
            hasher.update(e.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
    
    /// Verify the link's integrity
    pub fn verify_integrity(&self) -> bool {
        let computed = Self::compute_hash(&self.source, &self.target, &self.relation, &self.evidence);
        computed == self.hash
    }
    
    /// Check if this link represents a contradiction
    pub fn is_contradiction(&self) -> bool {
        self.relation == CausalRelation::Contradicts
    }
}

/// A complete causal chain from observations to claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalChain {
    /// The claim being proven
    pub claim: String,
    /// Ordered list of causal links
    pub links: Vec<CausalLink>,
    /// Root observations/facts
    pub observations: Vec<String>,
    /// Whether the chain is valid (no contradictions)
    pub is_valid: bool,
    /// Hash of the entire chain
    pub chain_hash: String,
}

impl CausalChain {
    /// Create a new empty causal chain
    pub fn new(claim: impl Into<String>, observations: Vec<String>) -> Self {
        let claim = claim.into();
        let chain_hash = Self::compute_base_hash(&claim, &observations);
        
        Self {
            claim,
            links: Vec::new(),
            observations,
            is_valid: true,
            chain_hash,
        }
    }
    
    fn compute_base_hash(claim: &str, observations: &[String]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(claim.as_bytes());
        for obs in observations {
            hasher.update(obs.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
    
    /// Add a link to the chain
    pub fn add_link(&mut self, link: CausalLink) -> Result<()> {
        // Check for contradictions
        if link.is_contradiction() {
            self.is_valid = false;
            return Err(ProofError::Contradiction(format!(
                "Contradiction between '{}' and '{}'",
                link.source, link.target
            )));
        }
        
        // Check that the link connects to existing chain
        if !self.links.is_empty() {
            let connects = self.links.iter().any(|l| {
                l.target == link.source || l.source == link.source
            }) || self.observations.contains(&link.source);
            
            if !connects {
                return Err(ProofError::CausalBreak {
                    step: self.links.len(),
                    reason: format!("Link source '{}' not connected to chain", link.source),
                });
            }
        }
        
        self.links.push(link);
        self.recompute_hash();
        Ok(())
    }
    
    fn recompute_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.claim.as_bytes());
        for obs in &self.observations {
            hasher.update(obs.as_bytes());
        }
        for link in &self.links {
            hasher.update(link.hash.as_bytes());
        }
        self.chain_hash = hex::encode(hasher.finalize());
    }
    
    /// Check if the chain supports the claim
    pub fn supports_claim(&self) -> bool {
        if !self.is_valid || self.links.is_empty() {
            return false;
        }
        
        // Check that final link targets or relates to the claim
        self.links.iter().any(|l| {
            l.target.contains(&self.claim) || self.claim.contains(&l.target)
        })
    }
    
    /// Get the contradiction measure (C)
    pub fn contradiction_measure(&self) -> u32 {
        self.links.iter().filter(|l| l.is_contradiction()).count() as u32
    }
    
    /// Check C=0 compliance
    pub fn is_c_zero(&self) -> bool {
        self.contradiction_measure() == 0 && self.is_valid
    }
    
    /// Verify the integrity of the entire chain
    pub fn verify_integrity(&self) -> bool {
        // Verify all links
        if !self.links.iter().all(|l| l.verify_integrity()) {
            return false;
        }
        
        // Verify chain hash
        let mut hasher = Sha256::new();
        hasher.update(self.claim.as_bytes());
        for obs in &self.observations {
            hasher.update(obs.as_bytes());
        }
        for link in &self.links {
            hasher.update(link.hash.as_bytes());
        }
        let computed = hex::encode(hasher.finalize());
        
        computed == self.chain_hash
    }
    
    /// Get the chain length
    pub fn len(&self) -> usize {
        self.links.len()
    }
    
    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }
    
    /// Convert chain to string representation
    pub fn to_string_chain(&self) -> Vec<String> {
        self.links.iter().map(|l| {
            let rel = match l.relation {
                CausalRelation::Causes => "→",
                CausalRelation::CausedBy => "←",
                CausalRelation::CorrelatedWith => "~",
                CausalRelation::Implies => "⟹",
                CausalRelation::Equivalent => "⟺",
                CausalRelation::Contradicts => "⊥",
            };
            format!("{} {} {}", l.source, rel, l.target)
        }).collect()
    }
}

/// Builder for constructing causal chains
pub struct CausalChainBuilder {
    chain: CausalChain,
}

impl CausalChainBuilder {
    /// Create a new builder
    pub fn new(claim: impl Into<String>) -> Self {
        Self {
            chain: CausalChain::new(claim, Vec::new()),
        }
    }
    
    /// Add an observation
    pub fn with_observation(mut self, obs: impl Into<String>) -> Self {
        self.chain.observations.push(obs.into());
        self
    }
    
    /// Add multiple observations
    pub fn with_observations(mut self, obs: Vec<String>) -> Self {
        self.chain.observations.extend(obs);
        self
    }
    
    /// Add a causal link
    pub fn with_link(
        mut self,
        source: impl Into<String>,
        target: impl Into<String>,
        relation: CausalRelation,
        evidence: Vec<String>,
    ) -> Result<Self> {
        let link = CausalLink::new(source, target, relation, evidence);
        self.chain.add_link(link)?;
        Ok(self)
    }
    
    /// Build the chain
    pub fn build(mut self) -> Result<CausalChain> {
        self.chain.recompute_hash();
        
        if !self.chain.is_c_zero() {
            return Err(ProofError::InvarianceViolation);
        }
        
        Ok(self.chain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_causal_link_creation() {
        let link = CausalLink::new(
            "observation A",
            "conclusion B",
            CausalRelation::Causes,
            vec!["evidence 1".to_string()],
        );
        
        assert_eq!(link.confidence, 1.0);
        assert!(link.verify_integrity());
    }
    
    #[test]
    fn test_causal_chain_creation() {
        let chain = CausalChain::new(
            "The conclusion",
            vec!["fact 1".to_string(), "fact 2".to_string()],
        );
        
        assert!(chain.is_valid);
        assert!(chain.is_c_zero());
        assert!(chain.is_empty());
    }
    
    #[test]
    fn test_chain_builder() {
        let chain = CausalChainBuilder::new("conclusion")
            .with_observation("fact A")
            .with_observation("fact B")
            .with_link(
                "fact A",
                "intermediate",
                CausalRelation::Implies,
                vec!["evidence".to_string()],
            )
            .unwrap()
            .with_link(
                "intermediate",
                "conclusion",
                CausalRelation::Implies,
                vec!["more evidence".to_string()],
            )
            .unwrap()
            .build()
            .unwrap();
        
        assert!(chain.is_c_zero());
        assert_eq!(chain.len(), 2);
    }
    
    #[test]
    fn test_contradiction_detection() {
        let link = CausalLink::new(
            "P",
            "not P",
            CausalRelation::Contradicts,
            vec![],
        );
        
        assert!(link.is_contradiction());
    }
}

