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

    // ── Version files (.terraform-version, .go-version, .bun-version, etc.) ──
    for manager_name in [
        "terraform-version",
        "terragrunt-version",
        "go-version",
        "python-version",
        "node-version",
        "nvmrc",
        "bun-version",
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
                            let latest_ver = tag.trim_start_matches(tag_strip).to_owned();
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

    // ── GitLab CI (.gitlab-ci.yml) ────────────────────────────────────────────
    for glci_path in manager_files(&detected, "gitlabci") {
        match client.get_raw_file(owner, repo, &glci_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gitlabci::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.dep.skip_reason.is_none())
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %glci_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted gitlab-ci images"
                );
                let dep_inputs: Vec<docker_datasource::DockerDepInput> = actionable
                    .iter()
                    .filter_map(|d| {
                        let tag = d.dep.tag.as_deref()?;
                        Some(docker_datasource::DockerDepInput {
                            dep_name: format!("{}:{tag}", d.dep.image),
                            image: d.dep.image.clone(),
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
                for d in deps.iter().filter(|d| d.dep.skip_reason.is_some()) {
                    file_deps.push(output::DepReport {
                        name: d.dep.image.clone(),
                        status: output::DepStatus::Skipped {
                            reason: format!("{:?}", d.dep.skip_reason.as_ref().unwrap())
                                .to_lowercase(),
                        },
                    });
                }
                for d in &actionable {
                    let dep_name = match &d.dep.tag {
                        Some(t) => format!("{}:{t}", d.dep.image),
                        None => d.dep.image.clone(),
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
                    file_deps.push(output::DepReport {
                        name: dep_name,
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: glci_path.clone(),
                    manager: "gitlabci".into(),
                    deps: file_deps,
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

    // ── CircleCI (.circleci/config.yml) ──────────────────────────────────────
    for cci_path in manager_files(&detected, "circleci") {
        match client.get_raw_file(owner, repo, &cci_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::circleci::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.dep.skip_reason.is_none())
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %cci_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted circleci images"
                );
                let dep_inputs: Vec<docker_datasource::DockerDepInput> = actionable
                    .iter()
                    .filter_map(|d| {
                        let tag = d.dep.tag.as_deref()?;
                        Some(docker_datasource::DockerDepInput {
                            dep_name: format!("{}:{tag}", d.dep.image),
                            image: d.dep.image.clone(),
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
                for d in deps.iter().filter(|d| d.dep.skip_reason.is_some()) {
                    file_deps.push(output::DepReport {
                        name: d.dep.image.clone(),
                        status: output::DepStatus::Skipped {
                            reason: format!("{:?}", d.dep.skip_reason.as_ref().unwrap())
                                .to_lowercase(),
                        },
                    });
                }
                for d in &actionable {
                    let dep_name = match &d.dep.tag {
                        Some(t) => format!("{}:{t}", d.dep.image),
                        None => d.dep.image.clone(),
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
                    file_deps.push(output::DepReport {
                        name: dep_name,
                        status,
                    });
                }
                repo_report.files.push(output::FileReport {
                    path: cci_path.clone(),
                    manager: "circleci".into(),
                    deps: file_deps,
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
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %cb_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted cloudbuild images"
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
                for dep in &deps {
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
                }
                repo_report.files.push(output::FileReport {
                    path: cb_path.clone(),
                    manager: "cloudbuild".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%cb_path, "cloudbuild.yaml not found")
            }
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
                let images: Vec<_> = deps
                    .iter()
                    .filter_map(|d| match d {
                        renovate_core::extractors::azure_pipelines::AzPipelinesDep::Container(
                            c,
                        ) => Some(c),
                        renovate_core::extractors::azure_pipelines::AzPipelinesDep::Task(_) => None,
                    })
                    .collect();
                let actionable: Vec<_> =
                    images.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %az_path,
                    total_images = images.len(), actionable = actionable.len(),
                    "extracted azure-pipelines images"
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
                for dep in &deps {
                    match dep {
                        renovate_core::extractors::azure_pipelines::AzPipelinesDep::Container(
                            img,
                        ) => {
                            if let Some(reason) = &img.skip_reason {
                                file_deps.push(output::DepReport {
                                    name: img.image.clone(),
                                    status: output::DepStatus::Skipped {
                                        reason: format!("{reason:?}").to_lowercase(),
                                    },
                                });
                            } else {
                                let dep_name = match &img.tag {
                                    Some(t) => format!("{}:{t}", img.image),
                                    None => img.image.clone(),
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
                                    Some(Err(docker_datasource::DockerHubError::NonDockerHub(
                                        _,
                                    ))) => output::DepStatus::Skipped {
                                        reason: "non-docker-hub registry".into(),
                                    },
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
                        renovate_core::extractors::azure_pipelines::AzPipelinesDep::Task(t) => {
                            // azure-pipelines-tasks datasource not yet implemented;
                            // emit as skipped so the dep is visible in output.
                            file_deps.push(output::DepReport {
                                name: format!("{}@{}", t.name, t.version),
                                status: output::DepStatus::Skipped {
                                    reason: "azure-pipelines-tasks datasource pending".into(),
                                },
                            });
                        }
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
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %bb_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted bitbucket-pipelines images"
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
                for dep in &deps {
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
                }
                repo_report.files.push(output::FileReport {
                    path: bb_path.clone(),
                    manager: "bitbucket-pipelines".into(),
                    deps: file_deps,
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
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %drone_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted droneci images"
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
                for dep in &deps {
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
                }
                repo_report.files.push(output::FileReport {
                    path: drone_path.clone(),
                    manager: "droneci".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%drone_path, ".drone.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%drone_path, %err, "failed to fetch .drone.yml");
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
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = if let Some(reason) = &dep.skip_reason {
                            output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            }
                        } else {
                            output::DepStatus::Skipped {
                                reason: "jenkins-plugins datasource pending".into(),
                            }
                        };
                        output::DepReport {
                            name: dep.artifact_id.clone(),
                            status,
                        }
                    })
                    .collect();
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
