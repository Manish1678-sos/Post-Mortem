param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

Write-Host "Running local build and validation for libsodium_clone..."
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

if ($Release) {
    cargo build --release --all-features
    cargo package --allow-dirty
}
