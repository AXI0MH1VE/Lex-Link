//! Attestation and signature system
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Attestation - Signed statement about an artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    /// Signer identity (DID, key fingerprint, etc.)
    #[serde(rename = "signer_id")]
    pub signer_id: String,
    
    /// Cryptographic signature (base64)
    pub signature: String,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Signer role
    pub role: SignerRole,
    
    /// Optional statement/claim
    pub statement: Option<String>,
}

/// Signer role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignerRole {
    /// System identity
    System,
    /// Human approver
    Approver,
    /// External auditor
    Auditor,
    /// Operator
    Operator,
}

/// Attestation chain - Linked sequence of attestations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationChain {
    /// Chain entries
    pub entries: Vec<ChainEntry>,
    
    /// Root attestation
    pub root: Attestation,
}

/// Chain entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainEntry {
    /// Attestation
    pub attestation: Attestation,
    
    /// Previous entry hash (links to previous)
    #[serde(rename = "previous_hash")]
    pub previous_hash: String,
    
    /// Entry hash
    pub hash: String,
}

impl AttestationChain {
    /// Create a new chain with root attestation
    pub fn new(root: Attestation) -> Self {
        Self {
            entries: vec![],
            root,
        }
    }
    
    /// Append an attestation to the chain
    pub fn append(&mut self, attestation: Attestation) {
        let previous_hash = if let Some(last) = self.entries.last() {
            last.hash.clone()
        } else {
            Self::hash_attestation(&self.root)
        };
        
        let entry_hash = Self::hash_attestation(&attestation);
        
        self.entries.push(ChainEntry {
            attestation,
            previous_hash,
            hash: entry_hash,
        });
    }
    
    /// Verify chain integrity
    pub fn verify_integrity(&self) -> bool {
        let mut prev_hash = Self::hash_attestation(&self.root);
        
        for entry in &self.entries {
            if entry.previous_hash != prev_hash {
                return false;
            }
            
            let computed_hash = Self::hash_attestation(&entry.attestation);
            if entry.hash != computed_hash {
                return false;
            }
            
            prev_hash = entry.hash.clone();
        }
        
        true
    }
    
    /// Hash an attestation
    fn hash_attestation(attestation: &Attestation) -> String {
        use sha2::{Digest, Sha256};
        
        let mut hasher = Sha256::new();
        hasher.update(attestation.signer_id.as_bytes());
        hasher.update(attestation.timestamp.to_rfc3339().as_bytes());
        hasher.update(format!("{:?}", attestation.role).as_bytes());
        if let Some(ref stmt) = attestation.statement {
            hasher.update(stmt.as_bytes());
        }
        
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_attestation_chain() {
        let root = Attestation {
            signer_id: "did:key:z6Mki...".to_string(),
            signature: "sig1".to_string(),
            timestamp: Utc::now(),
            role: SignerRole::System,
            statement: None,
        };
        
        let mut chain = AttestationChain::new(root);
        
        let entry = Attestation {
            signer_id: "did:key:z6Mk2...".to_string(),
            signature: "sig2".to_string(),
            timestamp: Utc::now(),
            role: SignerRole::Approver,
            statement: Some("Approved".to_string()),
        };
        
        chain.append(entry);
        
        assert!(chain.verify_integrity());
    }
}

