//! Parity coverage CLI — thin wrapper around `scripts/parity_coverage.py`.
//!
//! Mirrors the interface of the Python script so that
//! `cargo run -p parity-cli -- <cmd>` works identically to
//! `python3 scripts/parity_coverage.py <cmd>`.

use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

const PYTHON_SCRIPT: &str = "scripts/parity_coverage.py";

#[derive(Parser)]
#[command(name = "parity-cli")]
#[command(about = "Renovate parity coverage report")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Ledger summary (default)
    Report,
    /// Regenerate docs/parity/modules.md
    Ledger,
    /// List missing upstream tests for a module
    Gaps { module: String },
    /// List unresolved ported comments
    Orphans,
    /// Check every ported comment against upstream
    Verify,
    /// Emit raw analysis as JSON
    Json,
}

fn main() {
    let cli = Cli::parse();

    let mut cmd = Command::new("python3");
    cmd.arg(PYTHON_SCRIPT);

    match &cli.cmd {
        Cmd::Report => { /* default, no extra args */ }
        Cmd::Ledger => {
            // Output to stdout so `> docs/parity/modules.md` works.
            cmd.arg("--stdout");
            cmd.arg("ledger");
        }
        Cmd::Gaps { module } => {
            cmd.arg("gaps");
            cmd.arg(module);
        }
        Cmd::Orphans => {
            cmd.arg("orphans");
        }
        Cmd::Verify => {
            cmd.arg("verify");
        }
        Cmd::Json => {
            cmd.arg("json");
        }
    }

    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("Failed to spawn python3 {PYTHON_SCRIPT}: {e}"));

    std::process::exit(status.code().unwrap_or(1));
}
