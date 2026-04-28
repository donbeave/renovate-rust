//! Entry point for the `renovate` binary.
//!
//! This is the Rust reimplementation of `renovatebot/renovate`'s CLI. The
//! initial slice exposes only the early flags (`-v`/`--version` and `--help`)
//! and accepts repositories as positional arguments. Flag parsing and
//! behavior will grow slice-by-slice toward Renovate parity; see
//! `docs/parity/implementation-ledger.md` for the running plan.

// Allow user-facing CLI output. The workspace lints forbid this elsewhere so
// that print! calls cannot leak into library code.
#![allow(
    clippy::print_stdout,
    reason = "CLI surface — user-facing output belongs in this crate"
)]

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
    // Surface unknown flags as errors during early parsing once the full
    // option set is wired up. Kept default for now.
)]
struct Cli {
    /// Print the version and exit.
    #[arg(short = 'v', long = "version", action = ArgAction::SetTrue, global = true)]
    version: bool,

    /// Repositories to process (positional). Slice 1 only records them; later
    /// slices will dispatch into the worker pipeline.
    #[arg(value_name = "repositories")]
    repositories: Vec<String>,
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
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

    // Slice 1 stops here. Later slices replace this with the global worker
    // entry point.
    ExitCode::SUCCESS
}
