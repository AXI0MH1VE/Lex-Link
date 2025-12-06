//! Receipt generation and verification
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};

use crate::trace::TraceEnvelope;

/// A cryptographic receipt proving a claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    /// The claim that was verified
    pub claim: String,
    /// Evidence supporting the claim
    pub evidence: Vec<String>,
    /// The causal chain (string representation)
    pub causal_chain: Vec<String>,
    /// Axioms applied during verification
    pub axioms: Vec<String>,
    /// Whether C=0 (no contradictions)
    #[serde(rename = "C_zero")]
    pub c_zero: bool,
    /// Hash of the receipt contents
    pub hash: String,
    /// Cryptographic signature (base64 DER)
    pub signature: String,
    /// Timestamp of receipt generation
    pub timestamp: DateTime<Utc>,
    /// Substrate authority
    pub substrate: String,
    /// Projection identifier
    pub projection: String,
}

impl Receipt {
    /// Create a new receipt from a trace envelope
    pub fn from_trace(trace: &TraceEnvelope, sign_fn: impl FnOnce(&str) -> String) -> Self {
        let timestamp = Utc::now();
        
        let hash = Self::compute_hash(
            &trace.claim,
            &trace.observations,
            &trace.causal_chain,
            &trace.axioms,
            trace.is_c_zero(),
            &timestamp,
        );
        
        let signature = sign_fn(&hash);
        
        Self {
            claim: trace.claim.clone(),
            evidence: trace.observations.clone(),
            causal_chain: trace.causal_chain.clone(),
            axioms: trace.axioms.clone(),
            c_zero: trace.is_c_zero(),
            hash,
            signature,
            timestamp,
            substrate: trace.substrate.clone(),
            projection: trace.projection.clone(),
        }
    }
    
    fn compute_hash(
        claim: &str,
        evidence: &[String],
        causal_chain: &[String],
        axioms: &[String],
        c_zero: bool,
        timestamp: &DateTime<Utc>,
    ) -> String {
        let mut hasher = Sha256::new();
        
        hasher.update(claim.as_bytes());
        
        for e in evidence {
            hasher.update(e.as_bytes());
        }
        
        for link in causal_chain {
            hasher.update(link.as_bytes());
        }
        
        for axiom in axioms {
            hasher.update(axiom.as_bytes());
        }
        
        hasher.update([c_zero as u8]);
        hasher.update(timestamp.to_rfc3339().as_bytes());
        
        hex::encode(hasher.finalize())
    }
    
    /// Verify the receipt's hash integrity
    pub fn verify_hash(&self) -> bool {
        let computed = Self::compute_hash(
            &self.claim,
            &self.evidence,
            &self.causal_chain,
            &self.axioms,
            self.c_zero,
            &self.timestamp,
        );
        computed == self.hash
    }
    
    /// Verify the receipt's signature
    pub fn verify_signature(&self, verify_fn: impl FnOnce(&str, &str) -> bool) -> bool {
        verify_fn(&self.hash, &self.signature)
    }
    
    /// Full verification (hash + signature)
    pub fn verify(&self, verify_fn: impl FnOnce(&str, &str) -> bool) -> bool {
        self.verify_hash() && self.verify_signature(verify_fn)
    }
    
    /// Check if the receipt indicates a valid proof (C=0)
    pub fn is_valid_proof(&self) -> bool {
        self.c_zero
    }
    
    /// Convert to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Parse from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Builder for constructing receipts
pub struct ReceiptBuilder {
    claim: String,
    evidence: Vec<String>,
    causal_chain: Vec<String>,
    axioms: Vec<String>,
    c_zero: bool,
}

impl ReceiptBuilder {
    /// Create a new builder
    pub fn new(claim: impl Into<String>) -> Self {
        Self {
            claim: claim.into(),
            evidence: Vec::new(),
            causal_chain: Vec::new(),
            axioms: Vec::new(),
            c_zero: true,
        }
    }
    
    /// Add evidence
    pub fn with_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence.push(evidence.into());
        self
    }
    
