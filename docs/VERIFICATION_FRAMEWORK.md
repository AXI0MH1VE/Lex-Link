# Axiom Hive: Framework for Certified, Verifiable Intelligence

## Overview

The Verification Framework provides a complete system for creating, managing, and verifying proof artifacts that enable independent validation of AI system outputs. This framework implements the principles outlined in the comprehensive specification document.

## Core Components

### 1. Verification Bundle

The `VerificationBundle` is the central artifact that packages all evidence needed for independent verification:

- **Provenance**: Complete tracking of inputs, models, environments, and configurations
- **Execution Trace**: Optional intermediate computation steps
- **Tests**: Verification tests with expected outcomes
- **Outputs**: Final artifacts with content addresses
- **Signatures**: Cryptographic attestations from signers

### 2. Proof Artifact Builder

The `ProofArtifactBuilder` provides a fluent API for constructing verification bundles:

```rust
use verification::{ProofArtifactBuilder, ModelMetadata, EnvironmentManifest, DeterministicConfig};

let bundle = ProofArtifactBuilder::new()
    .with_model(model_metadata)
    .with_environment(environment_manifest)
    .with_config(deterministic_config)
    .with_input(input_provenance)
    .add_execution_step("preprocess", "sha256:preprocess_hash")
    .add_test("determinism_check", TestType::Determinism, "sha256:expected", Tolerance::Exact)
    .add_output("result", "sha256:result_hash", "hash://sha256/result")
    .build()?;
```

### 3. Verifier

The `Verifier` enables independent replay and validation:

```rust
use verification::Verifier;

let verifier = Verifier::new(|hash, sig| verify_signature(hash, sig));
let result = verifier.verify(&bundle);

if result.passed {
    println!("Verification successful!");
} else {
    for error in result.errors {
        eprintln!("Error: {}", error);
    }
}
```

### 4. Deterministic Configuration

Ensures reproducible inference through:

- **Fixed Seeds**: Master seed with PRNG state capture
- **Parameter Locking**: Temperature=0.0, top_p=1.0 for determinism
- **State Management**: Capture and restore PRNG state for replay

### 5. Attestation Chains

Cryptographic attestations linked in chains:

- **Root Attestation**: System identity
- **Chain Entries**: Linked sequence with previous hash references
- **Integrity Verification**: Chain validation ensures tamper-evidence

## Usage Examples

### Creating a Verification Bundle

```rust
use verification::*;
use chrono::Utc;

// Model metadata
let model = ModelMetadata {
    name: "axiomhive-llm-x".to_string(),
    version: "v3.2.1".to_string(),
    weights_hash: "sha256:...".to_string(),
    tokenizer_hash: "sha256:...".to_string(),
    card_uri: Some("https://...".to_string()),
};

// Environment manifest
let env = EnvironmentManifest {
    container_image_hash: "sha256:...".to_string(),
    os: "ubuntu:22.04".to_string(),
    deps: vec![
        Dependency {
            name: "torch".to_string(),
            version: "2.4.0".to_string(),
            hash: "sha256:...".to_string(),
        }
    ],
    hardware: Some(HardwareProfile {
        cpu: "x86_64".to_string(),
        gpu: Some("nvidia-a100".to_string()),
        driver: Some("535.54".to_string()),
        cuda: Some("12.1".to_string()),
    }),
};

// Deterministic config
let config = DeterministicConfig {
    seed: 42,
    parameters: ModelParameters {
        temperature: 0.0,
        top_p: 1.0,
        top_k: None,
        max_tokens: 1024,
        extra: Default::default(),
    },
};

// Input provenance
let input = DataProvenance {
    name: "input_document".to_string(),
    hash: "sha256:...".to_string(),
    source_uri: Some("https://...".to_string()),
    license: Some("CC-BY-4.0".to_string()),
    timestamp: Utc::now(),
    transformations: vec![],
};

// Build bundle
let bundle = ProofArtifactBuilder::new()
    .with_model(model)
    .with_environment(env)
    .with_config(config)
    .with_input(input)
    .add_execution_step("preprocess", "sha256:preprocess")
    .add_execution_step("infer", "sha256:infer")
    .add_test("determinism", TestType::Determinism, "sha256:expected", Tolerance::Exact)
    .add_output("final_text", "sha256:output", "hash://sha256/output")
    .build()?;
```

### Verifying a Bundle

```rust
use verification::Verifier;

// Create verifier with signature verification function
let verifier = Verifier::new(|hash: &str, sig: &str| {
    // Your signature verification logic
    verify_cryptographic_signature(hash, sig)
});

// Verify bundle
let result = verifier.verify(&bundle);

println!("Verification passed: {}", result.passed);
for test_result in result.test_results {
    println!("Test {}: {}", test_result.test_name, 
             if test_result.passed { "PASS" } else { "FAIL" });
    println!("  {}", test_result.message);
}
```

### Attestation Chains

```rust
use verification::{Attestation, AttestationChain, SignerRole};
use chrono::Utc;

// Create root attestation
let root = Attestation {
    signer_id: "did:key:z6Mki...".to_string(),
    signature: "base64:...".to_string(),
    timestamp: Utc::now(),
    role: SignerRole::System,
    statement: None,
};

// Create chain
let mut chain = AttestationChain::new(root);

// Append approver attestation
let approver = Attestation {
    signer_id: "did:key:z6Mk2...".to_string(),
    signature: "base64:...".to_string(),
    timestamp: Utc::now(),
    role: SignerRole::Approver,
    statement: Some("Approved for production".to_string()),
};

chain.append(approver);

// Verify chain integrity
assert!(chain.verify_integrity());
```

## Bundle Schema

The verification bundle follows this JSON schema (see specification Appendix B):

```json
{
  "bundle_version": "1.0.0",
  "content_address": "hash://sha256/...",
  "created_at": "2025-12-04T12:15:00Z",
  "signatures": [...],
  "provenance": {
    "inputs": [...],
    "model": {...},
    "environment": {...},
    "config": {...}
  },
  "execution_trace": {...},
  "tests": [...],
  "outputs": [...]
}
```

## Integration with Existing Systems

The verification framework integrates with:

- **SAP-4D Proof Engine**: Uses receipts and causal chains
- **Audit System**: Leverages binary proofs and audit trails
- **DSIF**: Provides verification bundles for consensus decisions
- **Invariance Layer**: Enforces C=0 policy in verification tests

## Best Practices

1. **Always use deterministic configs** for reproducible inference
2. **Capture all inputs** with full provenance
3. **Include execution traces** for complex computations
4. **Sign bundles** with appropriate roles (system, approver, auditor)
5. **Add verification tests** for each claim type
6. **Store bundles** in content-addressed storage
7. **Enable independent verification** by publishing bundles

## Security Considerations

- **Content Addressing**: All artifacts referenced by hash
- **Cryptographic Signatures**: Non-repudiation and integrity
- **Attestation Chains**: Tamper-evident linked records
- **Deterministic Execution**: Prevents variance-based attacks
- **Provenance Tracking**: Full chain of custody

## Future Enhancements

- Integration with content-addressed storage (IPFS, etc.)
- Hardware-backed signature support (HSM, TPM)
- Temporal logic property checking
- Cross-organization verification
- Public trust registries
- SDK for common languages (Python, TypeScript)

## References

- Full specification: See framework document
- Bundle schema: `verification/src/bundle.rs`
- Examples: `verification/src/*/tests`

