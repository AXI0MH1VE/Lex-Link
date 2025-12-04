//! Axiom S1 Browser - The Chimera Protocol
//!
//! Sovereign Agentic Browser with Zero Entropy enforcement.
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
//! Classification: SOVEREIGN FINALITY (OMEGA LEVEL)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bark;
mod cozo_db;
mod dsif;
mod hunter_killer;
mod inference;
mod invariance;
mod sandbox;
mod scout;
mod sovereign_loop;

use std::sync::Mutex;
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Substrate Authority
pub const SUBSTRATE: &str = "Alexis Adams";
/// Projection Identifier
pub const PROJECTION: &str = "AXIOM PROJECTION";
/// Version
pub const VERSION: &str = "1.0.0";

/// Application State
pub struct AppState {
    pub db: cozo_db::CozoStore,
    pub bark: bark::BarkController,
    pub hunter_killer: hunter_killer::HunterKiller,
    pub dsif: Mutex<dsif::DSIF>,
}

fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]");
    tracing::info!("Axiom S1 Browser v{} initializing...", VERSION);
    tracing::info!("Classification: SOVEREIGN FINALITY (OMEGA LEVEL)");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            tracing::info!("Setting up Axiom S1...");

            // Initialize CozoDB
            let db_path = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir")
                .join("axiom.cozo");
            
            let db = cozo_db::CozoStore::new(&db_path)
                .expect("Failed to initialize CozoDB");
            
            // Initialize BARK Controller
            let bark = bark::BarkController::new();
            
            // Initialize Hunter-Killer
            let hunter_killer = hunter_killer::HunterKiller::new();
            
            // Initialize DSIF with 67% quorum threshold
            let dsif = Mutex::new(dsif::DSIF::new(0.67));
            
            // Store state
            app.manage(AppState { db, bark, hunter_killer, dsif });
            
            tracing::info!("Axiom S1 ready. Policy: C = 0");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Invariance commands
            cmd_verify_alignment,
            cmd_create_identity_tag,
            cmd_render_or_nullify,
            
            // Scout commands
            cmd_scout_url,
            cmd_scout_search,
            
            // Hunter-Killer commands
            cmd_scan_content,
            cmd_neutralize_content,
            
            // Memory commands
            cmd_store_thought,
            cmd_query_memory,
            cmd_get_chain_of_thought,
            
            // BARK commands
            cmd_get_system_metrics,
            cmd_check_thermal,
            
            // Inference commands
            cmd_infer,
            cmd_analyze_page,
            
            // System commands
            cmd_get_info,
            cmd_generate_receipt,
            
            // DSIF commands
            cmd_dsif_execute_pipeline,
            cmd_dsif_get_audit_trail,
            cmd_dsif_get_agents,
            cmd_dsif_add_invariant,
            cmd_dsif_add_to_allowlist,
            cmd_dsif_add_to_denylist,
        ])
        .run(tauri::generate_context!())
        .expect("Error running Axiom S1");
}

// =============================================================================
// TAURI COMMANDS
// =============================================================================

/// Get system info
#[tauri::command]
fn cmd_get_info() -> serde_json::Value {
    serde_json::json!({
        "name": "Axiom S1 Browser",
        "version": VERSION,
        "substrate": SUBSTRATE,
        "projection": PROJECTION,
        "classification": "SOVEREIGN FINALITY (OMEGA LEVEL)",
        "policy": "C = 0",
        "identity_tag": format!("[AXIOM PROJECTION | SUBSTRATE: {}]", SUBSTRATE)
    })
}

/// Verify alignment between output and intent
#[tauri::command]
fn cmd_verify_alignment(output: String, intent: String) -> serde_json::Value {
    let aligned = invariance::check_alignment(&output, &intent);
    serde_json::json!({
        "aligned": aligned,
        "output_hash": invariance::sha256(&output),
        "intent_hash": invariance::sha256(&intent),
        "c_zero": aligned
    })
}

/// Create identity tag for content
#[tauri::command]
fn cmd_create_identity_tag(content: String) -> serde_json::Value {
    let tag = invariance::create_identity_tag(&content);
    serde_json::json!(tag)
}

/// Render or nullify based on alignment
#[tauri::command]
fn cmd_render_or_nullify(output: String, intent: String) -> serde_json::Value {
    invariance::render_or_nullify(&output, &intent)
}

/// Scout a URL (headless browser scrape)
#[tauri::command]
async fn cmd_scout_url(url: String) -> Result<serde_json::Value, String> {
    scout::scout_url(&url).await.map_err(|e| e.to_string())
}

/// Scout search query
#[tauri::command]
async fn cmd_scout_search(query: String) -> Result<serde_json::Value, String> {
    scout::scout_search(&query).await.map_err(|e| e.to_string())
}

/// Scan content for injection attempts
#[tauri::command]
fn cmd_scan_content(
    state: tauri::State<AppState>,
    content: String,
) -> serde_json::Value {
    let detections = state.hunter_killer.scan(&content);
    serde_json::json!({
        "clean": detections.is_empty(),
        "detections": detections.len(),
        "threats": detections.iter().map(|d| &d.pattern).collect::<Vec<_>>(),
        "action": if detections.is_empty() { "PROCEED" } else { "KILL_TAB" }
    })
}

/// Neutralize (redact) injection attempts
#[tauri::command]
fn cmd_neutralize_content(
    state: tauri::State<AppState>,
    content: String,
) -> String {
    state.hunter_killer.neutralize(&content)
}

