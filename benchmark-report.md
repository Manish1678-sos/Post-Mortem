# Benchmark Report

## Validation status

Fresh verification was completed on 2026-07-14.

- `cargo test -- --nocapture` passed with 3 tests green and 0 failures.
- The hash and secretbox parity checks matched the `sodiumoxide` reference implementation for the demo inputs and randomized differential cases.

## Benchmark command

```powershell
cargo bench --bench hash_bench --bench secretbox_bench -- --noplot
```

## Measured results

### Hashing (SHA-256)

- `our_sha256`: median about `192.43 ns`
- `libsodium_sha256`: median about `632.20 ns`

### Secretbox

- `our_secretbox`: median about `1.2631 µs`
- `libsodium_secretbox`: median about `1.0622 µs`

## Interpretation

These measurements show that the current Rust implementation is faster for the simple SHA-256 benchmark and slightly slower for the secretbox benchmark in this environment. The results are useful as a quick performance snapshot, but they should not be treated as a full platform-wide benchmark suite.

## Upstream libsodium suite

The original C-based libsodium test suite was not runnable in this environment because Docker and WSL were not available. Local correctness was still validated through the parity and differential tests against the `sodiumoxide` reference implementation.

