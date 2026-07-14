param(
    [string]$Version
)

$ErrorActionPreference = "Stop"

if (-not $Version) {
    throw "Version is required."
}

Write-Host "Generating release notes for version $Version..."

cargo metadata --format-version 1 | Out-Null
