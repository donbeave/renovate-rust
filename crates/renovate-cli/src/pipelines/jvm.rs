//! JVM ecosystem managers: Maven, Gradle, Kotlin Script, Ant, SBT, OSGi, Scalafmt, Clojure, Leiningen.

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

    // ── Apache Ant (build.xml) ────────────────────────────────────────────────
    for ant_path in manager_files(detected, "ant") {
        match client.get_raw_file(owner, repo, &ant_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ant::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "ant")
                    })
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
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: ant_path.clone(),
                        manager: "ant".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%ant_path, "build.xml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ant_path, %err, "failed to fetch build.xml");
                ctx.had_error = true;
            }
        }
    }

    // ── Maven (pom.xml) ───────────────────────────────────────────────────────
    // Two-pass Maven dedup: Maven multi-module projects share many dependencies
    // across parent + child POMs. Batch-fetch per unique groupId:artifactId.
    {
        let maven_files = manager_files(detected, "maven");
        let mut maven_file_deps: Vec<(
            String,
            Vec<renovate_core::extractors::maven::MavenExtractedDep>,
        )> = Vec::new();
        for maven_file_path in &maven_files {
            match client.get_raw_file(owner, repo, maven_file_path).await {
                Ok(Some(raw)) => match maven_extractor::extract(&raw.content) {
                    Ok(deps) => maven_file_deps.push((maven_file_path.clone(), deps)),
                    Err(err) => tracing::warn!(repo=%repo_slug, file=%maven_file_path, %err,
                        "failed to parse pom.xml"),
                },
                Ok(None) => {
                    tracing::warn!(repo=%repo_slug, file=%maven_file_path, "pom.xml not found")
                }
                Err(err) => {
                    tracing::error!(repo=%repo_slug, file=%maven_file_path, %err,
                        "failed to fetch pom.xml");
                    ctx.had_error = true;
                }
            }
        }
        let unique_coords: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            maven_file_deps
                .iter()
                .flat_map(|(_, deps)| deps.iter())
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "maven")
                        && !d.current_value.is_empty()
                })
                .filter(|d| seen.insert(d.dep_name.clone()))
                .map(|d| d.dep_name.clone())
                .collect()
        };
        tracing::debug!(
            repo = %repo_slug,
            files = maven_file_deps.len(),
            unique_artifacts = unique_coords.len(),
            "fetching maven versions (deduplicated)"
        );
        let latest_cache = maven_datasource::fetch_latest_batch(http, &unique_coords, 10).await;
        for (maven_file_path, deps) in maven_file_deps {
            let actionable: Vec<_> = deps
                .iter()
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "maven")
                        && !d.current_value.is_empty()
                })
                .collect();
            let update_map: HashMap<_, Result<maven_datasource::MavenUpdateSummary, _>> =
                actionable
                    .iter()
                    .map(|d| {
                        let latest = latest_cache.get(&d.dep_name).cloned().unwrap_or(None);
                        let summary = Ok::<_, maven_datasource::MavenError>(
                            maven_datasource::summary_from_cache(&d.current_value, &latest),
                        );
                        (d.dep_name.clone(), summary)
                    })
                    .collect();
            ctx.report.files.push(output::FileReport {
                path: maven_file_path.clone(),
                manager: "maven".into(),
                deps: build_dep_reports_maven(&deps, &actionable, &update_map),
            });
        }
    }

    // ── Kotlin Script (*.main.kts) ────────────────────────────────────────────
    for kts_path in manager_files(detected, "kotlin-script") {
        match client.get_raw_file(owner, repo, &kts_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::kotlin_script::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "kotlin-script"))
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
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── OSGi feature model (src/main/features/*.json) ─────────────────────────
    for osgi_path in manager_files(detected, "osgi") {
        match client.get_raw_file(owner, repo, &osgi_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::osgi::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "osgi")
                    })
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
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Gradle (.gradle / .gradle.kts / .versions.toml) ──────────────────────
    for gradle_file_path in manager_files(detected, "gradle") {
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
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Gradle Wrapper (gradle/wrapper/gradle-wrapper.properties) ────────────
    for gw_path in manager_files(detected, "gradle-wrapper") {
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
                    ctx.report.files.push(output::FileReport {
                        path: gw_path.clone(),
                        manager: "gradle-wrapper".into(),
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
                ctx.had_error = true;
            }
        }
    }

    // ── Maven Wrapper (.mvn/wrapper/maven-wrapper.properties) ────────────────
    for mw_path in manager_files(detected, "maven-wrapper") {
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
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !file_deps.is_empty() {
                    ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Scalafmt (.scalafmt.conf) ─────────────────────────────────────────────
    for sfmt_path in manager_files(detected, "scalafmt") {
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
                    ctx.report.files.push(output::FileReport {
                        path: sfmt_path.clone(),
                        manager: "scalafmt".into(),
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
                ctx.had_error = true;
            }
        }
    }

    // ── SBT (build.sbt / project/*.scala / project/build.properties) ────────
    for sbt_path in manager_files(detected, "sbt") {
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
                            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
                            name: dn,
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Clojure deps.edn / bb.edn ────────────────────────────────────────────
    for edn_path in manager_files(detected, "deps-edn") {
        match client.get_raw_file(owner, repo, &edn_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::deps_edn::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %edn_path,
                    total = deps.len(), "extracted deps-edn deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "deps-edn") {
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
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: edn_path.clone(),
                    manager: "deps-edn".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%edn_path, "deps.edn not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%edn_path, %err, "failed to fetch deps.edn");
                ctx.had_error = true;
            }
        }
    }

    // ── Leiningen (project.clj) ───────────────────────────────────────────────
    for lein_path in manager_files(detected, "leiningen") {
        match client.get_raw_file(owner, repo, &lein_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::leiningen::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "leiningen"))
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
                            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }
}
