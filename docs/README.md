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

### API Reference

- **[Verification Portal API](../portal/openapi.yaml)**
  - OpenAPI 3.0 specification
  - Binary proof endpoints

### Additional Resources

- [SECURITY.md](../SECURITY.md) - Security policies and threat model
- [README.md](../README.md) - Project overview
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
