//! sbt-plugin datasource.
//!
//! Fetches sbt plugin releases from sbt plugin repositories and Maven-compatible
//! registries. Navigates a three-level directory structure:
//! `artifact/scala_VERSION/sbt_VERSION/VERSION/`
//!
//! Renovate reference:
//! - `lib/modules/datasource/sbt-plugin/index.ts`

use std::collections::BTreeSet;

use crate::datasources::maven::parse_pom_info;
use crate::datasources::sbt_package::{extract_page_links, get_latest_version};
use crate::http::HttpClient;
use crate::versioning::maven::compare;

pub const SBT_PLUGINS_REPO: &str = "https://repo.scala-sbt.org/scalasbt/sbt-plugin-releases";
pub const MAVEN_REPO: &str = crate::datasources::sbt_package::MAVEN_REPO;

/// A single release entry from an sbt-plugin lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SbtPluginRelease {
    pub version: String,
}

/// Full release result for an sbt-plugin lookup.
#[derive(Debug, Clone)]
pub struct SbtPluginReleasesResult {
    pub releases: Vec<SbtPluginRelease>,
    pub dependency_url: String,
    pub registry_url: String,
    pub homepage: Option<String>,
    pub source_url: Option<String>,
}

/// Download page content, returning `None` on any error or non-2xx response.
async fn download_content(url: &str, http: &HttpClient) -> Option<String> {
    let url = if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{}/", url)
    };
    let resp = http.get_retrying(&url).await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.text().await.ok()
}

/// Return the last `/`-separated segment of an href.
///
/// Works for both relative paths (`scala_2.12`) and absolute URLs
/// (`https://host/path/scala_2.12`).
fn last_segment(href: &str) -> &str {
    href.rsplit('/').next().unwrap_or(href)
}

/// Filter map for both absolute and relative hrefs: return the last path
/// segment unless it starts with a dot.
fn non_dot_segment(href: &str) -> Option<String> {
    let seg = last_segment(href);
    if seg.starts_with('.') {
        None
    } else {
        Some(seg.to_string())
    }
}

/// Navigate the three-level `artifact/scala_VERSION/sbt_VERSION/VERSION/`
/// structure and collect version strings.
///
/// Mirrors `SbtPluginDatasource.resolvePluginReleases`.
async fn resolve_plugin_releases(
    search_root: &str,
    artifact: &str,
    scala_version: Option<&str>,
    http: &HttpClient,
) -> Option<Vec<String>> {
    let plugin_root = format!("{}/{}", search_root, artifact);

    let content = download_content(&plugin_root, http).await?;

    let scala_items = extract_page_links(&content, non_dot_segment);
    let scala_versions: Vec<String> = scala_items
        .iter()
        .map(|x| {
            x.strip_prefix("scala_")
                .map(str::to_owned)
                .unwrap_or_else(|| x.clone())
        })
        .collect();

    let search_versions: Vec<&str> = if let Some(sv) = scala_version {
        if scala_versions.iter().any(|v| v == sv) {
            vec![sv]
        } else {
            scala_versions.iter().map(|s| s.as_str()).collect()
        }
    } else {
        scala_versions.iter().map(|s| s.as_str()).collect()
    };

    let mut releases: BTreeSet<String> = BTreeSet::new();

    for sv in search_versions {
        let sub_root = format!("{}/scala_{}", plugin_root, sv);
        let Some(sub_content) = download_content(&sub_root, http).await else {
            continue;
        };

        let sbt_items = extract_page_links(&sub_content, non_dot_segment);
        for sbt_item in sbt_items {
            let releases_root = format!("{}/{}", sub_root, sbt_item);
            let Some(rel_content) = download_content(&releases_root, http).await else {
                continue;
            };

            let versions = extract_page_links(&rel_content, non_dot_segment);
            for v in versions {
                releases.insert(v);
            }
        }
    }

    if releases.is_empty() {
        return None;
    }

    let mut sorted: Vec<String> = releases.into_iter().collect();
    sorted.sort_by(|a, b| compare(a, b));
    Some(sorted)
}

