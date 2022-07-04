use assert_cmd::Command;
use indoc::indoc;

#[test]
fn test_create_new_pair() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-c", "DB_PASSWORD=348a1912"])
        .write_stdin(indoc! {r#"
          DB_URI=postgres://db/kvu
          DB_USERNAME=kvu
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
fn test_do_not_create_when_the_key_exists() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-c", "DB_PASSWORD=348a1912"])
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
        "#});
}
