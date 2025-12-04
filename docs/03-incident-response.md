# Incident Response Playbook

> **[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]**

## Severity Levels

| Level | Description | Response Time | Escalation |
|-------|-------------|---------------|------------|
| **P0** | Substrate compromise, key exposure | Immediate | Substrate |
| **P1** | Invariance violation spike, unsigned process | < 15 min | Substrate |
| **P2** | Single invariance failure, HK block | < 1 hour | Operations |
| **P3** | Performance degradation, minor anomaly | < 24 hours | Operations |

## P0: Critical Security Incident

### Indicators

- Substrate key compromise suspected
- Unauthorized signing detected
- HSM tampering alert
- Mass signature verification failures

### Response Procedure

#### Phase 1: Immediate (0-5 min)

```bash
# Activate kill-switch
bark-ctl freeze --component all

# Revoke potentially compromised keys
bark-ctl revoke-key --key-id 0x0003 --reason "suspected compromise"

# Isolate affected systems
bark-ctl isolate --target all
```

**Actions:**
1. [ ] Activate system-wide kill-switch
2. [ ] Revoke suspected compromised keys
3. [ ] Isolate all networked systems
4. [ ] Alert Substrate (Alexis Adams)

#### Phase 2: Containment (5-30 min)

```bash
# Block all external access
iptables -P INPUT DROP
iptables -P OUTPUT DROP
iptables -P FORWARD DROP

# Preserve forensic evidence
bark-forensics snapshot --output /secure/p0-$(date +%s)/

# Generate incident bundle
bark-forensics collect \
  --signed \
  --include-logs \
  --include-keys \
  --output /secure/incident-bundle.tar.gz
```

**Actions:**
1. [ ] Block all network access
2. [ ] Preserve forensic evidence
3. [ ] Document timeline of events
4. [ ] Notify key ceremony participants

#### Phase 3: Recovery (30 min - 24h)

1. **Assess damage**
   - Identify affected systems
   - Determine scope of compromise
   - Catalog potentially tainted outputs

2. **Deploy clean systems**
   - Fresh installation from verified images
   - Restore from known-good backups

3. **Key rotation ceremony**
   - Emergency ceremony if keys compromised
   - Generate new signing keys
   - Distribute new public keys

4. **System restoration**
   - Verify integrity of all components
   - Re-enable services incrementally
   - Monitor for anomalies

#### Phase 4: Post-Incident (24h+)

- [ ] Root cause analysis
- [ ] Update threat model
- [ ] Improve detection mechanisms
- [ ] Publish incident report (if applicable)

## P1: Invariance Violation Spike

### Indicators

- Multiple C ≠ 0 detections within 5 minutes
- Unsigned process execution attempts
- Hunter-Killer triggering repeatedly
- Audit service rejection rate > 5%

### Response Procedure

```bash
# Freeze affected component
bark-ctl freeze --component invariance

# Dump logs
bark-ctl dump-logs \
  --signed \
  --since "5 minutes ago" \
  --output /var/log/axiom/p1-$(date +%s).log

# Analyze violations
axiom-audit analyze \
  --input /var/log/axiom/p1-*.log \
  --report /tmp/violation-report.json

# Notify Substrate
bark-ctl notify \
  --channel substrate \
  --priority urgent \
  --message "Invariance violation spike detected"
```

**Decision Tree:**

```
Violation detected
    │
    ├── Single source?
    │   ├── Yes → Isolate source, investigate
    │   └── No → System-wide freeze
    │
    ├── Pattern identified?
    │   ├── Known attack → Apply countermeasures
    │   └── Unknown → Escalate to P0
    │
    └── Resolved?
        ├── Yes → Document and monitor
        └── No → Escalate
```

## P2: Single Invariance Failure

### Indicators

- Single C ≠ 0 detection
- Hunter-Killer single block
- Audit rejection for specific claim

### Response Procedure

```bash
# Log the incident
axiom-audit log \
  --level warning \
  --event "invariance_failure" \
  --details "$VIOLATION_DETAILS"

# Investigate
axiom-audit trace \
  --claim "$CLAIM" \
  --evidence "$EVIDENCE"

# If legitimate failure, no further action
# If suspicious, escalate to P1
```

## P3: Performance Degradation

### Indicators

- Browser startup > 1.5s
- Page summary P95 > 500ms
- Receipt issuance > 200ms
- SSM inference latency spike

### Response Procedure

```bash
# Collect metrics
axiom-metrics snapshot --output /tmp/metrics-$(date +%s).json

# Identify bottleneck
axiom-metrics analyze --input /tmp/metrics-*.json

# Apply remediation
# (specific to identified issue)
```

## Forensics Procedures

### Evidence Collection

```bash
# Full forensic collection
axiom-forensics collect \
  --output /secure/evidence-$(date +%s).bundle \
  --sign-key hsm://substrate \
  --include-hashes \
  --include-signatures \
  --include-logs \
  --include-memory-dump
```

### Chain of Custody

1. All evidence files signed with Substrate key
2. Hash manifest generated and signed
3. Transfer logged with timestamps
4. Storage location documented

### Evidence Format

```json
{
  "incident_id": "INC-2025-001",
  "collected_at": "2025-12-03T18:00:00Z",
  "collector": "axiom-forensics v1.0.0",
  "evidence": [
    {
      "type": "log",
      "path": "/var/log/axiom/audit.log",
      "hash": "sha256:...",
      "signature": "..."
    }
  ],
  "chain_of_custody": [
    {
      "action": "collected",
      "timestamp": "2025-12-03T18:00:00Z",
      "actor": "system"
    }
  ]
}
```

## Communication Templates

### P0 Alert

```
AXIOM HIVE - CRITICAL SECURITY INCIDENT

Severity: P0
Time: [TIMESTAMP]
Status: ACTIVE

Summary: [BRIEF DESCRIPTION]

Immediate Actions Taken:
- System freeze activated
- Keys revoked
- Forensics initiated

Required Response:
- Substrate authorization required
- Key ceremony may be needed

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
```

### P1 Notification

```
AXIOM HIVE - Invariance Alert

Severity: P1
Time: [TIMESTAMP]

Detected: [NUMBER] invariance violations in [TIMEFRAME]

Analysis: [PRELIMINARY FINDINGS]

Status: Investigating

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
```

## Runbook Maintenance

This playbook is reviewed and updated:
- After every P0/P1 incident
- Quarterly review
- After any system architecture changes

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Document: Incident Response Playbook
Version: 1.0.0
Classification: OPERATIONS
```

