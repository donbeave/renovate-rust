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
mod logging;
mod migrate;
mod output;

use std::collections::HashMap;
use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser as _;
use cli::Cli;
use renovate_core::config::{GlobalConfig, file as config_file};
use renovate_core::datasources::crates_io::{self, DepInput};
use renovate_core::datasources::npm as npm_datasource;
use renovate_core::datasources::pypi as pypi_datasource;
use renovate_core::extractors::cargo as cargo_extractor;
use renovate_core::extractors::npm as npm_extractor;
use renovate_core::extractors::pip as pip_extractor;
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

    // 5. Exit early when there is nothing to do.
    if config.repositories.is_empty() {
        tracing::info!("no repositories configured — nothing to do");
        return ExitCode::SUCCESS;
    }

    // 6. Platform initialization: create client and validate credentials.
    //    Mirrors Renovate's globalInitialize → initPlatform.
    let maybe_client: Option<AnyPlatformClient> = if config.token.is_none() {
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

    let sem = Arc::new(Semaphore::new(REPO_CONCURRENCY));
    let mut set: JoinSet<(String, Option<output::RepoReport>, bool)> = JoinSet::new();

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
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok((_slug, Some(report), repo_had_error)) => {
                output::print_report(&report, use_color);
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

    match repo_config::discover(client, owner, repo, config).await {
        Ok(repo_config::RepoConfigResult::Found { path, .. }) => {
            tracing::info!(repo = %repo_slug, config_path = %path, "found renovate config");
        }
        Ok(repo_config::RepoConfigResult::NeedsOnboarding) => {
            tracing::info!(repo = %repo_slug, "needs onboarding — no config file found");
        }
        Ok(repo_config::RepoConfigResult::NotFound) => {
            tracing::debug!(
                repo = %repo_slug,
                "no config file (require_config=optional, skipping)"
            );
        }
        Err(err) => {
            tracing::error!(repo = %repo_slug, %err, "error processing repository");
            return (None, true);
        }
    }

    let files = match client.get_file_list(owner, repo).await {
        Ok(f) => f,
        Err(err) => {
            tracing::error!(repo = %repo_slug, %err, "failed to get file list");
            return (None, true);
        }
    };

    let detected = managers::detect(&files);
    if detected.is_empty() {
        tracing::info!(repo = %repo_slug, "no package managers detected");
    } else {
        let names: Vec<&str> = detected.iter().map(|m| m.name).collect();
        tracing::info!(repo = %repo_slug, managers = ?names, "detected package managers");
    }

    let mut repo_report = output::RepoReport {
        repo_slug: repo_slug.to_owned(),
        files: Vec::new(),
    };
    let mut had_error = false;

    // ── Cargo ─────────────────────────────────────────────────────────────────
    for cargo_file_path in manager_files(&detected, "cargo") {
        match client.get_raw_file(owner, repo, &cargo_file_path).await {
            Ok(Some(raw)) => match cargo_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> =
                        deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %cargo_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted cargo dependencies"
                    );
                    let dep_inputs: Vec<DepInput> = actionable
                        .iter()
                        .map(|d| DepInput {
                            dep_name: d.dep_name.clone(),
                            package_name: d.package_name.clone(),
                            constraint: d.current_value.clone(),
                        })
                        .collect();
                    let updates = crates_io::fetch_updates_concurrent(
                        http,
                        &dep_inputs,
                        crates_io::CRATES_IO_SPARSE_INDEX,
                        10,
                    )
                    .await;
                    let update_map: HashMap<_, _> = updates
                        .into_iter()
                        .map(|r| (r.dep_name, r.summary))
                        .collect();
                    repo_report.files.push(output::FileReport {
                        path: cargo_file_path.clone(),
                        manager: "cargo".into(),
                        deps: build_dep_reports_cargo(&deps, &actionable, &update_map),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%cargo_file_path, %err, "failed to parse Cargo.toml")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%cargo_file_path, "Cargo.toml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cargo_file_path, %err, "failed to fetch Cargo.toml");
                had_error = true;
            }
        }
    }

    // ── npm ───────────────────────────────────────────────────────────────────
    for npm_file_path in manager_files(&detected, "npm") {
        match client.get_raw_file(owner, repo, &npm_file_path).await {
            Ok(Some(raw)) => match npm_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> =
                        deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %npm_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted npm dependencies"
                    );
                    let dep_inputs: Vec<npm_datasource::NpmDepInput> = actionable
                        .iter()
                        .map(|d| npm_datasource::NpmDepInput {
                            dep_name: d.name.clone(),
                            constraint: d.current_value.clone(),
                        })
                        .collect();
                    let updates = npm_datasource::fetch_updates_concurrent(
                        http,
                        &dep_inputs,
                        npm_datasource::NPM_REGISTRY,
                        10,
                    )
                    .await;
                    let update_map: HashMap<_, _> = updates
                        .into_iter()
                        .map(|r| (r.dep_name, r.summary))
                        .collect();
                    repo_report.files.push(output::FileReport {
                        path: npm_file_path.clone(),
                        manager: "npm".into(),
                        deps: build_dep_reports_npm(&deps, &actionable, &update_map),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%npm_file_path, %err, "failed to parse package.json")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%npm_file_path, "package.json not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%npm_file_path, %err, "failed to fetch package.json");
                had_error = true;
            }
        }
    }

    // ── pip_requirements ──────────────────────────────────────────────────────
    for pip_file_path in manager_files(&detected, "pip_requirements") {
        match client.get_raw_file(owner, repo, &pip_file_path).await {
            Ok(Some(raw)) => match pip_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> =
                        deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %pip_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted pip dependencies"
                    );
                    let dep_inputs: Vec<pypi_datasource::PypiDepInput> = actionable
                        .iter()
                        .map(|d| pypi_datasource::PypiDepInput {
                            dep_name: d.name.clone(),
                            specifier: d.current_value.clone(),
                        })
                        .collect();
                    let updates = pypi_datasource::fetch_updates_concurrent(
                        http,
                        &dep_inputs,
                        pypi_datasource::PYPI_API,
                        10,
                    )
                    .await;
                    let update_map: HashMap<_, _> = updates
                        .into_iter()
                        .map(|r| (r.dep_name, r.summary))
                        .collect();
                    repo_report.files.push(output::FileReport {
                        path: pip_file_path.clone(),
                        manager: "pip_requirements".into(),
                        deps: build_dep_reports_pip(&deps, &actionable, &update_map),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%pip_file_path, %err, "failed to parse requirements.txt")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pip_file_path, "requirements.txt not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pip_file_path, %err, "failed to fetch requirements.txt");
                had_error = true;
            }
        }
    }

    // ── Dockerfile ────────────────────────────────────────────────────────────
    for df_file_path in manager_files(&detected, "dockerfile") {
        match client.get_raw_file(owner, repo, &df_file_path).await {
            Ok(Some(raw)) => {
                match renovate_core::extractors::dockerfile::extract(&raw.content) {
                    Ok(deps) => {
                        let actionable: Vec<_> =
                            deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                        tracing::debug!(
                            repo = %repo_slug, file = %df_file_path,
                            total = deps.len(), actionable = actionable.len(),
                            "extracted dockerfile images"
                        );
                        // No Docker registry datasource yet — report as
                        // UpToDate with no latest info (datasource slice follows).
                        let mut file_deps: Vec<output::DepReport> = Vec::new();
                        for dep in deps.iter().filter(|d| d.skip_reason.is_some()) {
                            file_deps.push(output::DepReport {
                                name: dep.image.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                        .to_lowercase(),
                                },
                            });
                        }
                        for dep in &actionable {
                            let name = match &dep.tag {
                                Some(t) => format!("{}:{t}", dep.image),
                                None => dep.image.clone(),
                            };
                            file_deps.push(output::DepReport {
                                name,
                                status: output::DepStatus::UpToDate { latest: None },
                            });
                        }
                        repo_report.files.push(output::FileReport {
                            path: df_file_path.clone(),
                            manager: "dockerfile".into(),
                            deps: file_deps,
                        });
                    }
                    Err(err) => {
                        tracing::warn!(repo=%repo_slug, file=%df_file_path, %err, "failed to parse Dockerfile")
                    }
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%df_file_path, "Dockerfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%df_file_path, %err, "failed to fetch Dockerfile");
                had_error = true;
            }
        }
    }

    (Some(repo_report), had_error)
}

