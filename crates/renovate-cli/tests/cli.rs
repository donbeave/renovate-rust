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

// ── Logging ────────────────────────────────────────────────────────────────

#[test]
fn invalid_log_level_exits_1_with_fatal_message() {
    // Mirrors Renovate's validateLogLevel: exits 1 and writes a fatal-level
    // message when LOG_LEVEL is unrecognized.
    renovate()
        .env("LOG_LEVEL", "not_a_level")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Invalid log level"))
        .stderr(predicate::str::contains("not_a_level"));
}

#[test]
fn log_level_debug_does_not_crash() {
    renovate()
        .env("LOG_LEVEL", "debug")
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn log_level_fatal_does_not_crash() {
    // `fatal` is a valid Renovate level name; maps to error in tracing.
    renovate()
        .env("LOG_LEVEL", "fatal")
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn log_format_json_does_not_crash() {
    renovate()
        .env("LOG_FORMAT", "json")
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn no_color_env_does_not_crash() {
    renovate()
        .env("NO_COLOR", "1")
        .arg("--version")
        .assert()
        .success();
}

// ── Legacy-flag migration ───────────────────────────────────────────────────

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
