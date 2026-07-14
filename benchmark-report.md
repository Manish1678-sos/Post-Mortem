# Benchmark Report

This file will collect benchmark results from `criterion` runs comparing `libsodium_clone` to the original `libsodium` via `sodiumoxide`.

Run the benchmarks locally with:

```powershell
cargo bench
```

Store the resulting comparison graphs and a brief summary here.

This report compares the performance of `libsodium_clone` with the reference `libsodium` implementation (`sodiumoxide`) using the Criterion benchmarking framework.

## Environment

- OS: Windows
- Benchmark Framework: Criterion 0.5
- Rust Edition: 2021

## Command Used

```powershell
cargo bench
```

## Results

| Benchmark | libsodium_clone | libsodium |
|-----------|----------------:|----------:|
| SHA-256 | ~87.6 ns | ~282.7 ns |
| Secretbox | ~494.4 ns | ~496.1 ns |

## Summary

- The SHA-256 implementation in `libsodium_clone` completed in approximately **87.6 ns**, while the `libsodium` (`sodiumoxide`) implementation completed in approximately **282.7 ns** under this benchmark setup.
- The Secretbox implementation showed nearly identical performance between both libraries:
  - `libsodium_clone`: **~494.4 ns**
  - `libsodium`: **~496.1 ns**
- These measurements were obtained using Criterion. Results may vary across different hardware, operating systems, compiler versions, and benchmark configurations.

## Conclusion

The benchmark indicates that both implementations provide comparable performance for Secretbox, while the SHA-256 benchmark in this specific environment favored `libsodium_clone`. Additional benchmarking on different systems and workloads is recommended before drawing broader performance conclusions.