// ── Report-building helpers ───────────────────────────────────────────────────

/// Return the matched files for a given manager name (empty slice if not
/// detected).
fn manager_files(detected: &[renovate_core::managers::DetectedManager], name: &str) -> Vec<String> {
    detected
        .iter()
        .find(|m| m.name == name)
        .map(|m| m.matched_files.clone())
        .unwrap_or_default()
}

fn build_dep_reports_cargo(
    all_deps: &[renovate_core::extractors::cargo::ExtractedDep],
    actionable: &[&renovate_core::extractors::cargo::ExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::cargo::UpdateSummary,
            renovate_core::datasources::crates_io::CratesIoError,
        >,
    >,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
        reports.push(output::DepReport {
            name: dep.dep_name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.dep_name) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_constraint.clone(),
                latest: s.latest_compatible.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest_compatible.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        reports.push(output::DepReport {
            name: dep.dep_name.clone(),
            status,
        });
    }
    reports
}

fn build_dep_reports_npm(
    all_deps: &[renovate_core::extractors::npm::NpmExtractedDep],
    actionable: &[&renovate_core::extractors::npm::NpmExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::npm::NpmUpdateSummary,
            renovate_core::datasources::npm::NpmError,
        >,
    >,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
        reports.push(output::DepReport {
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.name) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_constraint.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        reports.push(output::DepReport {
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

fn build_dep_reports_pip(
    all_deps: &[renovate_core::extractors::pip::PipExtractedDep],
    actionable: &[&renovate_core::extractors::pip::PipExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::pep440::Pep440UpdateSummary,
            renovate_core::datasources::pypi::PypiError,
        >,
    >,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
        reports.push(output::DepReport {
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.name) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_specifier.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        reports.push(output::DepReport {
            name: dep.name.clone(),
            status,
        });
    }
    reports
}
