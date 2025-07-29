use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_no_crash() {
    let mut cmd = Command::cargo_bin("run-rs").unwrap();
    cmd.assert().success();
}

#[test]
fn test_option_rerun() {
    let mut cmd = Command::cargo_bin("run-rs").unwrap();
    cmd.arg("--rerun")
        .assert()
        .failure()
        .stderr(predicate::str::contains("WIP"));
}
