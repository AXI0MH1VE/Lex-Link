//! Deterministic inference configuration and controls
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Deterministic configuration for reproducible inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterministicConfig {
    /// Fixed random seed
    pub seed: u64,
    
    /// Model parameters
    pub parameters: ModelParameters,
}

/// Model parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    /// Temperature (0.0 for deterministic)
    pub temperature: f64,
    
    /// Top-p sampling
    #[serde(rename = "top_p")]
    pub top_p: f64,
    
    /// Top-k sampling
    #[serde(rename = "top_k")]
    pub top_k: Option<u32>,
    
    /// Maximum tokens
    #[serde(rename = "max_tokens")]
    pub max_tokens: u32,
    
    /// Additional parameters
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            temperature: 0.0, // Deterministic
            top_p: 1.0,
            top_k: None,
            max_tokens: 1024,
            extra: HashMap::new(),
        }
    }
}

/// Seed control for PRNG hygiene
#[derive(Debug, Clone)]
pub struct SeedControl {
    /// Master seed
    master_seed: u64,
    
    /// Current PRNG state
    prng_state: u64,
}

impl SeedControl {
    /// Create new seed control
    pub fn new(seed: u64) -> Self {
        Self {
            master_seed: seed,
            prng_state: seed,
        }
    }
    
    /// Get next deterministic value
    pub fn next_value(&mut self) -> u64 {
        // Simple LCG for deterministic sequence
        // In production, use a proper PRNG with state capture
        self.prng_state = self.prng_state.wrapping_mul(1103515245).wrapping_add(12345);
        self.prng_state
    }
    
    /// Reset to master seed
    pub fn reset(&mut self) {
        self.prng_state = self.master_seed;
    }
    
    /// Get current state (for capture/replay)
    pub fn state(&self) -> u64 {
        self.prng_state
    }
    
    /// Restore state (for replay)
    pub fn restore_state(&mut self, state: u64) {
        self.prng_state = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_seed_control() {
        let mut control = SeedControl::new(42);
        
        let v1 = control.next_value();
        let v2 = control.next_value();
        
        // Reset and replay
        control.reset();
        assert_eq!(control.next_value(), v1);
        assert_eq!(control.next_value(), v2);
    }
    
    #[test]
    fn test_deterministic_config() {
        let config = DeterministicConfig {
            seed: 42,
            parameters: ModelParameters::default(),
        };
        
        assert_eq!(config.parameters.temperature, 0.0);
        assert_eq!(config.seed, 42);
    }
}

