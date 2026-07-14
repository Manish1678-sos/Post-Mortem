# CI/CD

The CI pipeline verifies formatting, linting, and tests across Windows, Linux, and macOS.

## Continuous Integration

- `cargo fmt --all --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`

## Release

- Tag a release with `vX.Y.Z`.
- The release workflow validates the crate and publishes it to crates.io.
- Store the publish token in `CARGO_REGISTRY_TOKEN`.

## Maintenance

- Keep the release workflow and local deployment scripts in sync.
- Review security-sensitive dependency changes separately from feature work.
