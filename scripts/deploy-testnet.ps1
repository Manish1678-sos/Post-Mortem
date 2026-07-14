param(
    [string]$Registry = "testnet",
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"

Write-Host "Preparing prerelease package for registry '$Registry'..."

cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo package --allow-dirty

if ($DryRun) {
    Write-Host "Dry run complete. Skipping publish."
    exit 0
}

cargo publish --registry $Registry
