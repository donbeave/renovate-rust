//! Miscellaneous managers: Bazel, Nix, pre-commit, Ansible, Puppet, Jenkins, Conan, Haskell, and others.

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let http = ctx.http;
    let config = ctx.config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let detected = ctx.detected;
    let gh_api_base = github_tags_datasource::api_base_from_endpoint(config.endpoint.as_deref());
    let gh_http = if let Some(ref token) = config.token {
        HttpClient::with_token(token).unwrap_or_else(|_| http.clone())
    } else {
        http.clone()
    };
    let filtered_files = ctx.filtered_files;

    // ── Typst (*.typ) ────────────────────────────────────────────────────────
    for typ_path in manager_files(detected, "typst") {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.package_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.package_name, "typst") {
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.package_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: typ_path.clone(),
                        manager: "typst".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%typ_path, "typst file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%typ_path, %err, "failed to fetch typst file");
                ctx.had_error = true;
            }
        }
    }

    // ── cpanfile (Perl) ──────────────────────────────────────────────────────
    for cpan_path in manager_files(detected, "cpanfile") {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "cpanfile") {
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: cpan_path.clone(),
                        manager: "cpanfile".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cpan_path, "cpanfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cpan_path, %err, "failed to fetch cpanfile");
                ctx.had_error = true;
            }
        }
    }

    // ── Bazel Module (MODULE.bazel) ───────────────────────────────────────────
    for bm_path in manager_files(detected, "bazel-module") {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "bazel-module") {
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: bm_path.clone(),
                        manager: "bazel-module".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%bm_path, "MODULE.bazel not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bm_path, %err, "failed to fetch MODULE.bazel");
                ctx.had_error = true;
            }
        }
    }

    // ── Bazel WORKSPACE / .bzl http_archive() ────────────────────────────────
    for bazel_path in manager_files(detected, "bazel") {
        match client.get_raw_file(owner, repo, &bazel_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::bazel::{BazelSkipReason, BazelSource};
                let deps = renovate_core::extractors::bazel::extract(&raw.content);
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "bazel")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %bazel_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted bazel http_archive deps"
                );
                let mut dep_reports: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    BazelSkipReason::NoGithubUrl => "no-github-url".to_owned(),
                                    BazelSkipReason::MissingSha256 => "missing-sha256".to_owned(),
                                },
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "bazel") {
                        continue;
                    }
                    let status = match &dep.source {
                        BazelSource::GithubTags { repo: gh_repo } => {
                            match renovate_core::datasources::github_tags::fetch_latest_tag(
                                gh_repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        &dep.current_value, Some(stripped),
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
                            }
                        }
                        BazelSource::GithubReleases { repo: gh_repo } => {
                            match github_releases_datasource::fetch_latest_release(
                                gh_repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        &dep.current_value, Some(stripped),
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
                            }
                        }
                        BazelSource::Unsupported => output::DepStatus::Skipped {
                            reason: "no-github-url".to_owned(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: bazel_path.clone(),
                        manager: "bazel".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%bazel_path, "bazel file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bazel_path, %err, "failed to fetch bazel file");
                ctx.had_error = true;
            }
        }
    }

    // ── pre-commit (.pre-commit-config.yaml) ──────────────────────────────────
    for pc_path in manager_files(detected, "pre-commit") {
        match client.get_raw_file(owner, repo, &pc_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::pre_commit::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "pre-commit")
                    })
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Ansible Galaxy (requirements.yml) ────────────────────────────────────
    for ag_path in manager_files(detected, "ansible-galaxy") {
        match client.get_raw_file(owner, repo, &ag_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ansible_galaxy::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "ansible-galaxy")
                    })
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
                    } else if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "ansible-galaxy") {
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Ansible task files (tasks/*.yml) ─────────────────────────────────────
    for ansible_path in manager_files(detected, "ansible") {
        match client.get_raw_file(owner, repo, &ansible_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ansible::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %ansible_path, total = deps.len(), "extracted ansible images");
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Nix flakes (flake.nix / flake.lock) ──────────────────────────────────
    for flake_path in manager_files(detected, "nix") {
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
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.input_name, "nix")
                    })
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.input_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Conan (conanfile.txt / conanfile.py) ─────────────────────────────────
    for conan_path in manager_files(detected, "conan") {
        match client.get_raw_file(owner, repo, &conan_path).await {
            Ok(Some(raw)) => {
                let deps = if conan_path.ends_with(".py") {
                    renovate_core::extractors::conan::extract_py(&raw.content)
                } else {
                    renovate_core::extractors::conan::extract_txt(&raw.content)
                };
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.name, "conan")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %conan_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted conan deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        file_deps.push(output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "conan") {
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: conan_path.clone(),
                    manager: "conan".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%conan_path, "conanfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%conan_path, %err, "failed to fetch conanfile");
                ctx.had_error = true;
            }
        }
    }

    // ── Haskell Cabal (*.cabal) ───────────────────────────────────────────────
    for cabal_path in manager_files(detected, "haskell-cabal") {
        match client.get_raw_file(owner, repo, &cabal_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cabal::extract(&raw.content);
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        !repo_cfg.is_dep_ignored_for_manager(&d.package_name, "haskell-cabal")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %cabal_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted cabal deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.package_name, "haskell-cabal") {
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
                                    && !current_ver.contains(['<', '>', '&'])
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.package_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: cabal_path.clone(),
                    manager: "haskell-cabal".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cabal_path, "cabal file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cabal_path, %err, "failed to fetch cabal file");
                ctx.had_error = true;
            }
        }
    }

    // ── Jsonnet Bundler (jsonnetfile.json) ───────────────────────────────────
    for jb_path in manager_files(detected, "jsonnet-bundler") {
        match client.get_raw_file(owner, repo, &jb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::jsonnet_bundler::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        !d.github_repo.is_empty()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.remote, "jsonnet-bundler")
                    })
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.remote.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Vendir (vendir.yml) ───────────────────────────────────────────────────
    for vendir_path in manager_files(detected, "vendir") {
        match client.get_raw_file(owner, repo, &vendir_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::vendir::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored_for_manager(&d.chart_name, "vendir"))
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.chart_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Copier (.copier-answers.yml) ─────────────────────────────────────────
    for copier_path in manager_files(detected, "copier") {
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
                    ctx.report.files.push(output::FileReport {
                        path: copier_path.clone(),
                        manager: "copier".into(),
                        deps: vec![output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
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
                ctx.had_error = true;
            }
        }
    }

    // ── Batect (batect.yml / batect-bundle.yml) ───────────────────────────────
    for batect_path in manager_files(detected, "batect") {
        match client.get_raw_file(owner, repo, &batect_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::batect::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %batect_path,
                    total = deps.len(), "extracted batect images"
                );
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Batect wrapper script (`batect`) ─────────────────────────────────────
    for bw_path in manager_files(detected, "batect-wrapper") {
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
                    ctx.report.files.push(output::FileReport {
                        path: bw_path.clone(),
                        manager: "batect-wrapper".into(),
                        deps: vec![output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
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
                ctx.had_error = true;
            }
        }
    }

    // ── Git submodules (.gitmodules) ─────────────────────────────────────────
    // Note: git-submodules is disabled by default in Renovate (defaultConfig.enabled = false).
    // We still process it here; users enable it via enabledManagers or packageRules.
    // currentDigest (submodule commit SHA) is not extracted — requires the Git trees API.
    for gm_path in manager_files(detected, "git-submodules") {
        match client.get_raw_file(owner, repo, &gm_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::git_submodules::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %gm_path,
                    total = deps.len(),
                    "extracted git submodule deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "git-submodules") {
                        dep_reports.push(output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: "ignored".into(),
                            },
                        });
                        continue;
                    }
                    // Without currentDigest we cannot determine if an update is available.
                    // Report the dep with its branch as context; future slice adds digest lookup.
                    let status = if let Some(branch) = &dep.branch {
                        output::DepStatus::UpToDate {
                            latest: Some(branch.clone()),
                        }
                    } else {
                        output::DepStatus::UpToDate { latest: None }
                    };
                    dep_reports.push(output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.path.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: gm_path.clone(),
                        manager: "git-submodules".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%gm_path, ".gitmodules not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gm_path, %err, "failed to fetch .gitmodules");
                ctx.had_error = true;
            }
        }
    }

    // ── Puppet (Puppetfile) ───────────────────────────────────────────────────
    for pf_path in manager_files(detected, "puppet") {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "puppet") {
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: pf_path.clone(),
                        manager: "puppet".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%pf_path, "Puppetfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pf_path, %err, "failed to fetch Puppetfile");
                ctx.had_error = true;
            }
        }
    }

    // ── Jenkins plugins (plugins.txt / plugins.yml) ───────────────────────────
    for jenkins_path in manager_files(detected, "jenkins") {
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.artifact_id.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── OCB (OpenTelemetry Collector Builder) ─────────────────────────────────
    for ocb_path in manager_files(detected, "ocb") {
        match client.get_raw_file(owner, repo, &ocb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ocb::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %ocb_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted OCB go module deps"
                );
                let dep_inputs: Vec<gomod_datasource::GoModDepInput> = actionable
                    .iter()
                    .map(|d| {
                        // The collector otelcol_version is stored without a `v` prefix;
                        // the Go proxy always returns versions with `v`. Normalise here.
                        let ver = if d.dep_type == "collector" && !d.current_value.starts_with('v')
                        {
                            format!("v{}", d.current_value)
                        } else {
                            d.current_value.clone()
                        };
                        gomod_datasource::GoModDepInput {
                            module_path: d.dep_name.clone(),
                            current_value: ver,
                        }
                    })
                    .collect();
                let updates = gomod_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    gomod_datasource::GO_PROXY_BASE,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.module_path, r.summary))
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
                                pr_title: None,
                                release_timestamp: None,
                                current_version_timestamp: None,
                                name: d.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{reason:?}").to_lowercase(),
                                },
                            };
                        }
                        let status = match update_map.get(&d.dep_name) {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: d.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: ocb_path.clone(),
                    manager: "ocb".into(),
                    deps: dep_reports,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%ocb_path, "OCB config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ocb_path, %err, "failed to fetch OCB config");
                ctx.had_error = true;
            }
        }
    }

    // ── Homebrew formula (Formula/*.rb) ────────────────────────────────────────
    for hb_path in manager_files(detected, "homebrew") {
        match client.get_raw_file(owner, repo, &hb_path).await {
            Ok(Some(raw)) => {
                use homebrew_extractor::{GitHubUrlType, HomebrewSkipReason, HomebrewSource};
                let Some(dep) = homebrew_extractor::extract(&raw.content) else {
                    continue;
                };
                if repo_cfg.is_dep_ignored_for_manager(&dep.formula_name, "homebrew") {
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
                ctx.report.files.push(output::FileReport {
                    path: hb_path.clone(),
                    manager: "homebrew".into(),
                    deps: vec![output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
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
                ctx.had_error = true;
            }
        }
    }

    // ── Unity3D ProjectVersion.txt ─────────────────────────────────────────────
    for unity_path in manager_files(detected, "unity3d") {
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
                ctx.report.files.push(output::FileReport {
                    path: unity_path.clone(),
                    manager: "unity3d".into(),
                    deps: vec![output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
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
                ctx.had_error = true;
            }
        }
    }

    // ── Cloud Native Buildpacks (project.toml) ────────────────────────────────
    for bp_path in manager_files(detected, "buildpacks") {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
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
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "buildpacks") {
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Heroku/Render runtime.txt (Python version) ────────────────────────────
    for rt_path in manager_files(detected, "runtime-version") {
        match client.get_raw_file(owner, repo, &rt_path).await {
            Ok(Some(raw)) => {
                let Some(dep) = renovate_core::extractors::runtime_version::extract(&raw.content)
                else {
                    continue;
                };
                // Python CPython: GitHub Releases on python/cpython, tags like v3.11.0
                let status = match github_releases_datasource::fetch_latest_release(
                    "python/cpython",
                    &gh_http,
                    gh_api_base,
                )
                .await
                {
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
                    Err(e) => output::DepStatus::LookupError {
                        message: e.to_string(),
                    },
                };
                ctx.report.files.push(output::FileReport {
                    path: rt_path.clone(),
                    manager: "runtime-version".into(),
                    deps: vec![output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    }],
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%rt_path, "runtime.txt not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%rt_path, %err, "failed to fetch runtime.txt");
                ctx.had_error = true;
            }
        }
    }

    // ── Renovate config extends presets ───────────────────────────────────────
    for rc_path in manager_files(detected, "renovate-config-presets") {
        match client.get_raw_file(owner, repo, &rc_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::renovate_config_presets::{
                    PresetSkipReason, PresetSource,
                };
                let deps =
                    renovate_core::extractors::renovate_config_presets::extract(&raw.content);
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg
                                .is_dep_ignored_for_manager(&d.repo, "renovate-config-presets")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %rc_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted renovate config preset deps"
                );
                let mut dep_reports: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.repo.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    PresetSkipReason::UnspecifiedVersion => {
                                        "unspecified-version".to_owned()
                                    }
                                    PresetSkipReason::UnsupportedDatasource => {
                                        "unsupported-datasource".to_owned()
                                    }
                                },
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.repo, "renovate-config-presets") {
                        continue;
                    }
                    let status = match &dep.source {
                        PresetSource::GitHub => {
                            match github_tags_datasource::fetch_latest_tag(
                                &dep.repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        dep.current_value.trim_start_matches('v'),
                                        Some(stripped),
                                    );
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: tag,
                                        }
                                    } else {
                                        output::DepStatus::UpToDate { latest: Some(tag) }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        PresetSource::GitLab => {
                            match renovate_core::datasources::gitlab_tags::fetch_latest_tag(
                                &dep.repo,
                                http,
                                renovate_core::datasources::gitlab_tags::GITLAB_API,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        dep.current_value.trim_start_matches('v'),
                                        Some(stripped),
                                    );
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: tag,
                                        }
                                    } else {
                                        output::DepStatus::UpToDate { latest: Some(tag) }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                    };
                    dep_reports.push(output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.repo.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: rc_path.clone(),
                        manager: "renovate-config-presets".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::debug!(repo=%repo_slug, file=%rc_path, "renovate config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%rc_path, %err, "failed to fetch renovate config");
                ctx.had_error = true;
            }
        }
    }

    // ── Hermit (bin/.*.pkg filenames) ────────────────────────────────────────
    // Hermit encodes package name+version in hidden `.*.pkg` filenames inside
    // `bin/`.  We skip fetching file content and parse the path list directly.
    if !manager_files(detected, "hermit").is_empty() {
        let deps = renovate_core::extractors::hermit::extract_from_file_list(filtered_files);
        let actionable_count = deps.iter().filter(|d| d.skip_reason.is_none()).count();
        tracing::debug!(
            repo = %repo_slug,
            total = deps.len(), actionable = actionable_count,
            "extracted hermit package deps"
        );
        let mut dep_reports: Vec<output::DepReport> = Vec::new();
        for dep in &deps {
            if let Some(ref reason) = dep.skip_reason {
                dep_reports.push(output::DepReport {
                    branch_name: None,
                    group_name: None,
                    automerge: None,
                    labels: Vec::new(),
                    assignees: Vec::new(),
                    reviewers: Vec::new(),
                    pr_title: None,
                    release_timestamp: None,
                    current_version_timestamp: None,
                    name: dep.name.clone(),
                    status: output::DepStatus::Skipped {
                        reason: format!("{reason:?}").to_lowercase(),
                    },
                });
                continue;
            }
            if repo_cfg.is_dep_ignored_for_manager(&dep.name, "hermit") {
                continue;
            }
            let status = match renovate_core::datasources::hermit::fetch_latest(
                &dep.name,
                &dep.current_value,
                renovate_core::datasources::hermit::DEFAULT_REGISTRY,
                http,
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
                branch_name: None,
                group_name: None,
                automerge: None,
                labels: Vec::new(),
                assignees: Vec::new(),
                reviewers: Vec::new(),
                pr_title: None,
                release_timestamp: None,
                current_version_timestamp: None,
                name: dep.name.clone(),
                status,
            });
        }
        if !dep_reports.is_empty() {
            ctx.report.files.push(output::FileReport {
                path: "bin/".to_owned(),
                manager: "hermit".into(),
                deps: dep_reports,
            });
        }
    }
}
