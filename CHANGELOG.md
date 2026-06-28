# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog,
and this project follows Semantic Versioning.

## [Unreleased]

### Added

- End-to-end CLI integration tests covering default invocation and `greet` output.
- `sum` CLI subcommand for adding integer values.
- `greet --uppercase` option for emphasized greeting output.
- Global `--format text|json` output mode for CLI responses.
- `version` subcommand for printing the crate version.
- `checkin` subcommand with mood and energy prompts plus a next-step suggestion.
- `plan` subcommand that compiles up to three priorities into a practical day plan.
- Global `--out <file>` option to persist CLI output for app-side consumption.
- Global `--out-dir <dir>` option that writes timestamped artifacts plus a stable `latest` file.
- Library-first APIs (`CheckinInput`, `PlanInput`, `checkin_suggestion`, `build_day_plan`) for direct app integration.
- Typed structured APIs: `CheckinAdvice` with `CheckinStrategy`, plus `DayPlan` via `build_day_plan_data`.
- Serde-ready domain models (`Serialize` + `Deserialize`) for easier app JSON round-tripping.
- Integration examples for check-in and planning pipelines (`examples/checkin_pipeline.rs`, `examples/plan_pipeline.rs`).
- Stdin JSON bridge example for direct app-to-library piping (`examples/stdin_bridge.rs`).
