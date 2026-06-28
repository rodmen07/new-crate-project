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
