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

use serde::{Deserialize, Serialize};

// ── Data model ────────────────────────────────────────────────────────────────

/// Status of a single dependency after a registry lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub(crate) enum DepStatus {
    /// A newer version is available and the constraint should be bumped.
    #[serde(rename = "updateAvailable")]
    UpdateAvailable { current: String, latest: String },
    /// The installed constraint already resolves to the latest available version.
    #[serde(rename = "upToDate")]
    UpToDate { latest: Option<String> },
    /// The dep was skipped before a registry lookup (workspace protocol, local
    /// path, git URL, etc.).
    #[serde(rename = "skipped")]
    Skipped { reason: String },
    /// The registry lookup failed.
    #[serde(rename = "lookupError")]
    LookupError { message: String },
}

/// A single dependency entry in a report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DepReport {
    pub name: String,
    #[serde(flatten)]
    pub status: DepStatus,
}

/// All deps extracted from one manifest file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileReport {
    /// Relative path within the repository (e.g. `"package.json"`).
    pub path: String,
    /// Manager name (e.g. `"npm"`, `"cargo"`).
    pub manager: String,
    pub deps: Vec<DepReport>,
}

/// Full per-repository update report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RepoReport {
    #[serde(rename = "repoSlug")]
    pub repo_slug: String,
    pub files: Vec<FileReport>,
}

/// Per-file or per-repo dependency count breakdown included in JSON output.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DepStats {
    pub total: usize,
    #[serde(rename = "updateAvailable")]
    pub update_available: usize,
    #[serde(rename = "upToDate")]
    pub up_to_date: usize,
    pub skipped: usize,
    pub errors: usize,
}

impl DepStats {
    pub(crate) fn from_deps(deps: &[DepReport]) -> Self {
        let mut update_available = 0usize;
        let mut up_to_date = 0usize;
        let mut skipped = 0usize;
        let mut errors = 0usize;
        for dep in deps {
            match &dep.status {
                DepStatus::UpdateAvailable { .. } => update_available += 1,
                DepStatus::UpToDate { .. } => up_to_date += 1,
                DepStatus::Skipped { .. } => skipped += 1,
                DepStatus::LookupError { .. } => errors += 1,
            }
        }
        DepStats {
            total: deps.len(),
            update_available,
            up_to_date,
            skipped,
            errors,
        }
    }
}

/// A `FileReport` annotated with computed stats for JSON output.
#[derive(Debug, Clone, Serialize)]
struct JsonFileReport<'a> {
    path: &'a str,
    manager: &'a str,
    stats: DepStats,
    deps: &'a [DepReport],
}

/// A `RepoReport` annotated with computed stats for JSON output.
#[derive(Debug, Clone, Serialize)]
struct JsonRepoReport<'a> {
    #[serde(rename = "repoSlug")]
    repo_slug: &'a str,
    stats: DepStats,
    files: Vec<JsonFileReport<'a>>,
}

/// Print a JSON array of repository reports to stdout (with computed stats).
pub(crate) fn print_json_reports(reports: &[RepoReport]) {
    let json_reports: Vec<JsonRepoReport<'_>> = reports
        .iter()
        .map(|r| {
            let files: Vec<JsonFileReport<'_>> = r
                .files
                .iter()
                .map(|f| JsonFileReport {
                    path: &f.path,
                    manager: &f.manager,
                    stats: DepStats::from_deps(&f.deps),
                    deps: &f.deps,
                })
                .collect();
            let repo_stats = files.iter().fold(DepStats::default(), |mut acc, f| {
                acc.total += f.stats.total;
                acc.update_available += f.stats.update_available;
                acc.up_to_date += f.stats.up_to_date;
                acc.skipped += f.stats.skipped;
                acc.errors += f.stats.errors;
                acc
            });
            JsonRepoReport {
                repo_slug: &r.repo_slug,
                stats: repo_stats,
                files,
            }
        })
        .collect();

    match serde_json::to_string_pretty(&json_reports) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("{{\"error\": \"failed to serialize report: {e}\"}}"),
    }
}

