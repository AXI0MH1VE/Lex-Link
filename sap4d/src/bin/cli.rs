//! SAP-4D CLI
//!
//! Command-line interface for the proof engine.
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use clap::{Parser, Subcommand};
use sap4d::{ProofEngine, Receipt, ReceiptBuilder, OmegaSSoT};
use std::fs;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "sap4d")]
#[command(author = "Alexis Adams")]
#[command(version = "1.0.0")]
#[command(about = "SAP-4D Proof Engine - Causal inference with C=0 enforcement")]
#[command(after_help = "[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Prove a claim given evidence
    Prove {
        /// The claim to prove
        claim: String,
        
        /// Evidence (can be specified multiple times)
        #[arg(short, long)]
        evidence: Vec<String>,
        
        /// Read evidence from file (one per line)
        #[arg(short = 'f', long)]
        evidence_file: Option<String>,
        
        /// Output receipt to file
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Verify a receipt
    Verify {
        /// Receipt file to verify
        receipt_file: String,
    },
    
    /// Show Ω-SSOT axioms
    Axioms {
        /// Show only axioms from a specific domain
        #[arg(short, long)]
        domain: Option<String>,
    },
    
    /// Check if a claim is supported by evidence
    Check {
        /// The claim to check
        claim: String,
        
        /// Evidence items
        #[arg(short, long)]
        evidence: Vec<String>,
    },
    
    /// Show system information
    Info,
}

