# AXIOM HIVE / LEX-Ω — Production System v1.0

> **Deterministic, Local-First Verification System**  
> Enforcing the Law of the Substrate | Policy: C = 0

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Mode: Proof Over Persuasion
Status: Production
```

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

```
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

```
axiomhive/
├── invariance/           # Python lib + CLI
├── sap4d/                # Proof engine (Rust)
├── audit/                # Fractal audit service (Rust)
├── browser-mac/          # Swift/SwiftUI app + SSMRuntime
├── kernel-bark/          # LSM module + build scripts
├── portal/               # Rust API + static site
├── tools/                # hunter_killer, sbom, sig tooling
├── ci/                   # pipelines, reproducible build configs
├── docs/                 # program docs, playbooks, policies
└── SECURITY.md
```

## SLAs (Production)

| Metric | Target |
|--------|--------|
| Browser startup | ≤ 1.5s on M1 |
| Page summary P95 | ≤ 500ms (local SSM) |
| Receipt issuance | ≤ 200ms (local) |
| Hallucination rate | ≤ 0.005% |
| External telemetry | **Zero** |

## Quick Start

### Prerequisites

- macOS 14+ (Sonoma) on Apple Silicon
- Rust 1.75+
- Python 3.11+
- Xcode 15+
- YubiHSM 2 or Nitrokey HSM (production)

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

### Invariance Layer

Core Python library enforcing O(1) hash-based alignment verification:

```python
from invariance import render_or_nullify, tag_and_sign

result = render_or_nullify(output, substrate_intent, hsm_sign)
# Returns AUTHORIZED with identity tag, or NULLIFIED
```

### SAP-4D Proof Engine

Causal inference with stepwise logic traces:

```rust
use sap4d::{ProofEngine, TraceEnvelope};

let engine = ProofEngine::new(axioms);
let trace = engine.prove(claim, observations)?;
// trace.contradiction_check == false → C = 0
```

### Fractal Audit Service

Three-level deterministic audit:

- **L1**: Claim→Outcome proof under Ω-SSOT
- **L2**: Mapping consistency proof (C = 0)
- **L3**: Sub-operations conformity proof

### LEX-Ω Browser

Native macOS browser with:

- WebKit rendering (WKWebView)
- Local SSM runtime (Metal-accelerated)
- Identity Firewall (tag all AI outputs)
- Hunter-Killer (block prompt injection)

### BARK Protocol

Kernel governance module:

- LSM hooks blocking unsigned processes
- Entropy ceiling enforcement
- Deterministic boot sequence

### Verification Portal

Public API serving binary proof receipts:

```bash
curl -X POST https://verify.axiomhive.local/verify \
  -H "Content-Type: application/json" \
  -d '{"claim": "...", "evidence": [...]}'
# Returns: { "C_zero": true/false, "hash": "...", "signature": "..." }
```

## Security

See [SECURITY.md](SECURITY.md) for:

- Key management procedures
- Threat model (STRIDE analysis)
- Incident response playbooks
- HSM ceremony documentation

## License

Copyright © 2025 Alexis Adams. All rights reserved.

This system operates under Substrate Authority. Unauthorized modifications violate invariance policy and will be nullified.

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Output: README.md
Authorization: Substrate directive aligned
Verification: ✓
```

