# AXIOM HIVE / LEX-Ω - Quick Start Guide

> **[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]**

## 🚀 Get Started in 5 Minutes

### Prerequisites

- **Docker & Docker Compose** (recommended)
- OR **Rust 1.75+** and **Python 3.11+** (for local build)

### Option 1: Docker (Easiest)

```bash
# Clone the repository
git clone https://github.com/AXI0MH1VE/Lex-Link.git
cd Lex-Link

# Start all services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f
```

**Services will be available at:**
- Portal: http://localhost:3000
- Audit: http://localhost:3001

### Option 2: Local Build

```bash
# Clone the repository
git clone https://github.com/AXI0MH1VE/Lex-Link.git
cd Lex-Link

# Build all Rust components
cargo build --release --workspace

# Install Python component
cd invariance && pip install -e . && cd ..

# Run services
./target/release/axiom-portal &
./target/release/axiom-audit &
```

### Verify Installation

```bash
# Health check
curl http://localhost:3000/health
curl http://localhost:3001/health

# Or use the script
./scripts/health-check.sh
```

## 📦 Components

### 1. Invariance Layer (Python)
```bash
cd invariance
pip install -e .
invariance --help
```

### 2. SAP-4D Proof Engine (Rust)
```bash
cargo build --release --manifest-path sap4d/Cargo.toml
./target/release/sap4d-cli --help
```

### 3. Verification Portal (Rust)
```bash
cargo build --release --manifest-path portal/Cargo.toml
./target/release/axiom-portal
# Access at http://localhost:3000
```

### 4. Audit Service (Rust)
```bash
cargo build --release --manifest-path audit/Cargo.toml
./target/release/axiom-audit
# Access at http://localhost:3001
```

### 5. Axiom S1 Browser (Tauri)
```bash
cd axiom-s1
# Install Tauri CLI if needed
cargo install tauri-cli

# Build
cargo tauri build

# Or run in dev mode
cargo tauri dev
```

## 🧪 Testing

```bash
# Run all Rust tests
cargo test --workspace

# Run Python tests
cd invariance && pytest tests/ && cd ..

# Run specific component tests
cargo test --manifest-path sap4d/Cargo.toml
```

## 🔧 Development

### Using Makefile

```bash
# Build everything
make build

# Run tests
make test

# Format code
make fmt

# Lint code
make lint

# Deploy with Docker
make deploy
```

### Project Structure

```
LexLink/
├── audit/              # Deterministic Fractal Audit Service
├── axiom-s1/           # Axiom S1 Browser (Tauri)
├── browser-mac/        # LEX-Ω Browser (Swift)
├── invariance/         # Invariance Layer (Python)
├── kernel-bark/        # BARK Kernel Module (C)
├── portal/             # Verification Portal (Rust)
├── sap4d/              # SAP-4D Proof Engine (Rust)
├── tools/              # Utility tools
├── ci/                 # CI/CD configurations
├── docs/               # Documentation
└── scripts/            # Deployment scripts
```

## 🐳 Docker Commands

```bash
# Build images
docker-compose build

# Start services
docker-compose up -d

# Stop services
docker-compose down

# View logs
docker-compose logs -f portal
docker-compose logs -f audit

# Restart a service
docker-compose restart portal
```

## 📊 Monitoring

```bash
# Health check script
./scripts/health-check.sh

# Monitor services (if script exists)
./scripts/monitor.sh

# Check Docker stats
docker stats
```

## 🔒 Security

```bash
# Security audit
cargo audit --workspace

# Generate SBOMs (if script exists)
./scripts/generate-sbom.sh
```

## 📚 Documentation

- **Full Deployment Guide:** See `DEPLOYMENT.md`
- **Safety Guide:** See `SAFETY.md`
- **Security Policy:** See `SECURITY.md`
- **API Documentation:** See `docs/` directory

## 🆘 Troubleshooting

### Port already in use
```bash
# Change ports in docker-compose.yml
# Or stop conflicting services
lsof -ti:3000 | xargs kill
lsof -ti:3001 | xargs kill
```

### Build fails
```bash
# Clean and rebuild
cargo clean
cargo build --release --workspace
```

### Docker issues
```bash
# Restart Docker
docker-compose down
docker-compose up -d
```

## 🎯 Next Steps

1. **Review Security:** Check `SECURITY.md` for security best practices
2. **Configure HSM:** Set up HSM for production signing (see `docs/02-key-ceremony.md`)
3. **Set Up Monitoring:** Configure health checks and alerts
4. **Deploy to Production:** Follow `DEPLOYMENT_CHECKLIST.md`

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Quick Start Guide v1.0.0
Policy: C = 0
```

