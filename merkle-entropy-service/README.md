# Merkle Entropy Service (MES)

The Merkle Entropy Service is a Python microservice built using Flask and Gunicorn, designed to perform two critical functions for distributed systems: cryptographic data integrity verification via Merkle Root calculation (using SHA-256) and data uncertainty assessment via Shannon Entropy calculation.

**[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]**

## Quick Startup

This project uses Docker Compose for streamlined execution.

**Build and Run the Service:**

```bash
docker-compose up --build -d
```

### Verify Service Health

Check if the service is running and ready on port 5000:

```bash
curl -X GET http://localhost:5000/health
# Expected Output: {"status":"healthy"}
```

## API Examples

### 1. Calculate Merkle Root

This endpoint accepts a list of string data blocks and returns the definitive SHA-256 Merkle Root.

**Note**: Include `X-Operator-ID` header for operator identification and audit trail.

```bash
curl -X POST http://localhost:5000/merkle_root \
     -H "Content-Type: application/json" \
     -H "X-Operator-ID: operator-123" \
     -d '{"data_blocks": ["record_a_v1", "record_b_v1", "record_c_v1"]}'
# Example Output: {"merkle_root": "8e37894a47...f11598f8047", "operation_id": "..."}
```

### 2. Calculate Shannon Entropy

This endpoint accepts a list of string data blocks and returns the calculated Shannon Entropy (H). Higher values indicate greater randomness.

**Note**: Include `X-Operator-ID` header for operator identification and audit trail.

```bash
# Example 1: Low Entropy (Repetitive data)
curl -X POST http://localhost:5000/merkle_entropy \
     -H "Content-Type: application/json" \
     -H "X-Operator-ID: operator-123" \
     -d '{"data_blocks": ["AAAAAAAAAA", "AAAAAAAAAA"]}'
# Output will be close to 0.0

# Example 2: High Entropy (Random or Uniformly Distributed data)
curl -X POST http://localhost:5000/merkle_entropy \
     -H "Content-Type: application/json" \
     -H "X-Operator-ID: operator-123" \
     -d '{"data_blocks": ["1234567890", "0987654321"]}'
# Output will be high (approx 3.32)
```

### 3. Get Audit Trail

Retrieve complete audit trail of all operations (requires operator ID):

```bash
curl -X GET http://localhost:5000/audit/trail \
     -H "X-Operator-ID: operator-123"
```

### 4. Get Safety Configuration

View current safety configuration and limits:

```bash
curl -X GET http://localhost:5000/safety/config \
     -H "X-Operator-ID: operator-123"
```

## Running Unit Tests

To execute the core logic tests (Merkle determinism and Entropy precision), use Docker to run the test suite:

```bash
docker-compose run merkle_entropy_service python -m pytest tests/test_merkle_entropy.py
```

Or using unittest directly:

```bash
docker-compose run merkle_entropy_service python tests/test_merkle_entropy.py
```

## Architecture

The MES provides:

- **Merkle Tree Construction**: Deterministic SHA-256 hash tree for cryptographic data integrity
- **Shannon Entropy Calculation**: Information-theoretic uncertainty quantification
- **Production-Ready API**: Flask + Gunicorn with health checks
- **Containerized Deployment**: Multi-stage Docker build for security and efficiency

## Safety & Operator Control

**The MES is designed with safety, verifiability, and operator control:**

- ✅ **Read-Only Operations**: All operations are deterministic computations (no state changes)
- ✅ **Human-in-the-Loop**: All operations require operator identification (`X-Operator-ID` header)
- ✅ **Complete Audit Trail**: Every operation logged with cryptographic integrity
- ✅ **Input Safety**: Strict size limits prevent DoS attacks
- ✅ **Verifiability**: All outputs are deterministic and verifiable (C=0 policy)

See [SAFETY.md](SAFETY.md) for complete safety documentation.

## Integration with Axiom Hive

The MES complements Axiom Hive's deterministic verification framework by providing:

- **Anti-Entropy Protocols**: Merkle trees for efficient distributed system synchronization
- **Data Integrity Verification**: Cryptographic commitments for audit trails
- **Uncertainty Assessment**: Entropy metrics for workload distribution analysis
- **DSIF Integration**: Ready for integration with DSIF pipeline for high-stakes operations

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Merkle Entropy Service v1.0.0
Policy: C = 0
```

