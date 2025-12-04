# DSIF Implementation Guide

## Overview

The Deterministic Swarm Intelligence Framework (DSIF) has been integrated into the Axiom Hive system. DSIF provides a multi-agent consensus system with deterministic state transitions, formal verifiability, and strict action gating for high-stakes automation.

## Architecture

DSIF implements a six-phase pipeline:

1. **Input Hygiene** - Filters and quarantines untrusted inputs
2. **Policy Validation** - Validates actions against allowlist/denylist
3. **Simulation-before-actuation** - Simulates actions to predict outcomes
4. **Consensus Gating** - Multi-agent voting with quorum thresholds
5. **Controlled Actuation** - Executes approved actions safely
6. **Immutable Audit** - Records all state transitions immutably

## Components

### Agents

DSIF uses multiple agents with different roles:
- **Validator** - Validates inputs and enforces policies
- **Simulator** - Simulates actions before actuation
- **Consensus** - Participates in consensus voting (multiple agents)
- **Executor** - Executes approved actions
- **Auditor** - Audits all state transitions

### Invariants

Default safety invariants:
- **INV-001**: Zero Contradiction (C = 0)
- **INV-002**: Deterministic Output
- **INV-003**: Causal Closure
- **INV-004**: No Unauthorized Operations

### Trust Levels

Inputs are classified by trust level:
- **Untrusted** - Quarantined, advisory only
- **Verified** - Basic checks passed
- **Attested** - Cryptographic attestation present
- **Trusted** - Full provenance chain

## Usage

### Tauri Commands

DSIF is exposed via Tauri commands:

#### Execute Pipeline
```typescript
await invoke('cmd_dsif_execute_pipeline', {
  input: 'trusted:action input',
  action_type: 'Read', // or 'Write', 'Critical', 'Config'
  target: 'target-resource',
  parameters: { key: 'value' }
});
```

#### Get Audit Trail
```typescript
const trail = await invoke('cmd_dsif_get_audit_trail');
```

#### Get Agents
```typescript
const agents = await invoke('cmd_dsif_get_agents');
```

#### Add Invariant
```typescript
await invoke('cmd_dsif_add_invariant', {
  id: 'INV-005',
  name: 'Custom Invariant',
  property: 'property expression',
  domain: 'domain'
});
```

#### Add to Allowlist
```typescript
await invoke('cmd_dsif_add_to_allowlist', { item: 'allowed-resource' });
```

#### Add to Denylist
```typescript
await invoke('cmd_dsif_add_to_denylist', { item: 'blocked-resource' });
```

## Input Format

Inputs must be prefixed with trust level:
- `trusted:` - Full trust
- `attested:` - Cryptographic attestation
- `verified:` - Basic verification
- No prefix - Untrusted (quarantined)

## Action Types

- **Read** - Read-only operation
- **Write** - Write operation requiring consensus
- **Critical** - Critical operation requiring full quorum
- **Config** - System configuration change

## Quorum Threshold

Default quorum threshold is 67% (0.67). This means at least 67% of consensus agents must approve an action for it to proceed.

## Security Features

1. **Adversarial Pattern Detection** - Detects common injection patterns
2. **Input Quarantining** - Untrusted inputs are isolated
3. **Provenance Tracking** - All inputs tracked with hash and timestamp
4. **Invariant Checking** - Actions validated against safety properties
5. **Simulation** - Actions simulated before execution
6. **Consensus Gating** - Multi-agent approval required
7. **Immutable Audit** - All decisions recorded with cryptographic hashes

## Integration Points

DSIF integrates with existing Axiom Hive components:
- Uses `invariance` module for C=0 checks
- Uses `sandbox` for secure execution
- Uses `hunter_killer` for input filtering
- Uses `cozo_db` for audit trail storage (future enhancement)

## Example Workflow

1. User submits action request
2. DSIF receives input with `trusted:` prefix
3. Input hygiene checks for adversarial patterns
4. Policy validation checks allowlist/denylist
5. Action simulated to predict outcomes
6. Invariants checked against action
7. Consensus agents vote on action
8. If quorum met, action executed
9. All steps recorded in immutable audit trail

## Future Enhancements

- Integration with CozoDB for persistent audit trails
- Enhanced simulation with resource prediction
- Dynamic agent trust scoring
- Temporal logic property checking
- Integration with SAP-4D proof engine
- Human-in-the-loop escalation for critical actions

