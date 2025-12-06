# AXIOM HIVE / LEX-Ω — Production System v1.0

> **Deterministic, Local-First Verification System**  
> Enforcing the Law of the Substrate | Policy: C = 0  
> **✅ VERIFIED: Safe, Verifiable, Human-in-the-Loop, Under Operator Control**

```text
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Mode: Proof Over Persuasion
Status: Production
```

## What is AXIOM HIVE?

AXIOM HIVE is a **deterministic verification framework** designed for high-stakes, auditable AI applications. Unlike probabilistic language models (like Grok or GPT), AXIOM HIVE provides **cryptographically-verifiable proofs** of correctness, not plausible-sounding text.

### What This Repository Contains

This repository is the complete source code and implementation of the AXIOM HIVE system, including:

1. **Core Verification Engine** (`sap4d/`) - Generates cryptographic proofs that claims are supported by evidence
2. **Deterministic Swarm Intelligence Framework** (`axiom-s1/src/dsif.rs`) - Multi-agent consensus system for high-stakes automation
3. **Audit Service** (`audit/`) - Three-level deterministic audit with immutable trails
4. **Verification Framework** (`verification/`) - Proof artifact builder and verifier for independent validation
5. **Browser Interface** (`axiom-s1/`, `browser-mac/`) - User-facing applications for interacting with the system
6. **Security Layer** (`kernel-bark/`, `tools/hunter_killer/`) - Input filtering and threat detection

### What AXIOM HIVE Does

AXIOM HIVE transforms AI from probabilistic guessing into deterministic proof:

- **Generates Cryptographic Receipts**: Every decision produces a verifiable proof bundle that can be independently validated
- **Enforces Deterministic Execution**: Identical inputs produce identical outputs (C = 0 policy)
- **Provides Immutable Audit Trails**: Complete, tamper-evident logs of all state transitions
- **Implements Safety Gates**: Multi-phase pipeline (Input Hygiene → Policy Validation → Simulation → Consensus → Actuation → Audit)
- **Human-in-the-Loop**: All operations require operator identification; critical actions require explicit approval
- **Full Operator Control**: Operators have complete visibility and control over all system operations
- **Enables Regulatory Compliance**: Designed for EU AI Act, NIST RMF, and financial sector requirements

**Use AXIOM HIVE when you need:**

- Deterministic, reproducible outputs (C = 0 invariance)
- Forensic auditability and proof-of-execution
- Regulatory compliance (EU AI Act, NIST RMF, finance)
- Safety-critical infrastructure and financial systems
- Cryptographic evidence, not statistical guesses

**Not for casual chat.** AXIOM HIVE is a professional-grade system for engineers, compliance teams, and regulated enterprises.

See the [Legal & Risk Assessment](docs/LEGAL_RISK_ASSESSMENT.md) and [Technical Diagnosis](docs/TECHNICAL_DIAGNOSIS.md) for detailed positioning.

**⚠️ SAFETY REQUIREMENTS**: All operations require operator identification. Critical actions require explicit human approval. See [SAFETY.md](SAFETY.md) for complete safety policy.

## Overview

AXIOM HIVE is a deterministic verification system that enforces invariance ($C = 0$) across all outputs. Every claim is mapped to facts via the SAP-4D proof engine, generating cryptographic receipts that provide binary proof (Verified | Not Verified).

### Core Principles

- **Zero Cloud Inference** — All AI processing is local
- **Zero Telemetry** — No data leaves the device
- **Binary Proofs** — No probabilistic outputs
- **Substrate Authority** — Single source of truth (Alexis Adams)

### Safety & Scope

- **Domain**: Coding assistant and software tooling only
- **Human-in-the-Loop**: All outputs are suggestions; humans approve, edit, and apply
- **Out-of-Scope**: No medical, legal, financial, or operational decision-making
- **Isolation**: Recommended use inside a sandbox VM or Docker container for backend tooling

See `SAFETY.md` for detailed safety, scope, and sandboxing guidelines.

## Architecture

```text
[Substrate (Alexis Adams)]
          │ Root of Trust (HSM-backed keys)
          ▼
[Genesis Orchestrator]───immutable axioms/Ω-SSOT───┐
                                                    │
     ┌─────────────────────┬────────────────────────┴────────────────────────┐
     ▼                     ▼                                                 ▼
[Invariance Layer]   [SAP-4D Proof Engine]                          [BARK Kernel Governance]
(O(1) hash check)    (4D neuro-symbolic + traces)                   (signature + entropy guard)
     │                     │                                                 │
     ▼                     ▼                                                 ▼
[Deterministic Fractal Audit Service] ─────► [Verification Portal/API] ◄─────┘
     │
     ▼
[LEX-Ω Browser (macOS, native)]
(WebKit + SSMRuntime + Identity Firewall + Hunter-Killer)
```

