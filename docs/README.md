# AXIOM HIVE Documentation

> **[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]**

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

### API Reference

- **[Verification Portal API](../portal/openapi.yaml)**
  - OpenAPI 3.0 specification
  - Binary proof endpoints

### Additional Resources

- [SECURITY.md](../SECURITY.md) - Security policies and threat model
- [README.md](../README.md) - Project overview

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

