//! Parity reconciler for renovate-rust. Owns **two** generated maps:
//!
//! - `docs/parity/source-map.md` — every upstream `lib/**/*.ts` implementation
//!   file → its Rust counterpart(s) + per-file port status. Truth lives in
//!   `@parity` tags in the Rust source (see [`source`]).
//! - `docs/parity/test-map.md` — every upstream `it()`/`test()` in a `.spec.ts`
//!   file → ported / pending / deleted. Truth lives in `// Ported:` comments in
//!   the Rust tests (see [`test_map`]).
//!
//! Both files are regenerated, never hand-edited. The CLI knows where each
//! lives and writes it directly — no output redirection needed:
//!
//! ```text
//! cargo run -p parity-cli            # regenerate BOTH maps
//! cargo run -p parity-cli -- source  # regenerate the source map only
//! cargo run -p parity-cli -- test    # regenerate the test map only
//! cargo run -p parity-cli -- check   # CI guard: stale tags / deleted tests
//! ```
//!
//! Per-file lookups are intentionally absent — `grep docs/parity/source-map.md`
//! answers them against the generated table.

// This is a CLI: stdout is the report, stderr is diagnostics. Both intended.
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::str_to_string)]

mod source;
mod test_map;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// Default on-disk locations the CLI writes to (relative to the working dir,
/// i.e. the `renovate-rust/` repo root).
const SOURCE_MAP_PATH: &str = "docs/parity/source-map.md";
const TEST_MAP_PATH: &str = "docs/parity/test-map.md";

#[derive(Parser)]
#[command(
    name = "parity-cli",
    about = "Regenerate the source-map and test-map parity files for renovate-rust"
)]
struct Cli {
    /// Upstream Renovate checkout root (the directory containing `lib/`).
    #[arg(long, default_value = "../renovate", global = true)]
    upstream: PathBuf,

    /// Rust source root to scan for `@parity` tags and `// Ported:` comments.
    #[arg(long, default_value = "crates", global = true)]
    rust: PathBuf,

    /// Print the generated markdown to stdout instead of writing the file.
    #[arg(long, global = true)]
    stdout: bool,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Regenerate both parity files (default).
    All,
    /// Regenerate the source-map (`docs/parity/source-map.md`).
    Source,
    /// Regenerate the test-map (`docs/parity/test-map.md`).
    Test,
    /// CI guard: exit non-zero on stale `@parity` tags or deleted-upstream tests.
    Check,
    /// One-time: rewrite all `// Ported:` refs to canonical `lib/...` form.
    Normalize,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let lib = cli.upstream.join("lib");
    if !lib.is_dir() {
        eprintln!(
            "error: upstream lib dir not found: {} (pass --upstream)",
            lib.display()
        );
        return ExitCode::FAILURE;
    }

    match cli.cmd.unwrap_or(Cmd::All) {
        Cmd::All => {
            let s = gen_source(&lib, &cli.rust);
            let t = gen_test(&cli.upstream, &cli.rust);
            match (s, t) {
                (Ok(s), Ok(t)) => {
                    emit(&s, SOURCE_MAP_PATH, cli.stdout);
                    emit(&t, TEST_MAP_PATH, cli.stdout);
                    ExitCode::SUCCESS
                }
                _ => ExitCode::FAILURE,
            }
        }
        Cmd::Source => match gen_source(&lib, &cli.rust) {
            Ok(md) => {
                emit(&md, SOURCE_MAP_PATH, cli.stdout);
                ExitCode::SUCCESS
            }
            Err(()) => ExitCode::FAILURE,
        },
        Cmd::Test => match gen_test(&cli.upstream, &cli.rust) {
            Ok(md) => {
                emit(&md, TEST_MAP_PATH, cli.stdout);
                ExitCode::SUCCESS
            }
            Err(()) => ExitCode::FAILURE,
        },
        Cmd::Check => check(&cli.upstream, &cli.rust),
        Cmd::Normalize => {
            let specs = test_map::scan_specs(&cli.upstream);
            match test_map::normalize(&cli.rust, &specs) {
                Ok((rewritten, unresolved)) => {
                    eprintln!("normalize: {rewritten} refs rewritten, {unresolved} unresolved");
                    if unresolved > 0 {
                        ExitCode::FAILURE
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(e) => {
                    eprintln!("normalize error: {e}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}

fn gen_source(lib: &Path, rust: &Path) -> Result<String, ()> {
    let upstream = source::scan_upstream(lib);
    let tags = source::scan_tags(rust).map_err(|e| eprintln!("error scanning rust tags: {e}"))?;
    Ok(source::render_report(&upstream, &tags))
}

fn gen_test(upstream: &Path, rust: &Path) -> Result<String, ()> {
    let specs = test_map::scan_specs(upstream);
    let ported =
        test_map::scan_ported(rust).map_err(|e| eprintln!("error scanning ported: {e}"))?;
    Ok(test_map::render_report(&specs, &ported))
}

/// Write `md` to `path`, or print it if `--stdout`.
fn emit(md: &str, path: &str, to_stdout: bool) {
    if to_stdout {
        print!("{md}");
        return;
    }
    match std::fs::write(path, md) {
        Ok(()) => eprintln!("wrote {path}"),
        Err(e) => eprintln!("error writing {path}: {e}"),
    }
}

fn check(upstream_root: &Path, rust: &Path) -> ExitCode {
    let mut bad = false;

    // Source: stale @parity tags + bad statuses.
    let upstream = source::scan_upstream(&upstream_root.join("lib"));
    match source::scan_tags(rust) {
        Ok(tags) => {
            if source::report_stale(&upstream, &tags) {
                bad = true;
            }
        }
        Err(e) => {
            eprintln!("error scanning rust tags: {e}");
            bad = true;
        }
    }

    // Test: // Ported: comments whose upstream identity is gone (deleted).
    let specs = test_map::scan_specs(upstream_root);
    match test_map::scan_ported(rust) {
        Ok(ported) => {
            if test_map::report_orphans(&specs, &ported) {
                bad = true;
            }
        }
        Err(e) => {
            eprintln!("error scanning ported: {e}");
            bad = true;
        }
    }

    if bad {
        ExitCode::FAILURE
    } else {
        eprintln!("ok: source tags resolve, test refs resolve");
        ExitCode::SUCCESS
    }
}
