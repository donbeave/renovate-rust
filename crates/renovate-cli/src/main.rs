//! Entry point for the `renovate` binary.
//!
//! This is the Rust reimplementation of `renovatebot/renovate`'s CLI. Flag
//! parsing and behavior grow slice-by-slice toward Renovate parity; see
//! `docs/parity/implementation-ledger.md` for the running plan.

// Allow user-facing CLI output and error messages. The workspace lints forbid
// these elsewhere so print! calls cannot leak into library code.
#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "CLI surface — user-facing output and error messages belong in this crate"
)]

mod cli;
mod config_builder;
mod context;
mod logging;
mod migrate;
mod output;
mod pipeline_utils;
mod pipelines;
mod report_builders;

use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser as _;
use cli::Cli;
use renovate_core::config::Platform;
use renovate_core::config::{GlobalConfig, file as config_file};
use renovate_core::http::HttpClient;
use renovate_core::managers;
use renovate_core::platform::{AnyPlatformClient, PlatformError};
use renovate_core::repo_config;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

/// Maximum number of repositories processed concurrently.
///
/// Mirrors Renovate's `queue.concurrency` default. Each repo job
/// itself fans out concurrent datasource requests, so this is a
/// second level of bounded parallelism.
const REPO_CONCURRENCY: usize = 4;

