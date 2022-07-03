use assert_cmd::Command;
use indoc::indoc;
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

#[test]
fn test_replace_when_present() {
    Command::cargo_bin("kvu")
        .unwrap()
        .arg("DB_PASSWORD=348a1912")
        .write_stdin(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
          DB_PASSWORD=password
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
          DB_PASSWORD=348a1912
        "#});
}

#[test]
fn test_add_when_absent() {
    Command::cargo_bin("kvu")
        .unwrap()
        .arg("TOKEN=348a1912")
        .write_stdin(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
          DB_PASSWORD=password
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
          DB_PASSWORD=password
          TOKEN=348a1912
        "#});
}

#[test]
fn test_replace_multiple_arguments() {
    Command::cargo_bin("kvu")
        .unwrap()
        .arg("DB_USERNAME=key-value-update")
        .arg("DB_PASSWORD=348a1912")
        .write_stdin(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
          DB_PASSWORD=password
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=key-value-update
          DB_PASSWORD=348a1912
        "#});
}

#[test]
fn test_replace_multiple_arguments_and_add_new() {
    Command::cargo_bin("kvu")
        .unwrap()
        .arg("DB_USERNAME=key-value-update")
        .arg("DB_PASSWORD=348a1912")
        .arg("TOKEN=348a1912")
        .write_stdin(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
          DB_PASSWORD=password
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=key-value-update
          DB_PASSWORD=348a1912
          TOKEN=348a1912
        "#});
}