/// Aggregate statistics for a complete run across all repositories.
#[derive(Debug, Clone, Default)]
pub(crate) struct RunStats {
    /// Total repositories processed.
    pub repos_processed: usize,
    /// Repos with at least one update available.
    pub repos_with_updates: usize,
    /// Repos where everything is up to date.
    pub repos_up_to_date: usize,
    /// Repos where a platform or extraction error occurred.
    pub repos_with_errors: usize,
    /// Total dep records found across all files.
    pub total_deps: usize,
    /// Dep records with an update available.
    pub total_updates: usize,
    /// Dep records skipped (workspace, local path, git URL, etc.).
    pub total_skipped: usize,
    /// Dep records where the registry lookup failed.
    pub total_errors: usize,
}

impl RunStats {
    /// Merge statistics from one repository's report into the running totals.
    pub(crate) fn add_report(&mut self, report: &RepoReport) {
        self.repos_processed += 1;
        let mut repo_updates = 0usize;
        let mut repo_errors = 0usize;
        for file in &report.files {
            self.total_deps += file.deps.len();
            for dep in &file.deps {
                match &dep.status {
                    DepStatus::UpdateAvailable { .. } => {
                        self.total_updates += 1;
                        repo_updates += 1;
                    }
                    DepStatus::Skipped { .. } => self.total_skipped += 1,
                    DepStatus::LookupError { .. } => {
                        self.total_errors += 1;
                        repo_errors += 1;
                    }
                    DepStatus::UpToDate { .. } => {}
                }
            }
        }
        if repo_errors > 0 {
            self.repos_with_errors += 1;
        } else if repo_updates > 0 {
            self.repos_with_updates += 1;
        } else {
            self.repos_up_to_date += 1;
        }
    }
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
/// - `use_color`: pass `should_use_color()` for TTY-aware coloring.
/// - `quiet`: suppress per-dependency listing; show file-level summaries only.
pub(crate) fn print_report(report: &RepoReport, use_color: bool, quiet: bool) {
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

        if !quiet {
            for dep in &file.deps {
                println!("    {}", format_dep(dep, use_color));
            }
        }
        println!();
    }
}

