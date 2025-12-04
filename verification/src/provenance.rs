//! Provenance tracking for inputs, models, and environments
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Provenance information for a bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Input data provenance
    pub inputs: Vec<DataProvenance>,
    
    /// Model metadata
    pub model: ModelMetadata,
    
    /// Environment manifest
    pub environment: EnvironmentManifest,
    
    /// Configuration
    pub config: crate::deterministic::DeterministicConfig,
}

/// Data provenance for inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProvenance {
    /// Input name
    pub name: String,
    
    /// Content hash
    pub hash: String,
    
    /// Source URI
    #[serde(rename = "source_uri")]
    pub source_uri: Option<String>,
    
    /// License information
    pub license: Option<String>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Transformations applied
    pub transformations: Vec<Transformation>,
}

/// Transformation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    /// Transformation name
    pub name: String,
    
    /// Input hash (before)
    #[serde(rename = "input_hash")]
    pub input_hash: String,
    
    /// Output hash (after)
    #[serde(rename = "output_hash")]
    pub output_hash: String,
    
    /// Transformation code hash
    #[serde(rename = "code_hash")]
    pub code_hash: Option<String>,
}

/// Model metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model name
    pub name: String,
    
    /// Model version
    pub version: String,
    
    /// Weights hash
    #[serde(rename = "weights_hash")]
    pub weights_hash: String,
    
    /// Tokenizer hash
    #[serde(rename = "tokenizer_hash")]
    pub tokenizer_hash: String,
    
    /// Model card URI
    #[serde(rename = "card_uri")]
    pub card_uri: Option<String>,
}

/// Environment manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentManifest {
    /// Container image hash
    #[serde(rename = "container_image_hash")]
    pub container_image_hash: String,
    
    /// OS version
    pub os: String,
    
    /// Dependencies
    pub deps: Vec<Dependency>,
    
    /// Hardware profile (optional)
    pub hardware: Option<HardwareProfile>,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Package name
    pub name: String,
    
    /// Version
    pub version: String,
    
    /// Hash
    pub hash: String,
}

/// Hardware profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// CPU architecture
    pub cpu: String,
    
    /// GPU information
    pub gpu: Option<String>,
    
    /// Driver version
    pub driver: Option<String>,
    
    /// CUDA version (if applicable)
    pub cuda: Option<String>,
}

