//! Regression tests for the 2026-10 security audit: the API key must never
//! be required as a command-line argument.
//!
//! These tests never make a network call: `config set-key` only writes a
//! local file, and every test touches a temporary, isolated `$HOME`.

use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("coinpaprika-cli")
}

#[test]
fn config_set_key_reads_from_piped_stdin() {
    let home = tempfile::tempdir().unwrap();
    bin()
        .args(["config", "set-key"])
        .env("HOME", home.path())
        .write_stdin("sk_test_from_stdin\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("API key saved"));

    let saved = std::fs::read_to_string(home.path().join(".coinpaprika/config.json")).unwrap();
    assert!(saved.contains("sk_test_from_stdin"));
}

#[test]
fn config_set_key_rejects_an_empty_stdin_key() {
    let home = tempfile::tempdir().unwrap();
    bin()
        .args(["config", "set-key"])
        .env("HOME", home.path())
        .write_stdin("\n")
        .assert()
        .failure();
}

#[test]
fn config_set_key_with_argv_still_works_but_warns() {
    let home = tempfile::tempdir().unwrap();
    bin()
        .args(["config", "set-key", "sk_test_from_argv"])
        .env("HOME", home.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("other local users"));

    let saved = std::fs::read_to_string(home.path().join(".coinpaprika/config.json")).unwrap();
    assert!(saved.contains("sk_test_from_argv"));
}

#[test]
fn cli_flag_api_key_warns_that_it_is_visible_via_ps() {
    // Key resolution (and its warning) happens before the command runs,
    // so pairing --api-key with a command whose own validation fails
    // locally (never reaching the network) still exercises the warning.
    let home = tempfile::tempdir().unwrap();
    bin()
        .args(["--api-key", "sk_test_flag", "coin", ".."])
        .env("HOME", home.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("other local users"));
}
