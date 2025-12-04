//! Core audit types and receipt generation
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};

use crate::levels::AuditLevel;

/// Binary proof result - the fundamental output type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryProof {
    /// Proof exists and is valid
    ProofExists,
    /// No proof exists or proof is invalid
    NoProofExists,
}

impl BinaryProof {
    /// Check if proof exists
    pub fn exists(&self) -> bool {
        matches!(self, BinaryProof::ProofExists)
    }
    
    /// Convert from boolean
    pub fn from_bool(b: bool) -> Self {
        if b { BinaryProof::ProofExists } else { BinaryProof::NoProofExists }
    }
}

impl From<bool> for BinaryProof {
    fn from(b: bool) -> Self {
        BinaryProof::from_bool(b)
    }
}

/// Result of an audit at any level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    /// The audit level (L1, L2, or L3)
    pub level: AuditLevel,
    /// Binary proof outcome
    pub proof: BinaryProof,
    /// Claim that was audited
    pub claim: String,
    /// Evidence used
    pub evidence: Vec<String>,
    /// Axioms verified against
    pub axioms: Vec<String>,
    /// Whether C=0 was maintained
    pub c_zero: bool,
    /// Detailed findings (for internal use)
    pub findings: Vec<String>,
    /// Hash of the result
    pub hash: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl AuditResult {
    /// Create a new audit result
    pub fn new(
        level: AuditLevel,
        proof: BinaryProof,
        claim: impl Into<String>,
        evidence: Vec<String>,
        axioms: Vec<String>,
        c_zero: bool,
        findings: Vec<String>,
    ) -> Self {
        let claim = claim.into();
        let timestamp = Utc::now();
        let hash = Self::compute_hash(&level, &proof, &claim, &evidence, &axioms, c_zero, &timestamp);
        
        Self {
            level,
            proof,
            claim,
            evidence,
            axioms,
            c_zero,
            findings,
            hash,
            timestamp,
        }
    }
    
    fn compute_hash(
        level: &AuditLevel,
        proof: &BinaryProof,
        claim: &str,
        evidence: &[String],
        axioms: &[String],
        c_zero: bool,
        timestamp: &DateTime<Utc>,
    ) -> String {
        let mut hasher = Sha256::new();
        
        hasher.update(format!("{:?}", level).as_bytes());
        hasher.update(format!("{:?}", proof).as_bytes());
        hasher.update(claim.as_bytes());
        
        for e in evidence {
            hasher.update(e.as_bytes());
        }
        
        for a in axioms {
            hasher.update(a.as_bytes());
        }
        
        hasher.update(&[c_zero as u8]);
        hasher.update(timestamp.to_rfc3339().as_bytes());
        
        hex::encode(hasher.finalize())
    }
    
    /// Verify the result's integrity
    pub fn verify_integrity(&self) -> bool {
        let computed = Self::compute_hash(
            &self.level,
            &self.proof,
            &self.claim,
            &self.evidence,
            &self.axioms,
            self.c_zero,
            &self.timestamp,
        );
        computed == self.hash
    }
}

/// A cryptographic audit receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReceipt {
    /// Results from all audit levels
    pub results: Vec<AuditResult>,
    /// Final binary proof (all levels must pass)
    pub final_proof: BinaryProof,
    /// Overall C=0 status
    pub c_zero: bool,
    /// Combined hash of all results
    pub receipt_hash: String,
    /// Signature (base64 DER)
    pub signature: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Substrate authority
    pub substrate: String,
    /// Projection identifier
    pub projection: String,
}

impl AuditReceipt {
    /// Create a new audit receipt from results
    pub fn new(
        results: Vec<AuditResult>,
        sign_fn: impl FnOnce(&str) -> String,
    ) -> Self {
        let timestamp = Utc::now();
        
        // All levels must pass for final proof
        let all_pass = results.iter().all(|r| r.proof.exists());
        let final_proof = BinaryProof::from_bool(all_pass);
        
        // All levels must maintain C=0
        let c_zero = results.iter().all(|r| r.c_zero);
        
        let receipt_hash = Self::compute_hash(&results, &timestamp);
        let signature = sign_fn(&receipt_hash);
        
        Self {
            results,
            final_proof,
            c_zero,
            receipt_hash,
            signature,
            timestamp,
            substrate: crate::SUBSTRATE.to_string(),
            projection: crate::PROJECTION.to_string(),
        }
    }
    
