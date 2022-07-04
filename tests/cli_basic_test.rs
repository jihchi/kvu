use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn sanity_check() {
    Command::cargo_bin("kvu")
        .unwrap()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("print this help menu"))
        .stdout(predicate::str::contains("print the version"));
}

#[test]
fn test_do_nothing() {
    Command::cargo_bin("kvu").unwrap().assert().success();
}

#[test]
fn test_version() {
    Command::cargo_bin("kvu")
        .unwrap()
        .arg("-v")
        .assert()
        .success()
        .stdout(format!("{}\n", env!("CARGO_PKG_VERSION")));
}
