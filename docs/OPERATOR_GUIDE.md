# AXIOM HIVE Operator Guide

## What Operators Need to Know

This guide explains how operators (system administrators, compliance officers, engineers) use AXIOM HIVE in production.

## What is AXIOM HIVE?

AXIOM HIVE is a **deterministic verification system** that provides cryptographic proofs of correctness for AI-driven decisions. Unlike probabilistic systems that "guess" outputs, AXIOM HIVE generates verifiable, reproducible proofs.

### Key Concepts

- **C = 0 Policy**: Zero contradiction - all outputs must be consistent and verifiable
- **Deterministic Execution**: Identical inputs always produce identical outputs
- **Cryptographic Receipts**: Every decision generates a proof bundle that can be independently verified
- **DSIF Pipeline**: Six-phase safety gate for high-stakes actions

## Operator Workflows

### Workflow 1: Verify a Claim

**Use Case**: Prove that a system state or configuration is correct.

**Steps**:

1. **Prepare Evidence**
   ```bash
   # Collect evidence items
   evidence1="Firewall rules configured correctly"
   evidence2="All services running"
   evidence3="No security vulnerabilities detected"
   ```

2. **Generate Receipt**
   ```bash
   sap4d prove \
     --claim "System is secure and operational" \
     --evidence "$evidence1" \
     --evidence "$evidence2" \
     --evidence "$evidence3" \
     --output security-receipt.json
   ```

3. **Review Receipt**
   ```bash
   # View receipt contents
   cat security-receipt.json | jq .
   
   # Verify receipt integrity
   sap4d verify --receipt-file security-receipt.json
   ```

**What You Get**:
- Cryptographic receipt with hash and signature
- Binary proof: Verified (C=0) or Not Verified
- Causal chain linking evidence to claim
- Timestamp and audit trail reference

### Workflow 2: Execute Critical Action via DSIF

**Use Case**: Perform a high-stakes operation (e.g., infrastructure change, financial transaction).

**Steps**:

1. **Prepare Action Request**
   ```typescript
   // In Axiom S1 Browser or via API
   const action = {
     input: 'trusted:update-database-schema',
     action_type: 'Critical',
     target: 'production-database',
     parameters: {
       schema_version: 'v2.1',
       migration_script: 'sha256:abc123...'
     }
   };
   ```

2. **Execute Through DSIF Pipeline**
   ```typescript
   const result = await invoke('cmd_dsif_execute_pipeline', action);
   ```

3. **Monitor Pipeline Phases**
   - **Input Hygiene**: Check trust level and provenance
   - **Policy Validation**: Verify against allowlist/denylist
   - **Simulation**: Test action in sandbox
   - **Consensus**: Wait for agent votes (67% quorum)
   - **Actuation**: Execute if approved
   - **Audit**: Record immutable trail

4. **Review Decision**
   ```typescript
   // Get audit trail for this decision
   const trail = await invoke('cmd_dsif_get_audit_trail');
   const decision = trail.find(d => d.decision_id === result.decision.id);
   ```

**What You Get**:
- Decision approval or rejection with rationale
- Complete audit trail of all pipeline phases
- Consensus votes from each agent
- Simulation results and policy checks
- Cryptographic receipt for the decision

### Workflow 3: Audit Trail Review

**Use Case**: Investigate an incident or prepare compliance report.

**Steps**:

1. **Query Audit Service**
   ```bash
   # Get audit log root hash
   curl http://localhost:3001/log/hash
   
   # Get specific audit entry
   curl http://localhost:3001/audit/{receipt_hash}
   ```

2. **Review in Browser**
   - Open Axiom S1 Browser
   - Navigate to Audit Trail view
   - Filter by date, decision type, or agent
   - View complete decision paths

3. **Export for Compliance**
   ```bash
   # Export audit trail
   curl http://localhost:3001/log/export \
     --output audit-trail-$(date +%Y%m%d).json
   ```

**What You Get**:
- Immutable, tamper-evident audit log
- Complete decision history
- Cryptographic hashes for verification
- Human-readable rationales

### Workflow 4: Content Safety Verification

**Use Case**: Verify external content before processing.

**Steps**:

