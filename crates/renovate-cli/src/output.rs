//! Human-readable update report output.
//!
//! Collects per-dependency update status for one repository run and prints
//! a structured summary to stdout.  Debug-level detail continues to flow
//! through `tracing` on stderr; this module owns only the actionable,
//! user-facing layer.
//!
//! Color is enabled by default when stdout is an interactive TTY and the
//! `NO_COLOR` environment variable is unset.  It can also be forced off via
//! the `use_color` flag passed to [`print_report`].

use std::io::IsTerminal as _;

// ── Data model ────────────────────────────────────────────────────────────────

/// Status of a single dependency after a registry lookup.
#[derive(Debug, Clone)]
pub(crate) enum DepStatus {
    /// A newer version is available and the constraint should be bumped.
    UpdateAvailable { current: String, latest: String },
    /// The installed constraint already resolves to the latest available version.
    UpToDate { latest: Option<String> },
    /// The dep was skipped before a registry lookup (workspace protocol, local
    /// path, git URL, etc.).
    Skipped { reason: String },
    /// The registry lookup failed.
    LookupError { message: String },
}

/// A single dependency entry in a report.
#[derive(Debug, Clone)]
pub(crate) struct DepReport {
    pub name: String,
    pub status: DepStatus,
}

/// All deps extracted from one manifest file.
#[derive(Debug, Clone)]
pub(crate) struct FileReport {
    /// Relative path within the repository (e.g. `"package.json"`).
    pub path: String,
    /// Manager name (e.g. `"npm"`, `"cargo"`).
    pub manager: String,
    pub deps: Vec<DepReport>,
}

/// Full per-repository update report.
#[derive(Debug, Clone)]
pub(crate) struct RepoReport {
    pub repo_slug: String,
    pub files: Vec<FileReport>,
}

// ── Rendering ─────────────────────────────────────────────────────────────────

/// Return `true` when colored output should be used.
///
/// Color is enabled when:
/// - `NO_COLOR` is not set (or is empty), **and**
/// - stdout is an interactive terminal.
pub(crate) fn should_use_color() -> bool {
    let no_color = std::env::var("NO_COLOR")
        .ok()
        .is_some_and(|v| !v.is_empty());
    !no_color && std::io::stdout().is_terminal()
}

/// Print a repository update report to stdout.
///
/// Pass `use_color = should_use_color()` for the default terminal behavior,
/// or `false` to force plain text (e.g. in tests or when piped).
pub(crate) fn print_report(report: &RepoReport, use_color: bool) {
    let bar = "─".repeat(60);
    println!("{}", dim(&bar, use_color));
    println!(" Repository: {}", bold(&report.repo_slug, use_color));
    println!("{}", dim(&bar, use_color));

    if report.files.is_empty() {
        println!("  {}", dim("no managed files found", use_color));
        println!();
        return;
    }

    for file in &report.files {
        let updates: usize = file
            .deps
            .iter()
            .filter(|d| matches!(d.status, DepStatus::UpdateAvailable { .. }))
            .count();
        let skipped: usize = file
            .deps
            .iter()
            .filter(|d| matches!(d.status, DepStatus::Skipped { .. }))
            .count();
        let errors: usize = file
            .deps
            .iter()
            .filter(|d| matches!(d.status, DepStatus::LookupError { .. }))
            .count();

        let file_label = format!(
            "  {} [{}]  {} deps",
            bold(&file.path, use_color),
            dim(&file.manager, use_color),
            file.deps.len(),
        );
        let counts = format_file_counts(updates, skipped, errors, use_color);
        println!("{file_label}  {counts}");

        for dep in &file.deps {
            println!("    {}", format_dep(dep, use_color));
        }
        println!();
    }
}

fn format_file_counts(updates: usize, skipped: usize, errors: usize, use_color: bool) -> String {
    let mut parts = Vec::new();
    if updates > 0 {
        parts.push(yellow(&format!("{updates} update(s) available"), use_color));
    }
    if errors > 0 {
        parts.push(red(&format!("{errors} error(s)"), use_color));
    }
    if skipped > 0 {
        parts.push(dim(&format!("{skipped} skipped"), use_color));
    }
    if parts.is_empty() {
        parts.push(green("all up to date", use_color));
    }
    parts.join("  ")
}

fn format_dep(dep: &DepReport, use_color: bool) -> String {
    match &dep.status {
        DepStatus::UpdateAvailable { current, latest } => {
            format!(
                "{} {}  {}  {} → {}",
                yellow("↑", use_color),
                bold(&dep.name, use_color),
                dim(current, use_color),
                dim("→", use_color),
                green(latest, use_color),
            )
        }
        DepStatus::UpToDate { latest } => {
            let latest_str = latest
                .as_deref()
                .map(|v| format!("  {}", dim(&format!("(latest: {v})"), use_color)))
                .unwrap_or_default();
            format!("{} {}{}", green("✓", use_color), dep.name, latest_str,)
        }
        DepStatus::Skipped { reason } => {
            format!(
                "{} {}  {}",
                dim("–", use_color),
                dim(&dep.name, use_color),
                dim(&format!("[{reason}]"), use_color),
            )
        }
        DepStatus::LookupError { message } => {
            format!(
                "{} {}  {}",
                red("✗", use_color),
                dep.name,
                dim(message, use_color),
            )
        }
    }
}