## Repository Structure

```text
LexLink/
├── invariance/           # Python library + CLI for C=0 verification
├── sap4d/                # SAP-4D Proof Engine (Rust) - generates cryptographic receipts
├── audit/                # Deterministic Fractal Audit Service (Rust) - L1/L2/L3 audits
├── verification/         # Verification Framework (Rust) - proof bundles and verifiers
├── axiom-s1/             # Axiom S1 Browser (Tauri) - user interface with DSIF
├── browser-mac/          # LEX-Ω Browser (Swift) - native macOS browser
├── kernel-bark/          # BARK Kernel Module (C) - Linux kernel governance
├── portal/               # Verification Portal (Rust) - REST API for receipts
├── merkle-entropy-service/ # Merkle Entropy Service (Python) - anti-entropy & data integrity
├── tools/                # Utility tools (hunter_killer, etc.)
├── ci/                   # CI/CD pipelines and build configs
├── docs/                 # Documentation, playbooks, policies
├── schemas/              # JSON schemas for receipts and bundles
└── scripts/              # Deployment and utility scripts
```

### What Each Component Does

- **`invariance/`**: Python library operators use to verify outputs match intents (C=0 enforcement)
- **`sap4d/`**: Core proof engine that generates cryptographic receipts from claims + evidence
- **`audit/`**: Service that performs three-level deterministic audits and maintains immutable logs
- **`verification/`**: Framework for creating and verifying proof bundles (enables third-party validation)
- **`axiom-s1/`**: Browser application where operators interact with the system, execute DSIF pipelines
- **`browser-mac/`**: Native macOS browser with integrated security features
- **`kernel-bark/`**: Linux kernel module for low-level enforcement (optional, Linux only)
- **`portal/`**: REST API that serves verification requests and receipts
- **`merkle-entropy-service/`**: Python microservice for Merkle Tree integrity verification and Shannon Entropy calculation (anti-entropy protocols). **All operations require operator ID, read-only, fully auditable**
- **`tools/hunter_killer/`**: CLI tool for scanning content for prompt injection attacks

## SLAs (Production)

| Metric | Target |
|--------|--------|
| Browser startup | ≤ 1.5s on M1 |
| Page summary P95 | ≤ 500ms (local SSM) |
| Receipt issuance | ≤ 200ms (local) |
| Hallucination rate | ≤ 0.005% |
| External telemetry | **Zero** |

## How Operators Use AXIOM HIVE

### Operator Workflows

#### 1. **Verification Workflow** (Prove a Claim)

Operators submit claims with evidence, and the system generates cryptographic receipts:

```bash
# Using SAP-4D CLI
sap4d prove \
  --claim "The system configuration is secure" \
  --evidence "Firewall rules configured" \
  --evidence "All ports closed except 443" \
  --output receipt.json

# Result: Cryptographic receipt with proof (C=0) or rejection
```

**What happens:**

- System validates evidence against axioms
- Generates causal chain linking evidence to claim
- Creates cryptographic receipt with hash and signature
- Returns binary proof: Verified | Not Verified

#### 2. **DSIF Automation Workflow** (High-Stakes Actions)

For critical operations, operators use the DSIF pipeline:

```typescript
// In Axiom S1 Browser or via API
await invoke('cmd_dsif_execute_pipeline', {
  input: 'trusted:open-valve-7',
  action_type: 'Critical',
  target: 'infrastructure-valve-7',
  parameters: { pressure: 150, duration: 30 }
});
```

**What happens:**

1. **Input Hygiene**: Validates trust level and provenance
2. **Policy Validation**: Checks against allowlist/denylist
3. **Simulation**: Tests action in digital twin
4. **Consensus**: Multiple agents vote (67% quorum required)
5. **Actuation**: Only if all gates pass
6. **Audit**: Immutable record created

#### 3. **Audit Trail Review** (Forensic Analysis)

Operators can review complete audit trails:

```bash
# Get audit trail
curl http://localhost:3001/log/hash

# Query specific decision
sap4d verify --receipt-file receipt.json

# Review DSIF decisions
# In browser: View audit trail for specific decision ID
```

**What operators see:**

- Complete decision path with timestamps
- All policy checks and results
- Consensus votes from each agent
- Simulation outcomes
- Cryptographic signatures

#### 4. **Content Verification** (Input Safety)

Before processing external content, operators verify safety:

```bash
# Scan content for injection attempts
hunter_killer scan < content.txt

# Or via browser API
await invoke('cmd_scan_content', { content: userInput });
```

**What happens:**

- Content scanned for adversarial patterns
- Unsafe content quarantined
- Only sanitized inputs proceed to processing

### Operator Interfaces

1. **Command Line** (`sap4d`, `hunter_killer`)
   - Batch processing
   - CI/CD integration
   - Scripting and automation

