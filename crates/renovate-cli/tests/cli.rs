//! Integration tests for the `renovate` binary.
//!
//! These exercise the early-flag surface that exists in slice 1: `--version`
//! / `-v` (Renovate-compatible: bare version line, exit 0) and `--help`
//! (clap default, exit 0). The bare-version contract is the one most likely
//! to silently regress as the CLI grows, so it gets a snapshot-style match.

use assert_cmd::Command;
use predicates::prelude::*;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn renovate() -> Command {
    Command::cargo_bin("renovate").expect("binary 'renovate' built")
}

#[test]
fn version_long_flag_prints_bare_version() {
    renovate()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::eq(format!("{PKG_VERSION}\n")));
}

#[test]
fn version_short_flag_matches_long_flag() {
    renovate()
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::eq(format!("{PKG_VERSION}\n")));
}

#[test]
fn help_flag_succeeds_and_mentions_repositories() {
    renovate()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("repositories"));
}

#[test]
fn unknown_flag_exits_with_usage_error() {
    // clap exits with status 2 for usage errors, matching Renovate's
    // commander-based behavior.
    renovate()
        .arg("--this-flag-does-not-exist")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn no_args_succeeds() {
    // Slice 1 has no work to dispatch yet; an arg-less invocation must not
    // crash and must not produce stray output.
    renovate()
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn git_fs_legacy_flags_are_silently_dropped() {
    // Renovate's `migrateArgs` filters every `--git-fs*` token before the
    // option parser runs. Without that filter, clap would reject the flag
    // as unknown and exit 2. With it wired up, the flag disappears and the
    // CLI succeeds.
    renovate()
        .arg("--git-fs-something")
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}