/// Flat (sbt-package-style) artifact subdir discovery.
///
/// Mirrors `SbtPluginDatasource.getArtifactSubdirs`.
async fn get_artifact_subdirs(
    search_root: &str,
    artifact: &str,
    scala_version: Option<&str>,
    http: &HttpClient,
) -> Option<Vec<String>> {
    let content = download_content(search_root, http).await?;

    let mut subdirs = extract_page_links(&content, |href| {
        let path = last_segment(href);
        if path.starts_with(&format!("{artifact}_native"))
            || path.starts_with(&format!("{artifact}_sjs"))
        {
            return None;
        }
        if path == artifact || path.starts_with(&format!("{artifact}_")) {
            Some(path.to_string())
        } else {
            None
        }
    });

    if let Some(sv) = scala_version {
        let target = format!("{artifact}_{sv}");
        if subdirs.iter().any(|s| s == &target) {
            subdirs = vec![target];
        }
    }

    Some(subdirs)
}

/// Collect versions from artifact subdir listing.
///
/// Mirrors `SbtPluginDatasource.getPackageReleases`.
async fn get_package_releases(
    search_root: &str,
    artifact_subdirs: &[String],
    http: &HttpClient,
) -> Option<Vec<String>> {
    let mut all: BTreeSet<String> = BTreeSet::new();

    for subdir in artifact_subdirs {
        let pkg_url = format!("{}/{}", search_root, subdir);
        let Some(content) = download_content(&pkg_url, http).await else {
            continue;
        };

        let versions = extract_page_links(&content, |href| {
            let path = last_segment(href);
            if path.starts_with('.') {
                None
            } else {
                Some(path.to_string())
            }
        });

        for v in versions {
            all.insert(v);
        }
    }

    if all.is_empty() {
        return None;
    }

    let mut sorted: Vec<String> = all.into_iter().collect();
    sorted.sort_by(|a, b| compare(a, b));
    Some(sorted)
}

/// Fetch homepage and sourceUrl from POM files.
///
/// Mirrors `SbtPluginDatasource.getUrls`.
async fn get_urls(
    search_root: &str,
    artifact_dirs: &[String],
    version: &str,
    http: &HttpClient,
) -> (Option<String>, Option<String>) {
    for artifact_dir in artifact_dirs {
        let artifact = artifact_dir.split('_').next().unwrap_or(artifact_dir);
        let pom_names = if artifact_dir == artifact {
            vec![format!("{}-{}.pom", artifact_dir, version)]
        } else {
            vec![
                format!("{}-{}.pom", artifact_dir, version),
                format!("{}-{}.pom", artifact, version),
            ]
        };

        for pom_name in pom_names {
            let pom_url = format!("{}/{}/{}/{}", search_root, artifact_dir, version, pom_name);
            if let Some(body) = {
                let resp = http.get_retrying(&pom_url).await.ok();
                match resp {
                    Some(r) if r.status().is_success() => r.text().await.ok(),
                    _ => None,
                }
            } {
                let info = parse_pom_info(&body);
                // sbt-plugin strips .git suffix (sbt-plugin/index.ts line 147)
                let source_url = info
                    .source_url
                    .map(|s| s.strip_suffix(".git").map(str::to_owned).unwrap_or(s));
                return (info.homepage, source_url);
            }
        }
    }

    (None, None)
}