    /// Add multiple evidence items
    pub fn with_evidence_list(mut self, evidence: Vec<String>) -> Self {
        self.evidence.extend(evidence);
        self
    }
    
    /// Add a causal chain link
    pub fn with_causal_link(mut self, link: impl Into<String>) -> Self {
        self.causal_chain.push(link.into());
        self
    }
    
    /// Add causal chain
    pub fn with_causal_chain(mut self, chain: Vec<String>) -> Self {
        self.causal_chain = chain;
        self
    }
    
    /// Add an axiom
    pub fn with_axiom(mut self, axiom: impl Into<String>) -> Self {
        self.axioms.push(axiom.into());
        self
    }
    
    /// Add axioms
    pub fn with_axioms(mut self, axioms: Vec<String>) -> Self {
        self.axioms.extend(axioms);
        self
    }
    
    /// Set C=0 status
    pub fn with_c_zero(mut self, c_zero: bool) -> Self {
        self.c_zero = c_zero;
        self
    }
    
    /// Build the receipt
    pub fn build(self, sign_fn: impl FnOnce(&str) -> String) -> Receipt {
        let timestamp = Utc::now();
        
        let hash = Receipt::compute_hash(
            &self.claim,
            &self.evidence,
            &self.causal_chain,
            &self.axioms,
            self.c_zero,
            &timestamp,
        );
        
        let signature = sign_fn(&hash);
        
        Receipt {
            claim: self.claim,
            evidence: self.evidence,
            causal_chain: self.causal_chain,
            axioms: self.axioms,
            c_zero: self.c_zero,
            hash,
            signature,
            timestamp,
            substrate: crate::SUBSTRATE.to_string(),
            projection: crate::PROJECTION.to_string(),
        }
    }
}

/// Minimal receipt for binary proof (Verified | Not Verified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryReceipt {
    /// Whether C=0 (verified)
    #[serde(rename = "C_zero")]
    pub c_zero: bool,
    /// Hash of the full receipt
    pub hash: String,
    /// Signature
    pub signature: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl From<Receipt> for BinaryReceipt {
    fn from(receipt: Receipt) -> Self {
        Self {
            c_zero: receipt.c_zero,
            hash: receipt.hash,
            signature: receipt.signature,
            timestamp: receipt.timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_sign(hash: &str) -> String {
        // Mock signing for tests
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"MOCK_SIG:");
        hasher.update(hash.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    }
    
    fn mock_verify(hash: &str, signature: &str) -> bool {
        mock_sign(hash) == signature
    }
    
    #[test]
    fn test_receipt_builder() {
        let receipt = ReceiptBuilder::new("The claim is true")
            .with_evidence("Evidence A")
            .with_evidence("Evidence B")
            .with_causal_link("A → B")
            .with_axiom("A1_IDENTITY")
            .with_c_zero(true)
            .build(mock_sign);
        
        assert!(receipt.is_valid_proof());
        assert!(receipt.verify_hash());
        assert!(receipt.verify(mock_verify));
    }
    
    #[test]
    fn test_receipt_json_roundtrip() {
        let receipt = ReceiptBuilder::new("claim")
            .with_evidence("fact")
            .with_c_zero(true)
            .build(mock_sign);
        
        let json = receipt.to_json().unwrap();
        let parsed = Receipt::from_json(&json).unwrap();
        
        assert_eq!(receipt.claim, parsed.claim);
        assert_eq!(receipt.hash, parsed.hash);
    }
    
    #[test]
    fn test_binary_receipt() {
        let receipt = ReceiptBuilder::new("claim")
            .with_c_zero(true)
            .build(mock_sign);
        
        let binary: BinaryReceipt = receipt.into();
        
        assert!(binary.c_zero);
    }
    
    #[test]
    fn test_invalid_receipt() {
        let receipt = ReceiptBuilder::new("contradictory claim")
            .with_c_zero(false)
            .build(mock_sign);
        
        assert!(!receipt.is_valid_proof());
    }
}

