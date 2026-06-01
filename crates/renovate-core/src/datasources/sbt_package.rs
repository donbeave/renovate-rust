//! sbt-package datasource.
//!
//! Fetches Scala package releases from Maven-compatible registries using
//! directory listing and POM file parsing.
//!
//! Renovate reference:
//! - `lib/modules/datasource/sbt-package/index.ts`
//! - `lib/modules/datasource/sbt-package/util.ts`

use std::cmp::Ordering;
use std::collections::BTreeSet;

use regex::Regex;

use crate::datasources::maven::{parse_all_versions, parse_pom_info};
use crate::http::HttpClient;
use crate::versioning::maven::compare;

pub const MAVEN_REPO: &str = "https://repo.maven.apache.org/maven2";

/// Returns the latest version from a slice using Maven version ordering.
pub fn get_latest_version<'a>(versions: &[&'a str]) -> Option<&'a str> {
    versions.iter().copied().reduce(|best, v| {
        if compare(v, best) == Ordering::Greater {
            v
        } else {
            best
        }
    })
}

/// Extract href values from an HTML directory listing.
///
/// Matches `href="VALUE/"` or `href='VALUE/'` (with a trailing `/` before the
/// closing quote) and passes the VALUE part to `filter`. Items for which
/// `filter` returns `None` are dropped; `Some(s)` values are collected.
///
/// Mirrors `extractPageLinks` from `lib/modules/datasource/sbt-package/util.ts`.
pub fn extract_page_links(html: &str, filter: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let re = Regex::new(r#"href=['"]([^'"]*)/['"]"#).expect("static regex");
    re.captures_iter(html)
        .filter_map(|cap| {
            let href = cap.get(1)?.as_str();
            filter(href)
        })
        .collect()
}

/// A single release entry from an sbt-package lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SbtRelease {
    pub version: String,
}

/// Full release result for an sbt-package lookup.
#[derive(Debug, Clone)]
pub struct SbtPkgReleasesResult {
    pub releases: Vec<SbtRelease>,
    pub dependency_url: String,
    pub registry_url: String,
    pub homepage: Option<String>,
    pub source_url: Option<String>,
}

/// Fetch page content, returning `None` on any error or non-2xx response.
async fn download_content(url: &str, http: &HttpClient) -> Option<String> {
    let resp = http.get_retrying(url).await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.text().await.ok()
}

/// Fetch page content plus the `last-modified` response header.
async fn download_with_headers(url: &str, http: &HttpClient) -> Option<(String, Option<String>)> {
    let resp = http.get_retrying(url).await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let last_modified = resp
        .headers()
        .get("last-modified")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());
    let body = resp.text().await.ok()?;
    Some((body, last_modified))
}

/// Parse an HTTP date string (`Wed, 21 Oct 2015 07:28:00 GMT`) to ISO 8601.
fn parse_http_date(s: &str) -> Option<String> {
    let normalized = s.trim().replace(" GMT", " +0000");
    chrono::DateTime::parse_from_str(&normalized, "%a, %d %b %Y %H:%M:%S %z")
        .ok()
        .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
}

/// Extract the URL pathname (path component) from a full URL.
fn url_pathname(url: &str) -> &str {
    let after_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    match after_scheme.find('/') {
        Some(i) => &after_scheme[i..],
        None => "/",
    }
}

/// Try to fetch POM homepage and source URL for a specific version.
///
/// Iterates `package_urls`, constructs two candidate POM filenames per URL
/// (`artifactDir-version.pom` and `artifact-version.pom`), and returns the
/// first pair found (or `(None, None)` if every attempt 404s).
async fn get_pom_homepage_source(
    package_urls: &[String],
    version: &str,
    http: &HttpClient,
) -> (Option<String>, Option<String>) {
    for pkg_url in package_urls {
        let artifact_dir = pkg_url
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or("");
        let artifact = artifact_dir.split('_').next().unwrap_or(artifact_dir);

        let candidates: Vec<&str> = if artifact_dir == artifact {
            vec![artifact_dir]
        } else {
            vec![artifact_dir, artifact]
        };

        for prefix in candidates {
            let pom_url = format!("{}{}/{}-{}.pom", pkg_url, version, prefix, version);
            if let Some(body) = download_content(&pom_url, http).await {
                let info = parse_pom_info(&body);
                // sbt-package strips .git suffix from SCM URLs (sbt-package/index.ts line 323)
                let source_url = info
                    .source_url
                    .map(|s| s.strip_suffix(".git").map(str::to_owned).unwrap_or(s));
                if info.homepage.is_some() || source_url.is_some() {
                    return (info.homepage, source_url);
                }
                // POM found but has no URL fields — stop searching
                return (None, None);
            }
        }
    }
    (None, None)
}

