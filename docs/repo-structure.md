# Repository Structure

This repository starts with a single Rust crate and expands into module families that track libsodium's documentation hierarchy.

## Current Structure

- `Cargo.toml`: crate manifest and dependency boundary.
- `rust-toolchain.toml`: pinned toolchain for reproducible builds.
- `src/lib.rs`: public facade.
- `src/init.rs`: one-time initialization logic.
- `src/version.rs`: version and library identity reporting.
- `docs/architecture.md`: system architecture and planned module boundaries.

## Planned Structure

- `src/randombytes/`
- `src/memory/`
- `src/helpers/`
- `src/secret_key/`
- `src/public_key/`
- `src/hashing/`
- `src/password/`
- `src/advanced/`
- `tests/`
- `examples/`
- `scripts/`
- `.github/workflows/`

## Parity Principle

Each major libsodium feature family gets a Rust module boundary that can be tested independently and documented with matching examples.
