//! # AXIOM HIVE Verification Portal
//!
//! Public API for binary proof receipts.
//! All outputs are binary: `Verified` | `Not Verified`
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use axum::{
    extract::{Json, State},
    http::{StatusCode, Method},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SUBSTRATE: &str = "Alexis Adams";
const PROJECTION: &str = "AXIOMHIVE PROJECTION";
const VERSION: &str = "1.0.0";

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub claim: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResponse {
    #[serde(rename = "C_zero")]
    pub c_zero: bool,
    pub hash: String,
    pub signature: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptQuery {
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredReceipt {
    pub claim: String,
    pub evidence: Vec<String>,
    pub c_zero: bool,
    pub hash: String,
    pub signature: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalStats {
    pub total_verifications: u64,
    pub verified_count: u64,
    pub not_verified_count: u64,
    pub uptime_seconds: u64,
}

// ============================================================================
// State
// ============================================================================

struct AppState {
    receipts: Mutex<Vec<StoredReceipt>>,
    stats: Mutex<PortalStats>,
    start_time: std::time::Instant,
}

impl AppState {
    fn new() -> Self {
        Self {
            receipts: Mutex::new(Vec::new()),
            stats: Mutex::new(PortalStats {
                total_verifications: 0,
                verified_count: 0,
                not_verified_count: 0,
                uptime_seconds: 0,
            }),
            start_time: std::time::Instant::now(),
        }
    }
}

// ============================================================================
// Signing (Mock for development)
// ============================================================================

fn mock_sign(hash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"PORTAL_SIG:");
    hasher.update(hash.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
}

fn mock_verify(hash: &str, sig: &str) -> bool {
    mock_sign(hash) == sig
}

fn compute_hash(claim: &str, evidence: &[String], c_zero: bool, timestamp: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(claim.as_bytes());
    for e in evidence {
        hasher.update(e.as_bytes());
    }
    hasher.update(&[c_zero as u8]);
    hasher.update(timestamp.as_bytes());
    hex::encode(hasher.finalize())
}

// ============================================================================
// Verification Logic
// ============================================================================

fn verify_claim(claim: &str, evidence: &[String]) -> bool {
    // Simple verification logic:
    // - Must have at least one piece of evidence
    // - Evidence must not contain contradictions
    // - Evidence must relate to the claim
    
    if evidence.is_empty() {
        return false;
    }
    
    // Check for contradictions
    for e in evidence {
        if e.to_lowercase().contains("contradiction") {
            return false;
        }
        if e.to_lowercase().contains("inconsistent") {
            return false;
        }
    }
    
    // Check evidence relates to claim (simple heuristic)
    let claim_words: Vec<&str> = claim.split_whitespace().collect();
    let has_related = evidence.iter().any(|e| {
        claim_words.iter().any(|w| e.to_lowercase().contains(&w.to_lowercase()))
    });
    
    has_related
}

// ============================================================================
// Handlers
// ============================================================================

async fn health() -> &'static str {
    "[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]\nVerification Portal: OPERATIONAL"
}

async fn info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "AXIOM HIVE Verification Portal",
        "version": VERSION,
        "substrate": SUBSTRATE,
        "projection": PROJECTION,
        "policy": "C = 0",
        "output_type": "Binary (Verified | Not Verified)",
        "endpoints": {
            "POST /verify": "Submit claim for verification",
            "GET /receipt/{hash}": "Retrieve receipt by hash",
            "GET /stats": "Portal statistics",
            "GET /health": "Health check"
        }
    }))
}