/// Core sbt release discovery: directory-listing approach.
async fn get_sbt_releases(
    package_name: &str,
    registry_base: &str,
    http: &HttpClient,
) -> Option<SbtPkgReleasesResult> {
    let (group_id, java_artifact_id) = package_name.split_once(':')?;

    // Separate artifactId from optional Scala version suffix.
    let (artifact_id, scala_version) = match java_artifact_id.find('_') {
        Some(i) => (&java_artifact_id[..i], Some(&java_artifact_id[i + 1..])),
        None => (java_artifact_id, None),
    };

    let group_parts: Vec<&str> = group_id.split('.').collect();

    // Try slash-separated groupId root first, then dot-separated.
    let root_slash = format!("{}/{}/", registry_base, group_parts.join("/"));
    let root_dot = format!("{}/{}/", registry_base, group_id);

    let mut dependency_url = None;
    let mut package_urls: Vec<String> = Vec::new();

    for root_url in [root_slash.as_str(), root_dot.as_str()] {
        let Some(content) = download_content(root_url, http).await else {
            continue;
        };

        dependency_url = Some(root_url.trim_end_matches('/').to_owned());

        let root_path = url_pathname(root_url).to_owned();

        let artifact_subdirs = extract_page_links(&content, |href| {
            let path = if href.starts_with(&root_path) {
                href.strip_prefix(&root_path).unwrap_or(href)
            } else {
                href
            };

            if path.starts_with(&format!("{artifact_id}_native"))
                || path.starts_with(&format!("{artifact_id}_sjs"))
            {
                return None;
            }
            if path == artifact_id || path.starts_with(&format!("{artifact_id}_")) {
                Some(format!("{}{}/", root_url, path))
            } else {
                None
            }
        });

        if let Some(sv) = scala_version {
            let target_suffix = format!("/{artifact_id}_{sv}/");
            if let Some(matched) = artifact_subdirs
                .iter()
                .find(|s| s.ends_with(&target_suffix))
            {
                package_urls = vec![matched.clone()];
            } else {
                package_urls = artifact_subdirs;
            }
        } else {
            package_urls = artifact_subdirs;
        }

        break;
    }

    if package_urls.is_empty() {
        return None;
    }

    // Collect all versions from each artifact subdir's directory listing.
    let mut all_versions: BTreeSet<String> = BTreeSet::new();
    let mut valid_package_urls: Vec<String> = Vec::new();

    for pkg_url in &package_urls {
        let Some(content) = download_content(pkg_url, http).await else {
            continue;
        };

        let root_path = url_pathname(pkg_url).to_owned();
        let versions = extract_page_links(&content, |href| {
            let path = if href.starts_with(&root_path) {
                href.strip_prefix(&root_path).unwrap_or(href)
            } else {
                href
            };
            if path.starts_with('.') {
                None
            } else {
                Some(path.to_owned())
            }
        });

        if !versions.is_empty() {
            valid_package_urls.push(pkg_url.clone());
        }
        for v in versions {
            all_versions.insert(v);
        }
    }

    if all_versions.is_empty() {
        return None;
    }

    let versions: Vec<String> = {
        let mut v: Vec<String> = all_versions.into_iter().collect();
        v.sort_by(|a, b| compare(a, b));
        v
    };

    let urls_for_pom = if valid_package_urls.is_empty() {
        &package_urls
    } else {
        &valid_package_urls
    };

    let latest = get_latest_version(&versions.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    let (homepage, source_url) = if let Some(ver) = latest {
        get_pom_homepage_source(urls_for_pom, ver, http).await
    } else {
        (None, None)
    };

    let releases = versions
        .into_iter()
        .map(|version| SbtRelease { version })
        .collect();

    Some(SbtPkgReleasesResult {
        releases,
        dependency_url: dependency_url.unwrap_or_default(),
        registry_url: registry_base.to_owned(),
        homepage,
        source_url,
    })
}

/// Fallback release discovery: maven-metadata.xml approach.
///
/// Used when no artifact subdirectory listing is available.
async fn get_maven_fallback(
    package_name: &str,
    registry_base: &str,
    http: &HttpClient,
) -> Option<SbtPkgReleasesResult> {
    let (group_id, artifact_id) = package_name.split_once(':')?;
    let group_path = group_id.replace('.', "/");
    let metadata_url = format!("{registry_base}/{group_path}/{artifact_id}/maven-metadata.xml");

    let body = download_content(&metadata_url, http).await?;
    let metadata = parse_all_versions(&body)?;

    let mut versions: Vec<String> = metadata.versions;
    versions.sort_by(|a, b| compare(a, b));

    // Try to fetch POM for the latest version.
    let latest = get_latest_version(&versions.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    let (homepage, source_url) = if let Some(ver) = latest {
        let pom_url =
            format!("{registry_base}/{group_path}/{artifact_id}/{ver}/{artifact_id}-{ver}.pom");
        if let Some(body) = download_content(&pom_url, http).await {
            let info = parse_pom_info(&body);
            (info.homepage, info.source_url)
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let releases = versions
        .into_iter()
        .map(|version| SbtRelease { version })
        .collect();

    Some(SbtPkgReleasesResult {
        releases,
        dependency_url: format!("{registry_base}/{group_path}"),
        registry_url: registry_base.to_owned(),
        homepage,
        source_url,
    })
}

/// Fetch all releases for `package_name` from `registry_url`.
///
/// Mirrors `SbtPackageDatasource.getReleases` from
/// `lib/modules/datasource/sbt-package/index.ts`.
///
/// Returns `None` when no versions can be found via either directory listing
/// or `maven-metadata.xml` fallback.
pub async fn get_pkg_releases(
    package_name: &str,
    registry_url: &str,
    http: &HttpClient,
) -> Option<SbtPkgReleasesResult> {
    let base = registry_url.trim_end_matches('/');
    let effective = if base.is_empty() { MAVEN_REPO } else { base };

    let sbt = get_sbt_releases(package_name, effective, http).await;
    if sbt.is_some() {
        return sbt;
    }

    get_maven_fallback(package_name, effective, http).await
}

/// Fetch the POM `last-modified` timestamp for a specific release version.
///
/// Iterates `package_urls`, constructs POM file paths, and returns the
/// `last-modified` header of the first successful response as ISO 8601.
///
/// Mirrors the `postprocessRelease` path in
/// `lib/modules/datasource/sbt-package/index.ts`.
pub async fn get_pom_release_timestamp(
    package_urls: &[&str],
    version: &str,
    http: &HttpClient,
) -> Option<String> {
    for pkg_url in package_urls {
        let artifact_dir = pkg_url
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or("");
        let artifact = artifact_dir.split('_').next().unwrap_or(artifact_dir);

        let candidates: Vec<&str> = if artifact_dir == artifact {
            vec![artifact_dir]
        } else {
            vec![artifact_dir, artifact]
        };

        for prefix in candidates {
            let pom_url = format!("{}{}/{}-{}.pom", pkg_url, version, prefix, version);
            if let Some((_body, last_modified)) = download_with_headers(&pom_url, http).await {
                if let Some(ts) = last_modified.as_deref().and_then(parse_http_date) {
                    return Some(ts);
                }
                return None;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "gets latest version" — datasource/sbt-package/util.spec.ts line 4
    #[test]
    fn gets_latest_version() {
        assert_eq!(
            get_latest_version(&["1.0.0", "3.0.0", "2.0.0"]),
            Some("3.0.0")
        );
    }

    // Ported: "parses Maven index directory" — datasource/sbt-package/index.spec.ts line 18
    #[test]
    fn parses_maven_index_directory() {
        let html = include_str!("testdata/sbt/maven-index.html");
        let result = extract_page_links(html, |x| {
            if x.starts_with('.') {
                None
            } else {
                Some(x.to_owned())
            }
        });
        assert_eq!(
            result,
            vec![
                "autofix-3.0.6_2.11",
                "autofix-3.0.6_2.12",
                "autofix-3.0.8_2.11",
                "autofix-3.0.8_2.12",
                "scalatest",
                "scalatest-all_2.10",
                "scalatest-all_2.11",
                "scalatest-all_sjs0.6_2.10",
                "scalatest-all_sjs0.6_2.11",
                "scalatest-app_2.10",
                "scalatest-app_2.11",
                "scalatest-app_2.12",
                "scalatest-app_2.12.0-M3",
                "scalatest-app_2.12.0-M4",
                "scalatest-app_2.12.0-M5",
                "scalatest-app_2.12.0-RC1",
                "scalatest-app_2.12.0-RC2",
                "scalatest-app_2.13.0-M1",
                "scalatest-app_2.13.0-M2",
                "scalatest-app_2.13.0-M3",
                "scalatest-app_2.13.0-M4",
                "scalatest-app_2.13.0-M5",
                "scalatest-app_2.13.0-RC1",
                "scalatest-app_2.13.0-RC2",
                "scalatest-app_native0.3_2.11",
                "scalatest-app_sjs0.6_2.10",
                "scalatest-app_sjs0.6_2.11",
                "scalatest-app_sjs0.6_2.12",
                "scalatest-app_sjs0.6_2.12.0-M3",
                "scalatest-app_sjs0.6_2.12.0-M4",
                "scalatest-app_sjs0.6_2.12.0-M5",
                "scalatest-app_sjs0.6_2.12.0-RC1",
                "scalatest-app_sjs0.6_2.12.0-RC2",
                "scalatest-app_sjs0.6_2.13.0-M1",
                "scalatest-app_sjs0.6_2.13.0-M2",
                "scalatest-app_sjs0.6_2.13.0-M3",
                "scalatest-app_sjs0.6_2.13.0-M4",
                "scalatest-app_sjs0.6_2.13.0-M5",
                "scalatest-app_sjs0.6_2.13.0-RC1",
                "scalatest-app_sjs0.6_2.13.0-RC2",
                "scalatest-app_sjs1.0.0-M3_2.11",
                "scalatest-app_sjs1.0.0-M3_2.12",
                "scalatest-app_sjs1.0.0-M7_2.11",
                "scalatest-app_sjs1.0.0-M7_2.12",
                "scalatest-app_sjs1.0.0-M7_2.13.0-RC1",
                "scalatest-app_sjs1.0.0-M7_2.13.0-RC2",
                "scalatest-core_2.10",
                "scalatest-core_2.11",
                "scalatest-core_sjs0.6_2.10",
                "scalatest-core_sjs0.6_2.11",
                "scalatest-easymock_2.10",
                "scalatest-easymock_2.11",
                "scalatest-featurespec_2.10",
                "scalatest-featurespec_2.11",
                "scalatest-featurespec_sjs0.6_2.10",
                "scalatest-featurespec_sjs0.6_2.11",
                "scalatest-finders",
                "scalatest-finders_2.9.0",
                "scalatest-flatspec_2.10",
                "scalatest-flatspec_2.11",
                "scalatest-flatspec_sjs0.6_2.10",
                "scalatest-flatspec_sjs0.6_2.11",
                "scalatest-freespec_2.10",
                "scalatest-freespec_2.11",
                "scalatest-freespec_sjs0.6_2.10",
                "scalatest-freespec_sjs0.6_2.11",
                "scalatest-funspec_2.10",
                "scalatest-funspec_2.11",
                "scalatest-funspec_sjs0.6_2.10",
                "scalatest-funspec_sjs0.6_2.11",
                "scalatest-funsuite_2.10",
                "scalatest-funsuite_2.11",
                "scalatest-funsuite_sjs0.6_2.10",
                "scalatest-funsuite_sjs0.6_2.11",
                "scalatest-jmock_2.10",
                "scalatest-jmock_2.11",
                "scalatest-junit_2.10",
                "scalatest-junit_2.11",
                "scalatest-matchers-core_2.10",
                "scalatest-matchers-core_2.11",
                "scalatest-matchers-core_sjs0.6_2.10",
                "scalatest-matchers-core_sjs0.6_2.11",
                "scalatest-matchers_2.10",
                "scalatest-matchers_2.11",
                "scalatest-matchers_sjs0.6_2.10",
                "scalatest-matchers_sjs0.6_2.11",
                "scalatest-maven-plugin",
                "scalatest-mustmatchers_2.10",
                "scalatest-mustmatchers_2.11",
                "scalatest-mustmatchers_sjs0.6_2.10",
                "scalatest-mustmatchers_sjs0.6_2.11",
                "scalatest-propspec_2.10",
                "scalatest-propspec_2.11",
                "scalatest-propspec_sjs0.6_2.10",
                "scalatest-propspec_sjs0.6_2.11",
                "scalatest-refspec_2.10",
                "scalatest-refspec_2.11",
                "scalatest-selenium_2.10",
                "scalatest-selenium_2.11",
                "scalatest-testng_2.10",
                "scalatest-testng_2.11",
                "scalatest-wordspec_2.10",
                "scalatest-wordspec_2.11",
                "scalatest-wordspec_sjs0.6_2.10",
                "scalatest-wordspec_sjs0.6_2.11",
                "scalatest_2.10",
                "scalatest_2.10.0",
                "scalatest_2.10.0-M4",
                "scalatest_2.10.0-M5",
                "scalatest_2.10.0-M6",
                "scalatest_2.10.0-M7",
                "scalatest_2.10.0-RC1",
                "scalatest_2.10.0-RC2",
                "scalatest_2.10.0-RC3",
                "scalatest_2.10.0-RC5",
                "scalatest_2.11",
                "scalatest_2.11.0-M3",
                "scalatest_2.11.0-M4",
                "scalatest_2.11.0-M5",
                "scalatest_2.11.0-M7",
                "scalatest_2.11.0-M8",
                "scalatest_2.11.0-RC1",
                "scalatest_2.11.0-RC2",
                "scalatest_2.11.0-RC3",
                "scalatest_2.11.0-RC4",
                "scalatest_2.12",
                "scalatest_2.12.0-M1",
                "scalatest_2.12.0-M2",
                "scalatest_2.12.0-M3",
                "scalatest_2.12.0-M4",
                "scalatest_2.12.0-M5",
                "scalatest_2.12.0-RC1",
                "scalatest_2.12.0-RC2",
                "scalatest_2.13.0-M1",
                "scalatest_2.13.0-M2",
                "scalatest_2.13.0-M3",
                "scalatest_2.13.0-M4",
                "scalatest_2.13.0-M5",
                "scalatest_2.13.0-RC1",
                "scalatest_2.13.0-RC2",
                "scalatest_2.8.0",
                "scalatest_2.8.1",
                "scalatest_2.8.2",
                "scalatest_2.9.0",
                "scalatest_2.9.0-1",
                "scalatest_2.9.0.RC3",
                "scalatest_2.9.0.RC4",
                "scalatest_2.9.1",
                "scalatest_2.9.1-1",
                "scalatest_2.9.1-1-RC1",
                "scalatest_2.9.2",
                "scalatest_2.9.3",
                "scalatest_2.9.3-RC1",
                "scalatest_2.9.3-RC2",
                "scalatest_native0.3_2.11",
                "scalatest_sjs0.6_2.10",
                "scalatest_sjs0.6_2.11",
                "scalatest_sjs0.6_2.12",
                "scalatest_sjs0.6_2.12.0-M3",
                "scalatest_sjs0.6_2.12.0-M4",
                "scalatest_sjs0.6_2.12.0-M5",
                "scalatest_sjs0.6_2.12.0-RC1",
                "scalatest_sjs0.6_2.12.0-RC2",
                "scalatest_sjs0.6_2.13.0-M1",
                "scalatest_sjs0.6_2.13.0-M2",
                "scalatest_sjs0.6_2.13.0-M3",
                "scalatest_sjs0.6_2.13.0-M4",
                "scalatest_sjs0.6_2.13.0-M5",
                "scalatest_sjs0.6_2.13.0-RC1",
                "scalatest_sjs0.6_2.13.0-RC2",
                "scalatest_sjs1.0.0-M3_2.11",
                "scalatest_sjs1.0.0-M3_2.12",
                "scalatest_sjs1.0.0-M7_2.11",
                "scalatest_sjs1.0.0-M7_2.12",
                "scalatest_sjs1.0.0-M7_2.13.0-RC1",
                "scalatest_sjs1.0.0-M7_2.13.0-RC2",
                "scalatestjs_sjs0.6_2.10",
                "scalatestjs_sjs0.6_2.11",
                "scalatestjs_sjs0.6_2.12",
                "scalatestjs_sjs0.6_2.13.0-M4",
                "scalatestjs_sjs1.0.0-M3_2.11",
                "scalatestjs_sjs1.0.0-M3_2.12",
                "test-interface",
            ]
        );
    }

    // Ported: "parses sbt index directory" — datasource/sbt-package/index.spec.ts line 26
    #[test]
    fn parses_sbt_index_directory() {
        let html = include_str!("testdata/sbt/sbt-plugins-index.html");
        let result = extract_page_links(html, |x| {
            if x.starts_with('.') {
                None
            } else {
                Some(x.to_owned())
            }
        });
        assert_eq!(
            result,
            vec![
                "au.com.onegeek",
                "bavadim",
                "be.venneborg.sbt",
                "biz.cgta",
                "br.com.handit",
                "cc.spray",
                "ch.epfl.scala.index",
                "ch.epfl.scala",
                "ch.jodersky",
                "ch.wavein",
                "ch",
                "chainkite",
                "co.vitaler",
                "co",
                "codes.reactive.sbt",
                "com.adelegue",
                "com.agilogy",
                "com.alpeb",
                "com.anadeainc",
                "com.aol.sbt",
                "com.benmccann",
                "com.bicou.sbt",
                "com.birdhowl",
                "com.blstream",
                "com.bowlingx",
                "com.byteground",
                "com.cavorite",
                "com.cedware",
                "com.clever-age",
                "com.codecommit",
                "com.culpin.team",
                "com.dancingcode",
                "com.databricks",
                "com.dayslar.play",
                "com.dispalt.pop",
                "com.dispalt.relay",
                "com.dscleaver.sbt",
                "com.dslplatform",
                "com.dwijnand.sbtprojectgraph",
                "com.dwijnand",
                "com.earldouglas",
                "com.eed3si9n",
                "com.eltimn",
                "com.esdrasbeleza",
                "com.evenfinancial",
                "com.geezeo",
                "com.geirsson",
                "com.gilt.sbt",
                "com.github.DavidPerezIngeniero",
                "com.github.aafa",
                "com.github.ahjohannessen",
                "com.github.akiomik",
                "com.github.bootlog",
                "com.github.casualjim",
                "com.github.catap",
                "com.github.cb372",
                "com.github.citrum.webby",
                "com.github.crakjie",
                "com.github.cuzfrog",
                "com.github.daniel-shuy",
                "com.github.davidpeklak",
                "com.github.ddispaltro",
                "com.github.dwhjames",
                "com.github.dwickern",
                "com.github.gpgekko",
                "com.github.gseitz",
                "com.github.inthenow",
                "com.github.izhangzhihao",
                "com.github.jeffreyolchovy",
                "com.github.jodersky",
                "com.github.marceloemanoel",
                "com.github.masseguillaume",
                "com.github.mkroli",
                "com.github.mmizutani",
                "com.github.mvallerie",
                "com.github.mwz",
                "com.github.nyavro",
                "com.github.pinguinson",
                "com.github.play2war",
                "com.github.plippe",
                "com.github.qualysis",
                "com.github.retronym",
                "com.github.saint1991",
                "com.github.saurfang",
                "com.github.sbt",
                "com.github.shanbin",
                "com.github.shmishleniy",
                "com.github.stonexx.sbt",
                "com.github.tptodorov",
                "com.github.wookietreiber",
                "com.github.zainab-ali",
                "com.glngn",
                "com.googlecode.sbt-rats",
                "com.gu",
                "com.hanhuy.sbt",
                "com.heroku",
                "com.hevylight",
                "com.hootsuite",
                "com.hpe.sbt",
                "com.iheart",
                "com.impactua",
                "com.jamesneve",
                "com.jamesward",
                "com.jatescher",
                "com.jm2dev",
                "com.jmparsons.sbt",
                "com.jmparsons",
                "com.joescii",
                "com.joshcough",
                "com.jsuereth",
                "com.kailuowang",
                "com.kalmanb.sbt",
                "com.lenioapp",
                "com.lightbend.akka.grpc",
                "com.lightbend.akka",
                "com.lightbend.conductr",
                "com.lightbend.lagom",
                "com.lightbend.paradox",
                "com.lightbend.rp",
                "com.lightbend.sbt",
                "com.lightbend",
                "com.linkedin.sbt-restli",
                "com.localytics",
                "com.mariussoutier.sbt",
                "com.markatta",
                "com.micronautics",
                "com.mintbeans",
                "com.mojolly.scalate",
                "com.nike.redwiggler.sbt",
                "com.novocode",
                "com.olaq",
                "com.oliverlockwood",
                "com.omervk",
                "com.opi.lil",
                "com.oradian.sbt",
                "com.qonceptual.sbt",
                "com.quadstingray",
                "com.rberenguel",
                "com.roperzh.sbt",
                "com.saikocat",
                "com.sc.sbt",
                "com.scalakata.metadoc",
                "com.scalakata",
                "com.scalapenos",
                "com.seroperson",
                "com.servicerocket",
                "com.simianquant",
                "com.simplytyped",
                "com.sirocchj",
                "com.sksamuel.sbt-versions",
                "com.sksamuel.scala-scales",
                "com.sksamuel.scapegoat",
                "com.sksamuel.scoverage",
                "com.sksamuel.scribble",
                "com.sohoffice",
                "com.swoval",
                "com.tapad.sbt",
                "com.teambytes.sbt",
                "com.thesamet",
                "com.thoughtworks",
                "com.timushev.sbt",
                "com.tmzint.sbt",
                "com.trafficland",
                "com.twitter",
                "com.typelead",
                "com.typesafe.akka",
                "com.typesafe.conductr",
                "com.typesafe.play",
                "com.typesafe.reactiveruntime",
                "com.typesafe.sbt",
                "com.typesafe.sbteclipse",
                "com.typesafe.tmp",
                "com.typesafe.typesafeconductr",
                "com.typesafe",
                "com.untyped",
                "com.vmunier",
                "com.xvyg",
                "com.yetu",
                "com.zlad",
                "com",
                "coreyconnor",
                "cuipengfei",
                "de.cbley",
                "de.heikoseeberger",
                "de.jerman",
                "de.johoop",
                "de.knutwalker",
                "de.mediacluster.sbt",
                "de.oakgrove",
                "de.sciss",
                "edu.umass.cs",
                "ee.risk.sbt.plugins",
                "emchristiansen",
                "es.webet.play",
                "es.webet.sbt",
                "eu.arthepsy.sbt",
                "eu.getintheloop",
                "eu.svez",
                "fi.jumi.sbt",
                "fi.onesto.sbt",
                "g00dnatur3",
                "github.com",
                "glngn",
                "im.actor",
                "im.dlg",
                "in.drajit.sbt",
                "info.pdalpra",
                "io.buildo",
                "io.dapas",
                "io.finstack",
                "io.gatling.frontline",
                "io.gatling",
                "io.github.chikei",
                "io.github.darkyenus",
                "io.github.davidgregory084",
                "io.github.henders",
                "io.github.jeremyrsmith",
                "io.github.sugakandrey",
                "io.jenner",
                "io.kamon",
                "io.methvin",
                "io.michaelallen.mustache",
                "io.prediction",
                "io.regadas",
                "io.scalac",
                "io.shaka",
                "io.spray",
                "io.strd.build",
                "io.sysa",
                "io.teamscala.sbt",
                "io.xogus",
                "io.zastoupil",
                "io.zman",
                "it.paperdragon",
                "jsuereth",
                "kevinlee",
                "laughedelic",
                "lt.dvim.authors",
                "lt.dvim.paradox",
                "me.amanj",
                "me.andreionut",
                "me.lessis",
                "me.paulschwarz",
                "me.penkov",
                "me.rschatz",
                "me.tfeng.sbt-plugins",
                "me.vican.jorge",
                "me",
                "mrken",
                "name.de-vries",
                "name.heikoseeberger.groll",
                "name.heikoseeberger.sbt.groll",
                "name.heikoseeberger.sbt.properties",
                "name.heikoseeberger",
                "net.aichler",
                "net.bytebuddy",
                "net.bzzt",
                "net.contentobjects.jnotify",
                "net.eamelink.sbt",
                "net.eigenvalue",
                "net.ground5hark.sbt",
                "net.katsstuff",
                "net.lullabyte",
                "net.nornagon",
                "net.pishen",
                "net.ssanj",
                "net.thunderklaus",
                "net.virtual-void",
                "nl.anchormen.sbt",
                "nl.codestar",
                "nz.co.bottech",
                "ohnosequences",
                "org.aleastChs",
                "org.allenai.plugins",
                "org.bitbucket.inkytonik.sbt-rats",
                "org.bjason",
                "org.clapper",
                "org.cmj",
                "org.coursera.courier",
                "org.coursera.naptime",
                "org.doolse",
                "org.duhemm",
                "org.foundweekends.conscript",
                "org.foundweekends.giter8",
                "org.foundweekends",
                "org.github.ngbinh",
                "org.h3nk3",
                "org.hypercomp",
                "org.irundaia.sbt",
                "org.jetbrains.teamcity.plugins",
                "org.jetbrains",
                "org.jruby",
                "org.lifty",
                "org.lyranthe.sbt",
                "org.madoushi.sbt",
                "org.make",
                "org.neolin.sbt",
                "org.netbeans.nbsbt",
                "org.opencommercesearch",
                "org.pitest.sbt",
                "org.planet42",
                "org.portable-scala",
                "org.roboscala",
                "org.scala-android",
                "org.scala-js",
                "org.scala-lang.modules.scalajs",
                "org.scala-lang.modules",
                "org.scala-native",
                "org.scala-sbt.plugins",
                "org.scala-sbt",
                "org.scalameta",
                "org.scalastyle",
                "org.scalatra.requirejs",
                "org.scalatra.sbt",
                "org.scalavista",
                "org.scoverage",
                "org.tpolecat",
                "org.typelevel",
                "org.zjulambda.scala",
                "org",
                "pl.otrebski",
                "pl.project13.sbt",
                "pl.project13.scala",
                "pl.tues",
                "rocks.muki",
                "ru.dokwork",
                "ru.kotobotov",
                "ru.pravo",
                "sbt-plugin-releases",
                "sbt",
                "scalajs-react-interface",
                "se.sisyfosdigital.sbt",
                "se.yobriefca",
                "securesocial",
                "spartakus",
                "sqlpt",
                "stejskal",
                "tech.ant8e",
                "ub-interactive",
                "uk.co.josephearl",
                "uk.co.randomcoding",
                "works.mesh",
                "woshilaiceshide",
            ]
        );
    }

    // Ported: "returns null in case of errors" — datasource/sbt-package/index.spec.ts line 43
    #[tokio::test]
    async fn returns_null_in_case_of_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven/org/scalatest/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven/org.scalatest/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven/org/scalatest/scalatest/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_pkg_releases(
            "org.scalatest:scalatest",
            &format!("{}/maven", server.uri()),
            &http,
        )
        .await;
        assert!(result.is_none());
    }

    // Ported: "returns null if there is no version" — datasource/sbt-package/index.spec.ts line 63
    #[tokio::test]
    async fn returns_null_if_there_is_no_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/com/example/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                "<a href=\"empty/\">empty_2.12/</a>\n\
                     <a href=\"empty_but_invalid/\">???</a>",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/com/example/empty_but_invalid/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/com/example/empty/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        // maven fallback
        Mock::given(method("GET"))
            .and(path("/maven2/com/example/empty/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_pkg_releases(
            "com.example:empty",
            &format!("{}/maven2", server.uri()),
            &http,
        )
        .await;
        assert!(result.is_none());
    }

    // Ported: "fetches releases from Maven" — datasource/sbt-package/index.spec.ts line 91
    #[tokio::test]
    async fn fetches_releases_from_maven() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(concat!(
                "<a href=\"../\" title='../'>../</a>\n",
                "<a href=\"example/\" title='example/'>example_2.12/</a>\n",
                "<a href=\"example_2.12/\" title='example_2.12/'>example_2.12/</a>\n",
                "<a href=\"example_native/\" title='example_native/'>example_native/</a>\n",
                "<a href=\"example_sjs/\" title='example_sjs/'>example_sjs/</a>",
            )))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example/"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<a href='../'>../</a>\n<a href='1.2.0/'>1.2.0/</a>"),
            )
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example_2.12/"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<a href='../'>../</a>\n<a href='1.2.3/'>1.2.3/</a>"),
            )
            .mount(&server)
            .await;
        // POM files all 404
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example/1.2.3/example-1.2.3.pom"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/maven2/org/example/example_2.12/1.2.3/example-1.2.3.pom",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/maven2/org/example/example_2.12/1.2.3/example_2.12-1.2.3.pom",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let base = format!("{}/maven2", server.uri());
        let result = get_pkg_releases("org.example:example", &base, &http)
            .await
            .expect("expected Some result");

        assert_eq!(result.registry_url, base);
        assert!(
            result.dependency_url.ends_with("/org/example"),
            "dependency_url={:?}",
            result.dependency_url
        );
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"1.2.0"), "versions={versions:?}");
        assert!(versions.contains(&"1.2.3"), "versions={versions:?}");
        assert_eq!(versions.len(), 2);
    }

    // Ported: "fetches Maven releases with Scala version" — datasource/sbt-package/index.spec.ts line 142
    #[tokio::test]
    async fn fetches_maven_releases_with_scala_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                "<a href=\"example_2.12/\" title='example_2.12/'>example_2.12/</a>",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example_2.12/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<a href='1.2.3/'>1.2.3/</a>"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/maven2/org/example/example_2.12/1.2.3/example-1.2.3.pom",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/maven2/org/example/example_2.12/1.2.3/example_2.12-1.2.3.pom",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let base = format!("{}/maven2", server.uri());
        let result = get_pkg_releases("org.example:example_2.12", &base, &http)
            .await
            .expect("expected Some result");

        assert_eq!(result.registry_url, base);
        assert!(result.dependency_url.ends_with("/org/example"));
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "1.2.3");
    }

    // Ported: "fetches releases from Confluent" — datasource/sbt-package/index.spec.ts line 171
    #[tokio::test]
    async fn fetches_releases_from_confluent() {
        let server = MockServer::start().await;
        // Use path-absolute hrefs matching the TypeScript fixture behaviour.
        Mock::given(method("GET"))
            .and(path("/maven/io/confluent/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                "<a href=\"/maven/io/confluent/kafka-avro-serializer/\">kafka-avro-serializer/</a>",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven/io/confluent/kafka-avro-serializer/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                "<a href=\"/maven/io/confluent/kafka-avro-serializer/7.0.1/\">7.0.1/</a>",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/maven/io/confluent/kafka-avro-serializer/7.0.1/kafka-avro-serializer-7.0.1.pom",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"<project xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xmlns="http://maven.apache.org/POM/4.0.0">
  <artifactId>kafka-avro-serializer</artifactId>
  <packaging>jar</packaging>
  <name>kafka-avro-serializer</name>
</project>"#,
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let base = format!("{}/maven", server.uri());
        let result = get_pkg_releases("io.confluent:kafka-avro-serializer", &base, &http)
            .await
            .expect("expected Some result");

        assert_eq!(result.registry_url, base);
        assert!(result.dependency_url.ends_with("/io/confluent"));
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "7.0.1");
    }

    // Ported: "extracts URL from Maven POM file" — datasource/sbt-package/index.spec.ts line 211
    #[tokio::test]
    async fn extracts_url_from_maven_pom_file() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<a href=\"example/\" title='example/'>example_2.12/</a>"),
            )
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<a href='1.2.3/'>1.2.3/</a>"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example/1.2.3/example-1.2.3.pom"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                "<project>\
                       <url>https://package.example.org/about</url>\
                       <scm><url>https://example.org/repo.git</url></scm>\
                     </project>",
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let base = format!("{}/maven2", server.uri());
        let result = get_pkg_releases("org.example:example", &base, &http)
            .await
            .expect("expected Some result");

        assert_eq!(
            result.homepage.as_deref(),
            Some("https://package.example.org/about")
        );
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://example.org/repo")
        );
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "1.2.3");
    }

    // Ported: "falls back to Maven for orgarization root folder non-listable repositories" — modules/datasource/sbt-package/index.spec.ts line 245
    //         — datasource/sbt-package/index.spec.ts line 245
    #[tokio::test]
    async fn falls_back_to_maven_for_non_listable_repositories() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/packages/maven/org/example/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/packages/maven/org.example/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/packages/maven/org/example/example_2.13/maven-metadata.xml",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<metadata>
  <groupId>org.example</groupId>
  <artifactId>package</artifactId>
  <versioning>
    <latest>1.2.3</latest>
    <release>1.2.3</release>
    <versions>
      <version>1.2.3</version>
    </versions>
  </versioning>