// ── ANSI helpers ──────────────────────────────────────────────────────────────

fn bold(s: &str, use_color: bool) -> String {
    if use_color {
        format!("\x1b[1m{s}\x1b[0m")
    } else {
        s.to_owned()
    }
}

fn dim(s: &str, use_color: bool) -> String {
    if use_color {
        format!("\x1b[2m{s}\x1b[0m")
    } else {
        s.to_owned()
    }
}

fn green(s: &str, use_color: bool) -> String {
    if use_color {
        format!("\x1b[32m{s}\x1b[0m")
    } else {
        s.to_owned()
    }
}

fn yellow(s: &str, use_color: bool) -> String {
    if use_color {
        format!("\x1b[33m{s}\x1b[0m")
    } else {
        s.to_owned()
    }
}

fn red(s: &str, use_color: bool) -> String {
    if use_color {
        format!("\x1b[31m{s}\x1b[0m")
    } else {
        s.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_report() -> RepoReport {
        RepoReport {
            repo_slug: "owner/myrepo".into(),
            files: vec![
                FileReport {
                    path: "package.json".into(),
                    manager: "npm".into(),
                    deps: vec![
                        DepReport {
                            name: "lodash".into(),
                            status: DepStatus::UpdateAvailable {
                                current: "4.17.21".into(),
                                latest: "4.18.0".into(),
                            },
                        },
                        DepReport {
                            name: "express".into(),
                            status: DepStatus::UpToDate {
                                latest: Some("4.18.2".into()),
                            },
                        },
                        DepReport {
                            name: "local-lib".into(),
                            status: DepStatus::Skipped {
                                reason: "local-path".into(),
                            },
                        },
                    ],
                },
                FileReport {
                    path: "Cargo.toml".into(),
                    manager: "cargo".into(),
                    deps: vec![DepReport {
                        name: "serde".into(),
                        status: DepStatus::UpToDate {
                            latest: Some("1.0.228".into()),
                        },
                    }],
                },
            ],
        }
    }

    #[test]
    fn print_report_plain_text_runs_without_panic() {
        // Basic smoke test: report renders without panicking in no-color mode.
        let report = make_report();
        // Capture via redirect isn't trivial; just verify no panic.
        print_report(&report, false);
    }

    #[test]
    fn print_report_empty_files() {
        let report = RepoReport {
            repo_slug: "owner/empty".into(),
            files: vec![],
        };
        print_report(&report, false);
    }

    #[test]
    fn format_dep_update_available_plain() {
        let dep = DepReport {
            name: "lodash".into(),
            status: DepStatus::UpdateAvailable {
                current: "4.17.21".into(),
                latest: "4.18.0".into(),
            },
        };
        let s = format_dep(&dep, false);
        assert!(s.contains("lodash"));
        assert!(s.contains("4.17.21"));
        assert!(s.contains("4.18.0"));
        assert!(s.contains('↑'));
    }

    #[test]
    fn format_dep_up_to_date_with_latest() {
        let dep = DepReport {
            name: "express".into(),
            status: DepStatus::UpToDate {
                latest: Some("4.18.2".into()),
            },
        };
        let s = format_dep(&dep, false);
        assert!(s.contains('✓'));
        assert!(s.contains("express"));
        assert!(s.contains("4.18.2"));
    }

    #[test]
    fn format_dep_skipped() {
        let dep = DepReport {
            name: "my-lib".into(),
            status: DepStatus::Skipped {
                reason: "workspace-protocol".into(),
            },
        };
        let s = format_dep(&dep, false);
        assert!(s.contains('–'));
        assert!(s.contains("workspace-protocol"));
    }

    #[test]
    fn format_dep_error() {
        let dep = DepReport {
            name: "bad-pkg".into(),
            status: DepStatus::LookupError {
                message: "404 Not Found".into(),
            },
        };
        let s = format_dep(&dep, false);
        assert!(s.contains('✗'));
        assert!(s.contains("404 Not Found"));
    }

    #[test]
    fn format_file_counts_all_up_to_date() {
        let s = format_file_counts(0, 0, 0, false);
        assert_eq!(s, "all up to date");
    }

    #[test]
    fn format_file_counts_with_updates() {
        let s = format_file_counts(2, 1, 0, false);
        assert!(s.contains("2 update(s) available"));
        assert!(s.contains("1 skipped"));
    }

    #[test]
    fn ansi_codes_present_when_color_enabled() {
        let s = bold("hello", true);
        assert!(s.contains("\x1b[1m"));
        assert!(s.contains("\x1b[0m"));
    }

    #[test]
    fn no_ansi_codes_when_color_disabled() {
        assert_eq!(bold("hello", false), "hello");
        assert_eq!(green("ok", false), "ok");
    }
}