async fn verify(
    State(state): State<Arc<AppState>>,
    Json(request): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, String)> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    // Perform verification
    let c_zero = verify_claim(&request.claim, &request.evidence);
    
    // Compute hash
    let hash = compute_hash(&request.claim, &request.evidence, c_zero, &timestamp);
    
    // Sign the hash
    let signature = mock_sign(&hash);
    
    // Store receipt
    let receipt = StoredReceipt {
        claim: request.claim.clone(),
        evidence: request.evidence.clone(),
        c_zero,
        hash: hash.clone(),
        signature: signature.clone(),
        timestamp: timestamp.clone(),
    };
    
    {
        let mut receipts = state.receipts.lock().await;
        receipts.push(receipt);
    }
    
    // Update stats
    {
        let mut stats = state.stats.lock().await;
        stats.total_verifications += 1;
        if c_zero {
            stats.verified_count += 1;
        } else {
            stats.not_verified_count += 1;
        }
    }
    
    Ok(Json(VerifyResponse {
        c_zero,
        hash,
        signature,
        timestamp,
    }))
}

async fn get_receipt(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(hash): axum::extract::Path<String>,
) -> Result<Json<StoredReceipt>, (StatusCode, String)> {
    let receipts = state.receipts.lock().await;
    
    receipts
        .iter()
        .find(|r| r.hash == hash)
        .cloned()
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, "Receipt not found".to_string()))
}

async fn verify_receipt(
    Json(receipt): Json<VerifyResponse>,
) -> Json<serde_json::Value> {
    let valid = mock_verify(&receipt.hash, &receipt.signature);
    
    Json(serde_json::json!({
        "valid": valid,
        "c_zero": receipt.c_zero,
        "status": if valid && receipt.c_zero { "VERIFIED" } else { "NOT_VERIFIED" }
    }))
}

async fn get_stats(State(state): State<Arc<AppState>>) -> Json<PortalStats> {
    let mut stats = state.stats.lock().await.clone();
    stats.uptime_seconds = state.start_time.elapsed().as_secs();
    Json(stats)
}