#[tokio::main]
async fn main() -> ExitCode {
    // 1. Initialize logging.
    match logging::init() {
        logging::InitResult::Ok => {}
        logging::InitResult::InvalidLevel(lvl) => {
            eprintln!(r#"{{"level":"fatal","msg":"Invalid log level","logLevel":{lvl:?}}}"#);
            return ExitCode::from(1);
        }
    }

    // 2. Legacy-flag migration.
    let raw: Vec<String> = std::env::args().collect();
    let migrated = migrate::migrate_args(&raw);

    // 3. Parse flags.
    let cli = match Cli::try_parse_from(&migrated) {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };

    if cli.version {
        println!("{}", renovate_core::VERSION);
        return ExitCode::SUCCESS;
    }

    // 4. Global config pipeline: defaults → file → CLI.
    let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let config_file_env = std::env::var("RENOVATE_CONFIG_FILE").ok();

    let base = match config_file::resolve_config_path(config_file_env.as_deref(), &cwd) {
        Ok(Some(path)) => {
            tracing::debug!(path = %path.display(), "loading global config file");
            match config_file::load(&path) {
                Ok(file_cfg) => {
                    tracing::debug!("global config file loaded");
                    config_file::merge_over_base(GlobalConfig::default(), file_cfg)
                }
                Err(err) => {
                    tracing::error!(%err, "failed to parse config file");
                    eprintln!("renovate: error parsing config file: {err}");
                    return ExitCode::from(1);
                }
            }
        }
        Ok(None) => {
            tracing::debug!("no global config file found, using defaults");
            GlobalConfig::default()
        }
        Err(err) => {
            tracing::error!(%err);
            eprintln!("renovate: {err}");
            return ExitCode::from(1);
        }
    };

    let config = config_builder::build(&cli, base);
    tracing::info!(
        platform = %config.platform,
        dry_run = ?config.dry_run,
        "config resolved"
    );

    // 5. For local platform, inject a synthetic repository slug when none is
    //    supplied so that the current working directory is scanned — mirroring
    //    how the original Renovate handles `--platform=local`.
    //
    //    The slug format "local/{dirname}" keeps owner/repo splitting intact
    //    while making log output and report headers show the directory name.
    let mut config = config;
    if config.platform == Platform::Local && config.repositories.is_empty() {
        let dir_name = cwd
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".")
            .to_owned();
        tracing::info!(dir = %cwd.display(), "local platform: scanning current directory");
        config.repositories = vec![format!("local/{dir_name}")];
    }

    if config.repositories.is_empty() {
        tracing::info!("no repositories configured — nothing to do");
        return ExitCode::SUCCESS;
    }

    // 6. Platform initialization: create client and validate credentials.
    //    Mirrors Renovate's globalInitialize → initPlatform.
    //    For local platform no token is needed — the client reads from disk.
    let maybe_client: Option<AnyPlatformClient> = if config.platform == Platform::Local {
        tracing::info!("local platform — skipping token validation");
        Some(AnyPlatformClient::local(&cwd))
    } else if config.token.is_none() {
        tracing::warn!(
            platform = %config.platform,
            "no token configured — platform operations will fail"
        );
        None
    } else {
        match AnyPlatformClient::create(&config) {
            Err(PlatformError::NotSupported(name)) => {
                tracing::warn!(
                    platform = %name,
                    "platform not yet implemented; skipping token validation"
                );
                None
            }
            Err(err) => {
                tracing::error!(%err, "failed to create platform client");
                eprintln!("renovate: platform initialization failed: {err}");
                return ExitCode::from(1);
            }
            Ok(client) => {
                match client.get_current_user().await {
                    Ok(user) => {
                        tracing::info!(
                            login = %user.login,
                            platform = %config.platform,
                            "authenticated"
                        );
                    }
                    Err(PlatformError::Unauthorized) => {
                        tracing::error!(platform = %config.platform, "token authentication failed");
                        eprintln!("renovate: authentication failed — check your token");
                        return ExitCode::from(1);
                    }
                    Err(err) => {
                        tracing::error!(%err, "platform authentication error");
                        eprintln!("renovate: platform error: {err}");
                        return ExitCode::from(1);
                    }
                }
                Some(client)
            }
        }
    };

    // 7. Process repositories in parallel (bounded by REPO_CONCURRENCY).
    let Some(client) = maybe_client else {
        tracing::warn!("no platform client available; skipping repository processing");
        return ExitCode::SUCCESS;
    };

    // One shared HTTP connection pool for all concurrent repo + datasource
    // requests throughout the entire run.
    let http = HttpClient::new().expect("failed to create HTTP client");
    let use_color = output::should_use_color();
    let quiet = cli.quiet;
    let output_format = cli.output_format;

    let sem = Arc::new(Semaphore::new(REPO_CONCURRENCY));
    let mut set: JoinSet<(String, Option<output::RepoReport>, bool)> = JoinSet::new();
    let mut run_stats = output::RunStats::default();

    for repo_slug in &config.repositories {
        let client = client.clone();
        let http = http.clone();
        let repo_slug = repo_slug.clone();
        let config = config.clone();
        let sem = Arc::clone(&sem);

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let (report, had_error) = process_repo(&client, &http, &repo_slug, &config).await;
            (repo_slug, report, had_error)
        });
    }

    let mut had_error = false;
    let mut all_reports: Vec<output::RepoReport> = Vec::new();
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok((_slug, Some(report), repo_had_error)) => {
                run_stats.add_report(&report);
                if output_format == cli::OutputFormat::Human {
                    output::print_report(&report, use_color, quiet);
                } else {
                    all_reports.push(report);
                }
                had_error |= repo_had_error;
            }
            Ok((_slug, None, repo_had_error)) => {
                had_error |= repo_had_error;
            }
            Err(join_err) => {
                tracing::error!(%join_err, "repository task panicked");
                had_error = true;
            }
        }
    }

    if output_format == cli::OutputFormat::Json {
        output::print_json_reports(&all_reports);
    } else {
        output::print_run_summary(&run_stats, use_color);
    }

    if had_error {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

/// Process a single repository and return its update report.
///
/// Returns `(Option<RepoReport>, had_error)`:
/// - `None` for the report means the repo was skipped (malformed slug, fatal
///   platform error) and no output should be shown.
/// - `had_error = true` signals the overall process should exit non-zero.
async fn process_repo(
    client: &AnyPlatformClient,
    http: &HttpClient,
    repo_slug: &str,
    config: &GlobalConfig,
) -> (Option<output::RepoReport>, bool) {
    let Some((owner, repo)) = repo_slug.split_once('/') else {
        tracing::warn!(repo = %repo_slug, "skipping malformed repository slug (expected owner/repo)");
        return (None, false);
    };

    tracing::info!(repo = %repo_slug, "processing repository");

    // Parse the per-repo config and apply top-level gates.
    let repo_cfg = match repo_config::discover(client, owner, repo, config).await {
        Ok(repo_config::RepoConfigResult::Found { path, config: rc }) => {
            tracing::info!(repo = %repo_slug, config_path = %path, "found renovate config");
            let rc = *rc;
            if !rc.enabled {
                tracing::info!(repo = %repo_slug, "renovate disabled in repo config — skipping");
                return (None, false);
            }
            // Check schedule: if configured, only process during the scheduled window.
            if !rc.schedule.is_empty() && !renovate_core::schedule::is_within_schedule(&rc.schedule)
            {
                tracing::info!(
                    repo = %repo_slug,
                    schedule = ?rc.schedule,
                    "outside schedule window — skipping"
                );
                return (None, false);
            }
            if !rc.ignore_deps.is_empty() || !rc.ignore_paths.is_empty() {
                tracing::debug!(
                    repo = %repo_slug,
                    ignore_deps = ?rc.ignore_deps,
                    ignore_paths = ?rc.ignore_paths,
                    "repo config filters active"
                );
            }
            rc
        }
        Ok(repo_config::RepoConfigResult::NeedsOnboarding) => {
            tracing::info!(repo = %repo_slug, "needs onboarding — no config file found");
            renovate_core::repo_config::RepoConfig::default()
        }
        Ok(repo_config::RepoConfigResult::NotFound) => {
            tracing::debug!(
                repo = %repo_slug,
                "no config file (require_config=optional, skipping)"
            );
            renovate_core::repo_config::RepoConfig::default()
        }
        Err(err) => {
            tracing::error!(repo = %repo_slug, %err, "error processing repository");
            return (None, true);
        }
    };

    let files = match client.get_file_list(owner, repo).await {
        Ok(f) => f,
        Err(err) => {
            tracing::error!(repo = %repo_slug, %err, "failed to get file list");
            return (None, true);
        }
    };

    // Filter out paths the repo config asks to ignore before detection.
    // Build the path matcher once so glob compilation is amortized.
    let path_matcher = repo_cfg.build_path_matcher();
    let filtered_files: Vec<String> = files
        .into_iter()
        .filter(|f| !path_matcher.is_ignored(f))
        .collect();

    let detected = {
        let all = managers::detect(&filtered_files);
        // Apply `enabledManagers` filter: when non-empty, only the listed
        // managers are active.  Empty list means all managers are active.
        if repo_cfg.enabled_managers.is_empty() {
            all
        } else {
            let filtered: Vec<_> = all
                .into_iter()
                .filter(|m| {
                    repo_cfg.is_manager_enabled(
                        m.name,
                        renovate_core::managers::is_disabled_by_default(m.name),
                    )
                })
                .collect();
            if !filtered.is_empty() {
                tracing::debug!(
                    repo = %repo_slug,
                    enabled = ?repo_cfg.enabled_managers,
                    "enabledManagers filter applied"
                );
            }
            filtered
        }
    };
    if detected.is_empty() {
        tracing::info!(repo = %repo_slug, "no package managers detected");
    } else {
        let names: Vec<&str> = detected.iter().map(|m| m.name).collect();
        tracing::info!(repo = %repo_slug, managers = ?names, "detected package managers");
    }

    let mut ctx = crate::context::RepoPipelineCtx {
        client,
        http,
        config,
        owner,
        repo,
        repo_slug,
        repo_cfg: &repo_cfg,
        detected: &detected,
        filtered_files: &filtered_files,
        report: output::RepoReport {
            repo_slug: repo_slug.to_owned(),
            files: Vec::new(),
        },
        had_error: false,
    };
    pipelines::process_all_managers(&mut ctx).await;
    let (mut repo_report, had_error) = (ctx.report, ctx.had_error);

    // Apply matchUpdateTypes packageRules blocking across all collected file reports.
    pipeline_utils::apply_update_blocking_to_report(&mut repo_report, &repo_cfg);
    // Apply ignoreVersions (global + per-rule) across all collected file reports.
    pipeline_utils::apply_version_ignore_to_report(&mut repo_report, &repo_cfg);

    (Some(repo_report), had_error)
}
