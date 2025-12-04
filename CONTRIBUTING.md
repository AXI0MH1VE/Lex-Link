# Contributing to AXIOM HIVE

Thank you for your interest in contributing to AXIOM HIVE. This document outlines the process for contributing code, documentation, and feedback.

---

## Code of Conduct

All contributors are expected to maintain a professional, respectful, and inclusive environment. Discrimination, harassment, and disruptive behavior are not tolerated.

---

## Contribution Guidelines

### Before You Start

1. **Read the docs** — Familiarize yourself with `docs/README.md` and the relevant component documentation.
2. **Check the issue tracker** — See if someone is already working on your proposed change.
3. **Understand the architecture** — Review `README.md` and the relevant module's design.

### Workflow

1. **Fork the repository** and create a feature branch:
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. **Make your changes** with clear commit messages:
   ```bash
   git commit -m "feat: description of feature" -m "Additional context if needed"
   ```

3. **Test your changes**:
   - For Rust: `cargo test --all`
   - For Python: `pytest invariance/tests/`
   - Ensure no regressions

4. **Build and verify**:
   ```bash
   cargo build --release --all
   ```

5. **Push your branch** and open a **Pull Request**:
   - Include a clear description of the change
   - Reference any related issues
   - Explain the rationale and any trade-offs

### Commit Message Style

Use conventional commits:
- `feat:` — New feature
- `fix:` — Bug fix
- `docs:` — Documentation only
- `refactor:` — Code refactoring
- `test:` — Test additions or changes
- `ci:` — CI/CD configuration
- `chore:` — Dependency or tooling updates

Example:
```
feat(dsif): add human-approver attestation gating

Require explicit human approval attestations for non-read state-changing actions.
Adds has_human_approval() and add_human_approval() methods to ControlledActuation.

Closes #42
```

---

## Development Setup

### Local Environment

```bash
# Clone the repository
git clone https://github.com/AXI0MH1VE/Lex-Link.git
cd Lex-Link

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python 3.11+
# (Use brew on macOS, apt on Linux, or your preferred package manager)

# Build workspace
cargo build --release --all
cd invariance && pip install -e . && cd ..

# Run tests
cargo test --all
pytest invariance/tests/
```

### Docker Development

```bash
# Build the development image
docker build -t axiom-hive-dev .

# Run a development container
docker run -it --rm -v $(pwd):/workspace axiom-hive-dev /bin/bash

# Inside the container
cd /workspace
cargo build --release --all
```

---

## Testing

### Unit Tests

All new code must include unit tests:

```bash
# Rust tests
cargo test --all

# Python tests
pytest invariance/tests/ -v
```

### Integration Tests

For features that touch multiple components, add integration tests:

```bash
# Example: Test proof engine + audit service
cargo test --all --test integration_tests
```

### Manual Testing

For UI components (browser, portal), perform manual testing:
- Test on target hardware (M1/M2 macOS for browser)
- Verify determinism (identical inputs → identical outputs)
- Check for side effects (network calls, file writes, etc.)

---

## Documentation

### Code Comments

- Document public APIs with rustdoc or pydoc
- Explain non-obvious logic with inline comments
- Include examples in docstrings where helpful

Example (Rust):
```rust
/// Validates that output is logically consistent with intent.
///
/// Returns `true` if C = 0 (zero contradiction), `false` otherwise.
///
/// # Examples
///
/// ```
/// let checker = InvariantCheck::new(&axioms);
/// assert!(checker.validate(&output, &intent));
/// ```
pub fn validate(&self, output: &Output, intent: &Intent) -> bool {
    // ...
}
```

### Architecture Docs

If your change affects system architecture, document it:
- Add diagrams in `docs/` (ASCII art or PNG)
- Update `docs/README.md` with a link to your doc
- Describe the design rationale

### Changelog

For user-facing changes, add a line to the relevant changelog:
- `docs/RELEASE_NOTES.md` for all changes
- Component-specific changelogs (if any) for deep technical changes

---

## Code Review Process

### What to Expect

1. At least one reviewer will examine your PR.
2. Reviewers will check for:
   - Correctness and clarity
   - Adherence to architecture and style
   - Test coverage
   - Documentation
3. You may be asked to make revisions.
4. Once approved, a maintainer will merge your PR.

### Standards

- **Determinism**: All changes must preserve C=0 invariants.
- **Security**: No new network calls, file writes, or side effects without justification.
- **Performance**: Avoid regressions; benchmark if relevant.
- **Accessibility**: Comments and docs must be clear.

---

## Reporting Issues

### Security Issues

**Do not open a public issue for security problems.** Instead, email `security@axiomhive.local` with details.

### Bug Reports

Include:
- Clear title and description
- Steps to reproduce
- Expected vs. actual behavior
- Environment (OS, Rust version, etc.)
- Relevant logs or error messages

### Feature Requests

Include:
- Clear problem statement
- Proposed solution
- Alternative approaches considered
- Use cases and examples

---

## Getting Help

- **Questions**: Open a discussion or issue with the tag `question`
- **Docs**: Check `docs/README.md` and linked docs
- **Architecture**: Review `README.md` system diagrams
- **Security concerns**: Contact `security@axiomhive.local`

---

## License & Attribution

By contributing, you agree that your contributions are licensed under the same license as the project (see `LICENSE`).

All contributions will be attributed to you in the commit history.

---

## Maintainers

Current maintainers:
- **Alexis Adams** (@AXI0MH1VE) — Substrate Authority, Design & Review

For questions about contribution status, contact via GitHub issues or the official channels.

---

Thank you for helping build AXIOM HIVE!
