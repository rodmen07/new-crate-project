# new-crate-project

Starter Rust CLI crate with testable command handling.

## Commands

- `new-crate-project` prints a readiness message.
- `new-crate-project greet --name <value>` prints a greeting.

## Development

- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Project Hygiene

- CI workflow at `.github/workflows/ci.yml` runs fmt, clippy, and tests.
- Changelog tracking lives in `CHANGELOG.md`.
