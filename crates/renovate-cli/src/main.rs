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
mod config_env;
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
use renovate_core::config::{DryRun, GlobalConfig, file as config_file};
use renovate_core::http::HttpClient;
use renovate_core::artifacts::{ArtifactConfig, ArtifactRegistry, UpdateArtifact, UpdatedDep};
use renovate_core::extractors::gomod_artifact_runner::GomodArtifactRunner;
use renovate_core::extractors::npm_post_update::artifact_runner::NpmArtifactRunner;
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

    let env_map = std::env::vars().collect();
    let base = match config_env::apply_to_base(&env_map, base) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("renovate: {err}");
            return ExitCode::from(1);
        }
    };

    let config = match config_builder::try_build(&cli, base) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("renovate: {err}");
            return ExitCode::from(1);
        }
    };
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

/// Build a minimal markdown PR body for a branch update.
///
/// Includes header/footer from repo config and a bullet list of deps.
/// Mirrors the early slices of Renovate's PR body generation.
fn build_pr_body(
    deps: &[report_builders::BranchDep],
    repo_cfg: &renovate_core::repo_config::RepoConfig,
) -> String {
    use renovate_core::branch;

    let mut body = String::new();

    // Header
    body.push_str(&branch::get_pr_header(repo_cfg.pr_header.as_deref()));

    // Update table
    let table_deps: Vec<renovate_core::branch::PrTableDep> = deps
        .iter()
        .filter_map(|bd| {
            if let output::DepStatus::UpdateAvailable { ref current, ref latest } = bd.dep.status {
                Some(renovate_core::branch::PrTableDep {
                    dep_name: bd.dep.name.clone(),
                    new_name: bd.dep.replacement_name.clone(),
                    dep_type: bd.dep.dep_type.clone(),
                    update_type: bd.dep.update_type.clone(),
                    current_value: Some(current.clone()),
                    new_value: Some(bd.dep.new_value.clone().unwrap_or_else(|| latest.clone())),
                })
            } else {
                None
            }
        })
        .collect();
    body.push_str(&branch::get_pr_updates_table(
        Some(&repo_cfg.pr_body_columns),
        &table_deps,
    ));

    // Extra notes
    for bd in deps {
        if let output::DepStatus::UpdateAvailable { .. } = bd.dep.status {
            let is_pin = bd.dep.range_strategy.as_deref() == Some("pin");
            let has_git_ref = bd.dep.pin_digests.unwrap_or(false);
            let notes = branch::get_pr_extra_notes(
                has_git_ref,
                bd.dep.update_type.as_deref().unwrap_or(""),
                is_pin,
            );
            if !notes.is_empty() && !body.contains(&notes) {
                body.push_str(&notes);
            }
        }
    }

    // Footer
    body.push_str(&branch::get_pr_footer(Some(&repo_cfg.pr_footer)));

    body
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
            // Pass the repo-level `timezone` so "after 9am" fires at the right local time.
            if !rc.schedule.is_empty()
                && !renovate_core::schedule::is_within_schedule_tz(
                    &rc.schedule,
                    rc.timezone.as_deref(),
                )
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
        .filter(|f| !path_matcher.is_ignored(f) && repo_cfg.is_path_included(f))
        .collect();

    let detected = {
        let all = managers::detect(&filtered_files);
        // Always filter using is_manager_enabled, which:
        // - When enabledManagers is empty: excludes disabled-by-default managers
        // - When enabledManagers is set: only includes explicitly listed managers
        // Mirrors Renovate's behavior: disabled-by-default managers (pre-commit, nix, etc.)
        // only run when explicitly listed in enabledManagers.
        let filtered: Vec<_> = all
            .into_iter()
            .filter(|m| {
                repo_cfg.is_manager_enabled(
                    m.name,
                    renovate_core::managers::is_disabled_by_default(m.name),
                )
            })
            .collect();
        if !repo_cfg.enabled_managers.is_empty() && !filtered.is_empty() {
            tracing::debug!(
                repo = %repo_slug,
                enabled = ?repo_cfg.enabled_managers,
                "enabledManagers filter applied"
            );
        }
        filtered
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
            draft_pr: repo_cfg.draft_pr,
            assign_automerge: repo_cfg.assign_automerge,
            files: Vec::new(),
        },
        had_error: false,
    };
    pipelines::process_all_managers(&mut ctx).await;
    let (mut repo_report, had_error) = (ctx.report, ctx.had_error);

    // Skip deps listed in ignoreDeps before any other filters.
    pipeline_utils::apply_ignore_deps_to_report(&mut repo_report, &repo_cfg);
    // Apply matchUpdateTypes packageRules blocking across all collected file reports.
    pipeline_utils::apply_update_blocking_to_report(&mut repo_report, &repo_cfg, repo_slug);
    // Apply ignoreVersions (global + per-rule) across all collected file reports.
    pipeline_utils::apply_version_ignore_to_report(&mut repo_report, &repo_cfg, repo_slug);

    // Branch / PR creation for UpdateAvailable deps.
    let mut had_error = had_error;
    let target_branch = repo_cfg
        .base_branches
        .first()
        .cloned()
        .unwrap_or_else(|| "main".to_owned());

    // Branchify: group UpdateAvailable deps by branch_name so deps sharing
    // a branch are coalesced into a single PR.  Mirrors Renovate's
    // `branchifyUpgrades` step.
    let branch_updates = report_builders::collect_branch_updates(&repo_report);

    // Artifact runner registry: maps manager names to lockfile generators.
    let mut artifact_registry = ArtifactRegistry::new();
    artifact_registry.register("npm", Box::new(NpmArtifactRunner));
    artifact_registry.register("gomod", Box::new(GomodArtifactRunner));

    // Manifest editing: apply newValue constraints to source files.
    // Only npm/package.json is supported in this slice; other managers
    // are logged and skipped.
    if config.dry_run.is_none() {
        for (branch, deps) in &branch_updates {
            // Group deps by file path so a single file is read once,
            // updated for all deps on this branch, then written back.
            let mut by_file: std::collections::BTreeMap<&str, Vec<&report_builders::BranchDep>> =
                std::collections::BTreeMap::new();
            for dep in deps {
                by_file.entry(&dep.file_path).or_default().push(dep);
            }

            for (file_path, file_deps) in by_file {
                if file_deps.is_empty() {
                    continue;
                }
                let manager = &file_deps[0].manager;

                match client.get_raw_file(owner, repo, file_path).await {
                    Ok(Some(raw)) => {
                        let mut content = raw.content;
                        // For gomod, re-extract to get line numbers needed by the update fn.
                        let gomod_extracted = if manager == "gomod" {
                            renovate_core::extractors::gomod::extract(&content)
                        } else {
                            Vec::new()
                        };
                        // For maven, re-extract to get file_replace_position needed by the update fn.
                        let maven_extracted = if manager == "maven" {
                            renovate_core::extractors::maven::extract(&content).unwrap_or_default()
                        } else {
                            Vec::new()
                        };
                        for bd in &file_deps {
                            if let output::DepStatus::UpdateAvailable { ref current, ref latest } =
                                bd.dep.status
                            {
                                let updated = match manager.as_str() {
                                    "npm" => {
                                        let upgrade = renovate_core::extractors::npm::NpmUpdateUpgrade {
                                            dep_type: bd.dep.dep_type.clone().unwrap_or_default(),
                                            dep_name: bd.dep.name.clone(),
                                            new_value: bd.dep.new_value.clone(),
                                            current_value: Some(current.clone()),
                                            ..Default::default()
                                        };
                                        renovate_core::extractors::npm::npm_update_dependency(
                                            &content, &upgrade,
                                        )
                                    }
                                    "gomod" => {
                                        let manager_data = gomod_extracted
                                            .iter()
                                            .find(|d| d.module_path == bd.dep.name)
                                            .and_then(|d| d.manager_data.clone());
                                        let upgrade = renovate_core::extractors::gomod::GoModUpdateUpgrade {
                                            dep_name: Some(bd.dep.name.clone()),
                                            dep_type: bd.dep.dep_type.clone(),
                                            update_type: bd.dep.update_type.clone(),
                                            new_value: Some(bd.dep.new_value.clone().unwrap_or_else(|| latest.clone())),
                                            current_value: Some(current.clone()),
                                            manager_data,
                                            ..Default::default()
                                        };
                                        renovate_core::extractors::gomod::gomod_update_dependency(
                                            &content, &upgrade,
                                        )
                                    }
                                    "maven" => {
                                        let file_replace_position = maven_extracted
                                            .iter()
                                            .find(|d| d.dep_name == bd.dep.name)
                                            .and_then(|d| d.file_replace_position);
                                        let Some(pos) = file_replace_position else {
                                            tracing::warn!(
                                                repo = %repo_slug,
                                                branch = %branch,
                                                file = %file_path,
                                                dep = %bd.dep.name,
                                                "maven dep missing file_replace_position"
                                            );
                                            continue;
                                        };
                                        let upgrade = renovate_core::extractors::maven::MavenUpdateUpgrade {
                                            dep_name: Some(bd.dep.name.clone()),
                                            new_value: Some(bd.dep.new_value.clone().unwrap_or_else(|| latest.clone())),
                                            current_value: Some(current.clone()),
                                            file_replace_position: pos,
                                            ..Default::default()
                                        };
                                        renovate_core::extractors::maven::maven_update_dependency(
                                            &content, &upgrade,
                                        )
                                    }
                                    "dockerfile" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        renovate_core::extractors::dockerfile::dockerfile_update_dependency(
                                            &content, &bd.dep.name, &new_value,
                                        )
                                    }
                                    "github-actions" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        renovate_core::extractors::github_actions::github_actions_update_dependency(
                                            &content, &bd.dep.name, current, &new_value,
                                        )
                                    }
                                    "docker-compose" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        renovate_core::extractors::docker_compose::docker_compose_update_dependency(
                                            &content, &bd.dep.name, &new_value,
                                        )
                                    }
                                    "buildkite" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        renovate_core::extractors::buildkite::buildkite_update_dependency(
                                            &content, &bd.dep.name, current, &new_value,
                                        )
                                    }
                                    "circleci" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        if bd.dep.name.contains(':') {
                                            // Docker image dep — reuse dockerfile logic.
                                            renovate_core::extractors::dockerfile::dockerfile_update_dependency(
                                                &content, &bd.dep.name, &new_value,
                                            )
                                        } else {
                                            // Orb dep.
                                            renovate_core::extractors::circleci::circleci_update_orb(
                                                &content, &bd.dep.name, current, &new_value,
                                            )
                                        }
                                    }
                                    "gitlabci" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        renovate_core::extractors::dockerfile::dockerfile_update_dependency(
                                            &content, &bd.dep.name, &new_value,
                                        )
                                    }
                                    "gitlabci-include" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        renovate_core::extractors::gitlabci_include::gitlabci_include_update_dependency(
                                            &content, &bd.dep.name, current, &new_value,
                                        )
                                    }
                                    "helm-values" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        renovate_core::extractors::helm_values::helm_values_update_dependency(
                                            &content, &bd.dep.name, current, &new_value,
                                        )
                                    }
                                    "travis" | "cloudbuild" | "droneci" | "woodpecker" | "bitbucket-pipelines" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        renovate_core::extractors::dockerfile::dockerfile_update_dependency(
                                            &content, &bd.dep.name, &new_value,
                                        )
                                    }
                                    "azure-pipelines" => {
                                        let new_value = bd.dep.new_value.clone().unwrap_or_else(|| latest.clone());
                                        // Azure Pipelines references Docker images in container resources
                                        // and task versions in `task:` references. For now, only Docker
                                        // images are supported.
                                        if bd.dep.name.contains(':') {
                                            renovate_core::extractors::dockerfile::dockerfile_update_dependency(
                                                &content, &bd.dep.name, &new_value,
                                            )
                                        } else {
                                            tracing::debug!(
                                                repo = %repo_slug,
                                                branch = %branch,
                                                file = %file_path,
                                                dep = %bd.dep.name,
                                                "azure-pipelines task update not yet supported"
                                            );
                                            None
                                        }
                                    }
                                    _ => {
                                        tracing::debug!(
                                            repo = %repo_slug,
                                            branch = %branch,
                                            file = %file_path,
                                            manager = %manager,
                                            "manifest editing not yet supported for this manager"
                                        );
                                        None
                                    }
                                };
                                match updated {
                                    Some(new_content) => content = new_content,
                                    None => {
                                        tracing::warn!(
                                            repo = %repo_slug,
                                            branch = %branch,
                                            file = %file_path,
                                            dep = %bd.dep.name,
                                            "failed to update dependency in file"
                                        );
                                    }
                                }
                            }
                        }
                        if let Err(err) = client.write_file(owner, repo, file_path, &content).await
                        {
                            tracing::error!(
                                repo = %repo_slug,
                                branch = %branch,
                                file = %file_path,
                                %err,
                                "failed to write updated manifest"
                            );
                            had_error = true;
                        } else {
                            tracing::info!(
                                repo = %repo_slug,
                                branch = %branch,
                                file = %file_path,
                                "updated manifest"
                            );
                            // Run artifact update (lockfile regeneration) for supported managers.
                            if manager == "npm" || manager == "gomod" {
                                if let Some(lock_file_dir) = client.local_working_dir() {
                                    let updated_deps: Vec<UpdatedDep> = file_deps
                                        .iter()
                                        .filter_map(|bd| {
                                            if let output::DepStatus::UpdateAvailable { ref current, .. } =
                                                bd.dep.status
                                            {
                                                Some(UpdatedDep {
                                                    dep_name: bd.dep.name.clone(),
                                                    current_value: Some(current.clone()),
                                                    new_value: bd.dep.new_value.clone(),
                                                    package_file: file_path.to_owned(),
                                                    manager: manager.to_owned(),
                                                    datasource: None,
                                                })
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();
                                    if !updated_deps.is_empty() {
                                        let artifact_input = UpdateArtifact {
                                            package_file_name: file_path.to_owned(),
                                            updated_deps,
                                            new_package_file_content: content.clone(),
                                            config: ArtifactConfig {
                                                lock_file_dir: lock_file_dir.to_path_buf(),
                                                ..Default::default()
                                            },
                                        };
                                        if let Some(runner) = artifact_registry.get(manager) {
                                            match runner.update_artifacts(&artifact_input).await {
                                                Ok(Some(results)) => {
                                                    for result in results {
                                                        if let Some(ref file_change) = result.file {
                                                            if let Err(err) = client
                                                                .write_file(
                                                                    owner,
                                                                    repo,
                                                                    &file_change.path,
                                                                    file_change.contents.as_deref().unwrap_or_default(),
                                                                )
                                                                .await
                                                            {
                                                                tracing::error!(
                                                                    repo = %repo_slug,
                                                                    branch = %branch,
                                                                    file = %file_change.path,
                                                                    %err,
                                                                    "failed to write artifact file"
                                                                );
                                                                had_error = true;
                                                            } else {
                                                                tracing::info!(
                                                                    repo = %repo_slug,
                                                                    branch = %branch,
                                                                    file = %file_change.path,
                                                                    "updated artifact file"
                                                                );
                                                            }
                                                        }
                                                        if let Some(ref err) = result.artifact_error {
                                                            tracing::error!(
                                                                repo = %repo_slug,
                                                                branch = %branch,
                                                                lock_file = %err.lock_file,
                                                                stderr = %err.stderr,
                                                                "artifact update error"
                                                            );
                                                            had_error = true;
                                                        }
                                                    }
                                                }
                                                Ok(None) => {
                                                    tracing::debug!(
                                                        repo = %repo_slug,
                                                        branch = %branch,
                                                        file = %file_path,
                                                        "no artifact changes"
                                                    );
                                                }
                                                Err(err) => {
                                                    tracing::error!(
                                                        repo = %repo_slug,
                                                        branch = %branch,
                                                        file = %file_path,
                                                        ?err,
                                                        "artifact update failed"
                                                    );
                                                    had_error = true;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    tracing::debug!(
                                        repo = %repo_slug,
                                        branch = %branch,
                                        file = %file_path,
                                        "skipping artifact update: no local working directory"
                                    );
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        tracing::warn!(
                            repo = %repo_slug,
                            branch = %branch,
                            file = %file_path,
                            "file not found for manifest editing"
                        );
                    }
                    Err(err) => {
                        tracing::error!(
                            repo = %repo_slug,
                            branch = %branch,
                            file = %file_path,
                            %err,
                            "failed to read file for manifest editing"
                        );
                        had_error = true;
                    }
                }
            }
        }
    }

    for (branch, deps) in branch_updates {
        let title = deps
            .first()
            .and_then(|d| d.dep.pr_title.as_deref())
            .unwrap_or("<no title>");
        let body = build_pr_body(&deps, &repo_cfg);
        match config.dry_run {
            Some(DryRun::Full) | Some(DryRun::Lookup) => {
                tracing::info!(
                    repo = %repo_slug,
                    branch = %branch,
                    title = %title,
                    "dry-run: would create branch and PR"
                );
            }
            _ => {
                match client
                    .create_pr(owner, repo, &branch, &target_branch, title, &body)
                    .await
                {
                    Ok(Some(pr_number)) => {
                        tracing::info!(
                            repo = %repo_slug,
                            branch = %branch,
                            pr_number,
                            "created PR"
                        );
                    }
                    Ok(None) => {
                        tracing::debug!(
                            repo = %repo_slug,
                            branch = %branch,
                            "PR already exists or skipped"
                        );
                    }
                    Err(err) => {
                        tracing::error!(
                            repo = %repo_slug,
                            branch = %branch,
                            %err,
                            "failed to create PR"
                        );
                        had_error = true;
                    }
                }
            }
        }
    }

    (Some(repo_report), had_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke-test that the NpmUpdateUpgrade mapping we build in process_repo
    /// actually works with npm_update_dependency.

    // Rust-specific: main behavior test
    #[test]
    fn npm_manifest_update_smoke() {
        let package_json = r#"{"dependencies":{"lodash":"^3.0.0"}}"#;
        let upgrade = renovate_core::extractors::npm::NpmUpdateUpgrade {
            dep_type: "dependencies".to_owned(),
            dep_name: "lodash".to_owned(),
            new_value: Some("^4.17.21".to_owned()),
            ..Default::default()
        };
        let updated = renovate_core::extractors::npm::npm_update_dependency(package_json, &upgrade)
            .expect("npm_update_dependency should succeed");
        assert!(
            updated.contains("\"lodash\": \"^4.17.21\"") || updated.contains("\"lodash\":\"^4.17.21\""),
            "updated package.json should contain new version; got: {updated}"
        );
    }

    // Rust-specific: main behavior test
    #[test]
    fn build_pr_body_includes_update_table() {
        let deps = vec![report_builders::BranchDep {
            file_path: "package.json".to_owned(),
            manager: "npm".to_owned(),
            dep: output::DepReport {
                name: "lodash".to_owned(),
                dep_type: Some("dependencies".to_owned()),
                update_type: Some("minor".to_owned()),
                status: output::DepStatus::UpdateAvailable {
                    current: "^3.0.0".to_owned(),
                    latest: "4.17.21".to_owned(),
                },
                ..output::DepReport {
                    name: String::new(),
                    branch_name: None,
                    group_name: None,
                    automerge: None,
                    labels: Vec::new(),
                    assignees: Vec::new(),
                    reviewers: Vec::new(),
                    update_type: None,
                    pr_priority: None,
                    pr_title: None,
                    release_timestamp: None,
                    current_version_timestamp: None,
                    dep_type: None,
                    package_name: None,
                    range_strategy: None,
                    follow_tag: None,
                    pin_digests: None,
                    versioning: None,
                    dependency_dashboard_approval: None,
                    replacement_name: None,
                    replacement_version: None,
                    new_value: None,
                    status: output::DepStatus::UpToDate { latest: None },
                }
            },
        }];
        let cfg = renovate_core::repo_config::RepoConfig::default();
        let body = build_pr_body(&deps, &cfg);
        assert!(body.contains("lodash"), "body should mention dep name; got: {body}");
        assert!(body.contains("^3.0.0"), "body should mention current version; got: {body}");
        assert!(body.contains("4.17.21"), "body should mention latest version; got: {body}");
    }
}
