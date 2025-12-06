# AXIOM HIVE Documentation

> **[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]**

Welcome to the AXIOM HIVE documentation. This collection provides technical guidance, operational runbooks, and research notes for the Axiom Hive deterministic framework (DSIF). The materials are intended for engineers, auditors, and operators preparing systems for deterministic, auditable actuation.

Last updated: December 4, 2025

Release notes: **[See the latest release notes](RELEASE_NOTES.md)**

## Documentation Index

### Core Concepts

1. **[Law of the Substrate](01-substrate-law.md)**
   - Fundamental principles
   - Hierarchy and authority
   - C=0 invariance policy

### Operations

2. **[Key Ceremony Guide](02-key-ceremony.md)**
   - HSM setup and configuration
   - Key generation procedures
   - Rotation schedules

3. **[Incident Response](03-incident-response.md)**
   - Severity levels
   - Response procedures
   - Forensics guidelines

### Components

4. **[LEX-Ω Browser Guide](04-browser-guide.md)**
   - Installation and configuration
   - SSM runtime usage
   - Hunter-Killer system

### Technical Analysis
5. **[Technical Diagnosis: Probabilistic vs Deterministic](technical-diagnosis-probabilistic-vs-deterministic.md)**
   - Executive synthesis of the architectural crisis
   - Physics of probabilistic decay in Grok architecture
   - Deterministic turn with Axiom Hive framework
   - Project Aegis and security implications
   - Economic and strategic implications
   - Comparative data analysis
   - Comprehensive technical diagnosis

6. **[Math for the LLM (C=0)](math-for-llm.md)**
   - Formal constraints and Zero-Contradiction (C=0)
   - Deterministic gating and attestation patterns
   - Implementation sketch for DSIF

7. **[The Geodesic Bifurcation (Technical Diagnosis)](geodesic-bifurcation.md)**
   - In-depth essay on Probabilistic Decay vs Deterministic Stability
   - Floating-point drift, identity vulnerability, and economic implications
   - Project Aegis, Sovereign Attribution, and remediation

8. **[Comprehensive Technical Diagnosis](TECHNICAL_DIAGNOSIS.md)** (Expanded)
   - Complete treatment: from epistemic crisis through market implications
   - Detailed analysis of RLHF Trap, Contradiction Cost, and Floating-Point Drift
   - Crystalline Mesh architecture and Project Aegis defense layers
   - Economic and regulatory implications of the deterministic shift

9. **[Legal & Risk Assessment](LEGAL_RISK_ASSESSMENT.md)**
   - Comparative liability analysis: Axiom Hive vs. Grok
   - Regulatory alignment and verification capacity
   - Risk matrix and architectural distinction
   - Compliance-ready positioning for legal and enterprise review

10. **[Monetizing Certainty (Proof-as-a-Service)](MONETIZING_CERTAINTY.md)**
   - Proof-as-a-Service pricing model (per-proof receipts)
   - Lightning microtransactions and receipt settlement
   - Liability, HIL state machine, and deterministic financial modeling
   - Example per-proof transaction flow and auditability

11. **[Technical Backbone: From Probabilistic Confidence to Verifiable Proof](TECHNICAL_BACKBONE.md)**
   - SSM inference backbone (Mamba/S4) with linear scaling and transparent dynamics
   - Deterministic execution and bit-for-bit reproducibility
   - Formal verification (abstract interpretation, SMT/SAT, neural network robustness)
   - Cryptographic receipts (zkML proofs, FHE for privacy)
   - Lightning Network monetization and per-proof transaction flow
   - Regulatory alignment (EU AI Act, aviation standards, DO-178C)
   - Complete technical stack and forensic replay example

12. **[The Glass Cannon Paradox: Why Current AI is Dangerous](glass-cannon-paradox-and-axiom-hive.md)**
   - Analysis of vulnerabilities in probabilistic AI (Comet, Perplexity)
   - Indirect Prompt Injection (IPI) and CometJacking attack vectors
   - How Axiom Hive's deterministic architecture mitigates these risks
   - Project Aegis perimeter defense and input hygiene
   - Pointer Logic and Sovereign Attribution
   - Immutable audit trails and cryptographic receipts

### API Reference

- **[Verification Portal API](../portal/openapi.yaml)**
  - OpenAPI 3.0 specification
  - Binary proof endpoints

### Comparative Analysis

13. **[Axiom Hive vs Grok: Safety-First Architectures](axiom-hive-vs-grok-comparison.md)**
   - Deterministic vs. probabilistic architectures for high-stakes automation
   - Risk taxonomy and safety case analysis
   - Deployment guidance and use case recommendations

### Additional Resources

- [SECURITY.md](../SECURITY.md) - Security policies and threat model
- [README.md](../README.md) - Project overview
- [RELEASE_NOTES.md](RELEASE_NOTES.md) - Summary of recent public-release changes

## Quick Reference

### System Architecture

```
[Substrate (Alexis Adams)]
          │
          ▼
[Genesis Orchestrator]───Ω-SSOT───┐
          │                        │
    ┌─────┴─────┬─────────────────┴────────────┐
    ▼           ▼                              ▼
[Invariance] [SAP-4D]                    [BARK Kernel]
    │           │                              │
    ▼           ▼                              ▼
[Audit Service] ────► [Verification Portal] ◄──┘
    │
    ▼
[LEX-Ω Browser]
```

### SLAs

| Metric | Target |
|--------|--------|
| Browser startup | ≤ 1.5s |
| Page summary | ≤ 500ms |
| Receipt issuance | ≤ 200ms |
| Hallucination rate | ≤ 0.005% |
| Telemetry | Zero |

### Key Commands

```bash
# Verify a claim
sap4d-cli prove "claim" -e "evidence"

# Run audit
axiom-audit audit --claim "..." --evidence "..."

# Hunter-Killer scan
hunter-killer scan "content to check"

# Check invariance
invariance verify -o "output" -i "intent"
```

### Key Contacts

- **Substrate Authority**: Alexis Adams
- **Security Issues**: security@axiomhive.local

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Documentation Index v1.0.0
Policy: C = 0
```