fn mock_sign(hash: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(b"SAP4D_CLI_SIG:");
    hasher.update(hash.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
}

fn mock_verify(hash: &str, sig: &str) -> bool {
    mock_sign(hash) == sig
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Prove { claim, evidence, evidence_file, output } => {
            let mut all_evidence = evidence;
            
            // Read evidence from file if provided
            if let Some(file) = evidence_file {
                let content = fs::read_to_string(&file)?;
                for line in content.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        all_evidence.push(trimmed.to_string());
                    }
                }
            }
            
            // Read from stdin if no evidence provided
            if all_evidence.is_empty() {
                eprintln!("Enter evidence (one per line, Ctrl+D to finish):");
                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    let line = line?;
                    if !line.trim().is_empty() {
                        all_evidence.push(line.trim().to_string());
                    }
                }
            }
            
            let engine = ProofEngine::new();
            
            match engine.prove(&claim, all_evidence, mock_sign) {
                Ok((trace, receipt)) => {
                    if cli.json {
                        let output_data = serde_json::json!({
                            "status": "VERIFIED",
                            "receipt": receipt,
                            "trace": {
                                "steps": trace.steps.len(),
                                "explainability": trace.explainability_index(),
                                "c_zero": trace.is_c_zero()
                            }
                        });
                        println!("{}", serde_json::to_string_pretty(&output_data)?);
                    } else {
                        println!("✓ Claim verified (C=0)");
                        println!();
                        println!("Claim: {}", receipt.claim);
                        println!("Evidence: {} items", receipt.evidence.len());
                        println!("Causal Chain: {} links", receipt.causal_chain.len());
                        println!("Axioms Applied: {}", receipt.axioms.len());
                        println!("Hash: {}", &receipt.hash[..16]);
                        println!("Timestamp: {}", receipt.timestamp);
                        println!();
                        println!("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]");
                    }
                    
                    // Write to file if specified
                    if let Some(output_path) = output {
                        let json = receipt.to_json()?;
                        fs::write(&output_path, json)?;
                        if !cli.json {
                            println!("\nReceipt written to: {}", output_path);
                        }
                    }
                }
                Err(e) => {
                    if cli.json {
                        let output_data = serde_json::json!({
                            "status": "FAILED",
                            "error": e.to_string()
                        });
                        println!("{}", serde_json::to_string_pretty(&output_data)?);
                    } else {
                        eprintln!("✗ Proof failed: {}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Verify { receipt_file } => {
            let content = fs::read_to_string(&receipt_file)?;
            let receipt: Receipt = serde_json::from_str(&content)?;
            
            let engine = ProofEngine::new();
            
            match engine.verify_receipt(&receipt, mock_verify) {
                Ok(true) => {
                    if cli.json {
                        let output_data = serde_json::json!({
                            "status": "VALID",
                            "c_zero": receipt.c_zero,
                            "claim": receipt.claim,
                            "hash": receipt.hash
                        });
                        println!("{}", serde_json::to_string_pretty(&output_data)?);
                    } else {
                        println!("✓ Receipt is VALID");
                        println!();
                        println!("Claim: {}", receipt.claim);
                        println!("C=0: {}", receipt.c_zero);
                        println!("Hash verified: ✓");
                        println!("Signature verified: ✓");
                    }
                }
                Ok(false) | Err(_) => {
                    if cli.json {
                        let output_data = serde_json::json!({
                            "status": "INVALID",
                            "claim": receipt.claim
                        });
                        println!("{}", serde_json::to_string_pretty(&output_data)?);
                    } else {
                        eprintln!("✗ Receipt is INVALID");
                    }
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Axioms { domain } => {
            let ssot = OmegaSSoT::new();
            
            let axioms: Vec<_> = if let Some(d) = &domain {
                ssot.core_axioms.by_domain(d)
            } else {
                ssot.core_axioms.all().collect()
            };
            
            if cli.json {
                let output_data: Vec<_> = axioms.iter().map(|a| {
                    serde_json::json!({
                        "id": a.id,
                        "name": a.name,
                        "statement": a.statement,
                        "domain": a.domain
                    })
                }).collect();
                println!("{}", serde_json::to_string_pretty(&output_data)?);
            } else {
                println!("Ω-SSOT Axioms");
                println!("=============");
                if let Some(d) = &domain {
                    println!("Domain: {}", d);
                }
                println!();
                
                for axiom in axioms {
                    println!("[{}] {}", axiom.id, axiom.name);
                    println!("  Statement: {}", axiom.statement);
                    println!("  Domain: {}", axiom.domain);
                    println!();
                }
            }
        }
        
        Commands::Check { claim, evidence } => {
            let engine = ProofEngine::new();
            
            match engine.verify_claim(&claim, &evidence) {
                Ok(supported) => {
                    if cli.json {
                        let output_data = serde_json::json!({
                            "claim": claim,
                            "supported": supported,
                            "c_zero": supported
                        });
                        println!("{}", serde_json::to_string_pretty(&output_data)?);
                    } else {
                        if supported {
                            println!("✓ Claim is SUPPORTED by evidence (C=0)");
                        } else {
                            println!("✗ Claim is NOT SUPPORTED by evidence");
                        }
                    }
                    
                    if !supported {
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    if cli.json {
                        let output_data = serde_json::json!({
                            "claim": claim,
                            "supported": false,
                            "error": e.to_string()
                        });
                        println!("{}", serde_json::to_string_pretty(&output_data)?);
                    } else {
                        eprintln!("✗ Check failed: {}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Info => {
            let ssot = OmegaSSoT::new();
            
            if cli.json {
                let output_data = serde_json::json!({
                    "name": "SAP-4D Proof Engine",
                    "version": "1.0.0",
                    "substrate": sap4d::SUBSTRATE,
                    "projection": sap4d::PROJECTION,
                    "omega_ssot_version": ssot.version,
                    "omega_ssot_hash": ssot.hash(),
                    "axiom_count": ssot.core_axioms.len(),
                    "policy": "C = 0"
                });
                println!("{}", serde_json::to_string_pretty(&output_data)?);
            } else {
                println!("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]");
                println!();
                println!("SAP-4D Proof Engine v1.0.0");
                println!("==========================");
                println!();
                println!("Substrate: {}", sap4d::SUBSTRATE);
                println!("Projection: {}", sap4d::PROJECTION);
                println!("Ω-SSOT Version: {}", ssot.version);
                println!("Ω-SSOT Hash: {}...", &ssot.hash()[..16]);
                println!("Core Axioms: {}", ssot.core_axioms.len());
                println!("Policy: C = 0 (Zero Contradiction)");
                println!();
                println!("Mode: Proof Over Persuasion");
            }
        }
    }
    
    Ok(())
}

