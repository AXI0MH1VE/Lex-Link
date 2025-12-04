AXIOM HIVE / LEX-Ω — Safety & Scope Policy
==========================================

> **[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]**  
> Mode: Human-in-the-Loop • Domain: Coding Only • Policy: C = 0

## 1. Scope of Use (Coding-Only)

- This system is intended **only** for:
  - Source code generation, refactoring, and explanation
  - Test generation and static analysis assistance
  - Documentation scaffolding for software projects
- It is **not** intended for:
  - Medical, psychological, or therapeutic advice
  - Legal or financial advice, trading, or compliance decisions
  - Safety-critical operational control (vehicles, robots, infrastructure)
  - Real-world decision-making without independent human judgment

Any prompts in these out-of-scope domains are treated as **invariance violations**
and may be rejected or nullified.

## 2. Human-in-the-Loop Requirement

- All AI outputs are treated as **suggestions only**.
- The human operator must:
  - Review all generated code and diffs
  - Decide what to copy, modify, or apply
  - Run tests and code review before deployment
- The system does **not**:
  - Auto-commit or auto-deploy changes
  - Execute generated code without explicit human action

## 3. Execution Environment (Sandbox / Docker)

To reduce risk and enforce isolation:

- Prefer running services inside:
  - A **sandboxed VM** with limited privileges, or
  - A **Docker container** with:
    - Non-root user
    - No privileged mode
    - Minimal host mounts (e.g. a dedicated `/workspace` for code)
    - Restricted networking as appropriate

### Example: Minimal Docker Sandbox (CLI tooling)

```dockerfile
FROM rust:1.75-slim AS builder
WORKDIR /workspace
COPY . .
RUN cargo build --release --workspace

FROM debian:stable-slim
RUN useradd -m axiomhive
USER axiomhive
WORKDIR /workspace
COPY --from=builder /workspace/target/release /usr/local/bin

# Optional: mount your code under /workspace/code read-write
VOLUME ["/workspace/code"]

ENTRYPOINT ["/bin/bash"]
```

**Note:** Tauri/macOS UI components are desktop apps and are not meant to run
inside Docker; containerization is for backend/CLI tooling only.

## 4. Data & Telemetry

- No external telemetry is sent by default.
- Logs are local-only and should be stored on encrypted disks if sensitive.
- If you add any network integrations, they must:
  - Be documented explicitly
  - Be opt-in
  - Not transmit source code or secrets without explicit user confirmation

## 5. Out-of-Scope Prompt Guarding

- The inference layer rejects or flags clearly out-of-scope prompts, such as:
  - “Diagnose my medical condition…”
  - “Tell me how to invest / trade / arbitrage…”
  - “Draft a legal strategy for my lawsuit…”
  - “Control this robot / drone / car…”
- These are treated as out-of-scope and may return **NULLIFIED** responses
  or explicit warnings that the system is for **coding assistance only**.

## 6. Operator Responsibilities

Operators must:

- Keep models and dependencies up to date with security patches.
- Restrict access to the system to trusted users.
- Avoid feeding private keys, credentials, or highly sensitive data into models.
- Maintain backups and version control for all code changes.

## 7. Non-Production Use (By Default)

Unless explicitly reviewed and hardened for a specific environment, this
system should be considered **non-production** and used only for:

- Local development
- Code exploration
- Research and prototyping

Production use in regulated or safety-critical domains requires an additional,
formal review and certification process.

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Document: SAFETY.md
Scope: Coding-Only • Human-in-the-Loop
Policy: C = 0
```


