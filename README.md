# new-crate-project

Starter Rust CLI crate with testable command handling.

## Commands

- `new-crate-project` prints a readiness message.
- `new-crate-project greet --name <value>` prints a greeting.
- `new-crate-project greet --name <value> --uppercase` prints an uppercase greeting.
- `new-crate-project sum <values...>` prints the integer total (empty input prints `0`).
- `new-crate-project version` prints the current crate version.
- `--format json` emits structured JSON (`command`, `message`) instead of plain text.

## Development

- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Project Hygiene

- CI workflow at `.github/workflows/ci.yml` runs fmt, clippy, and tests.
- Changelog tracking lives in `CHANGELOG.md`.
- Integration tests in `tests/cli.rs` cover binary-level behavior.