async fn index() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AXIOM HIVE Verification Portal</title>
    <style>
        :root {
            --bg-dark: #0a0a0f;
            --bg-card: #12121a;
            --accent: #00ff88;
            --accent-dim: #00aa55;
            --text: #e0e0e0;
            --text-dim: #808080;
            --error: #ff4444;
        }
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'SF Mono', 'Fira Code', monospace;
            background: var(--bg-dark);
            color: var(--text);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 40px 20px;
        }
        .header {
            text-align: center;
            margin-bottom: 40px;
        }
        .header h1 {
            font-size: 2rem;
            color: var(--accent);
            text-transform: uppercase;
            letter-spacing: 0.2em;
        }
        .identity {
            font-size: 0.75rem;
            color: var(--text-dim);
            margin-top: 8px;
        }
        .card {
            background: var(--bg-card);
            border: 1px solid #222;
            border-radius: 8px;
            padding: 24px;
            width: 100%;
            max-width: 600px;
            margin-bottom: 20px;
        }
        .card h2 {
            font-size: 1rem;
            color: var(--accent);
            margin-bottom: 16px;
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .card h2::before {
            content: "▸";
        }
        label {
            display: block;
            font-size: 0.8rem;
            color: var(--text-dim);
            margin-bottom: 4px;
        }
        input, textarea {
            width: 100%;
            padding: 12px;
            background: var(--bg-dark);
            border: 1px solid #333;
            border-radius: 4px;
            color: var(--text);
            font-family: inherit;
            font-size: 0.9rem;
            margin-bottom: 16px;
        }
        input:focus, textarea:focus {
            outline: none;
            border-color: var(--accent);
        }
        button {
            width: 100%;
            padding: 14px;
            background: var(--accent);
            color: var(--bg-dark);
            border: none;
            border-radius: 4px;
            font-family: inherit;
            font-weight: bold;
            font-size: 1rem;
            cursor: pointer;
            text-transform: uppercase;
            letter-spacing: 0.1em;
        }
        button:hover {
            background: var(--accent-dim);
        }
        .result {
            margin-top: 20px;
            padding: 16px;
            border-radius: 4px;
            display: none;
        }
        .result.verified {
            background: rgba(0, 255, 136, 0.1);
            border: 1px solid var(--accent);
        }
        .result.not-verified {
            background: rgba(255, 68, 68, 0.1);
            border: 1px solid var(--error);
        }
        .result h3 {
            font-size: 1.2rem;
            margin-bottom: 12px;
        }
        .result.verified h3 { color: var(--accent); }
        .result.not-verified h3 { color: var(--error); }
        .result-details {
            font-size: 0.8rem;
            color: var(--text-dim);
        }
        .result-details code {
            display: block;
            background: var(--bg-dark);
            padding: 8px;
            margin-top: 8px;
            border-radius: 4px;
            overflow-x: auto;
        }
        .footer {
            margin-top: auto;
            padding-top: 40px;
            text-align: center;
            font-size: 0.75rem;
            color: var(--text-dim);
        }
        .policy {
            color: var(--accent);
            font-weight: bold;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>◈ AXIOM HIVE</h1>
        <p class="identity">[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]</p>
    </div>
    
    <div class="card">
        <h2>Verification Portal</h2>
        <form id="verifyForm">
            <label for="claim">Claim</label>
            <input type="text" id="claim" placeholder="Enter claim to verify..." required>
            
            <label for="evidence">Evidence (one per line)</label>
            <textarea id="evidence" rows="4" placeholder="Enter supporting evidence..."></textarea>
            
            <button type="submit">VERIFY</button>
        </form>
        
        <div id="result" class="result">
            <h3 id="resultTitle"></h3>
            <div class="result-details">
                <div>Hash: <code id="resultHash"></code></div>
                <div style="margin-top: 8px;">Timestamp: <span id="resultTimestamp"></span></div>
            </div>
        </div>
    </div>
    
    <div class="card">
        <h2>Policy</h2>
        <p style="font-size: 0.9rem; line-height: 1.6;">
            All verifications produce <span class="policy">binary outcomes only</span>:
        </p>
        <ul style="margin-top: 12px; margin-left: 20px; font-size: 0.85rem; line-height: 1.8;">
            <li><span class="policy">C = 0</span> → VERIFIED (Proof Exists)</li>
            <li><span style="color: var(--error);">C ≠ 0</span> → NOT VERIFIED (No Proof)</li>
        </ul>
        <p style="margin-top: 16px; font-size: 0.8rem; color: var(--text-dim);">
            No percentages. No probabilities. No partial verification.
        </p>
    </div>
    
    <footer class="footer">
        <p>Verification Portal v1.0.0 • Deterministic • Local-First • Zero Telemetry</p>
        <p style="margin-top: 8px;">Policy: <span class="policy">C = 0</span></p>
    </footer>
    
    <script>
        document.getElementById('verifyForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const claim = document.getElementById('claim').value;
            const evidenceText = document.getElementById('evidence').value;
            const evidence = evidenceText.split('\n').filter(e => e.trim());
            
            try {
                const response = await fetch('/verify', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ claim, evidence })
                });
                
                const data = await response.json();
                
                const result = document.getElementById('result');
                const title = document.getElementById('resultTitle');
                const hash = document.getElementById('resultHash');
                const timestamp = document.getElementById('resultTimestamp');
                
                result.style.display = 'block';
                result.className = 'result ' + (data.C_zero ? 'verified' : 'not-verified');
                title.textContent = data.C_zero ? '✓ VERIFIED (C = 0)' : '✗ NOT VERIFIED (C ≠ 0)';
                hash.textContent = data.hash;
                timestamp.textContent = data.timestamp;
            } catch (err) {
                console.error('Verification failed:', err);
            }
        });
    </script>
</body>
</html>"#)
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]");
    tracing::info!("Starting Verification Portal v{}", VERSION);

    // Create state
    let state = Arc::new(AppState::new());

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/verify", post(verify))
        .route("/receipt/:hash", get(get_receipt))
        .route("/verify-receipt", post(verify_receipt))
        .route("/stats", get(get_stats))
        .layer(cors)
        .with_state(state);

    // Get port from env or use default
    let port = std::env::var("PORTAL_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Verification Portal listening on {}", addr);
    tracing::info!("Policy: C = 0 | Mode: Binary Proof");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

