# Upgrade Strategy

libsodium_clone should evolve with the same discipline expected from a security library.

## Versioning

- Patch releases: bug fixes, documentation, and build hardening only.
- Minor releases: additive APIs that preserve compatibility.
- Major releases: only when compatibility or guarantees must change.

## Dependency Policy

- Prefer small, audited dependencies.
- Avoid churn in cryptographic code paths.
- Upgrade dependencies in isolation and validate with the full test suite.

## Security Review Gates

- Review any entropy, memory, or crypto backend change as a security item.
- Require at least one maintainer review for release branches.
- Publish release notes with compatibility impact and migration guidance.
