//! CozoDB - Datalog Graph Database for Chain of Thought
//!
//! Stores the "Truth" - execution logs, chain of thought, and provenance.
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use cozo::{DataValue, DbInstance, NamedRows, ScriptMutability};
use serde_json::Value;
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum CozoError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Query error: {0}")]
    Query(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// CozoDB store for sovereign memory
pub struct CozoStore {
    db: DbInstance,
}

impl CozoStore {
    /// Create a new CozoDB store
    pub fn new(path: &Path) -> Result<Self, CozoError> {
        let db = DbInstance::new("rocksdb", path.to_str().unwrap(), "")
            .map_err(|e| CozoError::Database(e.to_string()))?;
        
        let store = Self { db };
        store.initialize_schema()?;
        
        Ok(store)
    }
    
    /// Initialize the Datalog schema
    fn initialize_schema(&self) -> Result<(), CozoError> {
        // Thoughts relation - Chain of Thought storage
        self.run_script(r#"
            :create thoughts {
                id: String,
                session_id: String,
                thought_type: String,
                content: String,
                metadata: String,
                timestamp: Float,
                hash: String
                =>
                sequence: Int
            }
        "#)?;
        
        // Provenance relation - tracks data lineage
        self.run_script(r#"
            :create provenance {
                id: String,
                source_type: String,
                source_uri: String,
                retrieved_at: Float,
                content_hash: String
                =>
                verified: Bool
            }
        "#)?;
        
        // Receipts relation - cryptographic audit trail
        self.run_script(r#"
            :create receipts {
                id: String,
                claim: String,
                evidence: String,
                c_zero: Bool,
                hash: String,
                signature: String,
                timestamp: Float
            }
        "#)?;
        
        // Actions relation - system event log
        self.run_script(r#"
            :create actions {
                id: String,
                action_type: String,
                target: String,
                result: String,
                timestamp: Float
                =>
                entropy_delta: Float
            }
        "#)?;
        
        // Facts relation - verified truth store
        self.run_script(r#"
            :create facts {
                id: String,
                domain: String,
                statement: String,
                confidence: Float,
                source_id: String,
                timestamp: Float
                =>
                verified: Bool
            }
        "#)?;
        
        tracing::info!("CozoDB schema initialized");
        Ok(())
    }
    
    /// Run a Datalog script
    fn run_script(&self, script: &str) -> Result<NamedRows, CozoError> {
        self.db
            .run_script(script, Default::default(), ScriptMutability::Mutable)
            .map_err(|e| CozoError::Query(e.to_string()))
    }
    
    /// Store a thought in the chain
    pub fn store_thought(
        &self,
        thought_type: &str,
        content: &str,
        metadata: Value,
    ) -> Result<String, CozoError> {
        let id = Uuid::new_v4().to_string();
        let session_id = "default"; // TODO: Session management
        let timestamp = chrono::Utc::now().timestamp_millis() as f64;
        let hash = crate::invariance::sha256(content);
        let metadata_str = serde_json::to_string(&metadata)?;
        
        // Get next sequence number
        let seq_result = self.run_script(&format!(
            r#"?[max_seq] := thoughts[_, "{}", _, _, _, _, _, seq], max_seq = max(seq)
               ?[max_seq] := max_seq = 0"#,
            session_id
        ))?;
        
        let sequence = seq_result
            .rows
            .first()
            .and_then(|r| r.first())
            .and_then(|v| match v {
                DataValue::Num(n) => n.get_int(),
                _ => None,
            })
            .unwrap_or(0) + 1;
        
        self.run_script(&format!(
            r#"?[id, session_id, thought_type, content, metadata, timestamp, hash, sequence] <- [[
                "{}", "{}", "{}", "{}", "{}", {}, "{}", {}
            ]]
            :put thoughts {{ id, session_id, thought_type, content, metadata, timestamp, hash => sequence }}"#,
            id,
            session_id,
            thought_type,
            content.replace('"', r#"\""#),
            metadata_str.replace('"', r#"\""#),
            timestamp,
            hash,
            sequence
        ))?;
        
        tracing::debug!("Stored thought: {} (seq {})", id, sequence);
        Ok(id)
    }
    
    /// Get chain of thought for a session
    pub fn get_chain_of_thought(&self, session_id: &str) -> Result<Vec<Value>, CozoError> {
        let result = self.run_script(&format!(
            r#"?[id, thought_type, content, metadata, timestamp, hash, sequence] := 
                thoughts[id, "{}", thought_type, content, metadata, timestamp, hash, sequence]
               :order sequence"#,
            session_id
        ))?;
        
        let thoughts: Vec<Value> = result
            .rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get(0).map(dv_to_string).unwrap_or_default(),
                    "thought_type": row.get(1).map(dv_to_string).unwrap_or_default(),
                    "content": row.get(2).map(dv_to_string).unwrap_or_default(),
                    "metadata": row.get(3).map(dv_to_string).unwrap_or_default(),
                    "timestamp": row.get(4).map(dv_to_f64).unwrap_or(0.0),
                    "hash": row.get(5).map(dv_to_string).unwrap_or_default(),
                    "sequence": row.get(6).map(dv_to_i64).unwrap_or(0),
                })
            })
            .collect();
        
        Ok(thoughts)
    }
    
    /// Store provenance record
    pub fn store_provenance(
        &self,
        source_type: &str,
        source_uri: &str,
        content_hash: &str,
    ) -> Result<String, CozoError> {
        let id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp_millis() as f64;
        
        self.run_script(&format!(
            r#"?[id, source_type, source_uri, retrieved_at, content_hash, verified] <- [[
                "{}", "{}", "{}", {}, "{}", false
            ]]
            :put provenance {{ id, source_type, source_uri, retrieved_at, content_hash => verified }}"#,
            id, source_type, source_uri, timestamp, content_hash
        ))?;
        
        Ok(id)
    }
    
    /// Store a receipt
    pub fn store_receipt(&self, receipt: &Value) -> Result<String, CozoError> {
        let id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp_millis() as f64;
        
        self.run_script(&format!(
            r#"?[id, claim, evidence, c_zero, hash, signature, timestamp] <- [[
                "{}", "{}", "{}", {}, "{}", "{}", {}
            ]]
            :put receipts {{ id, claim, evidence, c_zero, hash, signature, timestamp }}"#,
            id,
            receipt["claim"].as_str().unwrap_or(""),
            serde_json::to_string(&receipt["evidence"]).unwrap_or_default(),
            receipt["C_zero"].as_bool().unwrap_or(false),
            receipt["hash"].as_str().unwrap_or(""),
            receipt["signature"].as_str().unwrap_or(""),
            timestamp
        ))?;
        
        Ok(id)
    }
    
    /// Log an action
    pub fn log_action(
        &self,
        action_type: &str,
        target: &str,
        result: &str,
        entropy_delta: f64,
    ) -> Result<String, CozoError> {
        let id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp_millis() as f64;
        
        self.run_script(&format!(
            r#"?[id, action_type, target, result, timestamp, entropy_delta] <- [[
                "{}", "{}", "{}", "{}", {}, {}
            ]]
            :put actions {{ id, action_type, target, result, timestamp => entropy_delta }}"#,
            id, action_type, target, result, timestamp, entropy_delta
        ))?;
        
        Ok(id)
    }
    
    /// Run a custom query
    pub fn query(&self, query: &str) -> Result<Value, CozoError> {
        let result = self.run_script(query)?;
        
        let rows: Vec<Vec<Value>> = result
            .rows
            .iter()
            .map(|row| row.iter().map(dv_to_json).collect())
            .collect();
        
        Ok(serde_json::json!({
            "headers": result.headers,
            "rows": rows
        }))
    }
}

