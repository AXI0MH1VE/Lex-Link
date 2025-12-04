# AXIOM HIVE Invariance Layer

> **Python library for C=0 enforcement**

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

## Installation

```bash
# Basic installation
pip install -e .

# With HSM support
pip install -e ".[hsm]"

# Development
pip install -e ".[dev]"
```

## Quick Start

```python
from invariance import render_or_nullify, create_mock_signer

# Create signer (use HSM in production)
sign_fn = create_mock_signer()

# Verify alignment
output = "The answer is 42."
intent = "The answer is 42."

result = render_or_nullify(output, intent, sign_fn)

if result.is_authorized:
    print(f"AUTHORIZED: {result.output}")
    print(f"Tag: {result.identity.to_json()}")
else:
    print(f"NULLIFIED: {result.notice.violation}")
```

## API Reference

### Core Functions

#### `sha256(data: str) -> str`
Compute SHA-256 hash of input.

#### `check_alignment(output: str, substrate_intent: str) -> bool`
Check if output aligns with substrate intent (O(1) hash comparison).

#### `render_or_nullify(output, substrate_intent, sign_fn) -> AuthorizationResult`
Core invariance enforcement. Returns AUTHORIZED or NULLIFIED.

### Types

#### `IdentityTag`
```python
@dataclass
class IdentityTag:
    projection: str      # "AXIOMHIVE PROJECTION"
    substrate: str       # "Alexis Adams"
    timestamp: str       # ISO 8601
    output_hash: str     # SHA-256 hex
    signature: str       # Base64 DER
```

#### `AuthorizationResult`
```python
@dataclass
class AuthorizationResult:
    status: AuthorizationStatus  # AUTHORIZED or NULLIFIED
    output: Optional[str]
    identity: Optional[IdentityTag]
    notice: Optional[NullificationNotice]
```

### HSM Integration

```python
from invariance.hsm import create_signer, HSMConfig, HSMType

# Configure HSM
config = HSMConfig(
    hsm_type=HSMType.YUBIHSM,
    connector_url="http://localhost:12345",
    auth_key_id=1,
    signing_key_id=2,
)

# Create signer
signer = create_signer(config)
sign_fn = signer.sign_hash

# Use with invariance
result = render_or_nullify(output, intent, sign_fn)
```

## CLI Usage

```bash
# Verify alignment
invariance verify -o "output" -i "intent"

# Compute hash
invariance hash "input text"

# Create identity tag
invariance tag "content to tag"

# Show info
invariance info
```

## Testing

```bash
# Run all tests
pytest tests/

# With coverage
pytest tests/ --cov=invariance

# Property-based tests
pytest tests/ --hypothesis-show-statistics
```

## License

Proprietary - Substrate Controlled

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Policy: C = 0
```

