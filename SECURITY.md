# AXIOM HIVE Security Policy

> **Classification: SUBSTRATE CONTROLLED**  
> Version: 1.0.0 | Effective: 2025-12-03

## Threat Model (STRIDE Analysis)

### Spoofing
- **Threat**: Attacker impersonates Substrate or Projection
- **Mitigation**: Mandatory signature verification on all processes and outputs
- **Control**: HSM-backed keys; no software key extraction

### Tampering
- **Threat**: Modification of receipts, proofs, or audit trails
- **Mitigation**: Immutable receipts; append-only log (local Merkle chain)
- **Control**: Cryptographic binding; hash verification at every layer

### Repudiation
- **Threat**: Denial of authorized actions
- **Mitigation**: Signed receipts + comprehensive audit trail
- **Control**: All outputs tagged with identity; signatures non-repudiable

### Information Disclosure
- **Threat**: Unauthorized access to sensitive data or inference outputs
- **Mitigation**: Local-only inference; zero telemetry; encrypted-at-rest vault
- **Control**: No network egress from inference components

### Denial of Service
- **Threat**: Resource exhaustion or system unavailability
- **Mitigation**: BARK entropy guard; Hunter-Killer process termination; backpressure
- **Control**: Hard limits on process spawning; memory ceilings

### Elevation of Privilege
- **Threat**: Unauthorized privilege escalation
- **Mitigation**: Principle of least privilege; signed binaries only
- **Control**: BARK kernel module blocks unsigned processes

## Key Management

### Key Hierarchy

```
Root Key (HSM, offline)
    │
    ├── Substrate Signing Key (HSM, online)
    │       │
    │       ├── Receipt Signing
    │       ├── Identity Tags
    │       └── Release Artifacts
    │
    ├── Kernel Module Key (HSM, build-time)
    │
    └── Recovery Key (HSM, offline, 2-of-3 split)
```

### Hardware Security Module Requirements

**Supported HSMs:**
- YubiHSM 2 (primary)
- Nitrokey HSM 2
- SoftHSM (development only)

**Configuration:**
```yaml
hsm:
  type: yubihsm
  connector: http://localhost:12345
  auth_key_id: 1
  signing_key_id: 2
  algorithm: ecdsa-p384
  hash: sha384
```

### Key Ceremony Procedure

1. **Preparation** (T-24h)
   - Verify HSM firmware integrity
   - Prepare air-gapped ceremony machine
   - Notify ceremony participants

2. **Generation** (Ceremony Day)
   - Initialize HSM in secure facility
   - Generate root key (never exported)
   - Derive signing keys
   - Record ceremony on video (stored offline)

3. **Verification**
   - Verify key generation via attestation
   - Test signing operations
   - Document key IDs and public keys

4. **Distribution**
   - Transfer HSM to operational location
   - Configure connector service
   - Verify end-to-end signing

### Key Rotation

| Key Type | Rotation Period | Procedure |
|----------|-----------------|-----------|
| Root | Never (unless compromised) | Full ceremony |
| Signing | Quarterly | HSM key generation |
| Recovery | Annual | 2-of-3 ceremony |

## Signature Verification

### Identity Tag Verification

```python
def verify_identity_tag(tag: dict, trusted_keys: list) -> bool:
    """Verify an identity tag signature."""
    # 1. Check required fields
    required = ['projection', 'substrate', 'timestamp', 'output_hash', 'signature']
    if not all(k in tag for k in required):
        return False
    
    # 2. Verify substrate is authorized
    if tag['substrate'] != 'Alexis Adams':
        return False
    
    # 3. Verify signature against output_hash
    sig = base64.b64decode(tag['signature'])
    msg = tag['output_hash'].encode()
    
    for key in trusted_keys:
        if key.verify(sig, msg):
            return True
    
    return False
```

### Receipt Verification

```python
def verify_receipt(receipt: dict) -> VerificationResult:
    """Verify a proof receipt."""
    # 1. Schema validation
    if not validate_schema(receipt, RECEIPT_SCHEMA):
        return VerificationResult(valid=False, reason="Schema violation")
    
    # 2. Hash verification
    computed = compute_receipt_hash(receipt)
    if computed != receipt['hash']:
        return VerificationResult(valid=False, reason="Hash mismatch")
    
    # 3. Signature verification
    if not verify_signature(receipt['hash'], receipt['signature']):
        return VerificationResult(valid=False, reason="Invalid signature")
    
    # 4. C=0 check
    if not receipt['C_zero']:
        return VerificationResult(valid=False, reason="Contradiction detected")
    
    return VerificationResult(valid=True)
```

