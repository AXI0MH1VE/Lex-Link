//! Audit level implementations (L1, L2, L3)
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use sap4d::{ProofEngine, OmegaSSoT};

use crate::audit::{AuditResult, BinaryProof};
use crate::Result;

/// Audit level identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditLevel {
    /// L1: Claim→Outcome proof under Ω-SSOT
    L1,
    /// L2: Mapping consistency proof (C=0)
    L2,
    /// L3: Sub-operations conformity proof
    L3,
}

impl AuditLevel {
    /// Get level number
    pub fn number(&self) -> u8 {
        match self {
            AuditLevel::L1 => 1,
            AuditLevel::L2 => 2,
            AuditLevel::L3 => 3,
        }
    }
    
    /// Get level description
    pub fn description(&self) -> &'static str {
        match self {
            AuditLevel::L1 => "Claim→Outcome proof under Ω-SSOT",
            AuditLevel::L2 => "Mapping consistency proof (C=0)",
            AuditLevel::L3 => "Sub-operations conformity proof",
        }
    }
}

/// L1 Audit: Claim→Outcome proof under Ω-SSOT
pub struct L1Audit {
    ssot: OmegaSSoT,
    engine: ProofEngine,
}

impl L1Audit {
    /// Create a new L1 auditor
    pub fn new() -> Self {
        Self {
            ssot: OmegaSSoT::new(),
            engine: ProofEngine::new(),
        }
    }
    
    /// Perform L1 audit
    pub fn audit(&self, claim: &str, evidence: &[String]) -> Result<AuditResult> {
        let mut findings = Vec::new();
        
        // Step 1: Verify Ω-SSOT integrity
        if !self.ssot.verify_integrity() {
            findings.push("Ω-SSOT integrity check failed".to_string());
            return Ok(AuditResult::new(
                AuditLevel::L1,
                BinaryProof::NoProofExists,
                claim,
                evidence.to_vec(),
                vec![],
                false,
                findings,
            ));
        }
        findings.push("Ω-SSOT integrity verified".to_string());
        
        // Step 2: Check if claim violates any axioms
        if let Some(violated) = self.ssot.check_violation(claim) {
            findings.push(format!("Axiom violation: {}", violated.id));
            return Ok(AuditResult::new(
                AuditLevel::L1,
                BinaryProof::NoProofExists,
                claim,
                evidence.to_vec(),
                vec![violated.id.clone()],
                false,
                findings,
            ));
        }
        findings.push("No axiom violations detected".to_string());
        
        // Step 3: Verify claim is supported by evidence
        match self.engine.verify_claim(claim, evidence) {
            Ok(true) => {
                findings.push("Claim supported by evidence".to_string());
                let axioms: Vec<String> = self.ssot.core_axioms.all()
                    .map(|a| a.id.clone())
                    .collect();
                
                Ok(AuditResult::new(
                    AuditLevel::L1,
                    BinaryProof::ProofExists,
                    claim,
                    evidence.to_vec(),
                    axioms,
                    true,
                    findings,
                ))
            }
            Ok(false) => {
                findings.push("Claim not supported by evidence".to_string());
                Ok(AuditResult::new(
                    AuditLevel::L1,
                    BinaryProof::NoProofExists,
                    claim,
                    evidence.to_vec(),
                    vec![],
                    true, // No contradiction, just insufficient evidence
                    findings,
                ))
            }
            Err(e) => {
                findings.push(format!("Verification error: {}", e));
                Ok(AuditResult::new(
                    AuditLevel::L1,
                    BinaryProof::NoProofExists,
                    claim,
                    evidence.to_vec(),
                    vec![],
                    false,
                    findings,
                ))
            }
        }
    }
}

impl Default for L1Audit {
    fn default() -> Self {
        Self::new()
    }
}

/// L2 Audit: Mapping consistency proof (C=0)
pub struct L2Audit {
    engine: ProofEngine,
}

impl L2Audit {
    /// Create a new L2 auditor
    pub fn new() -> Self {
        Self {
            engine: ProofEngine::new(),
        }
    }
    
