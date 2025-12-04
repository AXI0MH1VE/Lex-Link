//! Audit Service HTTP Server
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axiom_audit::{
    AuditService, AuditReceipt,
    service::{AuditRequest, AuditResponse},
};

/// Application state
struct AppState {
    service: Mutex<AuditService>,
}

fn mock_sign(hash: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(b"AUDIT_SVC_SIG:");
    hasher.update(hash.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
}

fn mock_verify(hash: &str, sig: &str) -> bool {
    mock_sign(hash) == sig
}

/// Health check endpoint
async fn health() -> &'static str {
    "[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]\nAudit Service: OPERATIONAL"
}

/// Info endpoint
async fn info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "Deterministic Fractal Audit Service",
        "version": "1.0.0",
        "substrate": axiom_audit::SUBSTRATE,
        "projection": axiom_audit::PROJECTION,
        "levels": ["L1", "L2", "L3"],
        "policy": "C = 0",
        "output_type": "Binary (Proof Exists | No Proof Exists)"
    }))
}

/// Audit endpoint
async fn audit(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AuditRequest>,
) -> Result<Json<AuditResponse>, (StatusCode, String)> {
    let mut service = state.service.lock().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Lock error: {}", e))
    })?;
    
    let receipt = service.audit_with_ops(
        &request.claim,
        &request.evidence,
        &request.sub_operations,
        mock_sign,
    ).map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Audit error: {}", e))
    })?;
    
    Ok(Json(AuditResponse::from(receipt)))
}

/// Quick verify endpoint (L1 only)
async fn quick_verify(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AuditRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = state.service.lock().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Lock error: {}", e))
    })?;
    
    let proof = service.quick_verify(&request.claim, &request.evidence)
        .map_err(|e| {
            (StatusCode::BAD_REQUEST, format!("Verification error: {}", e))
        })?;
    
    Ok(Json(serde_json::json!({
        "proof_exists": proof.exists(),
        "claim": request.claim
    })))
}

/// Verify receipt endpoint
async fn verify_receipt(
    State(state): State<Arc<AppState>>,
    Json(receipt): Json<AuditReceipt>,
) -> Json<serde_json::Value> {
    let service = state.service.lock().unwrap();
    let valid = service.verify_receipt(&receipt, mock_verify);
    
    Json(serde_json::json!({
        "valid": valid,
        "receipt_hash": receipt.receipt_hash,
        "proof_exists": receipt.proof_exists(),
        "c_zero": receipt.c_zero
    }))
}

/// Get audit log hash
async fn log_hash(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let mut service = state.service.lock().unwrap();
    let hash = service.log_root_hash();
    
    Json(serde_json::json!({
        "log_root_hash": hash,
        "entries_count": service.log_entries().len()
    }))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    tracing::info!("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]");
    tracing::info!("Starting Deterministic Fractal Audit Service v1.0.0");
    
    // Create app state
    let state = Arc::new(AppState {
        service: Mutex::new(AuditService::new()),
    });
    
    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/audit", post(audit))
        .route("/verify", post(quick_verify))
        .route("/verify-receipt", post(verify_receipt))
        .route("/log/hash", get(log_hash))
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    // Get port from env or use default
    let port = std::env::var("AUDIT_PORT")
        .unwrap_or_else(|_| "3001".to_string());
    let addr = format!("127.0.0.1:{}", port);
    
    tracing::info!("Audit service listening on {}", addr);
    tracing::info!("Policy: C = 0 | Mode: Binary Proof");
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

