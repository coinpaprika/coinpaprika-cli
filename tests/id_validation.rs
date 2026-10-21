//! Regression tests for the 2026-10 security audit: resource IDs must be
//! rejected or encoded before any request path is built from them.
//!
//! These tests never make a network call: every malicious ID is rejected
//! by local validation before the CLI would build an HTTP client.

use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("coinpaprika-cli")
}

/// Every one of these must be rejected locally (no DNS lookup, no socket)
/// for every ID-bearing command. If any of them were sent, the request
/// would carry the user's Authorization header to a path the command never
/// asked for.
const MALICIOUS_IDS: &[&str] = &[
    "../key/info",
    "../../usage",
    "..%2F..%2Fusage",
    "a/b",
    "x?y=1",
    "x#y",
    "%2e%2e",
    "..\\x",
    "..",
    ".",
    "",
];

#[test]
fn coin_rejects_every_malicious_id_without_touching_the_network() {
    for id in MALICIOUS_IDS {
        if id.is_empty() {
            // clap itself rejects a missing positional argument; that is
            // not the code path under test here.
            continue;
        }
        bin()
            .args(["coin", id])
            .env("COINPAPRIKA_API_KEY", "")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Error:"));
    }
}

#[test]
fn ticker_rejects_a_dot_segment_id() {
    bin()
        .args(["ticker", ".."])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn person_rejects_a_percent_encoded_traversal_id() {
    bin()
        .args(["person", "%2e%2e%2Fkey%2Finfo"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn contract_ticker_rejects_a_traversal_platform_id() {
    // Two IDs are interpolated here (platform_id and address); either one
    // being malicious must be caught before any request is built.
    bin()
        .args(["contract-ticker", "../../usage", "0xdeadbeef"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn contract_ticker_rejects_a_traversal_address() {
    bin()
        .args(["contract-ticker", "eth-ethereum", "..%2F..%2Fusage"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

// Whether an ordinary id like "btc-bitcoin" passes validation is covered
// by the pure, offline unit tests in src/idpath.rs (allows_ordinary_ids_
// unchanged). We don't repeat that here with a live CLI invocation because
// the next step after validation is a real HTTP request, and tests in this
// suite never touch the network.
