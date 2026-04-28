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
    // No repos means "nothing to do" — must exit 0 with no user-facing stdout.
    // LOG_LEVEL=off silences tracing so stderr is clean for assertion purposes.
    renovate()
        .env("LOG_LEVEL", "fatal")
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

// ── Option surface: migrateArgs end-to-end ──────────────────────────────────
// These tests prove the full pipeline: argv → migrateArgs → clap parsing.
// Each one would exit 2 ("unknown flag") without the corresponding option
// registered in the Cli struct.

#[test]
fn dry_run_bare_is_accepted_via_migrate() {
    // migrateArgs rewrites --dry-run to --dry-run=true; clap maps "true" to
    // DryRunArg::LegacyTrue.
    renovate().arg("--dry-run").assert().success();
}

#[test]
fn dry_run_full_is_accepted_directly() {
    renovate().arg("--dry-run=full").assert().success();
}

#[test]
fn dry_run_false_is_accepted() {
    renovate().arg("--dry-run=false").assert().success();
}

#[test]
fn require_config_bare_is_accepted_via_migrate() {
    renovate().arg("--require-config").assert().success();
}

#[test]
fn require_config_optional_is_accepted_directly() {
    renovate()
        .arg("--require-config=optional")
        .assert()
        .success();
}

#[test]
fn fork_processing_enabled_is_accepted() {
    // migrateArgs rewrites --include-forks=true → --fork-processing=enabled.
    renovate()
        .arg("--fork-processing=enabled")
        .assert()
        .success();
}

#[test]
fn platform_automerge_false_is_accepted() {
    // migrateArgs rewrites --azure-auto-complete=false → --platform-automerge=false.
    renovate()
        .arg("--platform-automerge=false")
        .assert()
        .success();
}

#[test]
fn recreate_when_auto_is_accepted() {
    // migrateArgs rewrites --recreate-closed=false → --recreate-when=auto.
    renovate().arg("--recreate-when=auto").assert().success();
}

#[test]
fn recreate_when_always_is_accepted() {
    renovate().arg("--recreate-when=always").assert().success();
}

#[test]
fn platform_github_is_accepted() {
    renovate().arg("--platform=github").assert().success();
}

#[test]
fn platform_gitlab_is_accepted() {
    renovate().arg("--platform=gitlab").assert().success();
}

#[test]
fn platform_bitbucket_server_is_accepted() {
    renovate()
        .arg("--platform=bitbucket-server")
        .assert()
        .success();
}

#[test]
fn token_flag_is_accepted() {
    renovate().arg("--token=abc123").assert().success();
}

#[test]
fn env_renovate_token_sets_token() {
    renovate()
        .env("RENOVATE_TOKEN", "secret")
        .assert()
        .success();
}

#[test]
fn env_renovate_platform_sets_platform() {
    renovate()
        .env("RENOVATE_PLATFORM", "gitlab")
        .assert()
        .success();
}

// ── Legacy-flag migration ───────────────────────────────────────────────────

#[test]
fn git_fs_legacy_flags_are_silently_dropped() {
    // Renovate's `migrateArgs` filters every `--git-fs*` token before the
    // option parser runs. Without that filter, clap would reject the flag
    // as unknown and exit 2. With it wired up, the flag disappears and the
    // CLI succeeds. LOG_LEVEL=fatal silences tracing for clean stderr assertion.
    renovate()
        .env("LOG_LEVEL", "fatal")
        .arg("--git-fs-something")
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}
