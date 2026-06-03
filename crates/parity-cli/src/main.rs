//! Parity reconciler for renovate-rust. Owns **two** generated, split trees:
//!
//! - `docs/parity/source-mapping/` — every upstream `lib/**/*.ts` implementation
//!   file → its Rust counterpart(s) + per-file port status. Root `README.md` →
//!   one page per group. Truth lives in `@parity` tags (see [`source`]).
//! - `docs/parity/test-mapping/` — every upstream `it()`/`test()` → ported /
//!   pending / deleted. Root `README.md` → module page → per-spec page. Truth
//!   lives in `// Ported:` comments (see [`test_map`]).
//!
//! Both trees are regenerated, never hand-edited. Each run wipes and rebuilds
//! its tree, so removed upstream files/specs leave no orphan pages:
//!
//! ```text
//! cargo run -p parity-cli            # regenerate BOTH trees
//! cargo run -p parity-cli -- source  # regenerate the source-mapping tree only
//! cargo run -p parity-cli -- test    # regenerate the test-mapping tree only
//! cargo run -p parity-cli -- check   # CI guard: stale tags / deleted tests
//! ```

// This is a CLI: stdout is the report, stderr is diagnostics. Both intended.
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::str_to_string)]

mod paths;
mod source;
mod test_map;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

// Both maps are split trees (see `source::MAPPING_DIR` and
// `test_map::MAPPING_DIR`), each wiped and regenerated whole.

#[derive(Parser)]
#[command(
    name = "parity-cli",
    about = "Regenerate the source-mapping and test-mapping parity trees for renovate-rust"
)]
struct Cli {
    /// Upstream Renovate checkout root (the directory containing `lib/`).
    #[arg(long, default_value = "../renovate", global = true)]
    upstream: PathBuf,

    /// Rust source root to scan for `@parity` tags and `// Ported:` comments.
    #[arg(long, default_value = "crates", global = true)]
    rust: PathBuf,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Regenerate both parity trees (default).
    All,
    /// Regenerate the source-mapping tree (`docs/parity/source-mapping/`).
    Source,
    /// Regenerate the test-mapping tree (`docs/parity/test-mapping/`).
    Test,
    /// CI guard: exit non-zero on stale `@parity` tags or deleted-upstream tests.
    Check,
    /// List upstream `it()`s with no `// Ported:` for a module (e.g. `manager/cargo`).
    Gaps { module: String },
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
            let source_ok = write_source_pages(&lib, &cli.rust);
            let test_ok = write_test_pages(&cli.upstream, &cli.rust);
            if source_ok && test_ok {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Cmd::Source => {
            if write_source_pages(&lib, &cli.rust) {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Cmd::Test => {
            if write_test_pages(&cli.upstream, &cli.rust) {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Cmd::Check => check(&cli.upstream, &cli.rust),
        Cmd::Gaps { module } => {
            let specs = test_map::scan_specs(&cli.upstream);
            match test_map::scan_ported(&cli.rust) {
                Ok(ported) => {
                    test_map::gaps(&specs, &ported, &module);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("error scanning ported: {e}");
                    ExitCode::FAILURE
                }
            }
        }
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

/// Regenerate the split source-mapping tree. Returns false on error.
fn write_source_pages(lib: &Path, rust: &Path) -> bool {
    let upstream = source::scan_upstream(lib);
    let tags = match source::scan_tags(rust) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error scanning rust tags: {e}");
            return false;
        }
    };
    match source::write_pages(Path::new(source::MAPPING_DIR), &upstream, &tags) {
        Ok(n) => {
            eprintln!("wrote {n} pages under {}", source::MAPPING_DIR);
            true
        }
        Err(e) => {
            eprintln!("error writing source-mapping pages: {e}");
            false
        }
    }
}

/// Regenerate the split test-mapping tree. Returns false on error.
fn write_test_pages(upstream: &Path, rust: &Path) -> bool {
    let specs = test_map::scan_specs(upstream);
    let ported = match test_map::scan_ported(rust) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error scanning ported: {e}");
            return false;
        }
    };
    match test_map::write_pages(Path::new(test_map::MAPPING_DIR), &specs, &ported) {
        Ok(n) => {
            eprintln!("wrote {n} pages under {}", test_map::MAPPING_DIR);
            true
        }
        Err(e) => {
            eprintln!("error writing test-mapping pages: {e}");
            false
        }
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
