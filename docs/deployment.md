# Deployment

This repository is a Rust cryptography library, so deployment means building, packaging, versioning, and publishing the crate rather than deploying a network service.

## Local Deployment

1. Install the Rust toolchain from `rust-toolchain.toml`.
2. Run `powershell -ExecutionPolicy Bypass -File scripts/deploy-local.ps1`.
3. Review the generated build artifacts under `target/`.

## Testnet Deployment

For this project, "testnet" means a prerelease publish path to a non-production registry.

1. Configure the target registry in Cargo credentials.
2. Run `powershell -ExecutionPolicy Bypass -File scripts/deploy-testnet.ps1 -Registry testnet -DryRun`.
3. Remove `-DryRun` once the prerelease package is verified.

## Release Flow

1. Merge changes into the release branch.
2. Run the local validation script.
3. Tag the release with a semver tag.
4. Publish the crate and attach release notes.

## Upgrade Strategy

- Keep the public API stable unless a major version bump is approved.
- Update dependencies in a controlled pull request.
- Run the full test matrix after every dependency bump.
- Treat cryptographic backend changes as security-sensitive and review them separately.