    /// Perform L2 audit
    pub fn audit(&self, claim: &str, evidence: &[String], l1_result: &AuditResult) -> Result<AuditResult> {
        let mut findings = Vec::new();
        
        // Step 1: Verify L1 passed
        if !l1_result.proof.exists() {
            findings.push("L1 audit did not pass - L2 cannot proceed".to_string());
            return Ok(AuditResult::new(
                AuditLevel::L2,
                BinaryProof::NoProofExists,
                claim,
                evidence.to_vec(),
                vec![],
                false,
                findings,
            ));
        }
        findings.push("L1 audit verified".to_string());
        
        // Step 2: Verify mapping consistency
        // Each piece of evidence should map consistently to the claim
        let mut consistent = true;
        let mut c_value = 0u32;
        
        for (i, e) in evidence.iter().enumerate() {
            // Check if evidence is self-consistent
            if e.contains("contradiction") || e.contains("inconsistent") {
                findings.push(format!("Evidence {} contains inconsistency marker", i));
                consistent = false;
                c_value += 1;
            }
            
            // Check if evidence maps to claim
            // Simple heuristic: evidence should relate to claim
            if !claim.split_whitespace().any(|w| e.to_lowercase().contains(&w.to_lowercase())) {
                findings.push(format!("Evidence {} may not directly support claim", i));
            }
        }
        
        // Step 3: Verify C=0
        let c_zero = c_value == 0;
        if !c_zero {
            findings.push(format!("C={} (contradictions detected)", c_value));
            consistent = false;
        } else {
            findings.push("C=0 verified".to_string());
        }
        
        if consistent {
            findings.push("Mapping consistency verified".to_string());
        }
        
        Ok(AuditResult::new(
            AuditLevel::L2,
            BinaryProof::from_bool(consistent && c_zero),
            claim,
            evidence.to_vec(),
            vec!["A6_C_ZERO".to_string()],
            c_zero,
            findings,
        ))
    }
}

impl Default for L2Audit {
    fn default() -> Self {
        Self::new()
    }
}

/// L3 Audit: Sub-operations conformity proof
pub struct L3Audit {
    ssot: OmegaSSoT,
}

impl L3Audit {
    /// Create a new L3 auditor
    pub fn new() -> Self {
        Self {
            ssot: OmegaSSoT::new(),
        }
    }
    
    /// Perform L3 audit
    pub fn audit(
        &self,
        claim: &str,
        evidence: &[String],
        l1_result: &AuditResult,
        l2_result: &AuditResult,
        sub_operations: &[SubOperation],
    ) -> Result<AuditResult> {
        let mut findings = Vec::new();
        
        // Step 1: Verify L1 and L2 passed
        if !l1_result.proof.exists() || !l2_result.proof.exists() {
            findings.push("L1 or L2 audit did not pass - L3 cannot proceed".to_string());
            return Ok(AuditResult::new(
                AuditLevel::L3,
                BinaryProof::NoProofExists,
                claim,
                evidence.to_vec(),
                vec![],
                false,
                findings,
            ));
        }
        findings.push("L1 and L2 audits verified".to_string());
        
        // Step 2: Verify each sub-operation conforms
        let mut all_conform = true;
        
        for (i, op) in sub_operations.iter().enumerate() {
            if !op.verify_conformity(&self.ssot) {
                findings.push(format!("Sub-operation {} non-conformant: {}", i, op.name));
                all_conform = false;
            } else {
                findings.push(format!("Sub-operation {} conforms", i));
            }
        }
        
        // Step 3: Verify sub-operation chain integrity
        let chain_valid = SubOperation::verify_chain(sub_operations);
        if !chain_valid {
            findings.push("Sub-operation chain integrity failed".to_string());
            all_conform = false;
        } else {
            findings.push("Sub-operation chain integrity verified".to_string());
        }
        
        let c_zero = all_conform;
        
        Ok(AuditResult::new(
            AuditLevel::L3,
            BinaryProof::from_bool(all_conform),
            claim,
            evidence.to_vec(),
            vec!["A5_DETERMINISM".to_string(), "A7_CAUSAL_CLOSURE".to_string()],
            c_zero,
            findings,
        ))
    }
}

impl Default for L3Audit {
    fn default() -> Self {
        Self::new()
    }
}

