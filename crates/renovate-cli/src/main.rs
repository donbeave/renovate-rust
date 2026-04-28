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

use std::path::Path;
use std::process::ExitCode;

use clap::Parser as _;
use cli::Cli;
use renovate_core::config::{GlobalConfig, file as config_file};
use renovate_core::datasources::crates_io::{self, DepInput};
use renovate_core::extractors::cargo as cargo_extractor;
use renovate_core::managers;
use renovate_core::platform::{AnyPlatformClient, PlatformError};
use renovate_core::repo_config;

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

    // 7. Process each repository.
    let Some(client) = maybe_client else {
        tracing::warn!("no platform client available; skipping repository processing");
        return ExitCode::SUCCESS;
    };

    // Shared HTTP client for all datasource calls — one connection pool for
    // the entire run rather than one per dependency.
    let http = renovate_core::http::HttpClient::new().expect("failed to create HTTP client");

    let mut had_error = false;
    for repo_slug in &config.repositories {
        let Some((owner, repo)) = repo_slug.split_once('/') else {
            tracing::warn!(repo = %repo_slug, "skipping malformed repository slug (expected owner/repo)");
            continue;
        };

        tracing::info!(repo = %repo_slug, "processing repository");

        match repo_config::discover(&client, owner, repo, &config).await {
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
                had_error = true;
                continue;
            }
        }

        // Detect which package managers are present.
        let files = match client.get_file_list(owner, repo).await {
            Ok(f) => f,
            Err(err) => {
                tracing::error!(repo = %repo_slug, %err, "failed to get file list");
                had_error = true;
                continue;
            }
        };

        let detected = managers::detect(&files);
        if detected.is_empty() {
            tracing::info!(repo = %repo_slug, "no package managers detected");
        } else {
            let names: Vec<&str> = detected.iter().map(|m| m.name).collect();
            tracing::info!(repo = %repo_slug, managers = ?names, "detected package managers");
        }

        // Extract dependencies from Cargo.toml files.
        let cargo_files: Vec<_> = detected
            .iter()
            .find(|m| m.name == "cargo")
            .map(|m| m.matched_files.as_slice())
            .unwrap_or_default()
            .iter()
            .collect();

        for cargo_file_path in cargo_files {
            match client.get_raw_file(owner, repo, cargo_file_path).await {
                Ok(Some(raw)) => match cargo_extractor::extract(&raw.content) {
                    Ok(deps) => {
                        let actionable: Vec<_> =
                            deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                        let skipped = deps.len() - actionable.len();
                        tracing::info!(
                            repo = %repo_slug,
                            file = %cargo_file_path,
                            total = deps.len(),
                            actionable = actionable.len(),
                            skipped,
                            "extracted cargo dependencies"
                        );

                        // Look up available versions concurrently (bounded by 10
                        // simultaneous requests — matches Renovate's HTTP queue depth).
                        let dep_inputs: Vec<DepInput> = actionable
                            .iter()
                            .map(|d| DepInput {
                                dep_name: d.dep_name.clone(),
                                package_name: d.package_name.clone(),
                                constraint: d.current_value.clone(),
                            })
                            .collect();

                        let updates = crates_io::fetch_updates_concurrent(
                            &http,
                            &dep_inputs,
                            crates_io::CRATES_IO_SPARSE_INDEX,
                            10,
                        )
                        .await;

                        for result in &updates {
                            match &result.summary {
                                Ok(summary) => {
                                    let status = if summary.update_available {
                                        "update available"
                                    } else {
                                        "up to date"
                                    };
                                    tracing::info!(
                                        repo = %repo_slug,
                                        dep = %result.dep_name,
                                        constraint = %summary.current_constraint,
                                        latest = ?summary.latest_compatible,
                                        status,
                                    );
                                }
                                Err(err) => {
                                    tracing::warn!(
                                        dep = %result.dep_name,
                                        %err,
                                        "failed to fetch crate versions"
                                    );
                                }
                            }
                        }
                    }
                    Err(err) => {
                        tracing::warn!(repo = %repo_slug, file = %cargo_file_path, %err, "failed to parse Cargo.toml");
                    }
                },
                Ok(None) => {
                    tracing::warn!(repo = %repo_slug, file = %cargo_file_path, "Cargo.toml not found");
                }
                Err(err) => {
                    tracing::error!(repo = %repo_slug, file = %cargo_file_path, %err, "failed to fetch Cargo.toml");
                    had_error = true;
                }
            }
        }
    }

    if had_error {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