/// Print the aggregate run summary after all repositories have been processed.
pub(crate) fn print_run_summary(stats: &RunStats, use_color: bool) {
    let bar = "═".repeat(60);
    println!("{}", dim(&bar, use_color));

    let repos_line = format!(
        "  {} repositories processed",
        bold(&stats.repos_processed.to_string(), use_color)
    );
    println!("{repos_line}");

    if stats.repos_processed > 0 {
        if stats.repos_with_updates > 0 {
            println!(
                "    {} {} with updates available",
                yellow("↑", use_color),
                stats.repos_with_updates,
            );
        }
        if stats.repos_up_to_date > 0 {
            println!(
                "    {} {} up to date",
                green("✓", use_color),
                stats.repos_up_to_date,
            );
        }
        if stats.repos_with_errors > 0 {
            println!(
                "    {} {} with errors",
                red("✗", use_color),
                stats.repos_with_errors,
            );
        }
    }

    if stats.total_deps > 0 {
        let dep_summary = format!(
            "  {} dependencies  ·  {} updates  ·  {} skipped  ·  {} errors",
            bold(&stats.total_deps.to_string(), use_color),
            if stats.total_updates > 0 {
                yellow(&stats.total_updates.to_string(), use_color)
            } else {
                stats.total_updates.to_string()
            },
            dim(&stats.total_skipped.to_string(), use_color),
            if stats.total_errors > 0 {
                red(&stats.total_errors.to_string(), use_color)
            } else {
                stats.total_errors.to_string()
            },
        );
        println!("{dep_summary}");
    }

    println!("{}", dim(&bar, use_color));
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
            use renovate_core::versioning::semver_generic::{UpdateType, classify_semver_update};
            let type_label = match classify_semver_update(current, latest) {
                Some(UpdateType::Major) => {
                    format!(
                        "  {}",
                        if use_color {
                            "\x1b[31mmajor\x1b[0m"
                        } else {
                            "major"
                        }
                    )
                }
                Some(UpdateType::Minor) => {
                    format!(
                        "  {}",
                        if use_color {
                            "\x1b[33mminor\x1b[0m"
                        } else {
                            "minor"
                        }
                    )
                }
                Some(UpdateType::Patch) => {
                    format!(
                        "  {}",
                        if use_color {
                            "\x1b[32mpatch\x1b[0m"
                        } else {
                            "patch"
                        }
                    )
                }
                None => String::new(),
            };
            format!(
                "{} {}  {}  {} → {}{}",
                yellow("↑", use_color),
                bold(&dep.name, use_color),
                dim(current, use_color),
                dim("→", use_color),
                green(latest, use_color),
                type_label,
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
        let report = make_report();
        print_report(&report, false, false);
    }

    #[test]
    fn print_report_quiet_runs_without_panic() {
        let report = make_report();
        print_report(&report, false, true);
    }

    #[test]
    fn print_report_empty_files() {
        let report = RepoReport {
            repo_slug: "owner/empty".into(),
            files: vec![],
        };
        print_report(&report, false, false);
    }

    #[test]
    fn print_run_summary_empty_run() {
        let stats = RunStats::default();
        print_run_summary(&stats, false);
    }

    #[test]
    fn run_stats_add_report_accumulates() {
        let mut stats = RunStats::default();
        let report = make_report();
        stats.add_report(&report);
        assert_eq!(stats.repos_processed, 1);
        assert_eq!(stats.total_deps, 4); // lodash + express + local-lib + serde
        assert_eq!(stats.total_updates, 1); // lodash
        assert_eq!(stats.total_skipped, 1); // local-lib
        assert_eq!(stats.repos_with_updates, 1);
    }

    #[test]
    fn run_stats_two_repos() {
        let mut stats = RunStats::default();
        stats.add_report(&make_report());
        stats.add_report(&RepoReport {
            repo_slug: "owner/clean".into(),
            files: vec![FileReport {
                path: "Cargo.toml".into(),
                manager: "cargo".into(),
                deps: vec![DepReport {
                    name: "tokio".into(),
                    status: DepStatus::UpToDate {
                        latest: Some("1.0.0".into()),
                    },
                }],
            }],
        });
        assert_eq!(stats.repos_processed, 2);
        assert_eq!(stats.repos_with_updates, 1);
        assert_eq!(stats.repos_up_to_date, 1);
    }

    #[test]
    fn print_run_summary_with_updates() {
        let mut stats = RunStats::default();
        stats.add_report(&make_report());
        print_run_summary(&stats, false);
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

    // ── JSON output tests ─────────────────────────────────────────────────────

    #[test]
    fn dep_stats_counts_correctly() {
        let deps = vec![
            DepReport {
                name: "a".into(),
                status: DepStatus::UpdateAvailable {
                    current: "1.0.0".into(),
                    latest: "2.0.0".into(),
                },
            },
            DepReport {
                name: "b".into(),
                status: DepStatus::UpToDate { latest: None },
            },
            DepReport {
                name: "c".into(),
                status: DepStatus::Skipped {
                    reason: "local".into(),
                },
            },
            DepReport {
                name: "d".into(),
                status: DepStatus::LookupError {
                    message: "404".into(),
                },
            },
        ];
        let s = DepStats::from_deps(&deps);
        assert_eq!(s.total, 4);
        assert_eq!(s.update_available, 1);
        assert_eq!(s.up_to_date, 1);
        assert_eq!(s.skipped, 1);
        assert_eq!(s.errors, 1);
    }

    #[test]
    fn print_json_reports_produces_valid_json() {
        let report = make_report();
        // Just ensure it doesn't panic and produces non-empty output.
        // Capture via a channel is complex; test the underlying serialization.
        let json_reports: Vec<JsonRepoReport<'_>> = std::slice::from_ref(&report)
            .iter()
            .map(|r| {
                let files: Vec<JsonFileReport<'_>> = r
                    .files
                    .iter()
                    .map(|f| JsonFileReport {
                        path: &f.path,
                        manager: &f.manager,
                        stats: DepStats::from_deps(&f.deps),
                        deps: &f.deps,
                    })
                    .collect();
                let repo_stats = files.iter().fold(DepStats::default(), |mut acc, f| {
                    acc.total += f.stats.total;
                    acc.update_available += f.stats.update_available;
                    acc.up_to_date += f.stats.up_to_date;
                    acc.skipped += f.stats.skipped;
                    acc.errors += f.stats.errors;
                    acc
                });
                JsonRepoReport {
                    repo_slug: &r.repo_slug,
                    stats: repo_stats,
                    files,
                }
            })
            .collect();

        let json = serde_json::to_string_pretty(&json_reports).unwrap();
        assert!(json.contains("repoSlug"));
        assert!(json.contains("updateAvailable"));
        assert!(json.contains("stats"));
        assert!(json.contains("owner/myrepo"));
    }
}
