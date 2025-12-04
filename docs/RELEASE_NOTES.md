# Release Notes — Public Release (Draft)

Date: December 4, 2025

This release prepares the AXIOM HIVE documentation and key runtime safeguards for public distribution. It collects recent technical essays, security guidance, and deterministic runtime updates needed to make DSIF suitable for audited actuation.

## Highlights

- Added technical comparison: **Axiom Hive (DSIF)** vs **Grok** — `docs/axiom-hive-vs-grok-comparison.md`
- Added deterministic math primer: **Math for the LLM (C=0)** — `docs/math-for-llm.md`
- Added long-form technical diagnosis: **The Geodesic Bifurcation** — `docs/geodesic-bifurcation.md`
- Implemented a human-in-the-loop gating mechanism in the DSIF runtime to require an approver attestation for any non-read/state-changing action (code in `axiom-s1/src/dsif.rs`).
- Fixed several workspace build issues (missing `hex` dependency in some crates and minor type cleanups) so the repository builds successfully under a development profile.

## Why this matters

The added materials and runtime changes make it possible to operate DSIF with a deterministic, auditable control loop that: (1) constrains nondeterminism (C=0 policy), (2) enforces policy and simulation before actuation, and (3) requires explicit human approver attestations for state changes.

## Notable implementation notes

- The human-approval gate checks for attestations with an `approve:<decision_id>` statement and the approver role. Cryptographic signature verification/DID resolution is recommended but not yet enforced in code — consider integrating a DID resolver and signature validation in `verification/`.
- Operator-facing mechanisms (API/UI) to submit signed approvals are not yet included. Recommended next step: add an approval submission endpoint in the Portal and an admin UI in `axiom-s1`/`portal`.
- Determinism caveats: floating-point behavior across architectures can still introduce drift; consider fixed-point arithmetic or deterministic runtime containers for high-assurance deployments.

## How to cite or reference

When referencing this repository in documentation or academic work, cite the repository name `AXIOM HIVE` and date (December 4, 2025). For security inquiries and responsible disclosure, contact `security@axiomhive.local`.

## Next steps (suggested)

1. Add cryptographic attestation verification (DID/signature checks) into `verification/` and plug into DSIF gating.
2. Implement an approval submission API and minimal admin UI in the Portal.
3. Run a full docs spellcheck and security review prior to wide distribution.
4. Draft a short slide deck summarizing the Axiom Shift for stakeholder briefings.

---

For questions or to propose edits to these release notes, open a PR or contact the repository owners.