/// A sub-operation in the audit chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubOperation {
    /// Operation name
    pub name: String,
    /// Input to the operation
    pub input: String,
    /// Output from the operation
    pub output: String,
    /// Hash of the operation
    pub hash: String,
    /// Previous operation hash (for chain)
    pub prev_hash: Option<String>,
}

impl SubOperation {
    /// Create a new sub-operation
    pub fn new(
        name: impl Into<String>,
        input: impl Into<String>,
        output: impl Into<String>,
        prev_hash: Option<String>,
    ) -> Self {
        let name = name.into();
        let input = input.into();
        let output = output.into();
        
        let hash = Self::compute_hash(&name, &input, &output, &prev_hash);
        
        Self {
            name,
            input,
            output,
            hash,
            prev_hash,
        }
    }
    
    fn compute_hash(name: &str, input: &str, output: &str, prev: &Option<String>) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        hasher.update(input.as_bytes());
        hasher.update(output.as_bytes());
        if let Some(p) = prev {
            hasher.update(p.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
    
    /// Verify operation integrity
    pub fn verify_integrity(&self) -> bool {
        let computed = Self::compute_hash(&self.name, &self.input, &self.output, &self.prev_hash);
        computed == self.hash
    }
    
    /// Verify conformity with Ω-SSOT
    pub fn verify_conformity(&self, ssot: &OmegaSSoT) -> bool {
        // Check operation doesn't violate any axioms
        if ssot.check_violation(&self.output).is_some() {
            return false;
        }
        
        // Verify integrity
        self.verify_integrity()
    }
    
    /// Verify a chain of sub-operations
    pub fn verify_chain(ops: &[SubOperation]) -> bool {
        if ops.is_empty() {
            return true;
        }
        
        // First op should have no prev_hash
        if ops[0].prev_hash.is_some() {
            return false;
        }
        
        // Each subsequent op should reference the previous
        for i in 1..ops.len() {
            match &ops[i].prev_hash {
                Some(prev) if *prev == ops[i-1].hash => continue,
                _ => return false,
            }
        }
        
        // All ops should have valid integrity
        ops.iter().all(|op| op.verify_integrity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_l1_audit_pass() {
        let l1 = L1Audit::new();
        let result = l1.audit(
            "The conclusion follows",
            &["Evidence A".to_string(), "Evidence B".to_string()],
        ).unwrap();
        
        assert!(result.proof.exists());
        assert!(result.c_zero);
    }
    
    #[test]
    fn test_l1_audit_no_evidence() {
        let l1 = L1Audit::new();
        let result = l1.audit("Some claim", &[]).unwrap();
        
        assert!(!result.proof.exists());
    }
    
    #[test]
    fn test_l2_audit_pass() {
        let l1 = L1Audit::new();
        let l2 = L2Audit::new();
        
        let evidence = vec!["Supporting fact".to_string()];
        let l1_result = l1.audit("The claim", &evidence).unwrap();
        let l2_result = l2.audit("The claim", &evidence, &l1_result).unwrap();
        
        assert!(l2_result.proof.exists());
        assert!(l2_result.c_zero);
    }
    
    #[test]
    fn test_l2_audit_contradiction() {
        let l1 = L1Audit::new();
        let l2 = L2Audit::new();
        
        let evidence = vec!["contradiction in evidence".to_string()];
        let l1_result = l1.audit("The claim", &evidence).unwrap();
        let l2_result = l2.audit("The claim", &evidence, &l1_result).unwrap();
        
        assert!(!l2_result.proof.exists());
        assert!(!l2_result.c_zero);
    }
    
    #[test]
    fn test_sub_operation_chain() {
        let op1 = SubOperation::new("init", "start", "middle", None);
        let op2 = SubOperation::new("process", "middle", "end", Some(op1.hash.clone()));
        
        assert!(SubOperation::verify_chain(&[op1, op2]));
    }
    
    #[test]
    fn test_sub_operation_broken_chain() {
        let op1 = SubOperation::new("init", "start", "middle", None);
        let op2 = SubOperation::new("process", "middle", "end", Some("wrong_hash".to_string()));
        
        assert!(!SubOperation::verify_chain(&[op1, op2]));
    }
}