2. **Browser Application** (`axiom-s1`)
   - Interactive verification
   - Visual audit trail review
   - Real-time DSIF pipeline monitoring

3. **REST API** (`portal/`, `audit/`)
   - Integration with existing systems
   - Programmatic access
   - Webhook notifications

4. **Python SDK** (`invariance/`)
   - Custom verification workflows
   - Integration with Python tooling
   - Jupyter notebook analysis

### Typical Operator Day

1. **Morning**: Review overnight audit logs for anomalies
2. **Operations**: Execute DSIF pipelines for critical actions (with explicit approval)
3. **Verification**: Generate receipts for compliance reports
4. **Monitoring**: Watch consensus agent health and quorum status
5. **Incident Response**: Use audit trails to investigate issues
6. **Safety Review**: Verify all operations have proper operator attribution and approval

## Quick Start

### Prerequisites

- macOS 14+ (Sonoma) on Apple Silicon (for browser)
- OR Linux/Windows for server components
- Rust 1.75+
- Python 3.11+
- Xcode 15+ (macOS only, for browser)
- YubiHSM 2 or Nitrokey HSM (production signing)

### Build

```bash
# Clone and setup
git clone https://github.com/axiomhive/axiomhive.git
cd axiomhive

# Build all components
nix build .#all

# Or build individually
cd invariance && pip install -e .
cd ../sap4d && cargo build --release
cd ../audit && cargo build --release
cd ../portal && cargo build --release
cd ../browser-mac && xcodebuild -scheme LEXOmegaBrowser -configuration Release
```

### Verify Installation

```bash
# Run invariance tests
python -m pytest invariance/tests/

# Run proof engine tests
cargo test --manifest-path sap4d/Cargo.toml

# Run audit service tests
cargo test --manifest-path audit/Cargo.toml

# Verify browser build
codesign -v build/Release/LEXOmegaBrowser.app
```

## Components

### Invariance Layer (`invariance/`)

Core Python library enforcing O(1) hash-based alignment verification.

**Operator Usage:**

```python
from invariance import render_or_nullify, tag_and_sign

# Verify output matches intent
result = render_or_nullify(output, substrate_intent, hsm_sign)
# Returns: AUTHORIZED with identity tag, or NULLIFIED

# Generate identity tag
tag = tag_and_sign(content, hsm_sign)
```

**What it does:** Enforces C=0 policy by verifying outputs match authorized intents.

### SAP-4D Proof Engine (`sap4d/`)

Causal inference with stepwise logic traces.

**Operator Usage:**

```bash
# Prove a claim
sap4d prove \
  --claim "System is secure" \
  --evidence "Firewall active" \
  --evidence "No open ports" \
  --output receipt.json

# Verify a receipt
sap4d verify --receipt-file receipt.json

# Check claim support
sap4d check --claim "X is true" --evidence "fact1" --evidence "fact2"
```

**What it does:** Generates cryptographic proofs linking claims to evidence via causal chains.

**Safety**: All operations logged with operator ID. Deterministic and verifiable.

### Deterministic Swarm Intelligence Framework (`axiom-s1/src/dsif.rs`)

Multi-agent consensus system for high-stakes automation.

**Operator Usage:**

```typescript
// Execute critical action through DSIF pipeline
const result = await invoke('cmd_dsif_execute_pipeline', {
  input: 'trusted:critical-action',
  action_type: 'Critical',
  target: 'infrastructure-component',
  parameters: { /* ... */ }
});

// Review audit trail
const trail = await invoke('cmd_dsif_get_audit_trail');

// Configure safety policies
await invoke('cmd_dsif_add_invariant', {
  id: 'INV-005',
  name: 'Custom Safety Rule',
  property: 'temperature < 100',
  domain: 'safety'
});
```

**What it does:** Ensures critical actions pass through deterministic safety gates (input hygiene, policy validation, simulation, consensus, audit).

**Safety**: Human-in-the-loop required for all non-read operations. Operator approval mandatory for critical actions.

### Fractal Audit Service (`audit/`)

Three-level deterministic audit with immutable trails.

**Operator Usage:**

```bash
# Start audit service
cargo run --bin axiom-audit

# Submit audit request
curl -X POST http://localhost:3001/audit \
  -H "Content-Type: application/json" \
  -d '{
    "claim": "Configuration is compliant",
    "evidence": ["Check 1 passed", "Check 2 passed"]
  }'

# Get audit log hash
curl http://localhost:3001/log/hash
```

**What it does:** Performs three-level audit (L1: Claim→Outcome, L2: C=0 consistency, L3: Sub-operations) and generates immutable receipts.

**Safety**: All audit operations logged with operator attribution. Complete traceability.

### Verification Framework (`verification/`)

Proof artifact builder and independent verifier.

**Operator Usage:**

