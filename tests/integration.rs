use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_flag() {
    Command::cargo_bin("flow")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn version_flag() {
    Command::cargo_bin("flow")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn invalid_command_exits_with_error() {
    Command::cargo_bin("flow")
        .unwrap()
        .arg("nonexistent_cmd_xyz_12345")
        .assert()
        .failure()
        .stderr(predicate::str::contains("is not a valid executable"));
}
