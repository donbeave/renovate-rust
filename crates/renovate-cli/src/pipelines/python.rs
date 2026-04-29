//! Python ecosystem managers: pip, pip-compile, setup.py/cfg, Pipfile, pep621, Poetry, PEP 723, Pixi.

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let http = ctx.http;
    let config = ctx.config;
    let _ = config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let detected = ctx.detected;

    // ── pip_requirements + pip-compile: shared PyPI dedup ─────────────────────
    // Both managers use the same extractor and PyPI datasource. Deduplicate
    // package lookups across all their files in one batch.
    {
        // Collect (manager_name, file_path, file_content) for both managers.
        let pip_managers = [
            (
                "pip_requirements",
                manager_files(detected, "pip_requirements"),
            ),
            ("pip-compile", manager_files(detected, "pip-compile")),
        ];
        let mut pip_file_deps: Vec<(
            &'static str,
            String,
            Vec<renovate_core::extractors::pip::PipExtractedDep>,
        )> = Vec::new();
        for (manager_name, files) in &pip_managers {
            for file_path in files {
                match client.get_raw_file(owner, repo, file_path).await {
                    Ok(Some(raw)) => match pip_extractor::extract(&raw.content) {
                        Ok(deps) => pip_file_deps.push((manager_name, file_path.clone(), deps)),
                        Err(err) => tracing::warn!(repo=%repo_slug, file=%file_path, %err,
                            "failed to parse pip requirements"),
                    },
                    Ok(None) => tracing::warn!(repo=%repo_slug, file=%file_path,
                        "pip requirements file not found"),
                    Err(err) => {
                        tracing::error!(repo=%repo_slug, file=%file_path, %err,
                            "failed to fetch pip requirements file");
                        ctx.had_error = true;
                    }
                }
            }
        }
        let unique_pkg_names: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            pip_file_deps
                .iter()
                .flat_map(|(_, _, deps)| deps.iter())
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.name, "pip-compile")
                })
                .filter(|d| seen.insert(d.name.clone()))
                .map(|d| d.name.clone())
                .collect()
        };
        tracing::debug!(
            repo = %repo_slug,
            files = pip_file_deps.len(),
            unique_packages = unique_pkg_names.len(),
            "fetching pip package versions (deduplicated)"
        );
        let versions_cache = pypi_datasource::fetch_versions_batch(
            http,
            &unique_pkg_names,
            pypi_datasource::PYPI_API,
            10,
        )
        .await;
        for (manager_name, file_path, deps) in pip_file_deps {
            let actionable: Vec<_> = deps
                .iter()
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.name, "pip-compile")
                })
                .collect();
            let update_map: HashMap<
                _,
                Result<
                    renovate_core::versioning::pep440::Pep440UpdateSummary,
                    pypi_datasource::PypiError,
                >,
            > = actionable
                .iter()
                .map(|d| {
                    let summary = versions_cache
                        .get(&d.name)
                        .map(|entry| pypi_datasource::summary_from_cache(&d.current_value, entry))
                        .ok_or(pypi_datasource::PypiError::NotFound(d.name.clone()));
                    (d.name.clone(), summary)
                })
                .collect();
            ctx.report.files.push(output::FileReport {
                path: file_path.clone(),
                manager: manager_name.to_owned(),
                deps: build_dep_reports_pip(&deps, &actionable, &update_map),
            });
        }
    }

    // ── setup.py (pip_setup) ─────────────────────────────────────────────────
    for setup_py_path in manager_files(detected, "pip_setup") {
        match client.get_raw_file(owner, repo, &setup_py_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::pip_setup::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.name, "pip_setup")
                    })
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
                ctx.report.files.push(output::FileReport {
                    path: setup_py_path.clone(),
                    manager: "pip_setup".into(),
                    deps: build_dep_reports_pip(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%setup_py_path, "setup.py not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%setup_py_path, %err, "failed to fetch setup.py");
                ctx.had_error = true;
            }
        }
    }

    // ── setup.cfg ────────────────────────────────────────────────────────────
    for setup_cfg_path in manager_files(detected, "setup-cfg") {
        match client.get_raw_file(owner, repo, &setup_cfg_path).await {
            Ok(Some(raw)) => {
                let deps = setup_cfg_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.name, "setup-cfg")
                    })
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
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── homeassistant-manifest ────────────────────────────────────────────────
    for ha_path in manager_files(detected, "homeassistant-manifest") {
        match client.get_raw_file(owner, repo, &ha_path).await {
            Ok(Some(raw)) => {
                let deps = homeassistant_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg
                                .is_dep_ignored_for_manager(&d.name, "homeassistant-manifest")
                    })
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
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Pipfile (pipenv) ──────────────────────────────────────────────────────
    for pipfile_path in manager_files(detected, "pipenv") {
        match client.get_raw_file(owner, repo, &pipfile_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::pipfile::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.name, "pipenv")
                    })
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
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── pep621 (pyproject.toml) ───────────────────────────────────────────────
    for pep621_file_path in manager_files(detected, "pep621") {
        match client.get_raw_file(owner, repo, &pep621_file_path).await {
            Ok(Some(raw)) => match pep621_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| {
                            d.skip_reason.is_none()
                                && !repo_cfg.is_dep_ignored_for_manager(&d.name, "pep621")
                        })
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
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                    .to_lowercase(),
                            },
                        });
                    }
                    for dep in &actionable {
                        let summary = update_map.get(&dep.name);
                        let release_timestamp = summary
                            .and_then(|r| r.as_ref().ok())
                            .and_then(|s| s.latest_timestamp.clone());
                        let current_version_timestamp = summary
                            .and_then(|r| r.as_ref().ok())
                            .and_then(|s| s.current_version_timestamp.clone());
                        let status = match summary {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_priority: None,
                            pr_title: None,
                            release_timestamp,
                            current_version_timestamp,
                            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
                            package_name: None,
                            name: dep.name.clone(),
                            status,
                        });
                    }
                    ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Poetry (pyproject.toml) ───────────────────────────────────────────────
    for poetry_file_path in manager_files(detected, "poetry") {
        match client.get_raw_file(owner, repo, &poetry_file_path).await {
            Ok(Some(raw)) => match poetry_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| {
                            d.skip_reason.is_none()
                                && !repo_cfg.is_dep_ignored_for_manager(&d.name, "poetry")
                        })
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
                    ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── PEP 723 inline script metadata ────────────────────────────────────────
    for pep723_path in manager_files(detected, "pep723") {
        match client.get_raw_file(owner, repo, &pep723_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::pep723::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.name, "pep723")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %pep723_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted pep723 inline script deps"
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
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|d| {
                        if let Some(ref reason) = d.skip_reason {
                            return output::DepReport {
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
                                name: d.name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{reason:?}").to_lowercase(),
                                },
                            };
                        }
                        let summary = update_map.get(&d.name);
                        let release_timestamp = summary
                            .and_then(|r| r.as_ref().ok())
                            .and_then(|s| s.latest_timestamp.clone());
                        let status = match summary {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_priority: None,
                            pr_title: None,
                            release_timestamp,
                            current_version_timestamp: None,
                            dep_type: None,
                            package_name: None,
                            name: d.name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: pep723_path.clone(),
                    manager: "pep723".into(),
                    deps: dep_reports,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pep723_path, "Python script not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pep723_path, %err, "failed to fetch Python script");
                ctx.had_error = true;
            }
        }
    }

    // ── Pixi (pixi.toml) ──────────────────────────────────────────────────────
    for pixi_path in manager_files(detected, "pixi") {
        match client.get_raw_file(owner, repo, &pixi_path).await {
            Ok(Some(raw)) => {
                use renovate_core::datasources::conda as conda_datasource;
                use renovate_core::extractors::pixi::{PixiSkipReason, PixiSource};
                let deps = renovate_core::extractors::pixi::extract(&raw.content);
                let pypi_actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.source == PixiSource::Pypi
                            && d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "pixi")
                    })
                    .collect();
                let conda_actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.source == PixiSource::Conda
                            && d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "pixi")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %pixi_path,
                    total = deps.len(),
                    pypi = pypi_actionable.len(),
                    conda = conda_actionable.len(),
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
                let pypi_map: HashMap<_, _> = pypi_updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                // Conda lookups (sequential to keep API load manageable).
                let mut conda_map: HashMap<
                    String,
                    Result<conda_datasource::CondaUpdateSummary, String>,
                > = HashMap::new();
                for d in &conda_actionable {
                    let result =
                        conda_datasource::fetch_latest(&d.dep_name, &d.current_value, http)
                            .await
                            .map_err(|e| e.to_string());
                    conda_map.insert(d.dep_name.clone(), result);
                }
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        if let Some(reason) = &dep.skip_reason {
                            return output::DepReport {
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
                                name: dep.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: match reason {
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
                            let status = match conda_map.get(&dep.dep_name) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.current_value.clone(),
                                        latest: s.latest.clone().unwrap_or_default(),
                                    }
                                }
                                Some(Ok(s)) => output::DepStatus::UpToDate {
                                    latest: s.latest.clone(),
                                },
                                Some(Err(e)) => {
                                    output::DepStatus::LookupError { message: e.clone() }
                                }
                                None => output::DepStatus::UpToDate { latest: None },
                            };
                            return output::DepReport {
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
                                name: dep.dep_name.clone(),
                                status,
                            };
                        }
                        let status = match pypi_map.get(&dep.dep_name) {
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
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }
}