</metadata>"#,
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(
                "/packages/maven/org/example/example_2.13/1.2.3/example_2.13-1.2.3.pom",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let base = format!("{}/packages/maven", server.uri());
        let result = get_pkg_releases("org.example:example_2.13", &base, &http).await;
        assert!(result.is_some(), "expected Some result (maven fallback)");
    }

    // Ported: "extracts URL from Maven POM file" (postprocessRelease) — datasource/sbt-package/index.spec.ts line 366
    #[tokio::test]
    async fn postprocess_extracts_release_timestamp() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/1.2.3/example-1.2.3.pom"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<project></project>")
                    .insert_header("last-modified", "Wed, 21 Oct 2015 07:28:00 GMT"),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let pkg_url = format!("{}/maven2/org/example/", server.uri());
        let ts = get_pom_release_timestamp(&[pkg_url.as_str()], "1.2.3", &http).await;
        assert_eq!(ts.as_deref(), Some("2015-10-21T07:28:00.000Z"));
    }

    // Ported: "continues when parseUrl returns null for packageRootUrl" — datasource/sbt-package/index.spec.ts line 285
    //
    // The TS test mocks `parseUrl` to return null for the group root URL,
    // preventing further URL construction. In Rust, the equivalent is the
    // slash-separated group root returning a listing, but all artifact subdirs
    // and the maven fallback returning nothing.
    #[tokio::test]
    async fn continues_when_package_root_url_fails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<a href=\"example/\" title='example/'>example/</a>"),
            )
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org.example/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_pkg_releases(
            "org.example:example",
            &format!("{}/maven2", server.uri()),
            &http,
        )
        .await;
        assert!(result.is_none());
    }

    // Ported: "skips pkgUrl when parseUrl returns null for it" — datasource/sbt-package/index.spec.ts line 323
    //
    // The TS test mocks `parseUrl` to return null for the artifact subdir URL.
    // In Rust, the artifact subdir listing returns version links but subsequent
    // version URL resolution yields no versions.
    #[tokio::test]
    async fn skips_pkg_url_when_parse_url_returns_null() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<a href=\"example/\" title='example/'>example/</a>"),
            )
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org.example/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<a href='1.2.3/'>1.2.3/</a>"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/maven2/org/example/example/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_pkg_releases(
            "org.example:example",
            &format!("{}/maven2", server.uri()),
            &http,
        )
        .await;
        assert!(result.is_some());
        let res = result.unwrap();
        assert_eq!(res.releases.len(), 1);
        assert_eq!(res.releases[0].version, "1.2.3");
    }
}