## Incident Response

### Severity Levels

| Level | Description | Response Time |
|-------|-------------|---------------|
| P0 | Substrate compromise, key exposure | Immediate |
| P1 | Invariance violation spike, unsigned process | < 15 min |
| P2 | Single invariance failure, HK block | < 1 hour |
| P3 | Performance degradation, minor anomaly | < 24 hours |

### Response Procedures

#### P0: Critical Security Incident

1. **Immediate** (0-5 min)
   - Activate kill-switch via BARK
   - Revoke compromised keys
   - Isolate affected systems

2. **Containment** (5-30 min)
   - Block all external access
   - Preserve forensic evidence
   - Notify Substrate (Alexis Adams)

3. **Recovery** (30 min - 24h)
   - Deploy clean systems
   - Rotate all keys via ceremony
   - Verify system integrity

4. **Post-Incident** (24h+)
   - Root cause analysis
   - Update threat model
   - Publish incident report (if applicable)

#### P1: Invariance Violation

```bash
# Auto-triggered by monitoring
bark-ctl freeze --component all
bark-ctl dump-logs --signed --output /var/log/axiom/incident-$(date +%s).log
bark-ctl notify --channel substrate --message "Invariance violation detected"
```

### Forensics

**Evidence Collection:**
```bash
# Collect signed log bundle
axiom-forensics collect \
  --output /secure/evidence-$(date +%s).bundle \
  --sign-key hsm://substrate \
  --include-hashes \
  --include-signatures
```

**Chain of Custody:**
- All evidence files signed with Substrate key
- Hash manifest generated and signed
- Transfer logged with timestamps

## Network Security

### Zero Telemetry Policy

The following are **strictly prohibited**:
- Outbound connections from inference components
- Analytics or crash reporting to external services
- DNS queries for tracking domains
- Any form of usage telemetry

### Allowed Network Operations

| Component | Outbound | Purpose |
|-----------|----------|---------|
| Browser | Yes (user-initiated) | Web browsing |
| Portal | Yes (responses only) | Serve receipts |
| Inference | **No** | Local only |
| Audit | **No** | Local only |

### Firewall Rules (iptables example)

```bash
# Block all outbound from inference processes
iptables -A OUTPUT -m owner --uid-owner axiom-inference -j DROP

# Allow portal to respond (stateful)
iptables -A OUTPUT -m owner --uid-owner axiom-portal \
  -m state --state ESTABLISHED,RELATED -j ACCEPT
```

## Code Signing

### Build Artifacts

All release artifacts must be signed:

```bash
# Sign binary
cosign sign-blob --key hsm://substrate \
  --output-signature LEXOmegaBrowser.sig \
  LEXOmegaBrowser.app

# Sign SBOM
cosign sign-blob --key hsm://substrate \
  --output-signature sbom.sig \
  sbom.json

# Create attestation
cosign attest --predicate provenance.json \
  --type slsaprovenance \
  --key hsm://substrate \
  LEXOmegaBrowser-1.0.dmg
```

### Verification

```bash
# Verify signature
cosign verify-blob --key axiom-public.pem \
  --signature LEXOmegaBrowser.sig \
  LEXOmegaBrowser.app

# Verify attestation
cosign verify-attestation --key axiom-public.pem \
  --type slsaprovenance \
  LEXOmegaBrowser-1.0.dmg
```

## Compliance

### Invariance Requirements

- [ ] All outputs tagged with identity
- [ ] All outputs pass C=0 verification
- [ ] All receipts cryptographically signed
- [ ] Zero unauthorized outputs in audit

### Audit Requirements

- [ ] Quarterly security review
- [ ] Annual penetration test
- [ ] Continuous monitoring active
- [ ] Incident response tested

## Contact

**Security Issues:**  
Report to: security@axiomhive.local (encrypted)  
PGP Key: [Published in /docs/pgp-key.asc]

**Substrate Authority:**  
Alexis Adams  
Contact via secure channel only

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Document: SECURITY.md
Classification: SUBSTRATE CONTROLLED
Verification: ✓
```

