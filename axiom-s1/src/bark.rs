//! BARK Protocol - Binary Authority Regulatory Kernel
//!
//! Thermal Intelligence & C=0 Resource Enforcement
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, System, SystemExt};
use std::sync::{Arc, Mutex};

/// Maximum entropy budget (Hamiltonian energy ceiling)
const MAX_ENTROPY: f64 = 1000.0;

/// Thermal thresholds
const THERMAL_WARNING: f32 = 70.0;  // °C
const THERMAL_CRITICAL: f32 = 85.0; // °C
const THERMAL_SHUTDOWN: f32 = 95.0; // °C

/// BARK Controller
pub struct BarkController {
    system: Arc<Mutex<System>>,
    entropy_budget: Arc<Mutex<f64>>,
}

impl BarkController {
    /// Create a new BARK controller
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system: Arc::new(Mutex::new(system)),
            entropy_budget: Arc::new(Mutex::new(MAX_ENTROPY)),
        }
    }
    
    /// Refresh system metrics
    pub fn refresh(&self) {
        if let Ok(mut sys) = self.system.lock() {
            sys.refresh_all();
        }
    }
    
    /// Get current system metrics
    pub fn get_metrics(&self) -> serde_json::Value {
        self.refresh();
        
        let sys = self.system.lock().unwrap();
        let entropy = self.entropy_budget.lock().unwrap();
        
        let cpu_usage: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() 
            / sys.cpus().len() as f32;
        
        let memory_used = sys.used_memory();
        let memory_total = sys.total_memory();
        let memory_percent = (memory_used as f64 / memory_total as f64) * 100.0;
        
        serde_json::json!({
            "cpu": {
                "usage_percent": cpu_usage,
                "cores": sys.cpus().len(),
                "per_core": sys.cpus().iter().map(|c| c.cpu_usage()).collect::<Vec<_>>()
            },
            "memory": {
                "used_bytes": memory_used,
                "total_bytes": memory_total,
                "usage_percent": memory_percent
            },
            "entropy": {
                "current": MAX_ENTROPY - *entropy,
                "budget": *entropy,
                "max": MAX_ENTROPY,
                "usage_percent": ((MAX_ENTROPY - *entropy) / MAX_ENTROPY) * 100.0
            },
            "c_zero_compliant": *entropy > 0.0,
            "substrate": crate::SUBSTRATE,
            "projection": crate::PROJECTION
        })
    }
    
    /// Check thermal status
    pub fn check_thermal(&self) -> serde_json::Value {
        self.refresh();
        
        let sys = self.system.lock().unwrap();
        
        // Get CPU temperature (platform-specific)
        let temps: Vec<ThermalReading> = sys.components()
            .iter()
            .map(|c| ThermalReading {
                label: c.label().to_string(),
                current: c.temperature(),
                max: c.max(),
                critical: c.critical(),
            })
            .collect();
        
        // Find highest temperature
        let max_temp = temps.iter()
            .map(|t| t.current)
            .fold(0.0f32, |a, b| a.max(b));
        
        let status = if max_temp >= THERMAL_SHUTDOWN {
            ThermalStatus::Shutdown
        } else if max_temp >= THERMAL_CRITICAL {
            ThermalStatus::Critical
        } else if max_temp >= THERMAL_WARNING {
            ThermalStatus::Warning
        } else {
            ThermalStatus::Normal
        };
        
        serde_json::json!({
            "status": status.as_str(),
            "max_temperature": max_temp,
            "thresholds": {
                "warning": THERMAL_WARNING,
                "critical": THERMAL_CRITICAL,
                "shutdown": THERMAL_SHUTDOWN
            },
            "readings": temps,
            "action": match status {
                ThermalStatus::Shutdown => "HALT_ALL_INFERENCE",
                ThermalStatus::Critical => "REDUCE_WORKLOAD",
                ThermalStatus::Warning => "MONITOR",
                ThermalStatus::Normal => "PROCEED",
            }
        })
    }
    
    /// Consume entropy budget
    pub fn consume_entropy(&self, amount: f64) -> Result<(), BarkError> {
        let mut budget = self.entropy_budget.lock().unwrap();
        
        if *budget < amount {
            return Err(BarkError::EntropyExceeded {
                requested: amount,
                available: *budget,
            });
        }
        
        *budget -= amount;
        tracing::debug!("Entropy consumed: {} (remaining: {})", amount, *budget);
        
        Ok(())
    }
    
    /// Restore entropy budget
    pub fn restore_entropy(&self, amount: f64) {
        let mut budget = self.entropy_budget.lock().unwrap();
        *budget = (*budget + amount).min(MAX_ENTROPY);
    }
    
    /// Check if action is within entropy budget
    pub fn check_action(&self, entropy_cost: f64) -> ActionCheck {
        let budget = self.entropy_budget.lock().unwrap();
        let thermal = self.check_thermal();
        
        let thermal_status = thermal["status"].as_str().unwrap_or("UNKNOWN");
        
        if thermal_status == "SHUTDOWN" || thermal_status == "CRITICAL" {
            return ActionCheck {
                allowed: false,
                reason: format!("Thermal status: {}", thermal_status),
                c_zero: false,
            };
        }
        
        if *budget < entropy_cost {
            return ActionCheck {
                allowed: false,
                reason: format!(
                    "Entropy budget exceeded: requested {} but only {} available",
                    entropy_cost, *budget
                ),
                c_zero: false,
            };
        }
        
        ActionCheck {
            allowed: true,
            reason: "Action within Hamiltonian budget".to_string(),
            c_zero: true,
        }
    }
    
    /// Reset entropy budget (for new session)
    pub fn reset_entropy(&self) {
        let mut budget = self.entropy_budget.lock().unwrap();
        *budget = MAX_ENTROPY;
        tracing::info!("Entropy budget reset to {}", MAX_ENTROPY);
    }
}

