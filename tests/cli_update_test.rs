use assert_cmd::Command;
use indoc::indoc;

#[test]
fn test_update_existing_pair() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-u", "DB_PASSWORD=348a1912"])
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
fn test_does_not_update_when_the_key_does_not_exist() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-u", "DB_PASSWORD=348a1912"])
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