    fn compute_hash(results: &[AuditResult], timestamp: &DateTime<Utc>) -> String {
        let mut hasher = Sha256::new();
        
        for result in results {
            hasher.update(result.hash.as_bytes());
        }
        
        hasher.update(timestamp.to_rfc3339().as_bytes());
        
        hex::encode(hasher.finalize())
    }
    
    /// Verify the receipt's hash integrity
    pub fn verify_hash(&self) -> bool {
        let computed = Self::compute_hash(&self.results, &self.timestamp);
        computed == self.receipt_hash
    }
    
    /// Verify the receipt's signature
    pub fn verify_signature(&self, verify_fn: impl FnOnce(&str, &str) -> bool) -> bool {
        verify_fn(&self.receipt_hash, &self.signature)
    }
    
    /// Full verification
    pub fn verify(&self, verify_fn: impl FnOnce(&str, &str) -> bool) -> bool {
        // Verify all result hashes
        if !self.results.iter().all(|r| r.verify_integrity()) {
            return false;
        }
        
        // Verify receipt hash
        if !self.verify_hash() {
            return false;
        }
        
        // Verify signature
        self.verify_signature(verify_fn)
    }
    
    /// Check if proof exists
    pub fn proof_exists(&self) -> bool {
        self.final_proof.exists()
    }
    
    /// Convert to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Parse from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// Get a summary for the verification portal
    pub fn summary(&self) -> AuditSummary {
        AuditSummary {
            proof_exists: self.proof_exists(),
            c_zero: self.c_zero,
            levels_passed: self.results.iter().filter(|r| r.proof.exists()).count(),
            total_levels: self.results.len(),
            hash: self.receipt_hash.clone(),
            timestamp: self.timestamp,
        }
    }
}

/// Summary for public API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSummary {
    pub proof_exists: bool,
    pub c_zero: bool,
    pub levels_passed: usize,
    pub total_levels: usize,
    pub hash: String,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_sign(hash: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"MOCK_SIG:");
        hasher.update(hash.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    }
    
    fn mock_verify(hash: &str, sig: &str) -> bool {
        mock_sign(hash) == sig
    }
    
    #[test]
    fn test_binary_proof() {
        assert!(BinaryProof::ProofExists.exists());
        assert!(!BinaryProof::NoProofExists.exists());
        
        assert_eq!(BinaryProof::from_bool(true), BinaryProof::ProofExists);
        assert_eq!(BinaryProof::from_bool(false), BinaryProof::NoProofExists);
    }
    
    #[test]
    fn test_audit_result() {
        let result = AuditResult::new(
            AuditLevel::L1,
            BinaryProof::ProofExists,
            "Test claim",
            vec!["evidence".to_string()],
            vec!["axiom".to_string()],
            true,
            vec!["finding".to_string()],
        );
        
        assert!(result.verify_integrity());
        assert!(result.c_zero);
    }
    
    #[test]
    fn test_audit_receipt() {
        let results = vec![
            AuditResult::new(
                AuditLevel::L1,
                BinaryProof::ProofExists,
                "claim",
                vec![],
                vec![],
                true,
                vec![],
            ),
            AuditResult::new(
                AuditLevel::L2,
                BinaryProof::ProofExists,
                "claim",
                vec![],
                vec![],
                true,
                vec![],
            ),
        ];
        
        let receipt = AuditReceipt::new(results, mock_sign);
        
        assert!(receipt.proof_exists());
        assert!(receipt.c_zero);
        assert!(receipt.verify(mock_verify));
    }
    
    #[test]
    fn test_failed_audit() {
        let results = vec![
            AuditResult::new(
                AuditLevel::L1,
                BinaryProof::ProofExists,
                "claim",
                vec![],
                vec![],
                true,
                vec![],
            ),
            AuditResult::new(
                AuditLevel::L2,
                BinaryProof::NoProofExists, // Failed
                "claim",
                vec![],
                vec![],
                false,
                vec![],
            ),
        ];
        
        let receipt = AuditReceipt::new(results, mock_sign);
        
        assert!(!receipt.proof_exists());
        assert!(!receipt.c_zero);
    }
}

