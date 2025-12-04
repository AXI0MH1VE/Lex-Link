# Math for the LLM

This document captures the technical distinction between probabilistic LLMs (ex: Grok) and deterministic, formally‑verified control frameworks (Axiom Hive DSIF), and explains why "math for the LLM"—formal constraints and proofs—is the primary mitigation for unsafe actuation.

## The Problem: Soft Math in Probabilistic LLMs

- Probabilistic internals: standard LLMs compute token probability distributions and sample from them (temperature, top‑p/top‑k, etc.).
- Soft decisions and variance: sampling enables diversity but introduces nondeterminism and drift—identical prompts can yield different outputs across runs.
- Contradiction cost: conflicting training signals (safety filters vs. raw data) create ongoing friction and failure modes (hallucination, policy evasion). This friction is a runtime cost and a logical insecurity for actuation.

## The Fix: Hard Math — Deterministic, Proof‑Gated Outputs (C = 0)

- Formal constraints: wrap model outputs with symbolically checkable invariants (typed policy contracts, temporal logic properties).
- Proof gating: candidate outputs must pass mathematical checks (model checking, simulation, property testing) before they are permitted to affect actuators.
- C = 0 (Zero Contradiction): a design invariant meaning the system prevents contradictory paths at construction time. Outputs that would violate invariants are never considered for actuation.

## Why This Restores Control

- Resistant to weight nudging: soft manipulation of model weights changes distributions but cannot override a formally enforced invariant.
- Auditability and reproducibility: deterministic decision paths and signed attestations enable replay, forensics, and certification.
- Human sovereignty: human approver attestations become binary gates; a human must explicitly sign approval before any non‑read actions proceed.

## Implementation Sketch (Axiom Hive DSIF)

1. Deterministic model configuration
   - Force deterministic decoding (temperature = 0, fixed seed, top_k/top_p disabled) for any component that influences actuation.
2. Typed policy contracts
   - Encode invariants as typed, machine‑checkable contracts (e.g., "water_level >= T for N minutes").
3. Simulation and model checking
   - Run candidate actions in a digital twin; apply model checking / LTL assertions on results.
4. Consensus gating
   - N‑of‑M quorum among independent deterministic agents required to ratify an action.
5. Mandatory human‑in‑the‑loop
   - Any non‑read action must have a signed approver attestation (`approve:<decision_id>`) before actuation.
6. Immutable audit trail
   - Content‑addressed, signed decision bundles and attestations recorded for certification.

## Practical Notes

- Not all LLM outputs need to be deterministic — restrict determinism to the perception/control surface that can influence actuators.
- Human approvals should be integrated into the attestation system and logged as cryptographic receipts.
- Maintain separation of trust zones: untrusted social streams feed advisory channels only; only attested inputs may influence actuation.

## Scaling

- **Transformers (attention):** attention is quadratic in sequence length — doubling sequence length often ≈ 4× time/memory per layer. Dataset-size scaling also drives non-deterministic variability and heavy hyperparameter tuning.
- **State Space Models (SSM):** sequential linear-time recurrence with better cache locality and streaming behavior. Doubling sequence length ≈ 2× compute/memory; scaling is closer to linear and more predictable for long-horizon tasks.

## Transparency

- The SSM "A" (state-transition) matrix defines memory and dynamics explicitly: eigenvalues/eigenvectors reveal timescales, stability, and modal behavior.
- Auditability: blocks, sparsity, and spectral properties are inspectable — giving engineers a tractable window into "what the model remembers and how." This improves inspectability relative to transformer weights smeared across millions of parameters, though inspectability does not by itself guarantee correctness.

## Axiom Hive's Verifiable Output Pipeline

- **Inputs as axioms:** data and constraints enter under formal contracts (B‑Method invariants; Rust-guarded interfaces).
- **Deterministic execution:** Wasmtime sandboxing, fixed RNG seeds, deterministic memory/layout, and controlled runtimes ensure bit-for-bit identical runs across compliant machines.
- **Processing verification:** apply physics/controls checks (e.g., Koopman-linearized dynamics, Euler–Lagrange residuals); halt on invariant violation rather than guessing.
- **Cryptographic receipts:** ZKML proofs or succinct verification artifacts produce verifiable receipts; authenticated AEADs bind receipts to inputs, code, and environment.
- **Ledgered traceability:** each operation/event is hashed and chained into an immutable audit layer (BDK-backed or equivalent).

Result: a regulator, auditor, or investigator can recompute and reproduce the exact output and verify the proof chain end-to-end.

## Formal Verification, Generalized

- Borrowing discipline from safety-critical domains (B‑Method, avionics, rail), Axiom Hive applies invariant constraints, proof obligations, and halt-on-violation behavior to AI agents.
- This is distinct from RLHF/filters: proofs and invariants govern execution rather than probabilistic guardrails.

## "Mathematically Impossible Harm" Scope

- The guarantee applies to computation inside the deterministic control layer: if inputs satisfy declared axioms and constraints, the engine cannot produce outputs that violate verified invariants.
- External risks remain (corrupt sensors, poisoned inputs, human misuse). Those are mitigated by auditability, identity, and repeatability; they are not eliminated by internal proofs. The system halts when formal conditions fail.

## Why Identical Execution Matters

- **Reproducibility:** identical code paths → identical outputs → identical proofs.
- **Accountability:** a single signed receipt verifies everywhere; no "it worked on Machine A but not on Machine B."
- **Compliance:** supports traceability and explainability requirements (e.g., EU AI Act Article 50) that black-box probabilistic systems cannot meet reliably.

## Identity and Accountability

- Every output carries an attributable identity and a cryptographic signature tied to the host substrate and execution context.
- This prevents ownerless outputs; there is always a responsible principal and verifiable chain-of-custody.

## Bottom Line

- SSMs provide linear scaling and inspectable dynamics via the A-matrix.
- Axiom Hive wraps inference in a deterministic, formally verified, cryptographically receipted execution layer.
- The guarantee is scoped to adherence to axioms and invariants; external-world risk is handled via audit and halt semantics.

- Determinism + proofs + ledger + signatures → outputs that are reproducible, explainable, and regulator-ready.

## Conclusion

"Math for the LLM" is not philosophical: it is an engineering pattern that moves the safety boundary from probabilistic heuristics to formal, auditable gates. In high‑stakes automation, deterministic control, formal proofs, and mandatory human approvals are the practical mechanisms that preserve sovereignty and certifiability.