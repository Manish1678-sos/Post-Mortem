Write-Host "Running libsodium_clone parity tests..."
Invoke-Expression "cargo test -- --nocapture"

Write-Host "Running the differential parity harness..."
Invoke-Expression "cargo test --test differential"

Write-Host "To run benchmarks, use:"
Write-Host "  cargo bench"

Write-Host "To validate the original upstream libsodium suite, run the upstream project tests in Docker/WSL."
