param(
    [string]$TargetVersion = "latest"
)

$ErrorActionPreference = "Stop"

Write-Host "Refreshing dependency graph to $TargetVersion..."

cargo update
cargo fmt --all
cargo check --all-features
