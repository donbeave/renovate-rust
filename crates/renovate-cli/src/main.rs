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
use renovate_core::datasources::docker_hub as docker_datasource;
use renovate_core::datasources::github_tags as github_tags_datasource;
use renovate_core::datasources::gomod as gomod_datasource;
use renovate_core::datasources::maven as maven_datasource;
use renovate_core::datasources::npm as npm_datasource;
use renovate_core::datasources::nuget as nuget_datasource;
use renovate_core::datasources::packagist as packagist_datasource;
use renovate_core::datasources::pub_dev as pub_datasource;
use renovate_core::datasources::pypi as pypi_datasource;
use renovate_core::datasources::rubygems as rubygems_datasource;
use renovate_core::datasources::terraform as terraform_datasource;
use renovate_core::extractors::bundler as bundler_extractor;
use renovate_core::extractors::cargo as cargo_extractor;
use renovate_core::extractors::composer as composer_extractor;
use renovate_core::extractors::github_actions as github_actions_extractor;
use renovate_core::extractors::gomod as gomod_extractor;
use renovate_core::extractors::maven as maven_extractor;
use renovate_core::extractors::npm as npm_extractor;
use renovate_core::extractors::nuget as nuget_extractor;
use renovate_core::extractors::pep621 as pep621_extractor;
use renovate_core::extractors::pip as pip_extractor;
use renovate_core::extractors::poetry as poetry_extractor;
use renovate_core::extractors::pubspec as pubspec_extractor;
use renovate_core::extractors::terraform as terraform_extractor;
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
    let quiet = cli.quiet;

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
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok((_slug, Some(report), repo_had_error)) => {
                run_stats.add_report(&report);
                output::print_report(&report, use_color, quiet);
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

    output::print_run_summary(&run_stats, use_color);

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
            if !rc.enabled {
                tracing::info!(repo = %repo_slug, "renovate disabled in repo config — skipping");
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

    let detected = managers::detect(&filtered_files);
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
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| {
                            d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.dep_name)
                        })
                        .collect();
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

    // ── Dart/Flutter pub (pubspec.yaml) ───────────────────────────────────────
    for pub_file_path in manager_files(&detected, "pub") {
        match client.get_raw_file(owner, repo, &pub_file_path).await {
            Ok(Some(raw)) => {
                let deps = pubspec_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %pub_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted pub dependencies"
                );
                let dep_inputs: Vec<pub_datasource::PubDepInput> = actionable
                    .iter()
                    .map(|d| pub_datasource::PubDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = pub_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    pub_datasource::PUB_DEV_API,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                repo_report.files.push(output::FileReport {
                    path: pub_file_path.clone(),
                    manager: "pub".into(),
                    deps: build_dep_reports_pub(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pub_file_path, "pubspec.yaml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pub_file_path, %err,
                    "failed to fetch pubspec.yaml");
                had_error = true;
            }
        }
    }

    // ── NuGet (.csproj / .props / .targets) ──────────────────────────────────
    for nuget_file_path in manager_files(&detected, "nuget") {
        match client.get_raw_file(owner, repo, &nuget_file_path).await {
            Ok(Some(raw)) => match nuget_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| {
                            d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.package_id)
                        })
                        .collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %nuget_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted nuget dependencies"
                    );
                    let dep_inputs: Vec<nuget_datasource::NuGetDepInput> = actionable
                        .iter()
                        .map(|d| nuget_datasource::NuGetDepInput {
                            package_id: d.package_id.clone(),
                            current_value: d.current_value.clone(),
                        })
                        .collect();
                    let updates = nuget_datasource::fetch_updates_concurrent(
                        http,
                        &dep_inputs,
                        nuget_datasource::NUGET_API,
                        10,
                    )
                    .await;
                    let update_map: HashMap<_, _> = updates
                        .into_iter()
                        .map(|r| (r.package_id, r.summary))
                        .collect();
                    repo_report.files.push(output::FileReport {
                        path: nuget_file_path.clone(),
                        manager: "nuget".into(),
                        deps: build_dep_reports_nuget(&deps, &actionable, &update_map),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%nuget_file_path, %err,
                        "failed to parse nuget project file")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%nuget_file_path, "nuget file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%nuget_file_path, %err,
                    "failed to fetch nuget file");
                had_error = true;
            }
        }
    }

    // ── Composer (composer.json) ──────────────────────────────────────────────
    for composer_file_path in manager_files(&detected, "composer") {
        match client.get_raw_file(owner, repo, &composer_file_path).await {
            Ok(Some(raw)) => match composer_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                        .collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %composer_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted composer dependencies"
                    );
                    let dep_inputs: Vec<packagist_datasource::PackagistDepInput> = actionable
                        .iter()
                        .map(|d| packagist_datasource::PackagistDepInput {
                            package_name: d.name.clone(),
                            current_value: d.current_value.clone(),
                        })
                        .collect();
                    let updates = packagist_datasource::fetch_updates_concurrent(
                        http,
                        &dep_inputs,
                        packagist_datasource::PACKAGIST_API,
                        10,
                    )
                    .await;
                    let update_map: HashMap<_, _> = updates
                        .into_iter()
                        .map(|r| (r.package_name, r.summary))
                        .collect();
                    repo_report.files.push(output::FileReport {
                        path: composer_file_path.clone(),
                        manager: "composer".into(),
                        deps: build_dep_reports_composer(&deps, &actionable, &update_map),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%composer_file_path, %err,
                        "failed to parse composer.json")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%composer_file_path, "composer.json not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%composer_file_path, %err,
                    "failed to fetch composer.json");
                had_error = true;
            }
        }
    }

    // ── npm ───────────────────────────────────────────────────────────────────
    for npm_file_path in manager_files(&detected, "npm") {
        match client.get_raw_file(owner, repo, &npm_file_path).await {
            Ok(Some(raw)) => match npm_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                        .collect();
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
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                        .collect();
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

    // ── pep621 (pyproject.toml) ───────────────────────────────────────────────
    for pep621_file_path in manager_files(&detected, "pep621") {
        match client.get_raw_file(owner, repo, &pep621_file_path).await {
            Ok(Some(raw)) => match pep621_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                        .collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %pep621_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted pyproject.toml dependencies"
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

                    let mut file_deps: Vec<output::DepReport> = Vec::new();
                    for dep in deps.iter().filter(|d| d.skip_reason.is_some()) {
                        file_deps.push(output::DepReport {
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                    .to_lowercase(),
                            },
                        });
                    }
                    for dep in &actionable {
                        let status = match update_map.get(&dep.name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_specifier.clone(),
                                    latest: s.latest.clone().unwrap_or_default(),
                                }
                            }
                            Some(Ok(s)) => output::DepStatus::UpToDate {
                                latest: s.latest.clone(),
                            },
                            Some(Err(e)) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                            None => output::DepStatus::UpToDate { latest: None },
                        };
                        file_deps.push(output::DepReport {
                            name: dep.name.clone(),
                            status,
                        });
                    }
                    repo_report.files.push(output::FileReport {
                        path: pep621_file_path.clone(),
                        manager: "pep621".into(),
                        deps: file_deps,
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%pep621_file_path, %err,
                            "failed to parse pyproject.toml")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pep621_file_path, "pyproject.toml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pep621_file_path, %err,
                    "failed to fetch pyproject.toml");
                had_error = true;
            }
        }
    }

    // ── Poetry (pyproject.toml) ───────────────────────────────────────────────
    for poetry_file_path in manager_files(&detected, "poetry") {
        match client.get_raw_file(owner, repo, &poetry_file_path).await {
            Ok(Some(raw)) => match poetry_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                        .collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %poetry_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted poetry dependencies"
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
                        path: poetry_file_path.clone(),
                        manager: "poetry".into(),
                        deps: build_dep_reports_poetry(&deps, &actionable, &update_map),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%poetry_file_path, %err,
                        "failed to parse poetry pyproject.toml")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%poetry_file_path, "pyproject.toml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%poetry_file_path, %err,
                    "failed to fetch poetry pyproject.toml");
                had_error = true;
            }
        }
    }

    // ── Go modules (go.mod) ──────────────────────────────────────────────────
    for gomod_file_path in manager_files(&detected, "gomod") {
        match client.get_raw_file(owner, repo, &gomod_file_path).await {
            Ok(Some(raw)) => {
                let deps = gomod_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored(&d.module_path)
                            && !d.current_value.is_empty()
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gomod_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted go module dependencies"
                );
                let dep_inputs: Vec<gomod_datasource::GoModDepInput> = actionable
                    .iter()
                    .map(|d| gomod_datasource::GoModDepInput {
                        module_path: d.module_path.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = gomod_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    gomod_datasource::GO_PROXY_BASE,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.module_path, r.summary))
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: gomod_file_path.clone(),
                    manager: "gomod".into(),
                    deps: build_dep_reports_gomod(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%gomod_file_path, "go.mod not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gomod_file_path, %err, "failed to fetch go.mod");
                had_error = true;
            }
        }
    }

    // ── Maven (pom.xml) ───────────────────────────────────────────────────────
    for maven_file_path in manager_files(&detected, "maven") {
        match client.get_raw_file(owner, repo, &maven_file_path).await {
            Ok(Some(raw)) => match maven_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| {
                            d.skip_reason.is_none()
                                && !repo_cfg.is_dep_ignored(&d.dep_name)
                                && !d.current_value.is_empty()
                        })
                        .collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %maven_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted maven dependencies"
                    );
                    let dep_inputs: Vec<maven_datasource::MavenDepInput> = actionable
                        .iter()
                        .map(|d| maven_datasource::MavenDepInput {
                            dep_name: d.dep_name.clone(),
                            current_version: d.current_value.clone(),
                        })
                        .collect();
                    let updates =
                        maven_datasource::fetch_updates_concurrent(http, &dep_inputs, 10).await;
                    let update_map: HashMap<_, _> = updates
                        .into_iter()
                        .map(|r| (r.dep_name, r.summary))
                        .collect();
                    repo_report.files.push(output::FileReport {
                        path: maven_file_path.clone(),
                        manager: "maven".into(),
                        deps: build_dep_reports_maven(&deps, &actionable, &update_map),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%maven_file_path, %err, "failed to parse pom.xml")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%maven_file_path, "pom.xml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%maven_file_path, %err, "failed to fetch pom.xml");
                had_error = true;
            }
        }
    }

    // ── GitHub Actions ────────────────────────────────────────────────────────
    let gh_api_base = github_tags_datasource::api_base_from_endpoint(config.endpoint.as_deref());
    // Build an authenticated HTTP client for GitHub API calls (tag lookups).
    let gh_http = if let Some(ref token) = config.token {
        renovate_core::http::HttpClient::with_token(token).unwrap_or_else(|_| http.clone())
    } else {
        http.clone()
    };
    for gha_file_path in manager_files(&detected, "github-actions") {
        match client.get_raw_file(owner, repo, &gha_file_path).await {
            Ok(Some(raw)) => {
                let deps = github_actions_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !d.current_value.is_empty()
                            && !repo_cfg.is_dep_ignored(&d.action)
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gha_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted github-actions dependencies"
                );
                let dep_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.action.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &dep_inputs,
                    gh_api_base,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: gha_file_path.clone(),
                    manager: "github-actions".into(),
                    deps: build_dep_reports_github_actions(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%gha_file_path, "workflow file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gha_file_path, %err, "failed to fetch workflow file");
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
                        // Build Docker Hub dep inputs for images that have a tag.
                        let dep_inputs: Vec<docker_datasource::DockerDepInput> = actionable
                            .iter()
                            .filter_map(|d| {
                                let tag = d.tag.as_deref()?;
                                Some(docker_datasource::DockerDepInput {
                                    dep_name: format!("{}:{tag}", d.image),
                                    image: d.image.clone(),
                                    tag: tag.to_owned(),
                                })
                            })
                            .collect();

                        let updates = docker_datasource::fetch_updates_concurrent(
                            http,
                            &dep_inputs,
                            docker_datasource::DOCKER_HUB_API,
                            10,
                        )
                        .await;
                        let update_map: HashMap<_, _> = updates
                            .into_iter()
                            .map(|r| (r.dep_name, r.summary))
                            .collect();

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
                            let dep_name = match &dep.tag {
                                Some(t) => format!("{}:{t}", dep.image),
                                None => dep.image.clone(),
                            };
                            let status = match update_map.get(&dep_name) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: s.current_tag.clone(),
                                        latest: s.latest.clone().unwrap_or_default(),
                                    }
                                }
                                Some(Ok(s)) => output::DepStatus::UpToDate {
                                    latest: s.latest.clone(),
                                },
                                Some(Err(docker_datasource::DockerHubError::NonDockerHub(_))) => {
                                    output::DepStatus::Skipped {
                                        reason: "non-docker-hub registry".into(),
                                    }
                                }
                                Some(Err(e)) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            };
                            file_deps.push(output::DepReport {
                                name: dep_name,
                                status,
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

    // ── docker-compose ────────────────────────────────────────────────────────
    for compose_file_path in manager_files(&detected, "docker-compose") {
        match client.get_raw_file(owner, repo, &compose_file_path).await {
            Ok(Some(raw)) => {
                match renovate_core::extractors::docker_compose::extract(&raw.content) {
                    Ok(deps) => {
                        let actionable: Vec<_> =
                            deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                        tracing::debug!(
                            repo = %repo_slug, file = %compose_file_path,
                            total = deps.len(), actionable = actionable.len(),
                            "extracted docker-compose images"
                        );

                        let dep_inputs: Vec<docker_datasource::DockerDepInput> = actionable
                            .iter()
                            .filter_map(|d| {
                                let tag = d.tag.as_deref()?;
                                Some(docker_datasource::DockerDepInput {
                                    dep_name: format!("{}:{tag}", d.image),
                                    image: d.image.clone(),
                                    tag: tag.to_owned(),
                                })
                            })
                            .collect();

                        let updates = docker_datasource::fetch_updates_concurrent(
                            http,
                            &dep_inputs,
                            docker_datasource::DOCKER_HUB_API,
                            10,
                        )
                        .await;
                        let update_map: HashMap<_, _> = updates
                            .into_iter()
                            .map(|r| (r.dep_name, r.summary))
                            .collect();

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
                            let dep_name = match &dep.tag {
                                Some(t) => format!("{}:{t}", dep.image),
                                None => dep.image.clone(),
                            };
                            let status = match update_map.get(&dep_name) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: s.current_tag.clone(),
                                        latest: s.latest.clone().unwrap_or_default(),
                                    }
                                }
                                Some(Ok(s)) => output::DepStatus::UpToDate {
                                    latest: s.latest.clone(),
                                },
                                Some(Err(docker_datasource::DockerHubError::NonDockerHub(_))) => {
                                    output::DepStatus::Skipped {
                                        reason: "non-docker-hub registry".into(),
                                    }
                                }
                                Some(Err(e)) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            };
                            file_deps.push(output::DepReport {
                                name: dep_name,
                                status,
                            });
                        }
                        repo_report.files.push(output::FileReport {
                            path: compose_file_path.clone(),
                            manager: "docker-compose".into(),
                            deps: file_deps,
                        });
                    }
                    Err(err) => tracing::warn!(repo=%repo_slug, file=%compose_file_path, %err,
                        "failed to parse docker-compose file"),
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%compose_file_path, "compose file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%compose_file_path, %err,
                    "failed to fetch docker-compose file");
                had_error = true;
            }
        }
    }

    // ── Bundler (Gemfile) ─────────────────────────────────────────────────────
    for gemfile_path in manager_files(&detected, "bundler") {
        match client.get_raw_file(owner, repo, &gemfile_path).await {
            Ok(Some(raw)) => {
                let deps = bundler_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gemfile_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted bundler gems"
                );
                let dep_inputs: Vec<rubygems_datasource::GemDepInput> = actionable
                    .iter()
                    .map(|d| rubygems_datasource::GemDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = rubygems_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    rubygems_datasource::RUBYGEMS_API,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                repo_report.files.push(output::FileReport {
                    path: gemfile_path.clone(),
                    manager: "bundler".into(),
                    deps: build_dep_reports_bundler(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%gemfile_path, "Gemfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gemfile_path, %err, "failed to fetch Gemfile");
                had_error = true;
            }
        }
    }

    // ── Terraform (.tf / .tofu) ───────────────────────────────────────────────
    for tf_file_path in manager_files(&detected, "terraform") {
        match client.get_raw_file(owner, repo, &tf_file_path).await {
            Ok(Some(raw)) => {
                let deps = terraform_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %tf_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted terraform deps"
                );
                let dep_inputs: Vec<terraform_datasource::TerraformDepInput> = actionable
                    .iter()
                    .map(|d| terraform_datasource::TerraformDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                        kind: match d.dep_type {
                            terraform_extractor::TerraformDepType::Provider => {
                                terraform_datasource::TerraformLookupKind::Provider
                            }
                            terraform_extractor::TerraformDepType::Module => {
                                terraform_datasource::TerraformLookupKind::Module
                            }
                        },
                    })
                    .collect();
                let updates = terraform_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    terraform_datasource::TERRAFORM_REGISTRY,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                repo_report.files.push(output::FileReport {
                    path: tf_file_path.clone(),
                    manager: "terraform".into(),
                    deps: build_dep_reports_terraform(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%tf_file_path, "Terraform file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%tf_file_path, %err, "failed to fetch Terraform file");
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

fn build_dep_reports_github_actions(
    all_deps: &[renovate_core::extractors::github_actions::GithubActionsExtractedDep],
    actionable: &[&renovate_core::extractors::github_actions::GithubActionsExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::github_tags::GithubActionsUpdateSummary,
            renovate_core::datasources::github_tags::GithubTagsError,
        >,
    >,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
        reports.push(output::DepReport {
            name: dep.action.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.action) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
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
            name: dep.action.clone(),
            status,
        });
    }
    reports
}

fn build_dep_reports_maven(
    all_deps: &[renovate_core::extractors::maven::MavenExtractedDep],
    actionable: &[&renovate_core::extractors::maven::MavenExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::maven::MavenUpdateSummary,
            renovate_core::datasources::maven::MavenError,
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
                current: s.current_version.clone(),
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
            name: dep.dep_name.clone(),
            status,
        });
    }
    reports
}

fn build_dep_reports_pub(
    all_deps: &[renovate_core::extractors::pubspec::PubspecExtractedDep],
    actionable: &[&renovate_core::extractors::pubspec::PubspecExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::pub_dev::PubUpdateSummary,
            renovate_core::datasources::pub_dev::PubError,
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
                current: s.current_value.clone(),
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

fn build_dep_reports_nuget(
    all_deps: &[renovate_core::extractors::nuget::NuGetExtractedDep],
    actionable: &[&renovate_core::extractors::nuget::NuGetExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::nuget::NuGetUpdateSummary,
            renovate_core::datasources::nuget::NuGetError,
        >,
    >,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
        reports.push(output::DepReport {
            name: dep.package_id.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.package_id) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
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
            name: dep.package_id.clone(),
            status,
        });
    }
    reports
}

fn build_dep_reports_composer(
    all_deps: &[renovate_core::extractors::composer::ComposerExtractedDep],
    actionable: &[&renovate_core::extractors::composer::ComposerExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::packagist::PackagistUpdateSummary,
            renovate_core::datasources::packagist::PackagistError,
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
                current: s.current_value.clone(),
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

fn build_dep_reports_gomod(
    all_deps: &[renovate_core::extractors::gomod::GoModExtractedDep],
    actionable: &[&renovate_core::extractors::gomod::GoModExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::gomod::GoModUpdateSummary,
            renovate_core::datasources::gomod::GoModError,
        >,
    >,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
        reports.push(output::DepReport {
            name: dep.module_path.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.module_path) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
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
            name: dep.module_path.clone(),
            status,
        });
    }
    reports
}

fn build_dep_reports_poetry(
    all_deps: &[renovate_core::extractors::poetry::PoetryExtractedDep],
    actionable: &[&renovate_core::extractors::poetry::PoetryExtractedDep],
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

fn build_dep_reports_bundler(
    all_deps: &[renovate_core::extractors::bundler::BundlerExtractedDep],
    actionable: &[&renovate_core::extractors::bundler::BundlerExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::rubygems::GemUpdateSummary,
            renovate_core::datasources::rubygems::RubyGemsError,
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
                current: s.current_value.clone(),
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

fn build_dep_reports_terraform(
    all_deps: &[renovate_core::extractors::terraform::TerraformExtractedDep],
    actionable: &[&renovate_core::extractors::terraform::TerraformExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::terraform::TerraformUpdateSummary,
            renovate_core::datasources::terraform::TerraformError,
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
                current: s.current_value.clone(),
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
