//! Entry point for the `renovate` binary.
//!
//! This is the Rust reimplementation of `renovatebot/renovate`'s CLI. Flag
//! parsing and behavior grow slice-by-slice toward Renovate parity; see
//! `docs/parity/implementation-ledger.md` for the running plan.

// Allow user-facing CLI output. The workspace lints forbid this elsewhere so
// that print! calls cannot leak into library code.
#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "CLI surface — user-facing output and error messages belong in this crate"
)]

mod logging;
mod migrate;

use std::process::ExitCode;

use clap::{ArgAction, Parser};

/// Renovate-compatible CLI. Long-form help and the full flag set will be
/// added incrementally as parity slices land.
#[derive(Debug, Parser)]
#[command(
    name = "renovate",
    bin_name = "renovate",
    about = "Automated dependency updates. Flexible so you don't need to be.",
    long_about = None,
    // Disable clap's built-in version flag so we can use Renovate's lowercase
    // `-v`/`--version` and print the bare version string for compatibility
    // with `renovatebot/renovate`'s commander-based CLI.
    disable_version_flag = true,
)]
struct Cli {
    /// Print the version and exit.
    #[arg(short = 'v', long = "version", action = ArgAction::SetTrue, global = true)]
    version: bool,

    /// Repositories to process (positional). Later slices dispatch these
    /// into the worker pipeline.
    #[arg(value_name = "repositories")]
    repositories: Vec<String>,
}

fn main() -> ExitCode {
    // 1. Initialize logging before anything that might emit log records.
    //    Reads LOG_LEVEL (default "info") and LOG_FORMAT (default pretty).
    //    Mirrors Renovate's logger init in lib/logger/index.ts.
    match logging::init() {
        logging::InitResult::Ok => {}
        logging::InitResult::InvalidLevel(lvl) => {
            // Mirror Renovate's validateLogLevel: print a fatal-level message
            // and exit 1.
            eprintln!(r#"{{"level":"fatal","msg":"Invalid log level","logLevel":{lvl:?}}}"#);
            return ExitCode::from(1);
        }
    }

    // 2. Legacy-flag migration before the option parser sees argv.
    //    See migrate module docs for semantics.
    let raw: Vec<String> = std::env::args().collect();
    let migrated = migrate::migrate_args(&raw);

    // 3. Parse flags.
    let cli = match Cli::try_parse_from(&migrated) {
        Ok(cli) => cli,
        Err(err) => {
            // clap renders help/usage/errors itself, including the right
            // exit-status convention (0 for --help, 2 for usage errors).
            err.exit();
        }
    };

    if cli.version {
        println!("{}", renovate_core::VERSION);
        return ExitCode::SUCCESS;
    }

    // Later slices replace this stub with the global worker entry point.
    ExitCode::SUCCESS
}
