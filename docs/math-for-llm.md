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

## Conclusion

"Math for the LLM" is not philosophical: it is an engineering pattern that moves the safety boundary from probabilistic heuristics to formal, auditable gates. In high‑stakes automation, deterministic control, formal proofs, and mandatory human approvals are the practical mechanisms that preserve sovereignty and certifiability.