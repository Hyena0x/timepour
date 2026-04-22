# Changelog

## Unreleased

- Clarified the current git-only install path while the crates.io release is pending.
- Restore the terminal on TUI errors.
- Replaced minute/second flags with a single `DURATION` argument supporting `15`, `15m`, `90s`, `1:30`, and `1m30s`.
- Added CLI, timer, and renderer unit tests.
- Added GitHub Actions coverage for the Rust 1.85 minimum supported Rust version.
- Added GitHub Actions package verification for release readiness.
- Kept locked transitive TUI dependencies compatible with Rust 1.85.
- Removed let-chain syntax that is not stable on Rust 1.85.
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
