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
use renovate_core::datasources::bitrise as bitrise_datasource;
use renovate_core::datasources::crates_io::{self, DepInput};
use renovate_core::datasources::docker_hub as docker_datasource;
use renovate_core::datasources::github_releases as github_releases_datasource;
use renovate_core::datasources::github_tags as github_tags_datasource;
use renovate_core::datasources::gomod as gomod_datasource;
use renovate_core::datasources::helm as helm_datasource;
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
use renovate_core::extractors::gradle as gradle_extractor;
use renovate_core::extractors::helm as helm_extractor;
use renovate_core::extractors::homeassistant as homeassistant_extractor;
use renovate_core::extractors::homebrew as homebrew_extractor;
use renovate_core::extractors::maven as maven_extractor;
use renovate_core::extractors::npm as npm_extractor;
use renovate_core::extractors::nuget as nuget_extractor;
use renovate_core::extractors::pep621 as pep621_extractor;
use renovate_core::extractors::pip as pip_extractor;
use renovate_core::extractors::poetry as poetry_extractor;
use renovate_core::extractors::pubspec as pubspec_extractor;
use renovate_core::extractors::setup_cfg as setup_cfg_extractor;
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

    // ── setup.py (pip_setup) ─────────────────────────────────────────────────
    for setup_py_path in manager_files(&detected, "pip_setup") {
        match client.get_raw_file(owner, repo, &setup_py_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::pip_setup::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %setup_py_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted setup.py dependencies"
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
                    path: setup_py_path.clone(),
                    manager: "pip_setup".into(),
                    deps: build_dep_reports_pip(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%setup_py_path, "setup.py not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%setup_py_path, %err, "failed to fetch setup.py");
                had_error = true;
            }
        }
    }

    // ── setup.cfg ────────────────────────────────────────────────────────────
    for setup_cfg_path in manager_files(&detected, "setup-cfg") {
        match client.get_raw_file(owner, repo, &setup_cfg_path).await {
            Ok(Some(raw)) => {
                let deps = setup_cfg_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %setup_cfg_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted setup.cfg dependencies"
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
                    path: setup_cfg_path.clone(),
                    manager: "setup-cfg".into(),
                    deps: build_dep_reports_setup_cfg(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%setup_cfg_path, "setup.cfg not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%setup_cfg_path, %err, "failed to fetch setup.cfg");
                had_error = true;
            }
        }
    }

    // ── homeassistant-manifest ────────────────────────────────────────────────
    for ha_path in manager_files(&detected, "homeassistant-manifest") {
        match client.get_raw_file(owner, repo, &ha_path).await {
            Ok(Some(raw)) => {
                let deps = homeassistant_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %ha_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted homeassistant manifest dependencies"
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
                    path: ha_path.clone(),
                    manager: "homeassistant-manifest".into(),
                    deps: build_dep_reports_pip(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%ha_path, "manifest.json not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ha_path, %err, "failed to fetch manifest.json");
                had_error = true;
            }
        }
    }

    // ── HTML (cdnjs) ─────────────────────────────────────────────────────────
    for html_path in manager_files(&detected, "html") {
        match client.get_raw_file(owner, repo, &html_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::html::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %html_path,
                    total = deps.len(),
                    "extracted HTML cdnjs dependencies"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        continue;
                    }
                    let library = &dep.dep_name;
                    let status = match renovate_core::datasources::cdnjs::fetch_latest(
                        http,
                        library,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value.clone(),
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: html_path.clone(),
                        manager: "html".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%html_path, "HTML file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%html_path, %err, "failed to fetch HTML file");
                had_error = true;
            }
        }
    }

    // ── Typst (*.typ) ────────────────────────────────────────────────────────
    for typ_path in manager_files(&detected, "typst") {
        match client.get_raw_file(owner, repo, &typ_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::typst::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %typ_path,
                    total = deps.len(),
                    "extracted typst package deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.package_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.package_name) {
                        continue;
                    }
                    let status = match renovate_core::datasources::typst::fetch_latest(
                        http,
                        &dep.package_name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value,
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.package_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: typ_path.clone(),
                        manager: "typst".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%typ_path, "typst file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%typ_path, %err, "failed to fetch typst file");
                had_error = true;
            }
        }
    }

    // ── cpanfile (Perl) ──────────────────────────────────────────────────────
    for cpan_path in manager_files(&detected, "cpanfile") {
        match client.get_raw_file(owner, repo, &cpan_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cpanfile::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %cpan_path,
                    total = deps.len(),
                    "extracted cpanfile perl deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        continue;
                    }
                    let status = match renovate_core::datasources::cpan::fetch_latest(
                        http,
                        &dep.dep_name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value,
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: cpan_path.clone(),
                        manager: "cpanfile".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cpan_path, "cpanfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cpan_path, %err, "failed to fetch cpanfile");
                had_error = true;
            }
        }
    }

    // ── Pipfile (pipenv) ──────────────────────────────────────────────────────
    for pipfile_path in manager_files(&detected, "pipenv") {
        match client.get_raw_file(owner, repo, &pipfile_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::pipfile::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %pipfile_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted Pipfile dependencies"
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
                    path: pipfile_path.clone(),
                    manager: "pipenv".into(),
                    deps: build_dep_reports_pipfile(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pipfile_path, "Pipfile not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pipfile_path, %err, "failed to fetch Pipfile");
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

    // ── Apache Ant (build.xml) ────────────────────────────────────────────────
    for ant_path in manager_files(&detected, "ant") {
        match client.get_raw_file(owner, repo, &ant_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ant::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %ant_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted ant/maven dependencies"
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
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        if let Some(reason) = &dep.skip_reason {
                            return output::DepReport {
                                name: dep.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{reason:?}").to_lowercase(),
                                },
                            };
                        }
                        let status = match update_map.get(&dep.dep_name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_version.clone(),
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
                        output::DepReport {
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: ant_path.clone(),
                        manager: "ant".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%ant_path, "build.xml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ant_path, %err, "failed to fetch build.xml");
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

    // ── Kotlin Script (*.main.kts) ────────────────────────────────────────────
    for kts_path in manager_files(&detected, "kotlin-script") {
        match client.get_raw_file(owner, repo, &kts_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::kotlin_script::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %kts_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted kotlin script dependencies"
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
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.dep_name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_version.clone(),
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
                        output::DepReport {
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: kts_path.clone(),
                    manager: "kotlin-script".into(),
                    deps: dep_reports,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%kts_path, "kotlin script not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%kts_path, %err, "failed to fetch kotlin script");
                had_error = true;
            }
        }
    }

    // ── OSGi feature model (src/main/features/*.json) ─────────────────────────
    for osgi_path in manager_files(&detected, "osgi") {
        match client.get_raw_file(owner, repo, &osgi_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::osgi::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %osgi_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted osgi bundle deps"
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
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        if let Some(reason) = &dep.skip_reason {
                            return output::DepReport {
                                name: dep.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{reason:?}").to_lowercase(),
                                },
                            };
                        }
                        let status = match update_map.get(&dep.dep_name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_version.clone(),
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
                        output::DepReport {
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: osgi_path.clone(),
                        manager: "osgi".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%osgi_path, "osgi feature file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%osgi_path, %err, "failed to fetch osgi feature file");
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

                // Also extract container/services Docker images from this workflow file.
                let docker_deps = github_actions_extractor::extract_docker_images(&raw.content);
                let docker_actionable: Vec<_> = docker_deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none())
                    .collect();
                let docker_inputs: Vec<docker_datasource::DockerDepInput> = docker_actionable
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
                let docker_updates = docker_datasource::fetch_updates_concurrent(
                    http,
                    &docker_inputs,
                    docker_datasource::DOCKER_HUB_API,
                    10,
                )
                .await;
                let docker_update_map: HashMap<_, _> = docker_updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();

                let mut all_deps =
                    build_dep_reports_github_actions(&deps, &actionable, &update_map);
                for dep in &docker_deps {
                    if let Some(reason) = &dep.skip_reason {
                        all_deps.push(output::DepReport {
                            name: dep.image.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                    } else {
                        let dep_name = match &dep.tag {
                            Some(t) => format!("{}:{t}", dep.image),
                            None => dep.image.clone(),
                        };
                        let status = match docker_update_map.get(&dep_name) {
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
                        all_deps.push(output::DepReport {
                            name: dep_name,
                            status,
                        });
                    }
                }

                // Extract and report `runs-on:` runner label versions.
                let runner_deps = github_actions_extractor::extract_runner_labels(&raw.content);
                for rdep in &runner_deps {
                    let s = renovate_core::datasources::github_runners::update_summary(
                        &rdep.runner_name,
                        &rdep.current_value,
                    );
                    let dep_name = format!("{}-{}", rdep.runner_name, rdep.current_value);
                    let status = if s.update_available {
                        output::DepStatus::UpdateAvailable {
                            current: s.current.clone(),
                            latest: s.latest.unwrap_or_default(),
                        }
                    } else if s.deprecated {
                        output::DepStatus::Skipped {
                            reason: "deprecated runner".into(),
                        }
                    } else {
                        output::DepStatus::UpToDate { latest: s.latest }
                    };
                    all_deps.push(output::DepReport {
                        name: dep_name,
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: gha_file_path.clone(),
                    manager: "github-actions".into(),
                    deps: all_deps,
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
            Ok(Some(raw)) => match renovate_core::extractors::dockerfile::extract(&raw.content) {
                Ok(deps) => {
                    tracing::debug!(repo = %repo_slug, file = %df_file_path, total = deps.len(), "extracted dockerfile images");
                    repo_report.files.push(output::FileReport {
                        path: df_file_path.clone(),
                        manager: "dockerfile".into(),
                        deps: docker_hub_reports(http, &deps).await,
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%df_file_path, %err, "failed to parse Dockerfile")
                }
            },
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

    // ── gemspec (.gemspec) ────────────────────────────────────────────────────
    for gemspec_path in manager_files(&detected, "gemspec") {
        match client.get_raw_file(owner, repo, &gemspec_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gemspec::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !d.current_value.is_empty()
                            && !repo_cfg.is_dep_ignored(&d.name)
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gemspec_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted gemspec deps"
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
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if dep.current_value.is_empty() {
                        output::DepStatus::Skipped {
                            reason: "no-version".to_owned(),
                        }
                    } else if repo_cfg.is_dep_ignored(&dep.name) {
                        output::DepStatus::Skipped {
                            reason: "ignored".to_owned(),
                        }
                    } else {
                        match update_map.get(&dep.name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_value.clone(),
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
                        }
                    };
                    file_deps.push(output::DepReport {
                        name: dep.name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: gemspec_path.clone(),
                    manager: "gemspec".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%gemspec_path, ".gemspec not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gemspec_path, %err, "failed to fetch .gemspec");
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

    // ── Terragrunt (terragrunt.hcl) ───────────────────────────────────────────
    for tg_path in manager_files(&detected, "terragrunt") {
        match client.get_raw_file(owner, repo, &tg_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::terragrunt::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %tg_path,
                    total = deps.len(),
                    "extracted terragrunt module deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        continue;
                    }

                    use renovate_core::extractors::terragrunt::TerragruntSource;
                    let status = match &dep.source {
                        Some(TerragruntSource::GitHub(gh_repo)) => {
                            let tag_result =
                                renovate_core::datasources::github_tags::fetch_latest_tag(
                                    gh_repo,
                                    &gh_http,
                                    gh_api_base,
                                )
                                .await
                                .map_err(|e| e.to_string());
                            match tag_result {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let clean = dep.current_value.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(clean, Some(stripped));
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: tag.clone(),
                                        }
                                    } else {
                                        output::DepStatus::UpToDate { latest: Some(tag) }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError { message: e },
                            }
                        }
                        Some(TerragruntSource::TerraformRegistry { hostname, module }) => {
                            let registry_base = format!("https://{hostname}/api/v1/modules");
                            let inputs = vec![terraform_datasource::TerraformDepInput {
                                name: module.clone(),
                                current_value: dep.current_value.clone(),
                                kind: terraform_datasource::TerraformLookupKind::Module,
                            }];
                            let updates = terraform_datasource::fetch_updates_concurrent(
                                http,
                                &inputs,
                                &registry_base,
                                8,
                            )
                            .await;
                            let update_map: HashMap<_, _> =
                                updates.into_iter().map(|r| (r.name, r.summary)).collect();
                            match update_map.get(module) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: s.current_value.clone(),
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
                            }
                        }
                        _ => output::DepStatus::Skipped {
                            reason: "unsupported-source".into(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: tg_path.clone(),
                        manager: "terragrunt".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%tg_path, "terragrunt.hcl not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%tg_path, %err, "failed to fetch terragrunt.hcl");
                had_error = true;
            }
        }
    }

    // ── TFLint plugin (.tflint.hcl) ──────────────────────────────────────────
    for tflint_path in manager_files(&detected, "tflint-plugin") {
        match client.get_raw_file(owner, repo, &tflint_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::tflint_plugin::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %tflint_path,
                    total = deps.len(),
                    "extracted tflint plugin deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        continue;
                    }
                    let status = match github_releases_datasource::fetch_latest_release(
                        &dep.dep_name,
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    {
                        Ok(Some(tag)) => {
                            let stripped = tag.trim_start_matches('v');
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    &dep.current_value,
                                    Some(stripped),
                                );
                            if s.update_available {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest: stripped.to_owned(),
                                }
                            } else {
                                output::DepStatus::UpToDate {
                                    latest: Some(stripped.to_owned()),
                                }
                            }
                        }
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: tflint_path.clone(),
                        manager: "tflint-plugin".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%tflint_path, ".tflint.hcl not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%tflint_path, %err, "failed to fetch .tflint.hcl");
                had_error = true;
            }
        }
    }

    // ── Helm (Chart.yaml / requirements.yaml) ────────────────────────────────
    for helm_file_path in manager_files(&detected, "helmv3") {
        match client.get_raw_file(owner, repo, &helm_file_path).await {
            Ok(Some(raw)) => {
                let deps = helm_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %helm_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted helm chart deps"
                );
                let dep_inputs: Vec<helm_datasource::HelmDepInput> = actionable
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                        repository_url: d.repository.clone(),
                    })
                    .collect();
                let updates = helm_datasource::fetch_updates_concurrent(http, &dep_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                repo_report.files.push(output::FileReport {
                    path: helm_file_path.clone(),
                    manager: "helmv3".into(),
                    deps: build_dep_reports_helm(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%helm_file_path, "Chart.yaml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%helm_file_path, %err, "failed to fetch Chart.yaml");
                had_error = true;
            }
        }
    }

    // ── Helm Values (values.yaml) ─────────────────────────────────────────────
    for hv_path in manager_files(&detected, "helm-values") {
        match client.get_raw_file(owner, repo, &hv_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::helm_values::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %hv_path, total = deps.len(), "extracted helm values images");
                repo_report.files.push(output::FileReport {
                    path: hv_path.clone(),
                    manager: "helm-values".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%hv_path, "values.yaml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%hv_path, %err, "failed to fetch values.yaml");
                had_error = true;
            }
        }
    }

    // ── Helmfile (helmfile.yaml / helmfile.d/*.yaml) ──────────────────────────
    for hf_path in manager_files(&detected, "helmfile") {
        match client.get_raw_file(owner, repo, &hf_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::helmfile::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %hf_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted helmfile chart deps"
                );
                let dep_inputs: Vec<helm_datasource::HelmDepInput> = actionable
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                        repository_url: d.repository.clone(),
                    })
                    .collect();
                let updates = helm_datasource::fetch_updates_concurrent(http, &dep_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                repo_report.files.push(output::FileReport {
                    path: hf_path.clone(),
                    manager: "helmfile".into(),
                    deps: build_dep_reports_helm(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%hf_path, "helmfile not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%hf_path, %err, "failed to fetch helmfile");
                had_error = true;
            }
        }
    }

    // ── Fleet (fleet.yaml / GitRepo CRDs) ────────────────────────────────────
    for fleet_path in manager_files(&detected, "fleet") {
        match client.get_raw_file(owner, repo, &fleet_path).await {
            Ok(Some(raw)) => {
                let is_fleet_yaml =
                    renovate_core::extractors::fleet::is_fleet_yaml_path(&fleet_path);
                let extracted =
                    renovate_core::extractors::fleet::extract(&raw.content, is_fleet_yaml);
                tracing::debug!(
                    repo = %repo_slug, file = %fleet_path,
                    helm = extracted.helm_deps.len(),
                    git = extracted.git_deps.len(),
                    "extracted fleet deps"
                );

                let mut dep_reports: Vec<output::DepReport> = Vec::new();

                // Helm deps
                let helm_actionable: Vec<_> = extracted
                    .helm_deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored(&d.chart)
                            && !d.current_value.is_empty()
                    })
                    .collect();
                if !helm_actionable.is_empty() {
                    let dep_inputs: Vec<helm_datasource::HelmDepInput> = helm_actionable
                        .iter()
                        .map(|d| helm_datasource::HelmDepInput {
                            name: d.chart.clone(),
                            current_value: d.current_value.clone(),
                            repository_url: d.registry_url.clone(),
                        })
                        .collect();
                    let updates =
                        helm_datasource::fetch_updates_concurrent(http, &dep_inputs, 8).await;
                    let update_map: HashMap<_, _> =
                        updates.into_iter().map(|r| (r.name, r.summary)).collect();
                    for dep in &extracted.helm_deps {
                        if let Some(reason) = &dep.skip_reason {
                            dep_reports.push(output::DepReport {
                                name: dep.chart.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{reason:?}").to_lowercase(),
                                },
                            });
                            continue;
                        }
                        let status = match update_map.get(&dep.chart) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_value.clone(),
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
                        dep_reports.push(output::DepReport {
                            name: dep.chart.clone(),
                            status,
                        });
                    }
                }

                // Git repo deps
                for git_dep in &extracted.git_deps {
                    if let Some(ref reason) = git_dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: git_dep.repo_url.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    let repo_name = git_dep
                        .repo_url
                        .trim_end_matches('/')
                        .trim_end_matches(".git")
                        .trim_start_matches("https://github.com/")
                        .trim_start_matches("http://github.com/");
                    let tag_result = renovate_core::datasources::github_tags::fetch_latest_tag(
                        repo_name,
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    .map_err(|e| e.to_string());
                    let status = match tag_result {
                        Ok(Some(tag)) => {
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    &git_dep.current_value,
                                    Some(&tag),
                                );
                            if s.update_available {
                                output::DepStatus::UpdateAvailable {
                                    current: git_dep.current_value.clone(),
                                    latest: tag,
                                }
                            } else {
                                output::DepStatus::UpToDate { latest: Some(tag) }
                            }
                        }
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError { message: e },
                    };
                    dep_reports.push(output::DepReport {
                        name: git_dep.repo_url.clone(),
                        status,
                    });
                }

                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: fleet_path.clone(),
                        manager: "fleet".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%fleet_path, "fleet file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%fleet_path, %err, "failed to fetch fleet file");
                had_error = true;
            }
        }
    }

    // ── Kustomize (kustomization.yaml) ───────────────────────────────────────
    for kustomize_path in manager_files(&detected, "kustomize") {
        match client.get_raw_file(owner, repo, &kustomize_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::kustomize::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %kustomize_path,
                    total = deps.len(),
                    "extracted kustomize deps"
                );

                // Collect image and helm deps separately for datasource routing.
                let image_deps: Vec<_> = deps
                    .iter()
                    .filter_map(|d| {
                        if let renovate_core::extractors::kustomize::KustomizeDep::Image(i) = d {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();
                let helm_deps: Vec<_> = deps
                    .iter()
                    .filter_map(|d| {
                        if let renovate_core::extractors::kustomize::KustomizeDep::Helm(h) = d {
                            Some(h)
                        } else {
                            None
                        }
                    })
                    .collect();

                // Look up Docker images.
                let image_inputs: Vec<docker_datasource::DockerDepInput> = image_deps
                    .iter()
                    .filter(|i| i.skip_reason.is_none())
                    .filter_map(|i| {
                        let tag = i.tag.as_deref()?;
                        Some(docker_datasource::DockerDepInput {
                            dep_name: format!("{}:{tag}", i.image),
                            image: i.image.clone(),
                            tag: tag.to_owned(),
                        })
                    })
                    .collect();
                let image_updates = docker_datasource::fetch_updates_concurrent(
                    http,
                    &image_inputs,
                    docker_datasource::DOCKER_HUB_API,
                    10,
                )
                .await;
                let image_update_map: HashMap<_, _> = image_updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();

                // Look up Helm charts.
                let helm_inputs: Vec<helm_datasource::HelmDepInput> = helm_deps
                    .iter()
                    .filter(|h| !h.current_value.is_empty())
                    .map(|h| helm_datasource::HelmDepInput {
                        name: h.chart_name.clone(),
                        current_value: h.current_value.clone(),
                        repository_url: h.repository_url.clone(),
                    })
                    .collect();
                let helm_updates =
                    helm_datasource::fetch_updates_concurrent(http, &helm_inputs, 8).await;
                let helm_update_map: HashMap<_, _> = helm_updates
                    .into_iter()
                    .map(|r| (r.name, r.summary))
                    .collect();

                // Build dep reports.
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &image_deps {
                    if let Some(reason) = &dep.skip_reason {
                        file_deps.push(output::DepReport {
                            name: dep.image.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                    } else {
                        let dep_name = match &dep.tag {
                            Some(t) => format!("{}:{t}", dep.image),
                            None => dep.image.clone(),
                        };
                        let status = match image_update_map.get(&dep_name) {
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
                }
                for helm in &helm_deps {
                    let status = match helm_update_map.get(&helm.chart_name) {
                        Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: helm.current_value.clone(),
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
                    file_deps.push(output::DepReport {
                        name: helm.chart_name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: kustomize_path.clone(),
                    manager: "kustomize".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%kustomize_path, "kustomization.yaml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%kustomize_path, %err, "failed to fetch kustomization.yaml");
                had_error = true;
            }
        }
    }

    // ── Gradle (.gradle / .gradle.kts / .versions.toml) ──────────────────────
    for gradle_file_path in manager_files(&detected, "gradle") {
        match client.get_raw_file(owner, repo, &gradle_file_path).await {
            Ok(Some(raw)) => {
                // Route to the appropriate parser based on file extension.
                let deps: Vec<renovate_core::extractors::gradle::GradleExtractedDep> =
                    if gradle_file_path.ends_with(".toml") {
                        gradle_extractor::extract_version_catalog(&raw.content)
                    } else {
                        gradle_extractor::extract_build_file(&raw.content)
                    };

                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gradle_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted gradle deps"
                );

                // Reuse the Maven datasource — Gradle deps are Maven artifacts.
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
                    path: gradle_file_path.clone(),
                    manager: "gradle".into(),
                    deps: build_dep_reports_gradle(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%gradle_file_path, "Gradle file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gradle_file_path, %err, "failed to fetch Gradle file");
                had_error = true;
            }
        }
    }

    // ── Gradle Wrapper (gradle/wrapper/gradle-wrapper.properties) ────────────
    for gw_path in manager_files(&detected, "gradle-wrapper") {
        match client.get_raw_file(owner, repo, &gw_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::gradle_wrapper::extract(&raw.content)
                {
                    tracing::debug!(
                        repo = %repo_slug, file = %gw_path,
                        version = %dep.version, "extracted gradle-wrapper version"
                    );
                    let status = match renovate_core::datasources::gradle_version::fetch_latest(
                        http,
                        &dep.version,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_version,
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    repo_report.files.push(output::FileReport {
                        path: gw_path.clone(),
                        manager: "gradle-wrapper".into(),
                        deps: vec![output::DepReport {
                            name: "gradle".into(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%gw_path, "gradle-wrapper.properties not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gw_path, %err, "failed to fetch gradle-wrapper.properties");
                had_error = true;
            }
        }
    }

    // ── Maven Wrapper (.mvn/wrapper/maven-wrapper.properties) ────────────────
    for mw_path in manager_files(&detected, "maven-wrapper") {
        match client.get_raw_file(owner, repo, &mw_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::maven_wrapper::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %mw_path,
                    total = deps.len(), "extracted maven-wrapper versions"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = match maven_datasource::fetch_latest(&dep.package_name, http).await
                    {
                        Ok(Some(latest)) if latest != dep.version => {
                            output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest,
                            }
                        }
                        Ok(Some(latest)) => output::DepStatus::UpToDate {
                            latest: Some(latest),
                        },
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    file_deps.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !file_deps.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: mw_path.clone(),
                        manager: "maven-wrapper".into(),
                        deps: file_deps,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%mw_path, "maven-wrapper.properties not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%mw_path, %err, "failed to fetch maven-wrapper.properties");
                had_error = true;
            }
        }
    }

    // ── Scalafmt (.scalafmt.conf) ─────────────────────────────────────────────
    for sfmt_path in manager_files(&detected, "scalafmt") {
        match client.get_raw_file(owner, repo, &sfmt_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::scalafmt::extract(&raw.content) {
                    tracing::debug!(
                        repo = %repo_slug, file = %sfmt_path,
                        version = %dep.version, "extracted scalafmt version"
                    );
                    let input = github_tags_datasource::GithubActionsDepInput {
                        dep_name: renovate_core::extractors::scalafmt::SCALAFMT_REPO.to_owned(),
                        current_value: dep.version.clone(),
                    };
                    let updates = github_tags_datasource::fetch_updates_concurrent(
                        &gh_http,
                        &[input],
                        gh_api_base,
                        4,
                    )
                    .await;
                    let status = match updates.into_iter().next().map(|r| r.summary) {
                        Some(Ok(s)) if s.update_available => {
                            let latest = s
                                .latest
                                .as_deref()
                                .map(|l| l.strip_prefix('v').unwrap_or(l).to_owned())
                                .unwrap_or_default();
                            output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest,
                            }
                        }
                        Some(Ok(s)) => {
                            let latest = s
                                .latest
                                .as_deref()
                                .map(|l| l.strip_prefix('v').unwrap_or(l).to_owned());
                            output::DepStatus::UpToDate { latest }
                        }
                        Some(Err(e)) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                        None => output::DepStatus::UpToDate { latest: None },
                    };
                    repo_report.files.push(output::FileReport {
                        path: sfmt_path.clone(),
                        manager: "scalafmt".into(),
                        deps: vec![output::DepReport {
                            name: "scalafmt".into(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%sfmt_path, ".scalafmt.conf not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%sfmt_path, %err, "failed to fetch .scalafmt.conf");
                had_error = true;
            }
        }
    }

    // ── Mix (mix.exs) ─────────────────────────────────────────────────────────
    for mix_file_path in manager_files(&detected, "mix") {
        match client.get_raw_file(owner, repo, &mix_file_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::mix::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %mix_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted mix deps"
                );
                let dep_inputs: Vec<renovate_core::datasources::hex::HexDepInput> = actionable
                    .iter()
                    .map(|d| renovate_core::datasources::hex::HexDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = renovate_core::datasources::hex::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    renovate_core::datasources::hex::HEX_API,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
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
                    file_deps.push(output::DepReport {
                        name: dep.name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: mix_file_path.clone(),
                    manager: "mix".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%mix_file_path, "mix.exs not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%mix_file_path, %err, "failed to fetch mix.exs");
                had_error = true;
            }
        }
    }

    // ── Gleam (gleam.toml) ────────────────────────────────────────────────────
    for gleam_path in manager_files(&detected, "gleam") {
        match client.get_raw_file(owner, repo, &gleam_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gleam::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gleam_path,
                    total = deps.len(), "extracted gleam deps"
                );
                let dep_inputs: Vec<renovate_core::datasources::hex::HexDepInput> = actionable
                    .iter()
                    .map(|d| renovate_core::datasources::hex::HexDepInput {
                        name: d.name.clone(),
                        current_value: d.version.clone(),
                    })
                    .collect();
                let updates = renovate_core::datasources::hex::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    renovate_core::datasources::hex::HEX_API,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_value.clone(),
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
                        output::DepReport {
                            name: dep.name.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: gleam_path.clone(),
                    manager: "gleam".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%gleam_path, "gleam.toml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gleam_path, %err, "failed to fetch gleam.toml");
                had_error = true;
            }
        }
    }

    // ── Swift Package Manager (Package.swift) ─────────────────────────────────
    for spm_file_path in manager_files(&detected, "swift") {
        match client.get_raw_file(owner, repo, &spm_file_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::spm::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !d.owner_repo.is_empty())
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %spm_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted swift package deps"
                );
                // GitHub packages → github_tags datasource.
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter(|d| {
                        matches!(
                            d.git_host,
                            Some(renovate_core::extractors::spm::GitHost::GitHub)
                        )
                    })
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.owner_repo.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    http,
                    &gh_inputs,
                    github_tags_datasource::GITHUB_API,
                    8,
                )
                .await;
                // GitLab packages → gitlab_tags datasource.
                let gl_inputs: Vec<renovate_core::datasources::gitlab_tags::GitlabTagsDepInput> =
                    actionable
                        .iter()
                        .filter(|d| {
                            matches!(
                                d.git_host,
                                Some(renovate_core::extractors::spm::GitHost::GitLab)
                            )
                        })
                        .map(
                            |d| renovate_core::datasources::gitlab_tags::GitlabTagsDepInput {
                                dep_name: d.owner_repo.clone(),
                                current_value: d.current_value.clone(),
                            },
                        )
                        .collect();
                let gl_updates = renovate_core::datasources::gitlab_tags::fetch_updates_concurrent(
                    http,
                    &gl_inputs,
                    renovate_core::datasources::gitlab_tags::GITLAB_API,
                    8,
                )
                .await;
                // Unified map: dep_name → (update_available, latest, error_msg).
                let mut spm_map: HashMap<String, (bool, Option<String>, Option<String>)> =
                    HashMap::new();
                for r in gh_updates {
                    match r.summary {
                        Ok(s) => {
                            spm_map.insert(r.dep_name, (s.update_available, s.latest, None));
                        }
                        Err(e) => {
                            spm_map.insert(r.dep_name, (false, None, Some(e.to_string())));
                        }
                    }
                }
                for r in gl_updates {
                    match r.summary {
                        Ok(s) => {
                            spm_map.insert(r.dep_name, (s.update_available, s.latest, None));
                        }
                        Err(e) => {
                            spm_map.insert(r.dep_name, (false, None, Some(e.to_string())));
                        }
                    }
                }
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in deps.iter().filter(|d| d.skip_reason.is_some()) {
                    file_deps.push(output::DepReport {
                        name: dep.owner_repo.clone(),
                        status: output::DepStatus::Skipped {
                            reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                .to_lowercase(),
                        },
                    });
                }
                for dep in &actionable {
                    let status = match spm_map.get(&dep.owner_repo) {
                        Some((true, Some(latest), _)) => output::DepStatus::UpdateAvailable {
                            current: dep.current_value.clone(),
                            latest: latest.clone(),
                        },
                        Some((false, latest, None)) => output::DepStatus::UpToDate {
                            latest: latest.clone(),
                        },
                        Some((_, _, Some(err_msg))) => output::DepStatus::LookupError {
                            message: err_msg.clone(),
                        },
                        _ => output::DepStatus::Skipped {
                            reason: "lookup-pending".into(),
                        },
                    };
                    file_deps.push(output::DepReport {
                        name: dep.owner_repo.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: spm_file_path.clone(),
                    manager: "swift".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%spm_file_path, "Package.swift not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%spm_file_path, %err, "failed to fetch Package.swift");
                had_error = true;
            }
        }
    }

    // ── Mint (Mintfile) ───────────────────────────────────────────────────────
    for mint_path in manager_files(&detected, "mint") {
        match client.get_raw_file(owner, repo, &mint_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::mint::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %mint_path,
                    total = deps.len(), "extracted mint deps"
                );
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = deps
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.repo.clone(),
                        current_value: d.version.clone(),
                    })
                    .collect();
                let updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.repo) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.version.clone(),
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
                        output::DepReport {
                            name: dep.repo.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: mint_path.clone(),
                    manager: "mint".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%mint_path, "Mintfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%mint_path, %err, "failed to fetch Mintfile");
                had_error = true;
            }
        }
    }

    // ── CocoaPods (Podfile) ───────────────────────────────────────────────────
    for podfile_path in manager_files(&detected, "cocoapods") {
        match client.get_raw_file(owner, repo, &podfile_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cocoapods::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %podfile_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted cocoapods deps"
                );
                let dep_inputs: Vec<renovate_core::datasources::cocoapods::PodDepInput> =
                    actionable
                        .iter()
                        .map(|d| renovate_core::datasources::cocoapods::PodDepInput {
                            name: d.name.clone(),
                            current_value: d.current_value.clone(),
                        })
                        .collect();
                let updates = renovate_core::datasources::cocoapods::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    renovate_core::datasources::cocoapods::TRUNK_API,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
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
                    file_deps.push(output::DepReport {
                        name: dep.name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: podfile_path.clone(),
                    manager: "cocoapods".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%podfile_path, "Podfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%podfile_path, %err, "failed to fetch Podfile");
                had_error = true;
            }
        }
    }

    // ── pre-commit (.pre-commit-config.yaml) ──────────────────────────────────
    for pc_path in manager_files(&detected, "pre-commit") {
        match client.get_raw_file(owner, repo, &pc_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::pre_commit::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %pc_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted pre-commit hook deps"
                );

                // Partition actionable by host.
                let gh_actionable: Vec<_> = actionable
                    .iter()
                    .filter(|d| {
                        d.git_host == Some(renovate_core::extractors::pre_commit::GitHost::GitHub)
                    })
                    .collect();
                let gl_actionable: Vec<_> = actionable
                    .iter()
                    .filter(|d| {
                        d.git_host == Some(renovate_core::extractors::pre_commit::GitHost::GitLab)
                    })
                    .collect();

                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = gh_actionable
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.dep_name.clone(),
                        current_value: d.current_value.trim_matches('\'').to_owned(),
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let mut update_map: HashMap<String, (bool, Option<String>, Option<String>)> =
                    HashMap::new();
                for r in gh_updates {
                    match r.summary {
                        Ok(s) => {
                            update_map.insert(r.dep_name, (s.update_available, s.latest, None));
                        }
                        Err(e) => {
                            update_map.insert(r.dep_name, (false, None, Some(e.to_string())));
                        }
                    }
                }

                let gl_inputs: Vec<renovate_core::datasources::gitlab_tags::GitlabTagsDepInput> =
                    gl_actionable
                        .iter()
                        .map(
                            |d| renovate_core::datasources::gitlab_tags::GitlabTagsDepInput {
                                dep_name: d.dep_name.clone(),
                                current_value: d.current_value.trim_matches('\'').to_owned(),
                            },
                        )
                        .collect();
                let gl_updates = renovate_core::datasources::gitlab_tags::fetch_updates_concurrent(
                    http,
                    &gl_inputs,
                    renovate_core::datasources::gitlab_tags::GITLAB_API,
                    8,
                )
                .await;
                for r in gl_updates {
                    match r.summary {
                        Ok(s) => {
                            update_map.insert(r.dep_name, (s.update_available, s.latest, None));
                        }
                        Err(e) => {
                            update_map.insert(r.dep_name, (false, None, Some(e.to_string())));
                        }
                    }
                }

                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in deps.iter().filter(|d| d.skip_reason.is_some()) {
                    file_deps.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status: output::DepStatus::Skipped {
                            reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                .to_lowercase(),
                        },
                    });
                }
                for dep in &actionable {
                    let status = match update_map.get(&dep.dep_name) {
                        Some((true, Some(latest), _)) => output::DepStatus::UpdateAvailable {
                            current: dep.current_value.trim_matches('\'').to_owned(),
                            latest: latest.clone(),
                        },
                        Some((false, latest, None)) => output::DepStatus::UpToDate {
                            latest: latest.clone(),
                        },
                        Some((_, _, Some(err_msg))) => output::DepStatus::LookupError {
                            message: err_msg.clone(),
                        },
                        _ => output::DepStatus::UpToDate { latest: None },
                    };
                    file_deps.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: pc_path.clone(),
                    manager: "pre-commit".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pc_path, ".pre-commit-config.yaml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pc_path, %err, "failed to fetch .pre-commit-config.yaml");
                had_error = true;
            }
        }
    }

    // ── Ansible Galaxy (requirements.yml) ────────────────────────────────────
    for ag_path in manager_files(&detected, "ansible-galaxy") {
        match client.get_raw_file(owner, repo, &ag_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ansible_galaxy::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %ag_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted ansible-galaxy role deps"
                );
                // Only GitHub-URL-sourced roles are actionable right now.
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter_map(|d| {
                        if let renovate_core::extractors::ansible_galaxy::AnsibleGalaxySource::GitHub { owner_repo } = &d.source {
                            Some(github_tags_datasource::GithubActionsDepInput {
                                dep_name: owner_repo.clone(),
                                current_value: d.current_value.clone(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let update_map: HashMap<String, (bool, Option<String>, Option<String>)> = {
                    let mut m = HashMap::new();
                    for r in gh_updates {
                        match r.summary {
                            Ok(s) => {
                                m.insert(r.dep_name, (s.update_available, s.latest, None));
                            }
                            Err(e) => {
                                m.insert(r.dep_name, (false, None, Some(e.to_string())));
                            }
                        }
                    }
                    m
                };
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        output::DepStatus::Skipped {
                            reason: "ignored".to_owned(),
                        }
                    } else {
                        let gh_key = if let renovate_core::extractors::ansible_galaxy::AnsibleGalaxySource::GitHub { owner_repo } = &dep.source {
                            Some(owner_repo.as_str())
                        } else { None };
                        match gh_key.and_then(|k| update_map.get(k)) {
                            Some((true, Some(latest), _)) => output::DepStatus::UpdateAvailable {
                                current: dep.current_value.clone(),
                                latest: latest.clone(),
                            },
                            Some((false, latest, None)) => output::DepStatus::UpToDate {
                                latest: latest.clone(),
                            },
                            Some((_, _, Some(e))) => {
                                output::DepStatus::LookupError { message: e.clone() }
                            }
                            _ => output::DepStatus::UpToDate { latest: None },
                        }
                    };
                    file_deps.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: ag_path.clone(),
                    manager: "ansible-galaxy".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%ag_path, "requirements.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ag_path, %err, "failed to fetch requirements.yml");
                had_error = true;
            }
        }
    }

    // ── asdf (.tool-versions) ─────────────────────────────────────────────────
    for asdf_path in manager_files(&detected, "asdf") {
        match client.get_raw_file(owner, repo, &asdf_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::asdf::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.tool_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %asdf_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted asdf tool versions"
                );

                // Partition by datasource type.
                use renovate_core::extractors::asdf::AsdfDatasource;
                let gh_tag_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter_map(|d| {
                        if let Some(AsdfDatasource::GithubTags { repo, tag_strip }) = &d.datasource
                        {
                            Some(github_tags_datasource::GithubActionsDepInput {
                                dep_name: format!("{}|{}", repo, tag_strip),
                                current_value: d.current_value.clone(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                let gh_rel_inputs: Vec<github_releases_datasource::GithubReleasesDepInput> =
                    actionable
                        .iter()
                        .filter_map(|d| {
                            if let Some(AsdfDatasource::GithubReleases { repo, tag_strip }) =
                                &d.datasource
                            {
                                // Prepend tag_strip to current_value so comparison works with v-prefixed tags.
                                let cv = format!("{}{}", tag_strip, d.current_value);
                                Some(github_releases_datasource::GithubReleasesDepInput {
                                    dep_name: format!("{}|{}", repo, tag_strip),
                                    current_value: cv,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();

                // Build lookup map: "repo|tag_strip" → (update_available, latest_tag, err_msg)
                let mut lookup_map: HashMap<String, (bool, Option<String>, Option<String>)> =
                    HashMap::new();

                // GitHub Tags lookups.
                {
                    let unique_repos: std::collections::HashSet<&str> =
                        gh_tag_inputs.iter().map(|i| i.dep_name.as_str()).collect();
                    for key in unique_repos {
                        let (repo_name, tag_strip) = key.split_once('|').unwrap_or((key, ""));
                        match renovate_core::datasources::github_tags::fetch_latest_tag(
                            repo_name,
                            &gh_http,
                            gh_api_base,
                        )
                        .await
                        {
                            Ok(Some(tag)) => {
                                let version = tag.trim_start_matches(tag_strip).to_owned();
                                lookup_map.insert(key.to_owned(), (false, Some(version), None));
                            }
                            Ok(None) => {
                                lookup_map.insert(key.to_owned(), (false, None, None));
                            }
                            Err(e) => {
                                lookup_map
                                    .insert(key.to_owned(), (false, None, Some(e.to_string())));
                            }
                        }
                    }
                }

                // GitHub Releases lookups.
                {
                    let unique_repos: std::collections::HashSet<&str> =
                        gh_rel_inputs.iter().map(|i| i.dep_name.as_str()).collect();
                    for key in unique_repos {
                        let (repo_name, tag_strip) = key.split_once('|').unwrap_or((key, ""));
                        match github_releases_datasource::fetch_latest_release(
                            repo_name,
                            &gh_http,
                            gh_api_base,
                        )
                        .await
                        {
                            Ok(Some(tag)) => {
                                let version = tag.trim_start_matches(tag_strip).to_owned();
                                lookup_map.insert(key.to_owned(), (false, Some(version), None));
                            }
                            Ok(None) => {
                                lookup_map.insert(key.to_owned(), (false, None, None));
                            }
                            Err(e) => {
                                lookup_map
                                    .insert(key.to_owned(), (false, None, Some(e.to_string())));
                            }
                        }
                    }
                }

                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if !repo_cfg.is_dep_ignored(&dep.tool_name) {
                        let lookup_key = match &dep.datasource {
                            Some(AsdfDatasource::GithubTags { repo, tag_strip })
                            | Some(AsdfDatasource::GithubReleases { repo, tag_strip }) => {
                                Some(format!("{}|{}", repo, tag_strip))
                            }
                            None => None,
                        };
                        match lookup_key.as_deref().and_then(|k| lookup_map.get(k)) {
                            Some((_, Some(latest_ver), None)) => {
                                let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                    &dep.current_value,
                                    Some(latest_ver.as_str()),
                                );
                                if s.update_available {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.current_value.clone(),
                                        latest: latest_ver.clone(),
                                    }
                                } else {
                                    output::DepStatus::UpToDate {
                                        latest: Some(latest_ver.clone()),
                                    }
                                }
                            }
                            Some((_, None, None)) => output::DepStatus::UpToDate { latest: None },
                            Some((_, _, Some(err_msg))) => output::DepStatus::LookupError {
                                message: err_msg.clone(),
                            },
                            None => output::DepStatus::UpToDate { latest: None },
                        }
                    } else {
                        output::DepStatus::Skipped {
                            reason: "ignored".to_owned(),
                        }
                    };
                    file_deps.push(output::DepReport {
                        name: dep.tool_name.clone(),
                        status,
                    });
                }

                repo_report.files.push(output::FileReport {
                    path: asdf_path.clone(),
                    manager: "asdf".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%asdf_path, ".tool-versions not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%asdf_path, %err, "failed to fetch .tool-versions");
                had_error = true;
            }
        }
    }

    // ── mise (mise.toml / .mise.toml) ────────────────────────────────────────
    for mise_path in manager_files(&detected, "mise") {
        match client.get_raw_file(owner, repo, &mise_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::mise::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %mise_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted mise tool versions"
                );
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter_map(|d| {
                        let ds = d.datasource.as_ref()?;
                        let (repo_str, _) = match ds {
                            renovate_core::extractors::asdf::AsdfDatasource::GithubTags {
                                repo,
                                ..
                            } => (repo, false),
                            renovate_core::extractors::asdf::AsdfDatasource::GithubReleases {
                                repo,
                                ..
                            } => (repo, true),
                        };
                        Some(github_tags_datasource::GithubActionsDepInput {
                            dep_name: (*repo_str).to_owned(),
                            current_value: d.current_value.clone(),
                        })
                    })
                    .collect();
                let updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if let Some(ds) = &dep.datasource {
                        let (repo_str, tag_strip) = match ds {
                            renovate_core::extractors::asdf::AsdfDatasource::GithubTags {
                                repo,
                                tag_strip,
                            } => (*repo, *tag_strip),
                            renovate_core::extractors::asdf::AsdfDatasource::GithubReleases {
                                repo,
                                tag_strip,
                            } => (*repo, *tag_strip),
                        };
                        match update_map.get(repo_str) {
                            Some(Ok(s)) if s.update_available => {
                                let latest = s
                                    .latest
                                    .as_deref()
                                    .map(|l| l.strip_prefix(tag_strip).unwrap_or(l).to_owned())
                                    .unwrap_or_default();
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest,
                                }
                            }
                            Some(Ok(s)) => {
                                let latest = s
                                    .latest
                                    .as_deref()
                                    .map(|l| l.strip_prefix(tag_strip).unwrap_or(l).to_owned());
                                output::DepStatus::UpToDate { latest }
                            }
                            Some(Err(e)) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                            None => output::DepStatus::UpToDate { latest: None },
                        }
                    } else {
                        output::DepStatus::UpToDate { latest: None }
                    };
                    file_deps.push(output::DepReport {
                        name: dep.tool_name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: mise_path.clone(),
                    manager: "mise".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%mise_path, "mise.toml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%mise_path, %err, "failed to fetch mise.toml");
                had_error = true;
            }
        }
    }

    // ── Bazel Module (MODULE.bazel) ───────────────────────────────────────────
    for bm_path in manager_files(&detected, "bazel-module") {
        match client.get_raw_file(owner, repo, &bm_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::bazel_module::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %bm_path,
                    total = deps.len(),
                    "extracted bazel module deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.name) {
                        continue;
                    }
                    let status = match renovate_core::datasources::bazel::fetch_latest(
                        http,
                        &dep.name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value,
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: bm_path.clone(),
                        manager: "bazel-module".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%bm_path, "MODULE.bazel not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bm_path, %err, "failed to fetch MODULE.bazel");
                had_error = true;
            }
        }
    }

    // ── Azure Bicep (*.bicep) ─────────────────────────────────────────────────
    for bicep_path in manager_files(&detected, "bicep") {
        match client.get_raw_file(owner, repo, &bicep_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::bicep::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %bicep_path,
                    total = deps.len(),
                    "extracted bicep resource type deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        continue;
                    }
                    let status = match renovate_core::datasources::azure_bicep::fetch_latest(
                        http,
                        &dep.dep_name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value,
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: bicep_path.clone(),
                        manager: "bicep".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%bicep_path, "bicep file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bicep_path, %err, "failed to fetch bicep file");
                had_error = true;
            }
        }
    }

    // ── Version files (.terraform-version, .go-version, .bun-version, etc.) ──
    for manager_name in [
        "terraform-version",
        "terragrunt-version",
        "go-version",
        "python-version",
        "node-version",
        "nvmrc",
        "bun-version",
        "bazelisk",
        "ruby-version",
    ] {
        for vf_path in manager_files(&detected, manager_name) {
            match client.get_raw_file(owner, repo, &vf_path).await {
                Ok(Some(raw)) => {
                    let Some(dep) = renovate_core::extractors::version_file::extract(
                        &raw.content,
                        manager_name,
                    ) else {
                        continue;
                    };
                    tracing::debug!(
                        repo = %repo_slug, file = %vf_path, tool = dep.tool,
                        version = %dep.current_value, "extracted version file dep"
                    );

                    use renovate_core::extractors::asdf::AsdfDatasource;
                    let lookup_key = match &dep.datasource {
                        AsdfDatasource::GithubTags { repo, tag_strip } => {
                            format!("{}|{}", repo, tag_strip)
                        }
                        AsdfDatasource::GithubReleases { repo, tag_strip } => {
                            format!("{}|{}", repo, tag_strip)
                        }
                    };
                    let (repo_name, tag_strip) =
                        lookup_key.split_once('|').unwrap_or((&lookup_key, ""));

                    let tag_result = match &dep.datasource {
                        AsdfDatasource::GithubTags { .. } => {
                            renovate_core::datasources::github_tags::fetch_latest_tag(
                                repo_name,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            .map_err(|e| e.to_string())
                        }
                        AsdfDatasource::GithubReleases { .. } => {
                            github_releases_datasource::fetch_latest_release(
                                repo_name,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            .map_err(|e| e.to_string())
                        }
                    };

                    let status = match tag_result {
                        Ok(Some(tag)) => {
                            let stripped = tag.trim_start_matches(tag_strip);
                            // Ruby tags use underscores: `3_3_0` → `3.3.0`
                            let latest_ver = if manager_name == "ruby-version" {
                                stripped.replace('_', ".")
                            } else {
                                stripped.to_owned()
                            };
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    &dep.current_value,
                                    Some(latest_ver.as_str()),
                                );
                            if s.update_available {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest: latest_ver,
                                }
                            } else {
                                output::DepStatus::UpToDate {
                                    latest: Some(latest_ver),
                                }
                            }
                        }
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(msg) => output::DepStatus::LookupError { message: msg },
                    };

                    repo_report.files.push(output::FileReport {
                        path: vf_path.clone(),
                        manager: manager_name.to_owned(),
                        deps: vec![output::DepReport {
                            name: dep.tool.to_owned(),
                            status,
                        }],
                    });
                }
                Ok(None) => {
                    tracing::warn!(repo=%repo_slug, file=%vf_path, "version file not found")
                }
                Err(err) => {
                    tracing::error!(repo=%repo_slug, file=%vf_path, %err, "failed to fetch version file");
                    had_error = true;
                }
            }
        }
    }

    // ── Travis CI (.travis.yml) ───────────────────────────────────────────────
    for travis_path in manager_files(&detected, "travis") {
        match client.get_raw_file(owner, repo, &travis_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::travis::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %travis_path,
                    total = deps.len(), "extracted travis node_js versions"
                );
                // Reuse the Node.js GitHub Releases lookup (same as nvmrc/node-version).
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = deps
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: "nodejs/node".to_owned(),
                        current_value: d.version.clone(),
                    })
                    .collect();
                let updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    4,
                )
                .await;
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .zip(updates.iter())
                    .map(|(dep, result)| {
                        let status = match &result.summary {
                            Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest: s
                                    .latest
                                    .as_deref()
                                    .map(|l| l.strip_prefix('v').unwrap_or(l).to_owned())
                                    .unwrap_or_default(),
                            },
                            Ok(s) => output::DepStatus::UpToDate {
                                latest: s
                                    .latest
                                    .as_deref()
                                    .map(|l| l.strip_prefix('v').unwrap_or(l).to_owned()),
                            },
                            Err(e) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                        };
                        output::DepReport {
                            name: format!("node@{}", dep.version),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: travis_path.clone(),
                    manager: "travis".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%travis_path, ".travis.yml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%travis_path, %err, "failed to fetch .travis.yml");
                had_error = true;
            }
        }
    }

    // ── GitLab CI (.gitlab-ci.yml) ────────────────────────────────────────────
    for glci_path in manager_files(&detected, "gitlabci") {
        match client.get_raw_file(owner, repo, &glci_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gitlabci::extract(&raw.content);
                let docker_deps: Vec<_> = deps.iter().map(|d| d.dep.clone()).collect();
                tracing::debug!(repo = %repo_slug, file = %glci_path, total = deps.len(), "extracted gitlab-ci images");
                repo_report.files.push(output::FileReport {
                    path: glci_path.clone(),
                    manager: "gitlabci".into(),
                    deps: docker_hub_reports(http, &docker_deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%glci_path, ".gitlab-ci.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%glci_path, %err, "failed to fetch .gitlab-ci.yml");
                had_error = true;
            }
        }
    }

    // ── GitLab CI includes (.gitlab-ci.yml include: project refs) ────────────
    for glci_inc_path in manager_files(&detected, "gitlabci-include") {
        match client.get_raw_file(owner, repo, &glci_inc_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gitlabci_include::extract(&raw.content);
                if !deps.is_empty() {
                    tracing::debug!(
                        repo = %repo_slug, file = %glci_inc_path,
                        total = deps.len(), "extracted gitlab-ci include refs"
                    );
                    let gl_inputs: Vec<
                        renovate_core::datasources::gitlab_tags::GitlabTagsDepInput,
                    > = deps
                        .iter()
                        .map(
                            |d| renovate_core::datasources::gitlab_tags::GitlabTagsDepInput {
                                dep_name: d.project.clone(),
                                current_value: d.ref_value.clone(),
                            },
                        )
                        .collect();
                    let gl_updates =
                        renovate_core::datasources::gitlab_tags::fetch_updates_concurrent(
                            http,
                            &gl_inputs,
                            renovate_core::datasources::gitlab_tags::GITLAB_API,
                            8,
                        )
                        .await;
                    let update_map: HashMap<_, _> = gl_updates
                        .into_iter()
                        .map(|r| (r.dep_name, r.summary))
                        .collect();
                    let file_deps: Vec<output::DepReport> = deps
                        .iter()
                        .map(|dep| {
                            let status = match update_map.get(&dep.project) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.ref_value.clone(),
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
                            output::DepReport {
                                name: dep.project.clone(),
                                status,
                            }
                        })
                        .collect();
                    repo_report.files.push(output::FileReport {
                        path: glci_inc_path.clone(),
                        manager: "gitlabci-include".into(),
                        deps: file_deps,
                    });
                }
            }
            Ok(None) => {}
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%glci_inc_path, %err, "failed to fetch .gitlab-ci.yml (includes)");
                had_error = true;
            }
        }
    }

    // ── CircleCI (.circleci/config.yml) ──────────────────────────────────────
    for cci_path in manager_files(&detected, "circleci") {
        match client.get_raw_file(owner, repo, &cci_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::circleci::extract(&raw.content);
                let docker_deps: Vec<_> = deps.iter().map(|d| d.dep.clone()).collect();
                let orb_deps = renovate_core::extractors::circleci::extract_orbs(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %cci_path,
                    docker = docker_deps.len(), orbs = orb_deps.len(),
                    "extracted circleci deps"
                );
                let mut all_deps = docker_hub_reports(http, &docker_deps).await;
                if !orb_deps.is_empty() {
                    let orb_inputs: Vec<renovate_core::datasources::orb::OrbDepInput> = orb_deps
                        .iter()
                        .map(|o| renovate_core::datasources::orb::OrbDepInput {
                            package_name: o.package_name.clone(),
                            current_value: o.version.clone(),
                        })
                        .collect();
                    let orb_updates = renovate_core::datasources::orb::fetch_updates_concurrent(
                        http,
                        &orb_inputs,
                        6,
                    )
                    .await;
                    let orb_map: HashMap<_, _> = orb_updates
                        .into_iter()
                        .map(|r| (r.package_name, r.summary))
                        .collect();
                    for orb in &orb_deps {
                        let status = match orb_map.get(&orb.package_name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: orb.version.clone(),
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
                        all_deps.push(output::DepReport {
                            name: orb.package_name.clone(),
                            status,
                        });
                    }
                }
                repo_report.files.push(output::FileReport {
                    path: cci_path.clone(),
                    manager: "circleci".into(),
                    deps: all_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%cci_path, "circleci config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cci_path, %err, "failed to fetch circleci config");
                had_error = true;
            }
        }
    }

    // ── Buildkite pipeline YAML ───────────────────────────────────────────────
    for bk_path in manager_files(&detected, "buildkite") {
        match client.get_raw_file(owner, repo, &bk_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::buildkite::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %bk_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted buildkite plugins"
                );

                // Group by unique GitHub repo for batched lookups.
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter_map(|d| {
                        if let Some(
                            renovate_core::extractors::buildkite::BuildkiteDatasource::GithubTags {
                                repo: gr,
                            },
                        ) = &d.datasource
                        {
                            Some(github_tags_datasource::GithubActionsDepInput {
                                dep_name: gr.clone(),
                                current_value: d.current_value.clone(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;

                let update_map: HashMap<String, (bool, Option<String>, Option<String>)> = {
                    let mut m = HashMap::new();
                    for r in gh_updates {
                        match r.summary {
                            Ok(s) => {
                                m.insert(r.dep_name, (s.update_available, s.latest, None));
                            }
                            Err(e) => {
                                m.insert(r.dep_name, (false, None, Some(e.to_string())));
                            }
                        }
                    }
                    m
                };

                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if !repo_cfg.is_dep_ignored(&dep.dep_name) {
                        let gh_repo = dep.datasource.as_ref().map(
                            |renovate_core::extractors::buildkite::BuildkiteDatasource::GithubTags { repo: gr }| {
                                gr.as_str()
                            },
                        );
                        match gh_repo.and_then(|r| update_map.get(r)) {
                            Some((true, Some(latest), _)) => output::DepStatus::UpdateAvailable {
                                current: dep.current_value.clone(),
                                latest: latest.clone(),
                            },
                            Some((false, latest, None)) => output::DepStatus::UpToDate {
                                latest: latest.clone(),
                            },
                            Some((_, _, Some(err_msg))) => output::DepStatus::LookupError {
                                message: err_msg.clone(),
                            },
                            _ => output::DepStatus::UpToDate { latest: None },
                        }
                    } else {
                        output::DepStatus::Skipped {
                            reason: "ignored".to_owned(),
                        }
                    };
                    file_deps.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: bk_path.clone(),
                    manager: "buildkite".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%bk_path, "buildkite pipeline not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bk_path, %err, "failed to fetch buildkite pipeline");
                had_error = true;
            }
        }
    }

    // ── Cloud Build (cloudbuild.yaml) ─────────────────────────────────────────
    for cb_path in manager_files(&detected, "cloudbuild") {
        match client.get_raw_file(owner, repo, &cb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cloudbuild::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %cb_path, total = deps.len(), "extracted cloudbuild images");
                repo_report.files.push(output::FileReport {
                    path: cb_path.clone(),
                    manager: "cloudbuild".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cb_path, "cloudbuild.yaml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cb_path, %err, "failed to fetch cloudbuild.yaml");
                had_error = true;
            }
        }
    }

    // ── Azure Pipelines ───────────────────────────────────────────────────────
    for az_path in manager_files(&detected, "azure-pipelines") {
        match client.get_raw_file(owner, repo, &az_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::azure_pipelines::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %az_path, total = deps.len(), "extracted azure-pipelines deps");
                // Separate containers (docker lookup) from tasks (pending datasource).
                let container_images: Vec<_> = deps
                    .iter()
                    .filter_map(|d| match d {
                        renovate_core::extractors::azure_pipelines::AzPipelinesDep::Container(
                            c,
                        ) => Some(c.clone()),
                        renovate_core::extractors::azure_pipelines::AzPipelinesDep::Task(_) => None,
                    })
                    .collect();
                let mut file_deps = docker_hub_reports(http, &container_images).await;
                for dep in &deps {
                    if let renovate_core::extractors::azure_pipelines::AzPipelinesDep::Task(t) = dep
                    {
                        let status = match renovate_core::datasources::azure_pipelines_tasks::fetch_latest(
                            http,
                            &t.name,
                            &t.version,
                        )
                        .await
                        {
                            Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                current: t.version.clone(),
                                latest: s.latest.unwrap_or_default(),
                            },
                            Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                            Err(renovate_core::datasources::azure_pipelines_tasks::AzureTasksError::NotFound(_)) => {
                                output::DepStatus::Skipped {
                                    reason: "task not found in registry".into(),
                                }
                            }
                            Err(e) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                        };
                        file_deps.push(output::DepReport {
                            name: format!("{}@{}", t.name, t.version),
                            status,
                        });
                    }
                }
                repo_report.files.push(output::FileReport {
                    path: az_path.clone(),
                    manager: "azure-pipelines".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%az_path, "azure-pipelines file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%az_path, %err, "failed to fetch azure-pipelines file");
                had_error = true;
            }
        }
    }

    // ── Bitbucket Pipelines (*-pipelines.yml) ────────────────────────────────
    for bb_path in manager_files(&detected, "bitbucket-pipelines") {
        match client.get_raw_file(owner, repo, &bb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::bitbucket_pipelines::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %bb_path, total = deps.len(), "extracted bitbucket-pipelines images");
                repo_report.files.push(output::FileReport {
                    path: bb_path.clone(),
                    manager: "bitbucket-pipelines".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%bb_path, "bitbucket-pipelines file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bb_path, %err, "failed to fetch bitbucket-pipelines file");
                had_error = true;
            }
        }
    }

    // ── Drone CI (.drone.yml) ─────────────────────────────────────────────────
    for drone_path in manager_files(&detected, "droneci") {
        match client.get_raw_file(owner, repo, &drone_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::droneci::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %drone_path, total = deps.len(), "extracted droneci images");
                repo_report.files.push(output::FileReport {
                    path: drone_path.clone(),
                    manager: "droneci".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%drone_path, ".drone.yml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%drone_path, %err, "failed to fetch .drone.yml");
                had_error = true;
            }
        }
    }

    // ── Devbox (devbox.json) ──────────────────────────────────────────────────
    for db_path in manager_files(&detected, "devbox") {
        match client.get_raw_file(owner, repo, &db_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::devbox::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %db_path,
                    total = deps.len(), "extracted devbox deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = match renovate_core::datasources::devbox::fetch_latest(
                        http,
                        &dep.name,
                        &dep.version,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_version,
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    file_deps.push(output::DepReport {
                        name: dep.name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: db_path.clone(),
                    manager: "devbox".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%db_path, "devbox.json not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%db_path, %err, "failed to fetch devbox.json");
                had_error = true;
            }
        }
    }

    // ── Dev Container (devcontainer.json) ────────────────────────────────────
    for dc_path in manager_files(&detected, "devcontainer") {
        match client.get_raw_file(owner, repo, &dc_path).await {
            Ok(Some(raw)) => {
                let extracted = renovate_core::extractors::devcontainer::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %dc_path,
                    docker = extracted.docker_deps.len(),
                    version = extracted.version_deps.len(),
                    "extracted devcontainer deps"
                );
                let mut dep_reports = docker_hub_reports(http, &extracted.docker_deps).await;

                for vdep in &extracted.version_deps {
                    use renovate_core::extractors::asdf::AsdfDatasource;
                    let lookup_key = match &vdep.datasource {
                        AsdfDatasource::GithubTags { repo, tag_strip } => {
                            format!("{}|{}", repo, tag_strip)
                        }
                        AsdfDatasource::GithubReleases { repo, tag_strip } => {
                            format!("{}|{}", repo, tag_strip)
                        }
                    };
                    let (ds_repo, tag_strip) =
                        lookup_key.split_once('|').unwrap_or((&lookup_key, ""));
                    let tag_result = match &vdep.datasource {
                        AsdfDatasource::GithubTags { .. } => {
                            renovate_core::datasources::github_tags::fetch_latest_tag(
                                ds_repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            .map_err(|e| e.to_string())
                        }
                        AsdfDatasource::GithubReleases { .. } => {
                            github_releases_datasource::fetch_latest_release(
                                ds_repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            .map_err(|e| e.to_string())
                        }
                    };
                    let status = match tag_result {
                        Ok(Some(tag)) => {
                            let stripped = tag.trim_start_matches(tag_strip);
                            let latest_ver = if vdep.tool == "ruby" {
                                stripped.replace('_', ".")
                            } else {
                                stripped.to_owned()
                            };
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    &vdep.current_value,
                                    Some(latest_ver.as_str()),
                                );
                            if s.update_available {
                                output::DepStatus::UpdateAvailable {
                                    current: vdep.current_value.clone(),
                                    latest: latest_ver,
                                }
                            } else {
                                output::DepStatus::UpToDate {
                                    latest: Some(latest_ver),
                                }
                            }
                        }
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError { message: e },
                    };
                    dep_reports.push(output::DepReport {
                        name: vdep.tool.to_owned(),
                        status,
                    });
                }

                repo_report.files.push(output::FileReport {
                    path: dc_path.clone(),
                    manager: "devcontainer".into(),
                    deps: dep_reports,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%dc_path, "devcontainer.json not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%dc_path, %err, "failed to fetch devcontainer.json");
                had_error = true;
            }
        }
    }

    // ── Crow CI (.crow/*.yml) ─────────────────────────────────────────────────
    for crow_path in manager_files(&detected, "crow") {
        match client.get_raw_file(owner, repo, &crow_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::crow::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %crow_path, total = deps.len(), "extracted crow-ci images");
                repo_report.files.push(output::FileReport {
                    path: crow_path.clone(),
                    manager: "crow".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%crow_path, "crow CI file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%crow_path, %err, "failed to fetch crow CI file");
                had_error = true;
            }
        }
    }

    // ── Vela CI (.vela.yml) ───────────────────────────────────────────────────
    for vela_path in manager_files(&detected, "velaci") {
        match client.get_raw_file(owner, repo, &vela_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::velaci::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %vela_path, total = deps.len(), "extracted vela-ci images");
                repo_report.files.push(output::FileReport {
                    path: vela_path.clone(),
                    manager: "velaci".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%vela_path, ".vela.yml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%vela_path, %err, "failed to fetch .vela.yml");
                had_error = true;
            }
        }
    }

    // ── Quadlet (.container / .image / .volume) ───────────────────────────────
    for qlet_path in manager_files(&detected, "quadlet") {
        match client.get_raw_file(owner, repo, &qlet_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::quadlet::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %qlet_path, total = deps.len(), "extracted quadlet images");
                repo_report.files.push(output::FileReport {
                    path: qlet_path.clone(),
                    manager: "quadlet".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%qlet_path, "quadlet file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%qlet_path, %err, "failed to fetch quadlet file");
                had_error = true;
            }
        }
    }

    // ── Woodpecker CI (.woodpecker.yml / .woodpecker/*.yml) ──────────────────
    for wp_path in manager_files(&detected, "woodpecker") {
        match client.get_raw_file(owner, repo, &wp_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::woodpecker::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %wp_path, total = deps.len(), "extracted woodpecker images");
                repo_report.files.push(output::FileReport {
                    path: wp_path.clone(),
                    manager: "woodpecker".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%wp_path, "woodpecker config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%wp_path, %err, "failed to fetch woodpecker config");
                had_error = true;
            }
        }
    }

    // ── FluxCD system manifest (gotk-components.yaml) ────────────────────────
    for flux_path in manager_files(&detected, "flux") {
        match client.get_raw_file(owner, repo, &flux_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::flux::extract(&raw.content) {
                    tracing::debug!(
                        repo = %repo_slug, file = %flux_path,
                        version = %dep.version, "extracted flux version"
                    );
                    let status = match github_releases_datasource::fetch_latest_release(
                        renovate_core::extractors::flux::FLUX2_REPO,
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    {
                        Ok(Some(latest)) if latest != dep.version => {
                            output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest,
                            }
                        }
                        Ok(Some(latest)) => output::DepStatus::UpToDate {
                            latest: Some(latest),
                        },
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    repo_report.files.push(output::FileReport {
                        path: flux_path.clone(),
                        manager: "flux".into(),
                        deps: vec![output::DepReport {
                            name: renovate_core::extractors::flux::FLUX2_REPO.to_owned(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%flux_path, "gotk-components.yaml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%flux_path, %err, "failed to fetch gotk-components.yaml");
                had_error = true;
            }
        }
    }

    // ── Nix flakes (flake.nix / flake.lock) ──────────────────────────────────
    for flake_path in manager_files(&detected, "nix") {
        // flake.nix is the trigger file; actual data lives in sibling flake.lock.
        let lock_path = {
            let p = std::path::Path::new(&flake_path);
            let dir = p.parent().map(|d| d.to_str().unwrap_or("")).unwrap_or("");
            if dir.is_empty() {
                "flake.lock".to_owned()
            } else {
                format!("{dir}/flake.lock")
            }
        };
        match client.get_raw_file(owner, repo, &lock_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::nix::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.input_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %flake_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted nix flake inputs"
                );
                // Build GitHub Tags lookups for github-type inputs.
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter(|d| {
                        matches!(
                            d.input_type,
                            renovate_core::extractors::nix::FlakeInputType::Github
                        )
                    })
                    .filter_map(|d| {
                        // package_name is "https://github.com/owner/repo"
                        let pkg = d.package_name.as_deref()?;
                        let repo_path = pkg
                            .strip_prefix("https://github.com/")
                            .unwrap_or(pkg)
                            .to_owned();
                        Some(github_tags_datasource::GithubActionsDepInput {
                            dep_name: repo_path,
                            current_value: d.current_ref.clone().unwrap_or_default(),
                        })
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let gh_map: HashMap<String, (bool, Option<String>, Option<String>)> = {
                    let mut m = HashMap::new();
                    for r in gh_updates {
                        match r.summary {
                            Ok(s) => {
                                m.insert(r.dep_name, (s.update_available, s.latest, None));
                            }
                            Err(e) => {
                                m.insert(r.dep_name, (false, None, Some(e.to_string())));
                            }
                        }
                    }
                    m
                };
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = if let Some(reason) = &dep.skip_reason {
                            output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            }
                        } else if matches!(
                            dep.input_type,
                            renovate_core::extractors::nix::FlakeInputType::Github
                        ) {
                            let pkg = dep
                                .package_name
                                .as_deref()
                                .unwrap_or("")
                                .strip_prefix("https://github.com/")
                                .unwrap_or("")
                                .to_owned();
                            match gh_map.get(&pkg) {
                                Some((true, Some(latest), _)) => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.locked_rev.clone(),
                                        latest: latest.clone(),
                                    }
                                }
                                Some((_, latest, None)) => output::DepStatus::UpToDate {
                                    latest: latest.clone(),
                                },
                                Some((_, _, Some(err))) => output::DepStatus::LookupError {
                                    message: err.clone(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            }
                        } else {
                            output::DepStatus::Skipped {
                                reason: "non-github flake input (datasource pending)".into(),
                            }
                        };
                        output::DepReport {
                            name: dep.input_name.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: flake_path.clone(),
                    manager: "nix".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::debug!(repo=%repo_slug, file=%flake_path, "flake.lock not found (no lock file)")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%flake_path, %err, "failed to fetch flake.lock");
                had_error = true;
            }
        }
    }

    // ── SBT (build.sbt / project/*.scala / project/build.properties) ────────
    for sbt_path in manager_files(&detected, "sbt") {
        match client.get_raw_file(owner, repo, &sbt_path).await {
            Ok(Some(raw)) => {
                let deps = if sbt_path.ends_with("build.properties") {
                    renovate_core::extractors::sbt::extract_build_properties(&raw.content)
                        .map(|d| vec![d])
                        .unwrap_or_default()
                } else {
                    renovate_core::extractors::sbt::extract(&raw.content)
                };
                tracing::debug!(
                    repo = %repo_slug, file = %sbt_path,
                    total = deps.len(), "extracted sbt deps"
                );
                let dep_inputs: Vec<maven_datasource::MavenDepInput> = deps
                    .iter()
                    .map(|d| maven_datasource::MavenDepInput {
                        dep_name: d.dep_name(),
                        current_version: d.current_value.clone(),
                    })
                    .collect();
                let updates =
                    maven_datasource::fetch_updates_concurrent(http, &dep_inputs, 10).await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let dn = dep.dep_name();
                        let status = match update_map.get(&dn) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
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
                        output::DepReport { name: dn, status }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: sbt_path.clone(),
                    manager: "sbt".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%sbt_path, "sbt file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%sbt_path, %err, "failed to fetch sbt file");
                had_error = true;
            }
        }
    }

    // ── Meteor (package.js Npm.depends) ──────────────────────────────────────
    for meteor_path in manager_files(&detected, "meteor") {
        match client.get_raw_file(owner, repo, &meteor_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::meteor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored(&d.name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %meteor_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted meteor npm deps"
                );
                let npm_inputs: Vec<npm_datasource::NpmDepInput> = actionable
                    .iter()
                    .map(|d| npm_datasource::NpmDepInput {
                        dep_name: d.name.clone(),
                        constraint: d.current_value.clone(),
                    })
                    .collect();
                let npm_updates = npm_datasource::fetch_updates_concurrent(
                    http,
                    &npm_inputs,
                    npm_datasource::NPM_REGISTRY,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = npm_updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
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
                        output::DepReport {
                            name: dep.name.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: meteor_path.clone(),
                    manager: "meteor".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%meteor_path, "meteor package.js not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%meteor_path, %err, "failed to fetch meteor package.js");
                had_error = true;
            }
        }
    }

    // ── Cake build scripts (.cake) ────────────────────────────────────────────
    for cake_path in manager_files(&detected, "cake") {
        match client.get_raw_file(owner, repo, &cake_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cake::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        !d.current_value.is_empty() && !repo_cfg.is_dep_ignored(&d.package_name)
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %cake_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted cake deps"
                );
                let nuget_inputs: Vec<nuget_datasource::NuGetDepInput> = actionable
                    .iter()
                    .map(|d| nuget_datasource::NuGetDepInput {
                        package_id: d.package_name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let nuget_updates = nuget_datasource::fetch_updates_concurrent(
                    http,
                    &nuget_inputs,
                    nuget_datasource::NUGET_API,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = nuget_updates
                    .into_iter()
                    .map(|r| (r.package_id, r.summary))
                    .collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = if dep.current_value.is_empty() {
                            output::DepStatus::Skipped {
                                reason: "no-version".into(),
                            }
                        } else {
                            match update_map.get(&dep.package_name) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.current_value.clone(),
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
                            }
                        };
                        output::DepReport {
                            name: dep.package_name.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: cake_path.clone(),
                    manager: "cake".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cake_path, "cake script not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cake_path, %err, "failed to fetch cake script");
                had_error = true;
            }
        }
    }

    // ── Conan (conanfile.txt / conanfile.py) ─────────────────────────────────
    for conan_path in manager_files(&detected, "conan") {
        match client.get_raw_file(owner, repo, &conan_path).await {
            Ok(Some(raw)) => {
                let deps = if conan_path.ends_with(".py") {
                    renovate_core::extractors::conan::extract_py(&raw.content)
                } else {
                    renovate_core::extractors::conan::extract_txt(&raw.content)
                };
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %conan_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted conan deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        file_deps.push(output::DepReport {
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.name) {
                        continue;
                    }
                    let status = match renovate_core::datasources::conan::fetch_latest(
                        &gh_http,
                        &dep.name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: dep.current_value.clone(),
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(renovate_core::datasources::conan::ConanError::NotFound(_)) => {
                            output::DepStatus::Skipped {
                                reason: "package not found in conan-center-index".into(),
                            }
                        }
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    file_deps.push(output::DepReport {
                        name: dep.name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: conan_path.clone(),
                    manager: "conan".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%conan_path, "conanfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%conan_path, %err, "failed to fetch conanfile");
                had_error = true;
            }
        }
    }

    // ── Haskell Cabal (*.cabal) ───────────────────────────────────────────────
    for cabal_path in manager_files(&detected, "haskell-cabal") {
        match client.get_raw_file(owner, repo, &cabal_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cabal::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored(&d.package_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %cabal_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted cabal deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored(&dep.package_name) {
                        continue;
                    }
                    let status = match renovate_core::datasources::hackage::fetch_latest(
                        http,
                        &dep.package_name,
                    )
                    .await
                    {
                        Ok(s) => {
                            if let Some(ref l) = s.latest {
                                // Compare latest against the constraint if it's a plain version.
                                let current_ver =
                                    dep.current_value.trim_start_matches("==").trim().to_owned();
                                if !current_ver.is_empty()
                                    && !current_ver
                                        .contains(|c: char| c == '<' || c == '>' || c == '&')
                                    && l != &current_ver
                                {
                                    output::DepStatus::UpdateAvailable {
                                        current: current_ver,
                                        latest: l.clone(),
                                    }
                                } else {
                                    output::DepStatus::UpToDate {
                                        latest: s.latest.clone(),
                                    }
                                }
                            } else {
                                output::DepStatus::UpToDate { latest: None }
                            }
                        }
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    file_deps.push(output::DepReport {
                        name: dep.package_name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: cabal_path.clone(),
                    manager: "haskell-cabal".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cabal_path, "cabal file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cabal_path, %err, "failed to fetch cabal file");
                had_error = true;
            }
        }
    }

    // ── FVM Flutter Version Manager (.fvmrc / .fvm/fvm_config.json) ────────
    for fvm_path in manager_files(&detected, "fvm") {
        match client.get_raw_file(owner, repo, &fvm_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::fvm::extract(&raw.content) {
                    tracing::debug!(
                        repo = %repo_slug, file = %fvm_path,
                        version = %dep.version, "extracted fvm flutter version"
                    );
                    let status = match github_tags_datasource::fetch_latest_tag(
                        "flutter/flutter",
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    {
                        Ok(Some(latest)) if latest != dep.version => {
                            output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest,
                            }
                        }
                        Ok(Some(latest)) => output::DepStatus::UpToDate {
                            latest: Some(latest),
                        },
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    repo_report.files.push(output::FileReport {
                        path: fvm_path.clone(),
                        manager: "fvm".into(),
                        deps: vec![output::DepReport {
                            name: "flutter".to_owned(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%fvm_path, "fvm config not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%fvm_path, %err, "failed to fetch fvm config");
                had_error = true;
            }
        }
    }

    // ── Jsonnet Bundler (jsonnetfile.json) ───────────────────────────────────
    for jb_path in manager_files(&detected, "jsonnet-bundler") {
        match client.get_raw_file(owner, repo, &jb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::jsonnet_bundler::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !d.github_repo.is_empty() && !repo_cfg.is_dep_ignored(&d.remote))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %jb_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted jsonnet-bundler deps"
                );
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.github_repo.clone(),
                        current_value: d.version.clone(),
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let update_map: HashMap<String, (bool, Option<String>, Option<String>)> = {
                    let mut m = HashMap::new();
                    for r in gh_updates {
                        match r.summary {
                            Ok(s) => {
                                m.insert(r.dep_name, (s.update_available, s.latest, None));
                            }
                            Err(e) => {
                                m.insert(r.dep_name, (false, None, Some(e.to_string())));
                            }
                        }
                    }
                    m
                };
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = if dep.github_repo.is_empty() {
                            output::DepStatus::Skipped {
                                reason: "non-github remote".into(),
                            }
                        } else {
                            match update_map.get(&dep.github_repo) {
                                Some((true, Some(latest), _)) => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.version.clone(),
                                        latest: latest.clone(),
                                    }
                                }
                                Some((_, latest, None)) => output::DepStatus::UpToDate {
                                    latest: latest.clone(),
                                },
                                Some((_, _, Some(err))) => output::DepStatus::LookupError {
                                    message: err.clone(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            }
                        };
                        output::DepReport {
                            name: dep.remote.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: jb_path.clone(),
                    manager: "jsonnet-bundler".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%jb_path, "jsonnetfile.json not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%jb_path, %err, "failed to fetch jsonnetfile.json");
                had_error = true;
            }
        }
    }

    // ── Vendir (vendir.yml) ───────────────────────────────────────────────────
    for vendir_path in manager_files(&detected, "vendir") {
        match client.get_raw_file(owner, repo, &vendir_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::vendir::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored(&d.chart_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %vendir_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted vendir helm charts"
                );
                let helm_inputs: Vec<helm_datasource::HelmDepInput> = actionable
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.chart_name.clone(),
                        current_value: d.version.clone(),
                        repository_url: d.repo_url.clone(),
                    })
                    .collect();
                let updates =
                    helm_datasource::fetch_updates_concurrent(http, &helm_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.chart_name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.version.clone(),
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
                        output::DepReport {
                            name: dep.chart_name.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: vendir_path.clone(),
                    manager: "vendir".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%vendir_path, "vendir.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%vendir_path, %err, "failed to fetch vendir.yml");
                had_error = true;
            }
        }
    }

    // ── Copier (.copier-answers.yml) ─────────────────────────────────────────
    for copier_path in manager_files(&detected, "copier") {
        match client.get_raw_file(owner, repo, &copier_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::copier::extract(&raw.content) {
                    tracing::debug!(
                        repo = %repo_slug, file = %copier_path,
                        src = %dep.src_path, version = %dep.current_value,
                        "extracted copier template dep"
                    );
                    let status = if !dep.github_repo.is_empty() {
                        match github_tags_datasource::fetch_latest_tag(
                            &dep.github_repo,
                            &gh_http,
                            gh_api_base,
                        )
                        .await
                        {
                            Ok(Some(latest)) if latest != dep.current_value => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest,
                                }
                            }
                            Ok(Some(latest)) => output::DepStatus::UpToDate {
                                latest: Some(latest),
                            },
                            Ok(None) => output::DepStatus::UpToDate { latest: None },
                            Err(e) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                        }
                    } else {
                        output::DepStatus::Skipped {
                            reason: "non-github template source".into(),
                        }
                    };
                    repo_report.files.push(output::FileReport {
                        path: copier_path.clone(),
                        manager: "copier".into(),
                        deps: vec![output::DepReport {
                            name: dep.src_path.clone(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%copier_path, "copier answers file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%copier_path, %err, "failed to fetch copier answers file");
                had_error = true;
            }
        }
    }

    // ── Batect (batect.yml / batect-bundle.yml) ───────────────────────────────
    for batect_path in manager_files(&detected, "batect") {
        match client.get_raw_file(owner, repo, &batect_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::batect::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %batect_path,
                    total = deps.len(), "extracted batect images"
                );
                repo_report.files.push(output::FileReport {
                    path: batect_path.clone(),
                    manager: "batect".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%batect_path, "batect config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%batect_path, %err, "failed to fetch batect config");
                had_error = true;
            }
        }
    }

    // ── Batect wrapper script (`batect`) ─────────────────────────────────────
    for bw_path in manager_files(&detected, "batect-wrapper") {
        match client.get_raw_file(owner, repo, &bw_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::batect_wrapper::extract(&raw.content)
                {
                    tracing::debug!(
                        repo = %repo_slug, file = %bw_path,
                        version = %dep.version, "extracted batect wrapper version"
                    );
                    let status = match github_releases_datasource::fetch_latest_release(
                        renovate_core::extractors::batect_wrapper::BATECT_REPO,
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    {
                        Ok(Some(latest)) if latest != dep.version => {
                            output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest,
                            }
                        }
                        Ok(Some(latest)) => output::DepStatus::UpToDate {
                            latest: Some(latest),
                        },
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    repo_report.files.push(output::FileReport {
                        path: bw_path.clone(),
                        manager: "batect-wrapper".into(),
                        deps: vec![output::DepReport {
                            name: renovate_core::extractors::batect_wrapper::BATECT_REPO.to_owned(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%bw_path, "batect wrapper script not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bw_path, %err, "failed to fetch batect wrapper");
                had_error = true;
            }
        }
    }

    // ── XcodeGen (project.yml) ────────────────────────────────────────────────
    for xg_path in manager_files(&detected, "xcodegen") {
        match client.get_raw_file(owner, repo, &xg_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::xcodegen::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %xg_path,
                    total = deps.len(),
                    "extracted xcodegen swift package deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.name) {
                        continue;
                    }
                    let gh_repo = match &dep.source {
                        Some(renovate_core::extractors::xcodegen::XcodeGenSource::GitHub(r)) => {
                            r.as_str()
                        }
                        _ => {
                            dep_reports.push(output::DepReport {
                                name: dep.name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: "non-github-source".into(),
                                },
                            });
                            continue;
                        }
                    };
                    let tag_result = renovate_core::datasources::github_tags::fetch_latest_tag(
                        gh_repo,
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    .map_err(|e| e.to_string());
                    let status = match tag_result {
                        Ok(Some(tag)) => {
                            let stripped = tag.trim_start_matches('v');
                            let clean_current = dep.current_value.trim_start_matches('v');
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    clean_current,
                                    Some(stripped),
                                );
                            if s.update_available {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest: stripped.to_owned(),
                                }
                            } else {
                                output::DepStatus::UpToDate {
                                    latest: Some(stripped.to_owned()),
                                }
                            }
                        }
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError { message: e },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: xg_path.clone(),
                        manager: "xcodegen".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%xg_path, "project.yml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%xg_path, %err, "failed to fetch project.yml");
                had_error = true;
            }
        }
    }

    // ── Puppet (Puppetfile) ───────────────────────────────────────────────────
    for pf_path in manager_files(&detected, "puppet") {
        match client.get_raw_file(owner, repo, &pf_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::puppet::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %pf_path,
                    total = deps.len(),
                    "extracted puppet deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.name) {
                        continue;
                    }

                    use renovate_core::extractors::puppet::PuppetSource;
                    let status = match &dep.source {
                        PuppetSource::PuppetForge { forge_url } => {
                            let registry = forge_url.as_deref().unwrap_or("");
                            match renovate_core::datasources::puppet_forge::fetch_latest(
                                http,
                                &dep.name,
                                &dep.current_value,
                                registry,
                            )
                            .await
                            {
                                Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                    current: s.current_value,
                                    latest: s.latest.unwrap_or_default(),
                                },
                                Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        PuppetSource::GitHub(gh_repo) => {
                            let tag_result =
                                renovate_core::datasources::github_tags::fetch_latest_tag(
                                    gh_repo,
                                    &gh_http,
                                    gh_api_base,
                                )
                                .await
                                .map_err(|e| e.to_string());
                            match tag_result {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let clean = dep.current_value.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(clean, Some(stripped));
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: stripped.to_owned(),
                                        }
                                    } else {
                                        output::DepStatus::UpToDate {
                                            latest: Some(stripped.to_owned()),
                                        }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError { message: e },
                            }
                        }
                        PuppetSource::Git(_) => output::DepStatus::Skipped {
                            reason: "non-github-git".into(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: pf_path.clone(),
                        manager: "puppet".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%pf_path, "Puppetfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pf_path, %err, "failed to fetch Puppetfile");
                had_error = true;
            }
        }
    }

    // ── Ansible task files (tasks/*.yml) ─────────────────────────────────────
    for ansible_path in manager_files(&detected, "ansible") {
        match client.get_raw_file(owner, repo, &ansible_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ansible::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %ansible_path, total = deps.len(), "extracted ansible images");
                repo_report.files.push(output::FileReport {
                    path: ansible_path.clone(),
                    manager: "ansible".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%ansible_path, "ansible task file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ansible_path, %err, "failed to fetch ansible task file");
                had_error = true;
            }
        }
    }

    // ── Jenkins plugins (plugins.txt / plugins.yml) ───────────────────────────
    for jenkins_path in manager_files(&detected, "jenkins") {
        match client.get_raw_file(owner, repo, &jenkins_path).await {
            Ok(Some(raw)) => {
                let deps = if jenkins_path.ends_with(".txt") {
                    renovate_core::extractors::jenkins::extract_txt(&raw.content)
                } else {
                    renovate_core::extractors::jenkins::extract_yml(&raw.content)
                };
                tracing::debug!(
                    repo = %repo_slug, file = %jenkins_path,
                    total = deps.len(),
                    "extracted jenkins plugin deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if let Some(ver) = &dep.version {
                        match renovate_core::datasources::jenkins_plugins::fetch_latest(
                            http,
                            &dep.artifact_id,
                            ver,
                        )
                        .await
                        {
                            Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                current: ver.clone(),
                                latest: s.latest.unwrap_or_default(),
                            },
                            Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                            Err(e) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                        }
                    } else {
                        output::DepStatus::Skipped {
                            reason: "unspecified-version".into(),
                        }
                    };
                    file_deps.push(output::DepReport {
                        name: dep.artifact_id.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: jenkins_path.clone(),
                    manager: "jenkins".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%jenkins_path, "jenkins plugins file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%jenkins_path, %err, "failed to fetch jenkins plugins file");
                had_error = true;
            }
        }
    }

    // ── Clojure deps.edn / bb.edn ────────────────────────────────────────────
    for edn_path in manager_files(&detected, "deps-edn") {
        match client.get_raw_file(owner, repo, &edn_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::deps_edn::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %edn_path,
                    total = deps.len(), "extracted deps-edn deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        continue;
                    }
                    let latest = renovate_core::datasources::maven::fetch_latest_from_registry(
                        &dep.dep_name,
                        http,
                        renovate_core::datasources::maven::CLOJARS_BASE,
                    )
                    .await;
                    let latest = match latest {
                        Ok(Some(v)) => Ok(Some(v)),
                        Ok(None) => renovate_core::datasources::maven::fetch_latest_from_registry(
                            &dep.dep_name,
                            http,
                            renovate_core::datasources::maven::MAVEN_CENTRAL_BASE,
                        )
                        .await
                        .map_err(|e| e.to_string()),
                        Err(e) => Err(e.to_string()),
                    };
                    let status = match latest {
                        Ok(Some(ref l)) if l != &dep.current_value => {
                            output::DepStatus::UpdateAvailable {
                                current: dep.current_value.clone(),
                                latest: l.clone(),
                            }
                        }
                        Ok(Some(ref l)) => output::DepStatus::UpToDate {
                            latest: Some(l.clone()),
                        },
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError { message: e },
                    };
                    file_deps.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: edn_path.clone(),
                    manager: "deps-edn".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%edn_path, "deps.edn not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%edn_path, %err, "failed to fetch deps.edn");
                had_error = true;
            }
        }
    }

    // ── Leiningen (project.clj) ───────────────────────────────────────────────
    for lein_path in manager_files(&detected, "leiningen") {
        match client.get_raw_file(owner, repo, &lein_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::leiningen::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %lein_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted leiningen deps"
                );
                let dep_inputs: Vec<maven_datasource::MavenDepInput> = actionable
                    .iter()
                    .map(|d| maven_datasource::MavenDepInput {
                        dep_name: d.dep_name.clone(),
                        current_version: d.current_value.clone(),
                    })
                    .collect();
                // Try Clojars first, then fall back to Maven Central for each dep.
                let clojars_updates = {
                    let mut results = Vec::new();
                    for input in &dep_inputs {
                        let latest = renovate_core::datasources::maven::fetch_latest_from_registry(
                            &input.dep_name,
                            http,
                            renovate_core::datasources::maven::CLOJARS_BASE,
                        )
                        .await;
                        results.push((
                            input.dep_name.clone(),
                            input.current_version.clone(),
                            latest,
                        ));
                    }
                    results
                };
                // Build update map: prefer Clojars result if found, else Maven Central.
                let mut update_map: HashMap<String, Result<Option<String>, String>> =
                    HashMap::new();
                for (dep_name, current, clojars_result) in clojars_updates {
                    match clojars_result {
                        Ok(Some(v)) => {
                            update_map.insert(dep_name, Ok(Some(v)));
                        }
                        Ok(None) => {
                            // Not on Clojars, try Maven Central.
                            let central =
                                renovate_core::datasources::maven::fetch_latest_from_registry(
                                    &dep_name,
                                    http,
                                    renovate_core::datasources::maven::MAVEN_CENTRAL_BASE,
                                )
                                .await;
                            update_map.insert(dep_name, central.map_err(|e| e.to_string()));
                        }
                        Err(e) => {
                            update_map.insert(dep_name, Err(e.to_string()));
                        }
                    }
                    let _ = current;
                }
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.dep_name) {
                            Some(Ok(Some(latest))) if latest != &dep.current_value => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest: latest.clone(),
                                }
                            }
                            Some(Ok(Some(latest))) => output::DepStatus::UpToDate {
                                latest: Some(latest.clone()),
                            },
                            Some(Ok(None)) => output::DepStatus::UpToDate { latest: None },
                            Some(Err(e)) => output::DepStatus::LookupError { message: e.clone() },
                            None => output::DepStatus::UpToDate { latest: None },
                        };
                        output::DepReport {
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                repo_report.files.push(output::FileReport {
                    path: lein_path.clone(),
                    manager: "leiningen".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%lein_path, "project.clj not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%lein_path, %err, "failed to fetch project.clj");
                had_error = true;
            }
        }
    }

    // ── Bitrise CI (bitrise.yml / bitrise.yaml) ────────────────────────────────
    for br_path in manager_files(&detected, "bitrise") {
        match client.get_raw_file(owner, repo, &br_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::bitrise::{BitriseSkipReason, BitriseSource};
                let deps = renovate_core::extractors::bitrise::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %br_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted bitrise steps"
                );
                let mut dep_reports: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    BitriseSkipReason::UnspecifiedVersion => {
                                        "unspecified-version".to_owned()
                                    }
                                },
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        continue;
                    }
                    let current = dep.current_value.as_deref().unwrap_or("");
                    let status = match &dep.source {
                        BitriseSource::Git { repo_url } => {
                            match github_tags_datasource::fetch_latest_tag(
                                repo_url,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        current,
                                        Some(stripped),
                                    );
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: current.to_owned(),
                                            latest: stripped.to_owned(),
                                        }
                                    } else {
                                        output::DepStatus::UpToDate {
                                            latest: Some(stripped.to_owned()),
                                        }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        BitriseSource::Steplib { registry_url } => {
                            let registry = registry_url
                                .as_deref()
                                .unwrap_or(bitrise_datasource::DEFAULT_STEPLIB_URL);
                            match bitrise_datasource::fetch_latest(
                                http,
                                &dep.dep_name,
                                current,
                                registry,
                            )
                            .await
                            {
                                Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                    current: current.to_owned(),
                                    latest: s.latest.unwrap_or_default(),
                                },
                                Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        BitriseSource::Local => output::DepStatus::Skipped {
                            reason: "local-dependency".to_owned(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: br_path.clone(),
                        manager: "bitrise".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%br_path, "bitrise.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%br_path, %err, "failed to fetch bitrise.yml");
                had_error = true;
            }
        }
    }

    // ── Pixi (pixi.toml) ──────────────────────────────────────────────────────
    for pixi_path in manager_files(&detected, "pixi") {
        match client.get_raw_file(owner, repo, &pixi_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::pixi::{PixiSkipReason, PixiSource};
                let deps = renovate_core::extractors::pixi::extract(&raw.content);
                let pypi_actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.source == PixiSource::Pypi
                            && d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored(&d.dep_name)
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %pixi_path,
                    total = deps.len(), actionable = pypi_actionable.len(),
                    "extracted pixi deps"
                );
                let pypi_inputs: Vec<pypi_datasource::PypiDepInput> = pypi_actionable
                    .iter()
                    .map(|d| pypi_datasource::PypiDepInput {
                        dep_name: d.dep_name.clone(),
                        specifier: d.current_value.clone(),
                    })
                    .collect();
                let pypi_updates = pypi_datasource::fetch_updates_concurrent(
                    http,
                    &pypi_inputs,
                    pypi_datasource::PYPI_API,
                    10,
                )
                .await;
                let update_map: std::collections::HashMap<_, _> = pypi_updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        if let Some(reason) = &dep.skip_reason {
                            return output::DepReport {
                                name: dep.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: match reason {
                                        PixiSkipReason::CondaNotSupported => {
                                            "conda-not-supported".to_owned()
                                        }
                                        PixiSkipReason::InvalidVersion => {
                                            "invalid-version".to_owned()
                                        }
                                        PixiSkipReason::UnspecifiedVersion => {
                                            "unspecified-version".to_owned()
                                        }
                                    },
                                },
                            };
                        }
                        if dep.source == PixiSource::Conda {
                            return output::DepReport {
                                name: dep.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: "conda-not-supported".to_owned(),
                                },
                            };
                        }
                        let status = match update_map.get(&dep.dep_name) {
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
                        output::DepReport {
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: pixi_path.clone(),
                        manager: "pixi".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pixi_path, "pixi.toml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pixi_path, %err, "failed to fetch pixi.toml");
                had_error = true;
            }
        }
    }

    // ── Homebrew formula (Formula/*.rb) ────────────────────────────────────────
    for hb_path in manager_files(&detected, "homebrew") {
        match client.get_raw_file(owner, repo, &hb_path).await {
            Ok(Some(raw)) => {
                use homebrew_extractor::{GitHubUrlType, HomebrewSkipReason, HomebrewSource};
                let dep = match homebrew_extractor::extract(&raw.content) {
                    Some(d) => d,
                    None => continue,
                };
                if repo_cfg.is_dep_ignored(&dep.formula_name) {
                    continue;
                }
                let status = if let Some(reason) = &dep.skip_reason {
                    output::DepStatus::Skipped {
                        reason: match reason {
                            HomebrewSkipReason::InvalidSha256 => "invalid-sha256".to_owned(),
                            HomebrewSkipReason::UnsupportedUrl => "unsupported-url".to_owned(),
                            HomebrewSkipReason::MissingUrl => "missing-url".to_owned(),
                        },
                    }
                } else {
                    match &dep.source {
                        HomebrewSource::GitHub {
                            repo: gh_repo,
                            url_type,
                        } => {
                            let result = match url_type {
                                GitHubUrlType::Archive => {
                                    renovate_core::datasources::github_tags::fetch_latest_tag(
                                        gh_repo,
                                        &gh_http,
                                        gh_api_base,
                                    )
                                    .await
                                    .map_err(|e| e.to_string())
                                }
                                GitHubUrlType::Release => {
                                    github_releases_datasource::fetch_latest_release(
                                        gh_repo,
                                        &gh_http,
                                        gh_api_base,
                                    )
                                    .await
                                    .map_err(|e| e.to_string())
                                }
                            };
                            match result {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        &dep.current_value,
                                        Some(stripped),
                                    );
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: stripped.to_owned(),
                                        }
                                    } else {
                                        output::DepStatus::UpToDate {
                                            latest: Some(stripped.to_owned()),
                                        }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError { message: e },
                            }
                        }
                        HomebrewSource::Npm { package } => {
                            let npm_input = vec![npm_datasource::NpmDepInput {
                                dep_name: package.clone(),
                                constraint: dep.current_value.clone(),
                            }];
                            let mut updates = npm_datasource::fetch_updates_concurrent(
                                http,
                                &npm_input,
                                npm_datasource::NPM_REGISTRY,
                                1,
                            )
                            .await;
                            match updates.pop().map(|r| r.summary) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.current_value.clone(),
                                        latest: s.latest.unwrap_or_default(),
                                    }
                                }
                                Some(Ok(s)) => output::DepStatus::UpToDate { latest: s.latest },
                                Some(Err(e)) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            }
                        }
                        HomebrewSource::Unsupported(_) => output::DepStatus::Skipped {
                            reason: "unsupported-url".to_owned(),
                        },
                    }
                };
                repo_report.files.push(output::FileReport {
                    path: hb_path.clone(),
                    manager: "homebrew".into(),
                    deps: vec![output::DepReport {
                        name: dep.formula_name.clone(),
                        status,
                    }],
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%hb_path, "homebrew formula not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%hb_path, %err, "failed to fetch homebrew formula");
                had_error = true;
            }
        }
    }

    // ── Helmsman DSF (helmsman.yml / helmsman.d/*.yml) ────────────────────────
    for hsm_path in manager_files(&detected, "helmsman") {
        match client.get_raw_file(owner, repo, &hsm_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::helmsman::HelmsmanSkipReason;
                let deps = renovate_core::extractors::helmsman::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !repo_cfg.is_dep_ignored(&d.dep_name))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %hsm_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted helmsman deps"
                );
                let helm_inputs: Vec<helm_datasource::HelmDepInput> = actionable
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.chart_name.clone(),
                        current_value: d.current_value.clone(),
                        repository_url: d.registry_url.clone(),
                    })
                    .collect();
                let updates =
                    helm_datasource::fetch_updates_concurrent(http, &helm_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        if let Some(reason) = &dep.skip_reason {
                            return output::DepReport {
                                name: dep.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: match reason {
                                        HelmsmanSkipReason::UnspecifiedVersion => {
                                            "unspecified-version".to_owned()
                                        }
                                        HelmsmanSkipReason::InvalidChart => {
                                            "invalid-name".to_owned()
                                        }
                                        HelmsmanSkipReason::NoRepository => {
                                            "no-repository".to_owned()
                                        }
                                    },
                                },
                            };
                        }
                        let status = match update_map.get(&dep.chart_name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_value.clone(),
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
                        output::DepReport {
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: hsm_path.clone(),
                        manager: "helmsman".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%hsm_path, "helmsman file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%hsm_path, %err, "failed to fetch helmsman file");
                had_error = true;
            }
        }
    }

    // ── Unity3D ProjectVersion.txt ─────────────────────────────────────────────
    for unity_path in manager_files(&detected, "unity3d") {
        match client.get_raw_file(owner, repo, &unity_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::unity3d::Unity3dVersionKind;
                let Some(dep) = renovate_core::extractors::unity3d::extract(&raw.content) else {
                    continue;
                };
                let with_revision = dep.kind == Unity3dVersionKind::WithRevision;
                let status = match renovate_core::datasources::unity3d::fetch_latest_lts(
                    http,
                    with_revision,
                )
                .await
                {
                    Ok(s) => {
                        let latest_str = if with_revision {
                            s.latest_with_revision.clone()
                        } else {
                            s.latest.clone()
                        };
                        match latest_str {
                            Some(latest) if latest != dep.current_value => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest,
                                }
                            }
                            Some(latest) => output::DepStatus::UpToDate {
                                latest: Some(latest),
                            },
                            None => output::DepStatus::UpToDate { latest: None },
                        }
                    }
                    Err(e) => output::DepStatus::LookupError {
                        message: e.to_string(),
                    },
                };
                repo_report.files.push(output::FileReport {
                    path: unity_path.clone(),
                    manager: "unity3d".into(),
                    deps: vec![output::DepReport {
                        name: "Unity Editor".to_owned(),
                        status,
                    }],
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%unity_path, "ProjectVersion.txt not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%unity_path, %err, "failed to fetch ProjectVersion.txt");
                had_error = true;
            }
        }
    }

    // ── Cloud Native Buildpacks (project.toml) ────────────────────────────────
    for bp_path in manager_files(&detected, "buildpacks") {
        match client.get_raw_file(owner, repo, &bp_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::buildpacks::{
                    BuildpacksSkipReason, BuildpacksSource,
                };
                let deps = renovate_core::extractors::buildpacks::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %bp_path,
                    total = deps.len(),
                    "extracted buildpacks deps"
                );
                let mut dep_reports: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    BuildpacksSkipReason::DockerImage => "docker-image".to_owned(),
                                    BuildpacksSkipReason::NoVersion => "no-version".to_owned(),
                                    BuildpacksSkipReason::UnsupportedUri => {
                                        "unsupported-url".to_owned()
                                    }
                                },
                            },
                        });
                        continue;
                    }
                    if dep.source != BuildpacksSource::Registry {
                        continue;
                    }
                    if repo_cfg.is_dep_ignored(&dep.dep_name) {
                        continue;
                    }
                    let status =
                        match renovate_core::datasources::buildpacks_registry::fetch_latest(
                            http,
                            &dep.dep_name,
                            &dep.current_value,
                        )
                        .await
                        {
                            Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                current: dep.current_value.clone(),
                                latest: s.latest.unwrap_or_default(),
                            },
                            Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                            Err(e) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                        };
                    dep_reports.push(output::DepReport {
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    repo_report.files.push(output::FileReport {
                        path: bp_path.clone(),
                        manager: "buildpacks".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%bp_path, "project.toml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bp_path, %err, "failed to fetch project.toml");
                had_error = true;
            }
        }
    }

    // Apply matchUpdateTypes packageRules blocking across all collected file reports.
    apply_update_blocking_to_report(&mut repo_report, &repo_cfg);

    (Some(repo_report), had_error)
}

// ── Report-building helpers ───────────────────────────────────────────────────

/// Apply `packageRules` `matchUpdateTypes`+`enabled:false` blocking across all
/// file reports.  For each `UpdateAvailable` dep, classifies the semver bump
/// type and converts to `Skipped` when a matching rule blocks it.
fn apply_update_blocking_to_report(
    report: &mut output::RepoReport,
    repo_cfg: &renovate_core::repo_config::RepoConfig,
) {
    use renovate_core::versioning::semver_generic::classify_semver_update;
    for file in &mut report.files {
        let manager = file.manager.clone();
        let file_path = file.path.clone();
        for dep in &mut file.deps {
            if let output::DepStatus::UpdateAvailable {
                ref current,
                ref latest,
            } = dep.status
            {
                // Check allowedVersions restriction (file-path-aware).
                if repo_cfg.is_version_restricted_for_file(&dep.name, &manager, latest, &file_path)
                {
                    dep.status = output::DepStatus::Skipped {
                        reason: "blocked by packageRules (allowedVersions)".into(),
                    };
                    continue;
                }
                // Check matchUpdateTypes + matchCurrentVersion + matchFileNames + enabled:false.
                if let Some(update_type) = classify_semver_update(current, latest)
                    && repo_cfg.is_update_blocked_for_file(
                        &dep.name,
                        current,
                        update_type,
                        &manager,
                        &file_path,
                    )
                {
                    dep.status = output::DepStatus::Skipped {
                        reason: format!(
                            "blocked by packageRules (matchUpdateTypes: {:?})",
                            update_type
                        )
                        .to_lowercase(),
                    };
                }
            }
        }
    }
}

/// Return the matched files for a given manager name (empty slice if not
/// detected).
fn manager_files(detected: &[renovate_core::managers::DetectedManager], name: &str) -> Vec<String> {
    detected
        .iter()
        .find(|m| m.name == name)
        .map(|m| m.matched_files.clone())
        .unwrap_or_default()
}

/// Fetch Docker Hub updates for `deps` and build a `DepReport` list.
///
/// Identical logic is shared across Cloud Build, Drone CI, Bitbucket Pipelines,
/// GitLab CI, CircleCI, Dockerfile, Docker Compose, and similar managers.
async fn docker_hub_reports(
    http: &HttpClient,
    deps: &[renovate_core::extractors::dockerfile::DockerfileExtractedDep],
) -> Vec<output::DepReport> {
    let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
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
    let update_map: HashMap<String, _> = updates
        .into_iter()
        .map(|r| (r.dep_name, r.summary))
        .collect();

    let mut reports = Vec::new();
    for dep in deps {
        if let Some(reason) = &dep.skip_reason {
            reports.push(output::DepReport {
                name: dep.image.clone(),
                status: output::DepStatus::Skipped {
                    reason: format!("{reason:?}").to_lowercase(),
                },
            });
        } else {
            let dep_name = match &dep.tag {
                Some(t) => format!("{}:{t}", dep.image),
                None => dep.image.clone(),
            };
            let status = match update_map.get(&dep_name) {
                Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                    current: s.current_tag.clone(),
                    latest: s.latest.clone().unwrap_or_default(),
                },
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
            reports.push(output::DepReport {
                name: dep_name,
                status,
            });
        }
    }
    reports
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

fn build_dep_reports_helm(
    all_deps: &[renovate_core::extractors::helm::HelmExtractedDep],
    actionable: &[&renovate_core::extractors::helm::HelmExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::helm::HelmUpdateSummary,
            renovate_core::datasources::helm::HelmError,
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

fn build_dep_reports_gradle(
    all_deps: &[renovate_core::extractors::gradle::GradleExtractedDep],
    actionable: &[&renovate_core::extractors::gradle::GradleExtractedDep],
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

fn build_dep_reports_setup_cfg(
    all_deps: &[renovate_core::extractors::setup_cfg::SetupCfgDep],
    actionable: &[&renovate_core::extractors::setup_cfg::SetupCfgDep],
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

fn build_dep_reports_pipfile(
    all_deps: &[renovate_core::extractors::pipfile::PipfileDep],
    actionable: &[&renovate_core::extractors::pipfile::PipfileDep],
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
