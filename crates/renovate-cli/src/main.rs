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

use std::path::Path;
use std::process::ExitCode;

use clap::Parser as _;
use cli::Cli;
use renovate_core::config::{GlobalConfig, file as config_file};
use renovate_core::datasources::crates_io::{self, DepInput};
use renovate_core::datasources::npm as npm_datasource;
use renovate_core::extractors::cargo as cargo_extractor;
use renovate_core::extractors::npm as npm_extractor;
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

        // Collect per-file update results for the human-readable report.
        let mut repo_report = output::RepoReport {
            repo_slug: repo_slug.clone(),
            files: Vec::new(),
        };

        // ── Cargo ────────────────────────────────────────────────────────────
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
                        tracing::debug!(
                            repo = %repo_slug,
                            file = %cargo_file_path,
                            total = deps.len(),
                            actionable = actionable.len(),
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
                            &http,
                            &dep_inputs,
                            crates_io::CRATES_IO_SPARSE_INDEX,
                            10,
                        )
                        .await;

                        // Build a lookup map for update results.
                        let update_map: std::collections::HashMap<_, _> = updates
                            .into_iter()
                            .map(|r| (r.dep_name, r.summary))
                            .collect();

                        let mut file_deps: Vec<output::DepReport> = Vec::new();

                        // Skipped deps first (no registry lookup needed).
                        for dep in deps.iter().filter(|d| d.skip_reason.is_some()) {
                            file_deps.push(output::DepReport {
                                name: dep.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                        .to_lowercase(),
                                },
                            });
                        }

                        // Actionable deps — looked up in registry.
                        for dep in &actionable {
                            let status = match update_map.get(&dep.dep_name) {
                                Some(Ok(summary)) if summary.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: summary.current_constraint.clone(),
                                        latest: summary
                                            .latest_compatible
                                            .clone()
                                            .unwrap_or_default(),
                                    }
                                }
                                Some(Ok(summary)) => output::DepStatus::UpToDate {
                                    latest: summary.latest_compatible.clone(),
                                },
                                Some(Err(err)) => output::DepStatus::LookupError {
                                    message: err.to_string(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            };
                            file_deps.push(output::DepReport {
                                name: dep.dep_name.clone(),
                                status,
                            });
                        }

                        repo_report.files.push(output::FileReport {
                            path: cargo_file_path.clone(),
                            manager: "cargo".into(),
                            deps: file_deps,
                        });
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

        // ── npm ──────────────────────────────────────────────────────────────
        let npm_files: Vec<_> = detected
            .iter()
            .find(|m| m.name == "npm")
            .map(|m| m.matched_files.as_slice())
            .unwrap_or_default()
            .iter()
            .collect();

        for npm_file_path in npm_files {
            match client.get_raw_file(owner, repo, npm_file_path).await {
                Ok(Some(raw)) => match npm_extractor::extract(&raw.content) {
                    Ok(deps) => {
                        let actionable: Vec<_> =
                            deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                        tracing::debug!(
                            repo = %repo_slug,
                            file = %npm_file_path,
                            total = deps.len(),
                            actionable = actionable.len(),
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
                            &http,
                            &dep_inputs,
                            npm_datasource::NPM_REGISTRY,
                            10,
                        )
                        .await;

                        let update_map: std::collections::HashMap<_, _> = updates
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
                                Some(Ok(summary)) if summary.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: summary.current_constraint.clone(),
                                        latest: summary.latest.clone().unwrap_or_default(),
                                    }
                                }
                                Some(Ok(summary)) => output::DepStatus::UpToDate {
                                    latest: summary.latest.clone(),
                                },
                                Some(Err(err)) => output::DepStatus::LookupError {
                                    message: err.to_string(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            };
                            file_deps.push(output::DepReport {
                                name: dep.name.clone(),
                                status,
                            });
                        }

                        repo_report.files.push(output::FileReport {
                            path: npm_file_path.clone(),
                            manager: "npm".into(),
                            deps: file_deps,
                        });
                    }
                    Err(err) => {
                        tracing::warn!(repo = %repo_slug, file = %npm_file_path, %err, "failed to parse package.json");
                    }
                },
                Ok(None) => {
                    tracing::warn!(repo = %repo_slug, file = %npm_file_path, "package.json not found");
                }
                Err(err) => {
                    tracing::error!(repo = %repo_slug, file = %npm_file_path, %err, "failed to fetch package.json");
                    had_error = true;
                }
            }
        }

        // Print the human-readable summary for this repository.
        output::print_report(&repo_report, output::should_use_color());
    }

    if had_error {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