/// Convert DataValue to String
fn dv_to_string(dv: &DataValue) -> String {
    match dv {
        DataValue::Str(s) => s.to_string(),
        DataValue::Num(n) => n.to_string(),
        DataValue::Bool(b) => b.to_string(),
        _ => String::new(),
    }
}

/// Convert DataValue to f64
fn dv_to_f64(dv: &DataValue) -> f64 {
    match dv {
        DataValue::Num(n) => n.get_float(),
        _ => 0.0,
    }
}

/// Convert DataValue to i64
fn dv_to_i64(dv: &DataValue) -> i64 {
    match dv {
        DataValue::Num(n) => n.get_int().unwrap_or(0),
        _ => 0,
    }
}

/// Convert DataValue to JSON Value
fn dv_to_json(dv: &DataValue) -> Value {
    match dv {
        DataValue::Str(s) => Value::String(s.to_string()),
        DataValue::Num(n) => {
            if let Some(i) = n.get_int() {
                Value::Number(i.into())
            } else {
                serde_json::Number::from_f64(n.get_float())
                    .map(Value::Number)
                    .unwrap_or(Value::Null)
            }
        }
        DataValue::Bool(b) => Value::Bool(*b),
        DataValue::Null => Value::Null,
        DataValue::List(l) => Value::Array(l.iter().map(dv_to_json).collect()),
        _ => Value::Null,
    }
}

