use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn default_invocation_prints_readiness_message() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ready"));
}

#[test]
fn greet_subcommand_prints_name() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args(["greet", "--name", "Copilot"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Copilot!"));
}

#[test]
fn greet_subcommand_uppercase_prints_shouty_message() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args(["greet", "--name", "Copilot", "--uppercase"])
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO, COPILOT!"));
}

#[test]
fn sum_subcommand_prints_total() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args(["sum", "10", "-3", "4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("11"));
}

#[test]
fn sum_subcommand_without_values_defaults_to_zero() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args(["sum"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0"));
}

#[test]
fn json_output_mode_prints_structured_response() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args(["--format", "json", "greet", "--name", "Copilot"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"command\": \"greet\""))
        .stdout(predicate::str::contains("\"message\": \"Hello, Copilot!\""));
}

#[test]
fn version_subcommand_prints_version_text() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args(["version"])
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn version_subcommand_prints_version_json() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args(["--format", "json", "version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"command\": \"version\""))
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn checkin_subcommand_prints_guidance() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args([
        "checkin",
        "--mood",
        "3",
        "--energy",
        "4",
        "--friction",
        "task switching",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Check-in complete"))
    .stdout(predicate::str::contains("task switching"));
}

#[test]
fn checkin_subcommand_supports_json_output() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args([
        "--format", "json", "checkin", "--mood", "4", "--energy", "2",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"command\": \"checkin\""))
    .stdout(predicate::str::contains("Keep it light"));
}

#[test]
fn plan_subcommand_builds_day_plan() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args([
        "plan",
        "--priority",
        "Write reflection",
        "--priority",
        "Ship one small feature",
        "--stop",
        "17:30",
        "--effort",
        "medium",
        "--focus",
        "Keep scope tight",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Plan ready:"))
    .stdout(predicate::str::contains("Write reflection"))
    .stdout(predicate::str::contains("Stop target: 17:30"));
}

#[test]
fn plan_subcommand_supports_json_output() {
    let mut cmd = Command::cargo_bin("new-crate-project").unwrap();
    cmd.args([
        "--format",
        "json",
        "plan",
        "--priority",
        "Top task",
        "--effort",
        "low",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"command\": \"plan\""))
    .stdout(predicate::str::contains("Top task"));
}