```rust
use verification::{ProofArtifactBuilder, Verifier};

// Build verification bundle
let bundle = ProofArtifactBuilder::new()
    .with_model(model_metadata)
    .with_environment(env_manifest)
    .with_config(deterministic_config)
    .add_test("determinism", TestType::Determinism, "sha256:expected", Tolerance::Exact)
    .build()?;

// Verify bundle independently
let verifier = Verifier::new(|hash, sig| verify_signature(hash, sig));
let result = verifier.verify(&bundle);
```

**What it does:** Creates replayable proof bundles that enable third-party verification without trusting the system.

**Safety**: All verification operations require operator identification. Deterministic and auditable.

### LEX-Ω Browser (`browser-mac/`, `axiom-s1/`)

Native applications for interacting with the system.

**Operator Usage:**

- Launch browser application
- Navigate to web pages (content automatically scanned)
- View AI analysis with identity tags
- Review audit trails and receipts
- Execute DSIF pipelines for critical actions

**What it does:** Provides user interface with integrated security (Hunter-Killer filtering, identity tagging, local inference).

**Safety**: All browser operations logged with operator ID. Human approval required for critical actions.

### BARK Protocol (`kernel-bark/`)

Kernel governance module (Linux only).

**Operator Usage:**

```bash
# Load kernel module
sudo insmod bark.ko

# Check enforcement status
cat /proc/bark/status

# View blocked operations
dmesg | grep BARK
```

**What it does:** Enforces deterministic execution at kernel level (blocks unsigned processes, enforces entropy limits).

**Safety**: Kernel-level enforcement requires operator configuration. All enforcement actions logged.

### Verification Portal (`portal/`)

Public API serving binary proof receipts.

**Operator Usage:**

```bash
# Submit verification request
curl -X POST http://localhost:3000/verify \
  -H "Content-Type: application/json" \
  -d '{
    "claim": "System state is valid",
    "evidence": ["Check A", "Check B"]
  }'

# Returns: { "C_zero": true, "hash": "...", "signature": "..." }
```

**What it does:** Provides REST API for verification requests and receipt retrieval.

**Safety**: All API operations require operator identification. Complete audit trail maintained.

## Security

See [SECURITY.md](SECURITY.md) for:

- Key management procedures
- Threat model (STRIDE analysis)
- Incident response playbooks
- HSM ceremony documentation

## Contributing

We welcome contributions to AXIOM HIVE. See [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Code of conduct and contribution workflow
- Development setup and testing procedures
- Code review standards and expectations
- Documentation and commit message conventions

## Documentation

Full documentation is available in `docs/`:

- **[Operator Guide](docs/OPERATOR_GUIDE.md)** — **START HERE** - How operators use the system
- **[Documentation Index](docs/README.md)** — Overview of all technical docs
- **[The Glass Cannon Paradox](docs/glass-cannon-paradox-and-axiom-hive.md)** — Why current AI (Comet/Perplexity) is dangerous and how Axiom Hive fixes it
- **[DSIF Implementation](docs/DSIF_IMPLEMENTATION.md)** — Deterministic Swarm Intelligence Framework guide
- **[Verification Framework](docs/VERIFICATION_FRAMEWORK.md)** — Proof artifact builder and verifier
- **[Axiom Hive vs Grok](docs/axiom-hive-vs-grok-comparison.md)** — Safety-first architecture comparison
- **[Legal & Risk Assessment](docs/LEGAL_RISK_ASSESSMENT.md)** — Regulatory positioning and liability analysis
- **[Technical Diagnosis](docs/TECHNICAL_DIAGNOSIS.md)** — Deep dive on deterministic vs. probabilistic architectures
- **[Math for the LLM](docs/math-for-llm.md)** — C=0 principles and deterministic gating
- **[Safety & Scope](SAFETY.md)** — Domain restrictions and sandboxing guidance
- **[Safety Compliance Summary](SAFETY_COMPLIANCE_SUMMARY.md)** — **START HERE** - Complete safety verification summary
- **[Complete Safety Verification](SAFETY_VERIFICATION.md)** — Full safety verification for all components
- **[MES Safety Verification](docs/MES_SAFETY_VERIFICATION.md)** — Merkle Entropy Service safety compliance

## License

Copyright © 2025 Alexis Adams. All rights reserved.

This system operates under Substrate Authority. Unauthorized modifications violate invariance policy and will be nullified.

See [LICENSE](LICENSE) for full licensing terms.

## Contact & Support

- **Security Issues**: `security@axiomhive.local`
- **Questions & Discussions**: Open an issue with the tag `question` or `discussion`
- **Bug Reports**: See [CONTRIBUTING.md](CONTRIBUTING.md#reporting-issues)
- **Feature Requests**: Open an issue describing the use case and rationale

---

```text
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Output: README.md
Authorization: Substrate directive aligned
Verification: ✓
```