1. **Scan Content**
   ```bash
   # Using hunter_killer CLI
   hunter_killer scan < user-input.txt
   
   # Or via browser API
   const scanResult = await invoke('cmd_scan_content', {
     content: userInput
   });
   ```

2. **Review Detections**
   - Check for prompt injection patterns
   - Review threat severity
   - Decide on quarantine or sanitization

3. **Neutralize if Needed**
   ```bash
   # Neutralize detected threats
   hunter_killer neutralize < user-input.txt > sanitized.txt
   ```

**What You Get**:
- Threat detection report
- Sanitized content (if threats found)
- Audit log of scan operation

## Configuration

### Setting Up DSIF Policies

Operators configure safety policies:

```typescript
// Add custom invariant
await invoke('cmd_dsif_add_invariant', {
  id: 'INV-005',
  name: 'Temperature Limit',
  property: 'temperature < 100',
  domain: 'safety'
});

// Configure allowlist
await invoke('cmd_dsif_add_to_allowlist', {
  item: 'allowed-resource-1'
});

// Configure denylist
await invoke('cmd_dsif_add_to_denylist', {
  item: 'blocked-resource-1'
});
```

### Configuring Audit Service

```bash
# Set audit service configuration
export AUDIT_ENABLE_L3=true
export AUDIT_MAX_EVIDENCE=100
export AUDIT_ENABLE_LOGGING=true

# Start service
cargo run --bin axiom-audit
```

## Monitoring and Maintenance

### Health Checks

```bash
# Check all services
./scripts/health-check.sh

# Check specific service
curl http://localhost:3000/health
curl http://localhost:3001/health
```

### View System Metrics

```typescript
// In browser
const metrics = await invoke('cmd_get_system_metrics');
const thermal = await invoke('cmd_check_thermal');
```

### Review Agent Status

```typescript
// Get DSIF agent status
const agents = await invoke('cmd_dsif_get_agents');
// Returns: agent roles, states, trust scores
```

## Troubleshooting

### Decision Rejected

**Problem**: DSIF pipeline rejects action

**Check**:
1. Review audit trail for specific decision
2. Check which phase failed (input hygiene, policy, simulation, consensus)
3. Review invariant violations
4. Check agent consensus votes

**Solution**:
- Fix input trust level or provenance
- Update allowlist/denylist if needed
- Adjust invariants if policy is too strict
- Review simulation results for safety issues

### Receipt Verification Fails

**Problem**: `sap4d verify` reports invalid receipt

**Check**:
```bash
# Verify hash integrity
sap4d verify --receipt-file receipt.json --check-hash-only

# Verify signature
sap4d verify --receipt-file receipt.json --check-signature-only
```

**Solution**:
- Receipt may have been tampered with
- Signature key may have changed
- Regenerate receipt if original is lost

### Audit Service Not Responding

**Problem**: Cannot connect to audit service

**Check**:
```bash
# Check if service is running
ps aux | grep axiom-audit

# Check logs
tail -f /var/log/axiom-audit.log

# Check port
lsof -i :3001
```

**Solution**:
- Restart service: `systemctl restart axiom-audit`
- Check firewall rules
- Verify configuration

## Best Practices

1. **Always Review Receipts**: Don't trust outputs without verifying receipts
2. **Monitor Audit Trails**: Regularly review audit logs for anomalies
3. **Configure Policies Carefully**: Test invariants in staging before production
4. **Maintain Quorum**: Ensure sufficient consensus agents are active
5. **Backup Receipts**: Store receipts in secure, immutable storage
6. **Document Decisions**: Add human-readable notes to audit entries when needed

## Integration Examples

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Verify Configuration
  run: |
    sap4d prove \
      --claim "Deployment configuration is valid" \
      --evidence "Tests passed" \
      --evidence "Security scan clean" \
      --output deployment-receipt.json
    
    # Upload receipt as artifact
    gh release upload receipt.json deployment-receipt.json
```

### API Integration

```python
import requests

# Submit verification request
response = requests.post('http://localhost:3000/verify', json={
    'claim': 'System state is valid',
    'evidence': ['Check 1', 'Check 2']
})

receipt = response.json()
assert receipt['C_zero'] == True
```

## Support

- **Documentation**: See `docs/` directory
- **Issues**: Open GitHub issue with `operator-question` label
- **Security**: Contact `security@axiomhive.local`

