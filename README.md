# FESTEJO Protocol Ecosystem (FRSP v1.0)

FESTEJO is a modular, protocol-based ecosystem built on Stellar, utilizing a deterministic Rust execution layer to ensure cryptographic and logic consistency across distributed domain services.

## Core Architectural Principle: The FRSP v1.0 Standard
Every operation strictly adheres to the FESTEJO Rust Spec Protocol (FRSP) v1.0. The system operates on an immutable pipeline:

**Actor → Action → Proof → Validation → StellarRecord → Persistence**

### Determinism via FRSP
To eliminate cross-repo drift, FESTEJO uses a unified Rust core (`festejo-frsp`) providing:
- **Canonical Hashing:** `SHA256(canonical_json(data))` ensures identical outputs for identical inputs across all repositories.
- **Strict Proof Logic:** Centralized proof generation, signature verification, and replay protection.
- **Shared Data Models:** Universal `Actor`, `Action`, `Proof`, and `StellarRecord` structures.

## Repository Structure

| Repository | Domain | Role |
| :--- | :--- | :--- |
| `festejo-core-protocol` | Orchestration | Pipeline execution engine & audit logging. |
| `festejo-stellar-sdk` | Blockchain | Sole interface for Stellar signing & contract interactions. |
| `festejo-events` | Domain | Event production (Actions). |
| `festejo-identity` | Domain | Identity graph & passport system. |
| `festejo-economy` | Domain | Rewards, Grants, Marketplace. |
| `festejo-frsp` | Infrastructure | Shared Rust crate implementing the protocol specification. |

## Usage & Development

### Adding FRSP Compliance to a Domain
1. **Dependency:** Include `festejo-frsp` in the `Cargo.toml` of the repository's Rust module.
2. **Implementation:** Use `festejo_frsp::crypto` and `festejo_frsp::proof` for all deterministic operations (no reimplementation of crypto allowed in Python).
3. **Bindings:** Expose required API surface (e.g., `generate_hash`, `check_replay`) via `pyo3` to the Python orchestration layer.

### Building
```bash
# Compile Rust extensions
cd rust
cargo build --release
cd ..
maturin develop --release
```

## Legacy Compatibility
- **apps/legacy_app**: Read-only routing/compatibility layer with zero business logic execution rights.