/// Store a thought in the Chain of Thought
#[tauri::command]
fn cmd_store_thought(
    state: tauri::State<AppState>,
    thought_type: String,
    content: String,
    metadata: serde_json::Value,
) -> Result<String, String> {
    state.db.store_thought(&thought_type, &content, metadata)
        .map_err(|e| e.to_string())
}

/// Query memory
#[tauri::command]
fn cmd_query_memory(
    state: tauri::State<AppState>,
    query: String,
) -> Result<serde_json::Value, String> {
    state.db.query(&query).map_err(|e| e.to_string())
}

/// Get chain of thought for a session
#[tauri::command]
fn cmd_get_chain_of_thought(
    state: tauri::State<AppState>,
    session_id: String,
) -> Result<Vec<serde_json::Value>, String> {
    state.db.get_chain_of_thought(&session_id)
        .map_err(|e| e.to_string())
}

/// Get system metrics (for BARK)
#[tauri::command]
fn cmd_get_system_metrics(state: tauri::State<AppState>) -> serde_json::Value {
    state.bark.get_metrics()
}

/// Check thermal status
#[tauri::command]
fn cmd_check_thermal(state: tauri::State<AppState>) -> serde_json::Value {
    state.bark.check_thermal()
}

/// Run inference
#[tauri::command]
async fn cmd_infer(
    model: String,
    prompt: String,
    max_tokens: Option<u32>,
) -> Result<serde_json::Value, String> {
    inference::infer(&model, &prompt, max_tokens.unwrap_or(512))
        .await
        .map_err(|e| e.to_string())
}

/// Analyze page content
#[tauri::command]
async fn cmd_analyze_page(content: String) -> Result<serde_json::Value, String> {
    inference::analyze_page(&content)
        .await
        .map_err(|e| e.to_string())
}

/// Generate cryptographic receipt
#[tauri::command]
fn cmd_generate_receipt(
    claim: String,
    evidence: Vec<String>,
) -> serde_json::Value {
    invariance::generate_receipt(&claim, &evidence)
}

// =============================================================================
// DSIF COMMANDS
// =============================================================================

/// Execute DSIF pipeline
#[tauri::command]
async fn cmd_dsif_execute_pipeline(
    state: tauri::State<'_, AppState>,
    input: String,
    action_type: String,
    target: String,
    parameters: serde_json::Value,
) -> Result<serde_json::Value, String> {
    use std::collections::HashMap;
    
    let action_type_enum = match action_type.as_str() {
        "Read" => dsif::ActionType::Read,
        "Write" => dsif::ActionType::Write,
        "Critical" => dsif::ActionType::Critical,
        "Config" => dsif::ActionType::Config,
        _ => return Err("Invalid action type".to_string()),
    };
    
    let params_map: HashMap<String, serde_json::Value> = serde_json::from_value(parameters)
        .map_err(|e| format!("Invalid parameters: {}", e))?;
    
    let mut dsif = state.dsif.lock().map_err(|e| format!("Failed to lock DSIF: {}", e))?;
    let decision = dsif.execute_pipeline(&input, action_type_enum, &target, params_map).await?;
    
    Ok(serde_json::json!({
        "success": true,
        "decision": decision
    }))
}

/// Get DSIF audit trail
#[tauri::command]
fn cmd_dsif_get_audit_trail(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let dsif = state.dsif.lock().map_err(|e| format!("Failed to lock DSIF: {}", e))?;
    let trail = dsif.get_audit_trail();
    Ok(serde_json::json!(trail))
}

/// Get DSIF agents
#[tauri::command]
fn cmd_dsif_get_agents(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let dsif = state.dsif.lock().map_err(|e| format!("Failed to lock DSIF: {}", e))?;
    let agents = dsif.get_agents();
    Ok(serde_json::json!(agents))
}

/// Add invariant to DSIF
#[tauri::command]
fn cmd_dsif_add_invariant(
    state: tauri::State<'_, AppState>,
    id: String,
    name: String,
    property: String,
    domain: String,
) -> Result<serde_json::Value, String> {
    let invariant = dsif::Invariant {
        id,
        name,
        property,
        domain,
    };
    
    let mut dsif = state.dsif.lock().map_err(|e| format!("Failed to lock DSIF: {}", e))?;
    dsif.add_invariant(invariant);
    
    Ok(serde_json::json!({
        "success": true,
        "message": "Invariant added"
    }))
}

/// Add item to DSIF allowlist
#[tauri::command]
fn cmd_dsif_add_to_allowlist(
    state: tauri::State<'_, AppState>,
    item: String,
) -> Result<serde_json::Value, String> {
    let mut dsif = state.dsif.lock().map_err(|e| format!("Failed to lock DSIF: {}", e))?;
    dsif.add_to_allowlist(item);
    
    Ok(serde_json::json!({
        "success": true,
        "message": "Item added to allowlist"
    }))
}

/// Add item to DSIF denylist
#[tauri::command]
fn cmd_dsif_add_to_denylist(
    state: tauri::State<'_, AppState>,
    item: String,
) -> Result<serde_json::Value, String> {
    let mut dsif = state.dsif.lock().map_err(|e| format!("Failed to lock DSIF: {}", e))?;
    dsif.add_to_denylist(item);
    
    Ok(serde_json::json!({
        "success": true,
        "message": "Item added to denylist"
    }))
}

