//! Axiom definitions and the Ω-SSOT (Omega Single Source of Truth)
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// A single axiom in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Axiom {
    /// Unique identifier for the axiom
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Formal statement of the axiom
    pub statement: String,
    /// Domain this axiom applies to
    pub domain: String,
    /// Hash of the axiom content for integrity verification
    pub hash: String,
}

impl Axiom {
    /// Create a new axiom
    pub fn new(id: impl Into<String>, name: impl Into<String>, statement: impl Into<String>, domain: impl Into<String>) -> Self {
        let id = id.into();
        let name = name.into();
        let statement = statement.into();
        let domain = domain.into();
        
        let hash = Self::compute_hash(&id, &name, &statement, &domain);
        
        Self {
            id,
            name,
            statement,
            domain,
            hash,
        }
    }
    
    fn compute_hash(id: &str, name: &str, statement: &str, domain: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update(name.as_bytes());
        hasher.update(statement.as_bytes());
        hasher.update(domain.as_bytes());
        hex::encode(hasher.finalize())
    }
    
    /// Verify the axiom's integrity
    pub fn verify_integrity(&self) -> bool {
        let computed = Self::compute_hash(&self.id, &self.name, &self.statement, &self.domain);
        computed == self.hash
    }
}

/// A collection of axioms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxiomSet {
    axioms: HashMap<String, Axiom>,
    /// Hash of the entire axiom set
    set_hash: String,
}

impl AxiomSet {
    /// Create a new empty axiom set
    pub fn new() -> Self {
        Self {
            axioms: HashMap::new(),
            set_hash: String::new(),
        }
    }
    
    /// Add an axiom to the set
    pub fn add(&mut self, axiom: Axiom) {
        self.axioms.insert(axiom.id.clone(), axiom);
        self.recompute_hash();
    }
    
    /// Get an axiom by ID
    pub fn get(&self, id: &str) -> Option<&Axiom> {
        self.axioms.get(id)
    }
    
    /// Check if an axiom exists
    pub fn contains(&self, id: &str) -> bool {
        self.axioms.contains_key(id)
    }
    
    /// Get all axioms
    pub fn all(&self) -> impl Iterator<Item = &Axiom> {
        self.axioms.values()
    }
    
    /// Number of axioms
    pub fn len(&self) -> usize {
        self.axioms.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.axioms.is_empty()
    }
    
    /// Get axioms by domain
    pub fn by_domain(&self, domain: &str) -> Vec<&Axiom> {
        self.axioms.values()
            .filter(|a| a.domain == domain)
            .collect()
    }
    
    fn recompute_hash(&mut self) {
        let mut hasher = Sha256::new();
        let mut ids: Vec<_> = self.axioms.keys().collect();
        ids.sort();
        
        for id in ids {
            if let Some(axiom) = self.axioms.get(id) {
                hasher.update(axiom.hash.as_bytes());
            }
        }
        
        self.set_hash = hex::encode(hasher.finalize());
    }
    
    /// Get the hash of the axiom set
    pub fn hash(&self) -> &str {
        &self.set_hash
    }
    
    /// Verify integrity of all axioms
    pub fn verify_integrity(&self) -> bool {
        self.axioms.values().all(|a| a.verify_integrity())
    }
}

impl Default for AxiomSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Ω-SSOT: The Single Source of Truth for the Axiom Hive system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmegaSSoT {
    /// Core axioms that define the system
    pub core_axioms: AxiomSet,
    /// Version of the Ω-SSOT
    pub version: String,
    /// Substrate authority
    pub substrate: String,
    /// Creation timestamp
    pub created_at: String,
    /// Hash of the entire Ω-SSOT
    pub omega_hash: String,
}

impl OmegaSSoT {
    /// Create a new Ω-SSOT with default core axioms
    pub fn new() -> Self {
        let mut ssot = Self {
            core_axioms: AxiomSet::new(),
            version: "1.0.0".to_string(),
            substrate: crate::SUBSTRATE.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            omega_hash: String::new(),
        };
        
        // Add fundamental axioms
        ssot.add_fundamental_axioms();
        ssot.recompute_hash();
        
        ssot
    }
    
