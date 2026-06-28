# new-crate-project

Starter Rust CLI crate with testable command handling.

This crate is evolving into a practical companion utility for calm-daily-coach.

It can be used both as a CLI and as a Rust library directly inside app code.

## Commands

- `new-crate-project` prints a readiness message.
- `new-crate-project greet --name <value>` prints a greeting.
- `new-crate-project greet --name <value> --uppercase` prints an uppercase greeting.
- `new-crate-project sum <values...>` prints the integer total (empty input prints `0`).
- `new-crate-project version` prints the current crate version.
- `new-crate-project checkin --mood <1-5> --energy <1-5> [--friction <note>]` prints a practical next-step suggestion.
- `new-crate-project plan --priority <text> [--priority <text>] [--stop <HH:MM>] [--effort <low|medium|high>] [--focus <text>]` compiles a practical day plan.
- `--format json` emits structured JSON (`command`, `message`) instead of plain text.
- `--out <file>` writes the rendered output to disk so calm-daily-coach (or scripts) can read it.
- `--out-dir <dir>` writes both a timestamped artifact and `latest.<ext>` for stable app ingestion.

### Tandem Usage Example

`new-crate-project --format json --out artifacts/latest-checkin.json checkin --mood 3 --energy 4 --friction "task switching"`

`new-crate-project --format json --out-dir artifacts checkin --mood 3 --energy 4`

## Library Usage

```rust
use new_crate_project::{
	build_day_plan_data, checkin_advice, CheckinInput, EffortLevel, PlanInput,
};

let checkin = CheckinInput {
	mood: 3,
	energy: 4,
	friction: Some("task switching".to_string()),
};
let advice = checkin_advice(&checkin);

let plan = PlanInput {
	priorities: vec!["Ship one small feature".into(), "Write reflection".into()],
	stop: Some("17:30".into()),
	effort: EffortLevel::Medium,
	focus: Some("Keep scope tight".into()),
};
let day_plan = build_day_plan_data(&plan);

if matches!(advice.strategy, new_crate_project::CheckinStrategy::FrictionUnblock) {
	println!("Create a short unblock step in the UI");
}
assert!(!day_plan.priorities.is_empty());
```

Structured APIs for app integration:

- `checkin_advice(&CheckinInput) -> CheckinAdvice` returns `strategy` + `message`.
- `build_day_plan_data(&PlanInput) -> DayPlan` returns normalized priorities and metadata.
- `build_day_plan(&PlanInput) -> String` remains available for text rendering.

All core domain structs and enums in the library API derive both `Serialize` and `Deserialize`, so app code can round-trip them through JSON without adapter types.

## Development

- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Project Hygiene

- CI workflow at `.github/workflows/ci.yml` runs fmt, clippy, and tests.
- Changelog tracking lives in `CHANGELOG.md`.
- Integration tests in `tests/cli.rs` cover binary-level behavior.
