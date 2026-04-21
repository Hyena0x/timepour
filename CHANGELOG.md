# Changelog

## Unreleased

- Clarified the current git-only install path while the crates.io release is pending.
- Replaced minute/second flags with a single `DURATION` argument supporting `15`, `15m`, `90s`, `1:30`, and `1m30s`.
- Added CLI, timer, and renderer unit tests.
- Declared Rust 1.85 as the minimum supported Rust version.
- Started tracking Cargo.lock so git installs can use locked dependency versions.
- Added responsive terminal layout improvements and medium-sized pixel digits.
- Added GitHub issue and pull request templates.
- Updated the Rust CI workflow actions.

## v0.1.0 - 2026-04-16

- Initial public GitHub release.
- Added a terminal countdown with deterministic tetromino stacking.
- Added focus and break modes.
- Added GitHub Actions CI for formatting, clippy, and tests.
