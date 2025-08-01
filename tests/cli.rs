use std::path::Path;

use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

fn base_temp_fs() -> TempDir {
    let temp_fs = TempDir::new().unwrap();

    temp_fs
        .child(".config")
        .child("run-rs")
        .child("tasks.json")
        .touch()
        .unwrap();

    temp_fs
        .child("project")
        .child(".vscode")
        .child("tasks.json")
        .touch()
        .unwrap();

    temp_fs
}

fn base_command(root: &Path) -> Command {
    let mut cmd = Command::cargo_bin("run-rs").unwrap();
    cmd.env("HOME", root)
        .env("XDG_CONFIG_HOME", root.join(".config"))
        .env("RUN_RS_CONFIG", root.join(".config/run-rs"))
        .current_dir(root.join("project"));
    cmd
}

#[test]
fn success_base() {
    let temp_fs = base_temp_fs();
    let mut cmd = base_command(temp_fs.path());
    cmd.assert().success();
}

#[test]
fn change_work_dir() {
    let temp_fs = base_temp_fs();
    temp_fs
        .child("another_project")
        .child(".vscode")
        .child("tasks.json")
        .touch()
        .unwrap();
    let mut cmd = base_command(temp_fs.path());
    cmd.arg("--work-dir")
        .arg(temp_fs.path().join("another_project"))
        .arg("--list")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            temp_fs
                .join("another_project/.vscode/tasks.json")
                .to_str()
                .unwrap(),
        ));
}

#[test]
fn fail_on_missing_env_variables() {
    let mut cmd = Command::cargo_bin("run-rs").unwrap();
    cmd.env_remove("HOME")
        .env_remove("RUN_RS_CONFIG_DIR")
        .env_remove("XDG_CONFIG_HOME")
        .assert()
        .failure();
}

#[test]
fn fail_option_rerun() {
    let temp_fs = base_temp_fs();
    let mut cmd = base_command(temp_fs.path());
    cmd.arg("--rerun")
        .assert()
        .failure()
        .stderr(predicate::str::contains("WIP"));
}
