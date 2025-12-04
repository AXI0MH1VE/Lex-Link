# Key Ceremony Guide

> **[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]**

## Overview

This document describes the procedures for key generation, management, and rotation for the AXIOM HIVE system.

## Key Hierarchy

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

## Hardware Requirements

### Supported HSMs

| HSM | Model | Use Case |
|-----|-------|----------|
| YubiHSM 2 | Primary | Production signing |
| Nitrokey HSM 2 | Alternative | Production signing |
| SoftHSM | Development | Testing only |

### Ceremony Equipment

- [ ] Air-gapped laptop (no network capability)
- [ ] 2x YubiHSM 2 devices (primary + backup)
- [ ] Tamper-evident bags
- [ ] Video recording equipment
- [ ] Secure storage facility access
- [ ] Ceremony witness (minimum 2)

## Pre-Ceremony (T-24h)

### Checklist

1. [ ] Verify HSM firmware integrity
   ```bash
   yubihsm-connector --version
   yubihsm-shell -a get-device-info
   ```

2. [ ] Prepare air-gapped ceremony machine
   - Fresh OS installation
   - No network adapters enabled
   - Verified checksums of all software

3. [ ] Notify ceremony participants
   - Substrate (Alexis Adams) - Required
   - Witness 1 - Required
   - Witness 2 - Required

4. [ ] Prepare secure facility
   - Access control verified
   - No electronic devices (except ceremony equipment)
   - Video recording configured

## Ceremony Procedure

### Phase 1: Initialization (30 min)

1. **Verify participants**
   - All required participants present
   - ID verification complete
   - Video recording started

2. **Initialize HSM**
   ```bash
   # Set strong authentication password
   yubihsm-shell
   > connect
   > put authkey 0 0x0001 "Ceremony Auth" all generate:all
   > session close
   ```

3. **Generate root key**
   ```bash
   # Generate ECDSA P-384 key (never exported)
   yubihsm-shell
   > session open 1
   > generate asymmetric 0 0x0002 "Substrate Root Key" 1 sign-ecdsa ecp384
   > session close
   ```

### Phase 2: Derive Signing Keys (30 min)

1. **Generate signing key**
   ```bash
   yubihsm-shell
   > session open 1
   > generate asymmetric 0 0x0003 "Receipt Signing Key" 1 sign-ecdsa ecp384
   > get-public-key 0x0003
   > session close
   ```

2. **Export public key**
   ```bash
   yubihsm-shell
   > session open 1
   > get-public-key 0x0003 --out substrate-public.pem
   > session close
   ```

3. **Verify key generation**
   ```bash
   # Test sign operation
   echo "test" | yubihsm-shell -a sign-ecdsa -i 0x0003
   ```

### Phase 3: Recovery Key (45 min)

1. **Generate recovery key**
   ```bash
   yubihsm-shell
   > session open 1
   > generate wrap 0 0x0004 "Recovery Wrap Key" 1 export-wrapped:all
   > session close
   ```

2. **Split recovery key (2-of-3 Shamir)**
   - Use air-gapped tool to split
   - Each share to separate tamper-evident envelope
   - Each envelope to different secure location

3. **Verify recovery procedure**
   - Test key recovery with 2 shares
   - Confirm successful restoration

### Phase 4: Documentation (30 min)

1. **Record key IDs**
   ```yaml
   ceremony_date: 2025-12-03
   hsm_serial: XXXXXXXX
   keys:
     root: 0x0002
     signing: 0x0003
     recovery: 0x0004
   witnesses:
     - name: "Witness 1"
       signature: "..."
     - name: "Witness 2"
       signature: "..."
   substrate_signature: "..."
   ```

2. **Hash ceremony video**
   ```bash
   sha256sum ceremony-2025-12-03.mp4 > ceremony-hash.txt
   ```

3. **Sign ceremony record**
   ```bash
   yubihsm-shell -a sign-ecdsa -i 0x0003 ceremony-record.json
   ```

## Post-Ceremony

### Secure Storage

| Item | Location | Access |
|------|----------|--------|
| Primary HSM | Operational facility | Substrate |
| Backup HSM | Secure vault | Substrate + 1 witness |
| Recovery Share 1 | Location A | Holder A |
| Recovery Share 2 | Location B | Holder B |
| Recovery Share 3 | Location C | Holder C |
| Ceremony video | Encrypted archive | Substrate |

### Verification

```bash
# Verify HSM operational
yubihsm-shell -a get-object-info -i 0x0003

# Test signing
echo "verification test" | yubihsm-shell -a sign-ecdsa -i 0x0003

# Verify public key
openssl ec -pubin -in substrate-public.pem -text
```

## Key Rotation

### Schedule

| Key Type | Rotation Period | Procedure |
|----------|-----------------|-----------|
| Root | Never (unless compromised) | Full ceremony |
| Signing | Quarterly | Derive new key |
| Recovery | Annual | 2-of-3 ceremony |

### Quarterly Rotation

1. Generate new signing key
2. Export new public key
3. Update all services
4. Revoke old key after 30 days

```bash
# Generate new key
yubihsm-shell
> session open 1
> generate asymmetric 0 0x0005 "Signing Key Q2-2026" 1 sign-ecdsa ecp384
> get-public-key 0x0005 --out substrate-public-q2-2026.pem
> session close
```

## Emergency Procedures

### Key Compromise

1. **Immediate Actions**
   - Revoke compromised key
   - Alert all services
   - Begin emergency rotation

2. **Recovery**
   - Reconvene ceremony
   - Generate new keys
   - Update all systems

### HSM Failure

1. **Restore from backup**
   - Retrieve backup HSM
   - Verify configuration

2. **If no backup available**
   - Gather 2-of-3 recovery shares
   - Perform key recovery
   - Initialize new HSM

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Document: Key Ceremony Guide
Version: 1.0.0
Classification: SUBSTRATE CONTROLLED
```

