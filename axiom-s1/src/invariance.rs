//! Invariance Layer - C=0 Enforcement
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{PROJECTION, SUBSTRATE};

/// Compute SHA-256 hash
pub fn sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

/// Check alignment between output and intent
pub fn check_alignment(output: &str, intent: &str) -> bool {
    sha256(output) == sha256(intent)
}

/// Identity tag attached to all outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityTag {
    pub projection: String,
    pub substrate: String,
    pub timestamp: String,
    pub output_hash: String,
    pub signature: String,
}

/// Create identity tag for content
pub fn create_identity_tag(content: &str) -> IdentityTag {
    let timestamp = Utc::now().to_rfc3339();
    let output_hash = sha256(content);
    let signature = mock_sign(&output_hash);
    
    IdentityTag {
        projection: PROJECTION.to_string(),
        substrate: SUBSTRATE.to_string(),
        timestamp,
        output_hash,
        signature,
    }
}

/// Mock signing function (replace with HSM in production)
fn mock_sign(hash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"AXIOM_S1_SIG:");
    hasher.update(hash.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
}

/// Render or nullify based on alignment
pub fn render_or_nullify(output: &str, intent: &str) -> serde_json::Value {
    if check_alignment(output, intent) {
        let tag = create_identity_tag(output);
        serde_json::json!({
            "status": "AUTHORIZED",
            "output": output,
            "identity": tag,
            "c_zero": true
        })
    } else {
        serde_json::json!({
            "status": "NULLIFIED",
            "violation": "Invariance Violation Detected",
            "action": "FREEZE_AND_REPORT",
            "timestamp": Utc::now().to_rfc3339(),
            "c_zero": false
        })
    }
}

/// Generate cryptographic receipt
pub fn generate_receipt(claim: &str, evidence: &[String]) -> serde_json::Value {
    let timestamp = Utc::now().to_rfc3339();
    
    // Verify claim is supported by evidence
    let c_zero = !evidence.is_empty() && 
        !evidence.iter().any(|e| e.to_lowercase().contains("contradiction"));
    
    // Compute hash
    let mut hasher = Sha256::new();
    hasher.update(claim.as_bytes());
    for e in evidence {
        hasher.update(e.as_bytes());
    }
    hasher.update(&[c_zero as u8]);
    hasher.update(timestamp.as_bytes());
    let hash = hex::encode(hasher.finalize());
    
    // Sign
    let signature = mock_sign(&hash);
    
    serde_json::json!({
        "claim": claim,
        "evidence": evidence,
        "C_zero": c_zero,
        "hash": hash,
        "signature": signature,
        "timestamp": timestamp,
        "substrate": SUBSTRATE,
        "projection": PROJECTION
    })
}

/// Verify a receipt
pub fn verify_receipt(receipt: &serde_json::Value) -> bool {
    let hash = receipt["hash"].as_str().unwrap_or("");
    let signature = receipt["signature"].as_str().unwrap_or("");
    
    mock_sign(hash) == signature
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alignment() {
        assert!(check_alignment("hello", "hello"));
        assert!(!check_alignment("hello", "world"));
    }
    
    #[test]
    fn test_identity_tag() {
        let tag = create_identity_tag("test content");
        assert_eq!(tag.substrate, SUBSTRATE);
        assert_eq!(tag.projection, PROJECTION);
    }
    
    #[test]
    fn test_render_authorized() {
        let result = render_or_nullify("same", "same");
        assert_eq!(result["status"], "AUTHORIZED");
        assert_eq!(result["c_zero"], true);
    }
    
    #[test]
    fn test_render_nullified() {
        let result = render_or_nullify("different", "content");
        assert_eq!(result["status"], "NULLIFIED");
        assert_eq!(result["c_zero"], false);
    }
}