impl Default for BarkController {
    fn default() -> Self {
        Self::new()
    }
}

/// Thermal reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalReading {
    pub label: String,
    pub current: f32,
    pub max: f32,
    pub critical: Option<f32>,
}

/// Thermal status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalStatus {
    Normal,
    Warning,
    Critical,
    Shutdown,
}

impl ThermalStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThermalStatus::Normal => "NORMAL",
            ThermalStatus::Warning => "WARNING",
            ThermalStatus::Critical => "CRITICAL",
            ThermalStatus::Shutdown => "SHUTDOWN",
        }
    }
}

/// Action check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionCheck {
    pub allowed: bool,
    pub reason: String,
    pub c_zero: bool,
}

/// BARK errors
#[derive(Debug, thiserror::Error)]
pub enum BarkError {
    #[error("Entropy budget exceeded: requested {requested}, available {available}")]
    EntropyExceeded { requested: f64, available: f64 },
    
    #[error("Thermal limit exceeded")]
    ThermalExceeded,
    
    #[error("Action not authorized: C != 0")]
    NotAuthorized,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bark_creation() {
        let bark = BarkController::new();
        let metrics = bark.get_metrics();
        assert!(metrics["c_zero_compliant"].as_bool().unwrap());
    }
    
    #[test]
    fn test_entropy_consumption() {
        let bark = BarkController::new();
        
        assert!(bark.consume_entropy(100.0).is_ok());
        assert!(bark.consume_entropy(100.0).is_ok());
        
        let metrics = bark.get_metrics();
        assert!(metrics["entropy"]["current"].as_f64().unwrap() > 0.0);
    }
    
    #[test]
    fn test_entropy_exceeded() {
        let bark = BarkController::new();
        
        // Try to consume more than budget
        let result = bark.consume_entropy(MAX_ENTROPY + 1.0);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_action_check() {
        let bark = BarkController::new();
        
        let check = bark.check_action(50.0);
        assert!(check.allowed);
        assert!(check.c_zero);
    }
}

