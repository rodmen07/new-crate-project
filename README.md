# new-crate-project

Starter Rust CLI crate with testable command handling.

This crate is evolving into a practical companion utility for calm-daily-coach.

## Commands

- `new-crate-project` prints a readiness message.
- `new-crate-project greet --name <value>` prints a greeting.
- `new-crate-project greet --name <value> --uppercase` prints an uppercase greeting.
- `new-crate-project sum <values...>` prints the integer total (empty input prints `0`).
- `new-crate-project version` prints the current crate version.
- `new-crate-project checkin --mood <1-5> --energy <1-5> [--friction <note>]` prints a practical next-step suggestion.
- `new-crate-project plan --priority <text> [--priority <text>] [--stop <HH:MM>] [--effort <low|medium|high>] [--focus <text>]` compiles a practical day plan.
- `--format json` emits structured JSON (`command`, `message`) instead of plain text.

## Development

- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Project Hygiene

- CI workflow at `.github/workflows/ci.yml` runs fmt, clippy, and tests.
- Changelog tracking lives in `CHANGELOG.md`.
- Integration tests in `tests/cli.rs` cover binary-level behavior.