    fn add_fundamental_axioms(&mut self) {
        // Axiom 1: Law of Identity
        self.core_axioms.add(Axiom::new(
            "A1_IDENTITY",
            "Law of Identity",
            "∀x: x = x",
            "logic"
        ));
        
        // Axiom 2: Law of Non-Contradiction
        self.core_axioms.add(Axiom::new(
            "A2_NON_CONTRADICTION",
            "Law of Non-Contradiction",
            "∀P: ¬(P ∧ ¬P)",
            "logic"
        ));
        
        // Axiom 3: Law of Excluded Middle
        self.core_axioms.add(Axiom::new(
            "A3_EXCLUDED_MIDDLE",
            "Law of Excluded Middle",
            "∀P: P ∨ ¬P",
            "logic"
        ));
        
        // Axiom 4: Substrate Authority
        self.core_axioms.add(Axiom::new(
            "A4_SUBSTRATE_AUTHORITY",
            "Substrate Authority",
            "All authority derives from the Substrate (Alexis Adams)",
            "governance"
        ));
        
        // Axiom 5: Deterministic Output
        self.core_axioms.add(Axiom::new(
            "A5_DETERMINISM",
            "Deterministic Output",
            "∀(input, state): output = f(input, state) is deterministic",
            "computation"
        ));
        
        // Axiom 6: C=0 Invariance
        self.core_axioms.add(Axiom::new(
            "A6_C_ZERO",
            "C=0 Invariance",
            "Contradiction measure C must equal zero for valid output",
            "verification"
        ));
        
        // Axiom 7: Causal Closure
        self.core_axioms.add(Axiom::new(
            "A7_CAUSAL_CLOSURE",
            "Causal Closure",
            "Every effect must have a traceable cause within the system",
            "causality"
        ));
        
        // Axiom 8: Binary Proof
        self.core_axioms.add(Axiom::new(
            "A8_BINARY_PROOF",
            "Binary Proof",
            "All proofs yield binary outcomes: Verified | Not Verified",
            "verification"
        ));
    }
    
    fn recompute_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.core_axioms.hash().as_bytes());
        hasher.update(self.version.as_bytes());
        hasher.update(self.substrate.as_bytes());
        hasher.update(self.created_at.as_bytes());
        self.omega_hash = hex::encode(hasher.finalize());
    }
    
    /// Get the Ω-SSOT hash
    pub fn hash(&self) -> &str {
        &self.omega_hash
    }
    
    /// Verify integrity of the entire Ω-SSOT
    pub fn verify_integrity(&self) -> bool {
        if !self.core_axioms.verify_integrity() {
            return false;
        }
        
        // Recompute and verify hash
        let mut hasher = Sha256::new();
        hasher.update(self.core_axioms.hash().as_bytes());
        hasher.update(self.version.as_bytes());
        hasher.update(self.substrate.as_bytes());
        hasher.update(self.created_at.as_bytes());
        let computed = hex::encode(hasher.finalize());
        
        computed == self.omega_hash
    }
    
    /// Check if a statement violates any core axiom
    pub fn check_violation(&self, statement: &str) -> Option<&Axiom> {
        // Check for explicit contradictions
        if statement.contains("P ∧ ¬P") || statement.contains("contradiction") {
            return self.core_axioms.get("A2_NON_CONTRADICTION");
        }
        
        None
    }
}

impl Default for OmegaSSoT {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_axiom_creation() {
        let axiom = Axiom::new("TEST", "Test Axiom", "x = x", "logic");
        assert_eq!(axiom.id, "TEST");
        assert!(axiom.verify_integrity());
    }
    
    #[test]
    fn test_axiom_integrity_check() {
        let mut axiom = Axiom::new("TEST", "Test Axiom", "x = x", "logic");
        assert!(axiom.verify_integrity());
        
        // Tamper with statement
        axiom.statement = "x != x".to_string();
        assert!(!axiom.verify_integrity());
    }
    
    #[test]
    fn test_axiom_set() {
        let mut set = AxiomSet::new();
        assert!(set.is_empty());
        
        set.add(Axiom::new("A1", "Axiom 1", "statement 1", "domain"));
        set.add(Axiom::new("A2", "Axiom 2", "statement 2", "domain"));
        
        assert_eq!(set.len(), 2);
        assert!(set.contains("A1"));
        assert!(set.verify_integrity());
    }
    
    #[test]
    fn test_omega_ssot_creation() {
        let ssot = OmegaSSoT::new();
        
        assert_eq!(ssot.substrate, crate::SUBSTRATE);
        assert!(!ssot.core_axioms.is_empty());
        assert!(ssot.verify_integrity());
    }
    
    #[test]
    fn test_omega_ssot_fundamental_axioms() {
        let ssot = OmegaSSoT::new();
        
        assert!(ssot.core_axioms.contains("A1_IDENTITY"));
        assert!(ssot.core_axioms.contains("A2_NON_CONTRADICTION"));
        assert!(ssot.core_axioms.contains("A6_C_ZERO"));
    }
}