/// Fetch all releases for an sbt plugin `package_name` from `registry_url`.
///
/// The lookup strategy:
/// 1. Build search roots (dot-separated groupId first for non-MAVEN registries,
///    then slash-separated).
/// 2. For each search root try `resolvePluginReleases` (3-level Ivy layout).
/// 3. If that returns nothing, fall back to flat Maven directory listing
///    (`getArtifactSubdirs` + `getPackageReleases` + `getUrls`).
/// 4. Return the first registry that yields versions.
///
/// Mirrors `SbtPluginDatasource.getReleases` from
/// `lib/modules/datasource/sbt-plugin/index.ts`.
pub async fn get_plugin_releases(
    package_name: &str,
    registry_url: &str,
    http: &HttpClient,
) -> Option<SbtPluginReleasesResult> {
    let (group_id, java_artifact_id) = package_name.split_once(':')?;
    let (artifact, scala_version) = match java_artifact_id.find('_') {
        Some(i) => (&java_artifact_id[..i], Some(&java_artifact_id[i + 1..])),
        None => (java_artifact_id, None),
    };

    let group_parts: Vec<&str> = group_id.split('.').collect();
    let base = registry_url.trim_end_matches('/');

    // For MAVEN_REPO, skip dot-separated root (optimization matching TypeScript).
    let mut search_roots: Vec<String> = Vec::new();
    if !base.starts_with(MAVEN_REPO) {
        search_roots.push(format!("{}/{}", base, group_parts.join(".")));
    }
    search_roots.push(format!("{}/{}", base, group_parts.join("/")));

    for search_root in &search_roots {
        // Try Ivy-style 3-level navigation first.
        let versions = resolve_plugin_releases(search_root, artifact, scala_version, http).await;
        let urls = (None, None);

        let (versions, urls) = if let Some(v) = versions {
            (v, urls)
        } else {
            // Fall back to flat Maven-style listing.
            let Some(subdirs) =
                get_artifact_subdirs(search_root, artifact, scala_version, http).await
            else {
                continue;
            };

            let Some(v) = get_package_releases(search_root, &subdirs, http).await else {
                continue;
            };

            let latest = get_latest_version(&v.iter().map(|s| s.as_str()).collect::<Vec<_>>());
            let u = if let Some(ver) = latest {
                get_urls(search_root, &subdirs, ver, http).await
            } else {
                (None, None)
            };
            (v, u)
        };

        let dependency_url = format!("{}/{}", search_root, artifact);
        let releases = versions
            .into_iter()
            .map(|version| SbtPluginRelease { version })
            .collect();

        return Some(SbtPluginReleasesResult {
            releases,
            dependency_url,
            registry_url: base.to_string(),
            homepage: urls.0,
            source_url: urls.1,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::datasources::sbt_package::extract_page_links;

    // Ported: "parses Maven index directory" — datasource/sbt-plugin/index.spec.ts line 15
    #[test]
    fn parses_maven_index_directory() {
        let html = include_str!("testdata/sbt/maven-index.html");
        let result = extract_page_links(html, |x| {
            if x.starts_with('.') {
                None
            } else {
                Some(x.to_string())
            }
        });
        assert!(!result.is_empty());
        assert!(result.contains(&"scalatest".to_string()));
        assert!(result.contains(&"test-interface".to_string()));
        assert!(!result.iter().any(|s| s.starts_with('.')));
    }

    // Ported: "parses sbt index directory" — datasource/sbt-plugin/index.spec.ts line 23
    #[test]
    fn parses_sbt_index_directory() {
        let html = include_str!("testdata/sbt/sbt-plugins-index.html");
        let result = extract_page_links(html, |x| {
            if x.starts_with('.') {
                None
            } else {
                Some(x.to_string())
            }
        });
        assert!(!result.is_empty());
        assert!(result.contains(&"com.eed3si9n".to_string()));
        assert!(!result.iter().any(|s| s.starts_with('.')));
    }

    // Ported: "returns null in case of errors" — datasource/sbt-plugin/index.spec.ts line 39
    #[tokio::test]
    async fn returns_null_in_case_of_errors() {
        let server = MockServer::start().await;

        // All paths return 404.
        for p in [
            "/maven/org/scalatest/scalatest/",
            "/maven/org/scalatest/",
            "/maven/org.scalatest/scalatest/",
            "/maven/org.scalatest/",
        ] {
            Mock::given(method("GET"))
                .and(path(p))
                .respond_with(ResponseTemplate::new(404))
                .mount(&server)
                .await;
        }

        let http = HttpClient::new().unwrap();
        let result = get_plugin_releases(
            "org.scalatest:scalatest",
            &format!("{}/maven", server.uri()),
            &http,
        )
        .await;
        assert!(result.is_none());
    }

    // Ported: "fetches sbt plugins" — datasource/sbt-plugin/index.spec.ts line 88
    #[tokio::test]
    async fn fetches_sbt_plugins() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/foundweekends/sbt-bintray/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<html><body>",
                "<pre><a href=\"../\">../</a></pre>",
                "<pre><a href=\"scala_2.12/\">scala_2.12/</a></pre>",
                "</body></html>",
            )))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/foundweekends/sbt-bintray/scala_2.12/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<html><body>",
                "<pre><a href=\"../\">../</a></pre>",
                "<pre><a href=\"sbt_1.0/\">sbt_1.0/</a></pre>",
                "</body></html>",
            )))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/maven2/org/foundweekends/sbt-bintray/scala_2.12/sbt_1.0/",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<html><body>",
                "<pre><a href=\"../\">../</a></pre>",
                "<pre><a href=\"0.5.5/\">0.5.5/</a></pre>",
                "</body></html>",
            )))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let base = format!("{}/maven2", server.uri());
        let result = get_plugin_releases("org.foundweekends:sbt-bintray", &base, &http)
            .await
            .expect("expected Some result");

        assert_eq!(result.registry_url, base);
        assert!(
            result
                .dependency_url
                .ends_with("/org/foundweekends/sbt-bintray"),
            "dependency_url={:?}",
            result.dependency_url
        );
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "0.5.5");
    }

    // Ported: "fetches sbt plugins 2" — datasource/sbt-plugin/index.spec.ts line 157
    #[tokio::test]
    async fn fetches_sbt_plugins_2() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/foundweekends/sbt-bintray/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<html><body>",
                "<pre><a href=\"../\">../</a></pre>",
                "<pre><a href=\"scala_2.12/\">scala_2.12/</a></pre>",
                "</body></html>",
            )))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/foundweekends/sbt-bintray/scala_2.12/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<html><body>",
                "<pre><a href=\"../\">../</a></pre>",
                "<pre><a href=\"sbt_1.0/\">sbt_1.0/</a></pre>",
                "</body></html>",
            )))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/maven2/org/foundweekends/sbt-bintray/scala_2.12/sbt_1.0/",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<html><body>",
                "<pre><a href=\"../\">../</a></pre>",
                "<pre><a href=\"0.5.5/\">0.5.5/</a></pre>",
                "</body></html>",
            )))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let base = format!("{}/maven2", server.uri());
        // Package name includes scala version suffix (_2.12)
        let result = get_plugin_releases("org.foundweekends:sbt-bintray_2.12", &base, &http)
            .await
            .expect("expected Some result");

        assert_eq!(result.registry_url, base);
        assert!(
            result
                .dependency_url
                .ends_with("/org/foundweekends/sbt-bintray")
        );
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "0.5.5");
    }

    // Ported: "extracts URL from Maven POM file" — datasource/sbt-plugin/index.spec.ts line 226
    #[tokio::test]
    async fn extracts_url_from_maven_pom_file() {
        let server = MockServer::start().await;

        // Root listing for io.get-coursier
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<a href=\"../\">../</a>\n",
                "<a href=\"sbt-coursier_2.10_0.13/\">sbt-coursier_2.10_0.13/</a>\n",
                "<a href=\"sbt-coursier_2.12_1.0/\">sbt-coursier_2.12_1.0/</a>\n",
                "<a href=\"sbt-coursier_2.12_1.0.0-M5/\">sbt-coursier_2.12_1.0.0-M5/</a>\n",
                "<a href=\"sbt-coursier_2.12_1.0.0-M6/\">sbt-coursier_2.12_1.0.0-M6/</a>",
            )))
            .mount(&server)
            .await;

        // resolvePluginReleases tries sbt-coursier/ first — 404
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        // Version listings for each subdir
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<a href=\"2.0.0-RC2/\">2.0.0-RC2/</a>\n",
                "<a href=\"2.0.0-RC6-1/\">2.0.0-RC6-1/</a>\n",
                "<a href=\"2.0.0-RC6-2/\">2.0.0-RC6-2/</a>\n",
                "<a href=\"2.0.0-RC6-6/\">2.0.0-RC6-6/</a>",
            )))
            .mount(&server)
            .await;
        // Other subdirs return 404
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.10_0.13/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0.0-M5/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0.0-M6/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        // POM files
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.10_0.13/2.0.0-RC6-6/sbt-coursier_2.10_0.13-2.0.0-RC6-6.pom"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.10_0.13/2.0.0-RC6-6/sbt-coursier-2.0.0-RC6-6.pom"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0/2.0.0-RC6-6/sbt-coursier_2.12_1.0-2.0.0-RC6-6.pom"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0/2.0.0-RC6-6/sbt-coursier-2.0.0-RC6-6.pom"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(
                    "<project>\
                       <url>https://get-coursier.io/</url>\
                       <scm><url>https://github.com/coursier/sbt-coursier</url></scm>\
                     </project>",
                ),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let base = format!("{}/maven2", server.uri());
        let result = get_plugin_releases("io.get-coursier:sbt-coursier", &base, &http)
            .await
            .expect("expected Some result");

        assert!(
            result
                .dependency_url
                .ends_with("/io/get-coursier/sbt-coursier")
        );
        assert_eq!(result.releases.len(), 4);
        assert_eq!(result.homepage.as_deref(), Some("https://get-coursier.io/"));
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/coursier/sbt-coursier")
        );
    }

    // Ported: "handles absolute and root relative paths" — datasource/sbt-plugin/index.spec.ts line 312
    #[tokio::test]
    async fn handles_absolute_and_root_relative_paths() {
        let server = MockServer::start().await;
        let base = format!("{}/maven2", server.uri());

        // Root listing uses absolute hrefs
        let root_html = format!(
            concat!(
                "<a href=\"{base}/io/\">../</a>\n",
                "<a href=\"{base}/io/get-coursier/sbt-coursier_2.10_0.13/\">sbt-coursier_2.10_0.13/</a>\n",
                "<a href=\"{base}/io/get-coursier/sbt-coursier_2.12_1.0/\">sbt-coursier_2.12_1.0/</a>\n",
                "<a href=\"{base}/io/get-coursier/sbt-coursier_2.12_1.0.0-M5/\">sbt-coursier_2.12_1.0.0-M5/</a>\n",
                "<a href=\"{base}/io/get-coursier/sbt-coursier_2.12_1.0.0-M6/\">sbt-coursier_2.12_1.0.0-M6/</a>",
            ),
            base = base
        );
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(root_html))
            .mount(&server)
            .await;

        // resolvePluginReleases tries sbt-coursier/ first — 404
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        // Version listing uses mixed absolute and root-relative hrefs
        let ver_html = format!(
            concat!(
                "<a href=\"{base}/io/get-coursier/sbt-coursier_2.12_1.0/2.0.0-RC2/\">2.0.0-RC2/</a>\n",
                "<a href=\"{base}/io/get-coursier/sbt-coursier_2.12_1.0/2.0.0-RC6-1/\">2.0.0-RC6-1/</a>\n",
                "<a href=\"/maven2/io/get-coursier/sbt-coursier_2.12_1.0/2.0.0-RC6-2/\">2.0.0-RC6-2/</a>\n",
                "<a href=\"/maven2/io/get-coursier/sbt-coursier_2.12_1.0/2.0.0-RC6-6/\">2.0.0-RC6-6/</a>",
            ),
            base = base
        );
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(ver_html))
            .mount(&server)
            .await;

        // Other subdirs 404
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.10_0.13/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0.0-M5/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0.0-M6/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        // POM files
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.10_0.13/2.0.0-RC6-6/sbt-coursier_2.10_0.13-2.0.0-RC6-6.pom"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.10_0.13/2.0.0-RC6-6/sbt-coursier-2.0.0-RC6-6.pom"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0/2.0.0-RC6-6/sbt-coursier_2.12_1.0-2.0.0-RC6-6.pom"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/io/get-coursier/sbt-coursier_2.12_1.0/2.0.0-RC6-6/sbt-coursier-2.0.0-RC6-6.pom"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(
                    "<project>\
                       <url>https://get-coursier.io/</url>\
                       <scm><url>https://github.com/coursier/sbt-coursier</url></scm>\
                     </project>",
                ),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_plugin_releases("io.get-coursier:sbt-coursier", &base, &http)
            .await
            .expect("expected Some result");

        assert!(
            result
                .dependency_url
                .ends_with("/io/get-coursier/sbt-coursier")
        );
        assert_eq!(result.releases.len(), 4);
        assert_eq!(result.homepage.as_deref(), Some("https://get-coursier.io/"));
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/coursier/sbt-coursier")
        );
    }
}
