//! Sovereign Loop - The Core Workflow
//!
//! Phase 1: SENSE (Scout scrapes)
//! Phase 2: FILTER (Firewall strips injections)
//! Phase 3: SYNTHESIZE (Brain analyzes)
//! Phase 4: AUDIT (Gavel generates receipt)
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use crate::{hunter_killer, inference, invariance, scout};
use serde::{Deserialize, Serialize};

/// Sovereign Loop result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopResult {
    pub phase: String,
    pub success: bool,
    pub data: serde_json::Value,
    pub receipt: Option<serde_json::Value>,
    pub c_zero: bool,
}

/// Execute the full sovereign loop
pub async fn execute(
    intent: &str,
    target_url: Option<&str>,
) -> Result<LoopResult, String> {
    // Phase 1: SENSE
    let sensed = if let Some(url) = target_url {
        scout::scout_url(url).await.map_err(|e| e.to_string())?
    } else {
        serde_json::json!({"note": "No URL provided"})
    };
    
    // Phase 2: FILTER
    let hk = hunter_killer::HunterKiller::new();
    let content = sensed["content"].as_str().unwrap_or("");
    let audit = hk.audit_content(content);
    
    if audit.action == hunter_killer::Action::KillTab {
        return Ok(LoopResult {
            phase: "FILTER".to_string(),
            success: false,
            data: serde_json::json!({"threat": audit.threat}),
            receipt: None,
            c_zero: false,
        });
    }
    
    let filtered = hk.neutralize(content);
    
    // Phase 3: SYNTHESIZE
    let synthesis = inference::synthesize(&sensed, intent)
        .await
        .map_err(|e| e.to_string())?;
    
    // Phase 4: AUDIT
    let evidence = vec![
        sensed["url"].as_str().unwrap_or("").to_string(),
        format!("Content hash: {}", sensed["hash"].as_str().unwrap_or("")),
    ];
    
    let receipt = invariance::generate_receipt(intent, &evidence);
    
    Ok(LoopResult {
        phase: "COMPLETE".to_string(),
        success: true,
        data: synthesis,
        receipt: Some(receipt),
        c_zero: true,
    })
}

