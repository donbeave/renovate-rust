//! Version file managers: asdf, mise, tool-specific version files, Devbox.

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

    // ── asdf (.tool-versions) ─────────────────────────────────────────────────
    for asdf_path in manager_files(detected, "asdf") {
        match client.get_raw_file(owner, repo, &asdf_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::asdf::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.tool_name, "asdf")
                    })
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
                            Ok(Some((tag, _))) => {
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
                    } else if !repo_cfg.is_dep_ignored_for_manager(&dep.tool_name, "asdf") {
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
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.tool_name.clone(),
                        status,
                    });
                }

                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── mise (mise.toml / .mise.toml) ────────────────────────────────────────
    for mise_path in manager_files(detected, "mise") {
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
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.tool_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: mise_path.clone(),
                    manager: "mise".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%mise_path, "mise.toml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%mise_path, %err, "failed to fetch mise.toml");
                ctx.had_error = true;
            }
        }
    }

    // ── Version files (.terraform-version, .go-version, .bun-version, etc.) ──
    for manager_name in [
        "terraform-version",
        "terragrunt-version",
        "go-version",
        "python-version",
        "pyenv", // .python-version (Renovate alias)
        "node-version",
        "nodenv", // .node-version (Renovate alias)
        "nvmrc",
        "nvm", // .nvmrc (Renovate alias)
        "bun-version",
        "bazelisk",
        "ruby-version",
    ] {
        for vf_path in manager_files(detected, manager_name) {
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
                            .map(|r| r.map(|(tag, _)| tag))
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

                    ctx.report.files.push(output::FileReport {
                        path: vf_path.clone(),
                        manager: manager_name.to_owned(),
                        deps: vec![output::DepReport {
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
                            dependency_dashboard_approval: None,
                            replacement_name: None,
                            replacement_version: None,
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
                    ctx.had_error = true;
                }
            }
        }
    }

    // ── Devbox (devbox.json) ──────────────────────────────────────────────────
    for db_path in manager_files(detected, "devbox") {
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
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: db_path.clone(),
                    manager: "devbox".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%db_path, "devbox.json not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%db_path, %err, "failed to fetch devbox.json");
                ctx.had_error = true;
            }
        }
    }
}
