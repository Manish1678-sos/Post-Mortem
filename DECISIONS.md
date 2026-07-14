# DECISIONS

This file documents key architectural and implementation decisions made during the libsodium_clone hackathon port.

## Scope
- Provide a Rust-native API surface inspired by libsodium with safe defaults and small audited primitives.

## Crypto primitives
- Use Rust crates for primitives where appropriate (e.g. `sha2` for SHA-256) to avoid reimplementing low-level operations incorrectly.
- Provide a compact facade that mirrors libsodium naming and operational expectations (initialization boundary, zeroization, constant-time comparisons).

## Backends
- The project currently implements high-level helpers directly in Rust and uses `sodiumoxide` only as a test/reference dependency for parity checks.

## Testing and validation
- Functional parity is checked by running identical inputs through this crate and the original libsodium via `sodiumoxide`.
- Differential testing harnesses and fuzzing are planned; `tests/differential.rs` provides a corpus-based check.

## Benchmarking
- Use `criterion` for reproducible benchmarks comparing our implementation versus libsodium where feasible.

## Trade-offs
- Reusing proven crates (sha2) reduces risk but means the project is more of a glue layer than a full reimplementation of every crypto primitive.

## Next steps
- Add more primitives, run the upstream libsodium test-suite in CI (Docker/WSL), and add automated fuzzing integration.
