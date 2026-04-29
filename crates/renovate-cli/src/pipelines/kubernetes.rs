//! Kubernetes ecosystem managers: Kustomize, raw manifests, FluxCD, Tekton, ArgoCD, Crossplane, Glasskube, Sveltos.

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

    // ── Kustomize (kustomization.yaml) ───────────────────────────────────────
    for kustomize_path in manager_files(detected, "kustomize") {
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
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
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
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
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        update_type: None,
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: helm.chart_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
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
                ctx.had_error = true;
            }
        }
    }

    // ── Kubernetes manifests (k8s/, kubernetes/, manifests/) ─────────────────
    for k8s_path in manager_files(detected, "kubernetes") {
        match client.get_raw_file(owner, repo, &k8s_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::kubernetes::{KubernetesDep, KubernetesSkipReason};
                let deps = renovate_core::extractors::kubernetes::extract(&raw.content);
                let actionable: Vec<&KubernetesDep> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.image_name, "kubernetes")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %k8s_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted kubernetes image deps"
                );
                let dep_inputs: Vec<docker_datasource::DockerDepInput> = actionable
                    .iter()
                    .map(|d| docker_datasource::DockerDepInput {
                        dep_name: format!("{}:{}", d.image_name, d.current_value),
                        image: d.image_name.clone(),
                        tag: d.current_value.clone(),
                    })
                    .collect();
                let updates = docker_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    docker_datasource::DOCKER_HUB_API,
                    8,
                )
                .await;
                let update_map: HashMap<String, _> = updates
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
                                pr_title: None,
                                release_timestamp: None,
                                current_version_timestamp: None,
                                name: dep.image_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: match reason {
                                        KubernetesSkipReason::DigestPinned => {
                                            "digest-pinned".to_owned()
                                        }
                                        KubernetesSkipReason::NonDockerHub => {
                                            "non-docker-hub".to_owned()
                                        }
                                        KubernetesSkipReason::NoVersion => "no-version".to_owned(),
                                    },
                                },
                            };
                        }
                        let key = format!("{}:{}", dep.image_name, dep.current_value);
                        let status = match update_map.get(&key) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_tag.clone(),
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
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.image_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: k8s_path.clone(),
                        manager: "kubernetes".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%k8s_path, "kubernetes manifest not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%k8s_path, %err, "failed to fetch kubernetes manifest");
                ctx.had_error = true;
            }
        }
    }

    // ── FluxCD system manifest (gotk-components.yaml) ────────────────────────
    for flux_path in manager_files(detected, "flux") {
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
                    ctx.report.files.push(output::FileReport {
                        path: flux_path.clone(),
                        manager: "flux".into(),
                        deps: vec![output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
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
                ctx.had_error = true;
            }
        }
    }

    // ── Tekton resources (tekton/) ────────────────────────────────────────────
    for tekton_path in manager_files(detected, "tekton") {
        match client.get_raw_file(owner, repo, &tekton_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::tekton::KubernetesSkipReason;
                let deps = renovate_core::extractors::tekton::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.image_name, "tekton")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %tekton_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted tekton step image deps"
                );
                let dep_inputs: Vec<docker_datasource::DockerDepInput> = actionable
                    .iter()
                    .map(|d| docker_datasource::DockerDepInput {
                        dep_name: format!("{}:{}", d.image_name, d.current_value),
                        image: d.image_name.clone(),
                        tag: d.current_value.clone(),
                    })
                    .collect();
                let updates = docker_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    docker_datasource::DOCKER_HUB_API,
                    8,
                )
                .await;
                let update_map: HashMap<String, _> = updates
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
                                pr_title: None,
                                release_timestamp: None,
                                current_version_timestamp: None,
                                name: dep.image_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: match reason {
                                        KubernetesSkipReason::DigestPinned => {
                                            "digest-pinned".to_owned()
                                        }
                                        KubernetesSkipReason::NonDockerHub => {
                                            "non-docker-hub".to_owned()
                                        }
                                        KubernetesSkipReason::NoVersion => "no-version".to_owned(),
                                    },
                                },
                            };
                        }
                        let key = format!("{}:{}", dep.image_name, dep.current_value);
                        let status = match update_map.get(&key) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_tag.clone(),
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
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.image_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: tekton_path.clone(),
                        manager: "tekton".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%tekton_path, "tekton resource not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%tekton_path, %err, "failed to fetch tekton resource");
                ctx.had_error = true;
            }
        }
    }

    // ── ArgoCD Application manifests ─────────────────────────────────────────
    for argo_path in manager_files(detected, "argocd") {
        match client.get_raw_file(owner, repo, &argo_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::argocd::{ArgocdSkipReason, ArgocdSource};
                let deps = renovate_core::extractors::argocd::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %argo_path,
                    total = deps.len(), "extracted argocd deps"
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
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    ArgocdSkipReason::UnspecifiedVersion => {
                                        "unspecified-version".to_owned()
                                    }
                                    ArgocdSkipReason::InvalidConfig => "invalid-config".to_owned(),
                                },
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "argocd") {
                        continue;
                    }
                    let status = match &dep.source {
                        ArgocdSource::Helm {
                            registry_url,
                            chart_name,
                        } => {
                            match helm_datasource::fetch_latest(chart_name, registry_url, http)
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
                        }
                        ArgocdSource::Git { repo_url } => {
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
                        ArgocdSource::Unsupported => output::DepStatus::Skipped {
                            reason: "unsupported-datasource".to_owned(),
                        },
                    };
                    dep_reports.push(output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        update_type: None,
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: argo_path.clone(),
                        manager: "argocd".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%argo_path, "argocd manifest not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%argo_path, %err, "failed to fetch argocd manifest");
                ctx.had_error = true;
            }
        }
    }

    // ── Crossplane packages (crossplane/) ─────────────────────────────────────
    for xp_path in manager_files(detected, "crossplane") {
        match client.get_raw_file(owner, repo, &xp_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::crossplane::CrossplaneSkipReason;
                let deps = renovate_core::extractors::crossplane::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %xp_path,
                    total = deps.len(), "extracted crossplane package deps"
                );
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = if let Some(reason) = &dep.skip_reason {
                            output::DepStatus::Skipped {
                                reason: match reason {
                                    CrossplaneSkipReason::UnsupportedRegistry => {
                                        "unsupported-registry".to_owned()
                                    }
                                    CrossplaneSkipReason::MissingPackage => {
                                        "missing-package".to_owned()
                                    }
                                },
                            }
                        } else {
                            output::DepStatus::UpToDate { latest: None }
                        };
                        output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: format!("{}: {}", dep.kind, dep.package),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: xp_path.clone(),
                        manager: "crossplane".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%xp_path, "crossplane manifest not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%xp_path, %err, "failed to fetch crossplane manifest");
                ctx.had_error = true;
            }
        }
    }

    // ── Glasskube packages (glasskube/) ───────────────────────────────────────
    for gk_path in manager_files(detected, "glasskube") {
        match client.get_raw_file(owner, repo, &gk_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::glasskube::extract(&raw.content);
                let actionable_count = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored_for_manager(&d.package_name, "glasskube"))
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %gk_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted glasskube package deps"
                );
                let mut dep_reports: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.package_name, "glasskube") {
                        continue;
                    }
                    let status = match renovate_core::datasources::glasskube_packages::fetch_latest(
                        http,
                        &dep.package_name,
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
                        update_type: None,
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.package_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: gk_path.clone(),
                        manager: "glasskube".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%gk_path, "glasskube manifest not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gk_path, %err, "failed to fetch glasskube manifest");
                ctx.had_error = true;
            }
        }
    }

    // ── Sveltos ClusterProfile/Profile (sveltos/) ────────────────────────────
    for sv_path in manager_files(detected, "sveltos") {
        match client.get_raw_file(owner, repo, &sv_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::sveltos::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %sv_path,
                    total = deps.len(),
                    "extracted sveltos helm chart deps"
                );
                let dep_inputs: Vec<helm_datasource::HelmDepInput> = deps
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.chart_name.clone(),
                        current_value: d.current_value.clone(),
                        repository_url: d.registry_url.clone(),
                    })
                    .collect();
                let updates = helm_datasource::fetch_updates_concurrent(http, &dep_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|d| {
                        let status = match update_map.get(&d.chart_name) {
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
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: d.chart_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: sv_path.clone(),
                    manager: "sveltos".into(),
                    deps: dep_reports,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%sv_path, "sveltos manifest not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%sv_path, %err, "failed to fetch sveltos manifest");
                ctx.had_error = true;
            }
        }
    }
}
