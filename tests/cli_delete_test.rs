use assert_cmd::Command;
use indoc::indoc;

#[test]
fn test_delete_existing_pair() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-d", "DB_PASSWORD"])
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
        "#});
}

#[test]
fn test_does_nothing_when_the_key_does_not_exist() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-d", "DB_PASSWORD"])
        .write_stdin(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
        "#});
}
