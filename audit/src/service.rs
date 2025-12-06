//! Audit service implementation
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use crate::audit::{AuditReceipt, BinaryProof};
// AuditResult is not directly used in this module
use crate::levels::{L1Audit, L2Audit, L3Audit, SubOperation};
use crate::merkle::MerkleLog;
use crate::Result;

/// Configuration for the audit service
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Enable L3 audit (sub-operation conformity)
    pub enable_l3: bool,
    /// Maximum evidence items
    pub max_evidence: usize,
    /// Enable audit logging
    pub enable_logging: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enable_l3: true,
            max_evidence: 100,
            enable_logging: true,
        }
    }
}

/// The main audit service
pub struct AuditService {
    l1: L1Audit,
    l2: L2Audit,
    l3: L3Audit,
    config: AuditConfig,
    log: MerkleLog,
}

impl AuditService {
    /// Create a new audit service
    pub fn new() -> Self {
        Self {
            l1: L1Audit::new(),
            l2: L2Audit::new(),
            l3: L3Audit::new(),
            config: AuditConfig::default(),
            log: MerkleLog::new(),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: AuditConfig) -> Self {
        Self {
            l1: L1Audit::new(),
            l2: L2Audit::new(),
            l3: L3Audit::new(),
            config,
            log: MerkleLog::new(),
        }
    }
    
    /// Perform full audit and generate receipt
    pub fn audit(
        &mut self,
        claim: &str,
        evidence: &[String],
        sign_fn: impl FnOnce(&str) -> String,
    ) -> Result<AuditReceipt> {
        self.audit_with_ops(claim, evidence, &[], sign_fn)
    }
    
    /// Perform full audit with sub-operations
    pub fn audit_with_ops(
        &mut self,
        claim: &str,
        evidence: &[String],
        sub_ops: &[SubOperation],
        sign_fn: impl FnOnce(&str) -> String,
    ) -> Result<AuditReceipt> {
        let mut results = Vec::new();
        
        // L1 Audit
        let l1_result = self.l1.audit(claim, evidence)?;
        if self.config.enable_logging {
            self.log.append(format!("L1: {} - {:?}", claim, l1_result.proof));
        }
        results.push(l1_result.clone());
        
        // L2 Audit
        let l2_result = self.l2.audit(claim, evidence, &l1_result)?;
        if self.config.enable_logging {
            self.log.append(format!("L2: {} - {:?}", claim, l2_result.proof));
        }
        results.push(l2_result.clone());
        
        // L3 Audit (if enabled and sub-operations provided)
        if self.config.enable_l3 {
            let l3_result = self.l3.audit(claim, evidence, &l1_result, &l2_result, sub_ops)?;
            if self.config.enable_logging {
                self.log.append(format!("L3: {} - {:?}", claim, l3_result.proof));
            }
            results.push(l3_result);
        }
        
        // Generate receipt
        let receipt = AuditReceipt::new(results, sign_fn);
        
        if self.config.enable_logging {
            self.log.append(format!("Receipt: {} - {:?}", receipt.receipt_hash, receipt.final_proof));
        }
        
        Ok(receipt)
    }
    
    /// Quick verification (L1 only)
    pub fn quick_verify(&self, claim: &str, evidence: &[String]) -> Result<BinaryProof> {
        let result = self.l1.audit(claim, evidence)?;
        Ok(result.proof)
    }
    
    /// Verify a receipt
    pub fn verify_receipt(
        &self,
        receipt: &AuditReceipt,
        verify_fn: impl FnOnce(&str, &str) -> bool,
    ) -> bool {
        receipt.verify(verify_fn)
    }
    
    /// Get audit log root hash
    pub fn log_root_hash(&mut self) -> Option<String> {
        self.log.root_hash()
    }
    
    /// Get audit log entries
    pub fn log_entries(&self) -> &[crate::merkle::LogEntry] {
        self.log.entries()
    }
}

impl Default for AuditService {
    fn default() -> Self {
        Self::new()
    }
}

/// Request for audit API
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AuditRequest {
    pub claim: String,
    pub evidence: Vec<String>,
    #[serde(default)]
    pub sub_operations: Vec<SubOperation>,
}

/// Response from audit API
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuditResponse {
    pub proof_exists: bool,
    pub c_zero: bool,
    pub receipt_hash: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<AuditReceipt>,
}

impl From<AuditReceipt> for AuditResponse {
    fn from(receipt: AuditReceipt) -> Self {
        Self {
            proof_exists: receipt.proof_exists(),
            c_zero: receipt.c_zero,
            receipt_hash: receipt.receipt_hash.clone(),
            timestamp: receipt.timestamp.to_rfc3339(),
            receipt: Some(receipt),
        }
    }
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
    fn test_audit_service_creation() {
        let service = AuditService::new();
        assert!(service.log.is_empty());
    }
    
    #[test]
    fn test_full_audit() {
        let mut service = AuditService::new();
        
        let receipt = service.audit(
            "The claim is valid",
            &["Evidence A".to_string(), "Evidence B".to_string()],
            mock_sign,
        ).unwrap();
        
        assert!(receipt.verify(mock_verify));
    }
    
    #[test]
    fn test_quick_verify() {
        let service = AuditService::new();
        
        let proof = service.quick_verify(
            "Simple claim",
            &["Supporting evidence".to_string()],
        ).unwrap();
        
        assert!(proof.exists());
    }
    
    #[test]
    fn test_audit_logging() {
        let mut service = AuditService::new();
        
        service.audit(
            "Logged claim",
            &["Evidence".to_string()],
            mock_sign,
        ).unwrap();
        
        assert!(!service.log_entries().is_empty());
        assert!(service.log_root_hash().is_some());
    }
    
    #[test]
    fn test_audit_with_sub_ops() {
        let mut service = AuditService::new();
        
        let ops = vec![
            SubOperation::new("init", "start", "middle", None),
        ];
        
        let receipt = service.audit_with_ops(
            "Claim with operations",
            &["Evidence".to_string()],
            &ops,
            mock_sign,
        ).unwrap();
        
        // Should have 3 results (L1, L2, L3)
        assert_eq!(receipt.results.len(), 3);
    }
}

