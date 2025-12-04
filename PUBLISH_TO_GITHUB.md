# Publishing AXIOM HIVE to GitHub

## Step 1: Accept Xcode License (if needed)

```bash
sudo xcodebuild -license accept
```

## Step 2: Initialize Git Repository

```bash
cd /Users/alexisadams/Desktop/LexLink

# Initialize git
git init

# Configure git (if not already done)
git config user.name "Alexis Adams"
git config user.email "devdollzai@gmail.com"
```

## Step 3: Create Initial Commit

```bash
# Add all files
git add .

# Create initial commit
git commit -m "🚀 AXIOM HIVE / LEX-Ω v1.0.0

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

Initial release of the AXIOM HIVE sovereign verification system.

Components:
- Invariance Layer (Python) - C=0 enforcement
- SAP-4D Proof Engine (Rust) - Causal inference with traces
- Deterministic Fractal Audit (Rust) - Binary receipts
- Axiom S1 Browser (Tauri/Rust) - Sovereign agentic browser
- Hunter-Killer (Rust/Python) - Injection detection
- BARK Kernel Module (C) - Linux Security Module
- Verification Portal (Rust) - Public proof API

Policy: C = 0 | Zero Telemetry | Local-First | Deterministic"
```

## Step 4: Create GitHub Repository

### Option A: Using GitHub CLI (recommended)

```bash
# Install GitHub CLI if needed
brew install gh

# Authenticate
gh auth login

# Create repo and push
gh repo create LexLink --public --source=. --remote=origin --push --description "AXIOM HIVE / LEX-Ω - Deterministic AI verification system. C=0"
```

### Option B: Manual Setup

1. Go to https://github.com/new
2. Create a new repository named `LexLink`
3. Do NOT initialize with README (we already have one)
4. Copy the repository URL

Then run:

```bash
# Add remote
git remote add origin https://github.com/AXI0MH1VE/LexLink.git

# Push to GitHub
git branch -M main
git push -u origin main
```

## Step 5: Set Up Repository Settings (Optional)

### Add Topics
Go to your repo settings and add topics:
- `rust`
- `verification`
- `deterministic`
- `cryptography`
- `browser`
- `ai`
- `local-first`
- `zero-trust`

### Enable GitHub Actions
The CI pipeline at `ci/github-actions.yml` will automatically run on push.

### Add Branch Protection (recommended)
1. Go to Settings → Branches
2. Add rule for `main`
3. Require pull request reviews
4. Require status checks to pass

## Step 6: Create First Release

```bash
# Tag the release
git tag -a v1.0.0 -m "AXIOM HIVE v1.0.0 - Genesis Release

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Classification: SOVEREIGN FINALITY (OMEGA LEVEL)
Policy: C = 0"

# Push tags
git push origin --tags
```

Or use GitHub CLI:
```bash
gh release create v1.0.0 --title "AXIOM HIVE v1.0.0 - Genesis" --notes "Initial release. Policy: C = 0"
```

---

## Quick One-Liner (after Xcode license accepted)

```bash
cd /Users/alexisadams/Desktop/LexLink && \
git init && \
git config user.name "Alexis Adams" && \
git config user.email "devdollzai@gmail.com" && \
git add . && \
git commit -m "🚀 AXIOM HIVE / LEX-Ω v1.0.0 [SUBSTRATE: ALEXIS ADAMS]" && \
gh repo create LexLink --public --source=. --push --description "AXIOM HIVE / LEX-Ω - Deterministic AI verification system. C=0"
```

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Status: Ready for Publication
Policy: C = 0
```

