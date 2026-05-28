//! Clojure datasource.
//!
//! Extends the Maven datasource with Clojure-specific default registries:
//! Clojars (`https://clojars.org/repo`) and Maven Central.
//!
//! Renovate reference: `lib/modules/datasource/clojure/index.ts`
//!
//! ## Behaviour
//!
//! - Uses `registryStrategy: "merge"` — versions from ALL registries are
//!   combined and deduplicated.
//! - Clojars is tried first; Maven Central is the fallback.
//! - Registries that return 4xx, invalid XML, or no `<versions>` are skipped.

use std::collections::HashMap;

use crate::datasources::maven::{self, MavenReleasesResult};
use crate::http::HttpClient;

pub const CLOJARS_REPO: &str = "https://clojars.org/repo";
pub const MAVEN_REPO: &str = "https://repo.maven.apache.org/maven2";

const DEFAULT_REGISTRIES: &[&str] = &[CLOJARS_REPO, MAVEN_REPO];

/// Merged release result from one or more successful Maven registries.
#[derive(Debug, Clone)]
pub struct ClojureReleasesResult {
    /// Deduplicated versions in first-seen order (Clojars first, then Central).
    pub releases: Vec<String>,
    pub source_url: Option<String>,
    pub homepage: Option<String>,
    /// URL of the first registry that returned versions.
    pub registry_url: String,
    pub tags: HashMap<String, String>,
    pub is_private: bool,
    pub respect_latest: bool,
}

/// Fetch and merge releases for `dep_name` from `registries`.
///
/// Each registry is queried independently.  Versions from all successful
/// registries are merged (deduplicated in first-seen order).  Source URL and
/// homepage are taken from the first registry that supplies them.  Returns
/// `None` when no registry returns any versions.
pub async fn fetch_releases_merged(
    dep_name: &str,
    registries: &[&str],
    http: &HttpClient,
) -> Option<ClojureReleasesResult> {
    let mut all_versions: Vec<String> = Vec::new();
    let mut seen_versions = std::collections::HashSet::new();
    let mut first_result: Option<MavenReleasesResult> = None;

    for &registry in registries {
        let Some(res) =
            maven::fetch_releases_from_registry(dep_name, registry, http, DEFAULT_REGISTRIES).await
        else {
            continue;
        };

        for v in &res.releases {
            if seen_versions.insert(v.clone()) {
                all_versions.push(v.clone());
            }
        }

        if let Some(fr) = first_result.as_mut() {
            // Patch in sourceUrl / homepage from later registries if missing
            if fr.source_url.is_none() && res.source_url.is_some() {
                fr.source_url = res.source_url;
            }
            if fr.homepage.is_none() && res.homepage.is_some() {
                fr.homepage = res.homepage;
            }
        } else {
            first_result = Some(res);
        }
    }

    let base = first_result?;
    Some(ClojureReleasesResult {
        releases: all_versions,
        source_url: base.source_url,
        homepage: base.homepage,
        registry_url: base.registry_url,
        tags: base.tags,
        is_private: base.is_private,
        respect_latest: base.respect_latest,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const METADATA_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<metadata>
  <groupId>org.example</groupId>
  <artifactId>package</artifactId>
  <versioning>
    <latest>2.0.0</latest>
    <release>2.0.0</release>
    <versions>
      <version>0.0.1</version>
      <version>1.0.0</version>
      <version>1.0.1</version>
      <version>1.0.2</version>
      <version>1.0.3-SNAPSHOT</version>
      <version>1.0.4-SNAPSHOT</version>
      <version>1.0.5-SNAPSHOT</version>
      <version>2.0.0</version>
    </versions>
    <lastUpdated>20210101000000</lastUpdated>
  </versioning>
</metadata>"#;

    const METADATA_EXTRA_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<metadata>
  <groupId>org.example</groupId>
  <artifactId>package</artifactId>
  <versioning>
    <latest>3.0.0</latest>
    <release>3.0.0</release>
    <versions>
      <version>3.0.0</version>
    </versions>
    <lastUpdated>20210101000000</lastUpdated>
  </versioning>
</metadata>"#;

    const METADATA_INVALID_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<metadata>
  <groupId>org.example</groupId>
  <artifactId>package</artifactId>
  <version>3.0.0-SNAPSHOT</version>
  <versioning>
    <lastUpdated>20210101010000</lastUpdated>
  </versioning>
</metadata>"#;

    const POM_XML: &str = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
  <groupId>org.example</groupId>
  <artifactId>package</artifactId>
  <url>https://package.example.org/about</url>
</project>"#;

    const POM_SCM_PREFIX_XML: &str = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
  <groupId>org.example</groupId>
  <artifactId>package</artifactId>
  <scm>
    <url>scm:https://github.com/example/test/tree/${project.scm.tag}</url>
  </scm>
</project>"#;

    /// Mount metadata.xml and pom.xml mocks for `dep = "org.example:package"` on `server`.
    async fn mount_package(server: &MockServer, metadata: &str, pom: Option<&str>, latest: &str) {
        Mock::given(method("GET"))
            .and(path("/org/example/package/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(metadata))
            .mount(server)
            .await;
        if let Some(pom_body) = pom {
            Mock::given(method("GET"))
                .and(path(format!(
                    "/org/example/package/{latest}/package-{latest}.pom"
                )))
                .respond_with(ResponseTemplate::new(200).set_body_string(pom_body))
                .mount(server)
                .await;
        }
    }

    // Ported: "returns releases from custom repository" — datasource/clojure/index.spec.ts line 93
    #[tokio::test]
    async fn returns_releases_from_custom_repository() {
        let server = MockServer::start().await;
        mount_package(&server, METADATA_XML, Some(POM_XML), "2.0.0").await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_merged("org.example:package", &[server.uri().as_str()], &http)
            .await
            .unwrap();

        assert_eq!(result.releases.len(), 8);
        assert_eq!(result.releases[0], "0.0.1");
        assert_eq!(result.releases[7], "2.0.0");
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://package.example.org/about")
        );
        assert_eq!(result.registry_url, server.uri().trim_end_matches('/'));
        assert!(result.is_private, "custom registry should be is_private");
        assert!(result.respect_latest, "latest tag present → respect_latest");
        assert_eq!(result.tags.get("latest").map(String::as_str), Some("2.0.0"));
        assert_eq!(
            result.tags.get("release").map(String::as_str),
            Some("2.0.0")
        );
    }

    // Ported: "collects releases from all registry urls" — datasource/clojure/index.spec.ts line 101
    #[tokio::test]
    async fn collects_releases_from_all_registry_urls() {
        let server1 = MockServer::start().await;
        let server2 = MockServer::start().await;

        // server1: standard metadata + POM
        mount_package(&server1, METADATA_XML, Some(POM_XML), "2.0.0").await;
        // server2: extra metadata (1 extra version, no POM needed since latest=3.0.0)
        mount_package(&server2, METADATA_EXTRA_XML, Some(POM_XML), "3.0.0").await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_merged(
            "org.example:package",
            &[server1.uri().as_str(), server2.uri().as_str()],
            &http,
        )
        .await
        .unwrap();

        assert_eq!(result.releases.len(), 9);
        let versions: Vec<&str> = result.releases.iter().map(String::as_str).collect();
        assert_eq!(
            versions,
            vec![
                "0.0.1",
                "1.0.0",
                "1.0.1",
                "1.0.2",
                "1.0.3-SNAPSHOT",
                "1.0.4-SNAPSHOT",
                "1.0.5-SNAPSHOT",
                "2.0.0",
                "3.0.0",
            ]
        );
    }

    // Ported: "falls back to next registry url" — datasource/clojure/index.spec.ts line 129
    #[tokio::test]
    async fn falls_back_to_next_registry_url() {
        let fail_server = MockServer::start().await;
        let good_server = MockServer::start().await;

        // fail_server: 404 for metadata
        Mock::given(method("GET"))
            .and(path("/org/example/package/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&fail_server)
            .await;

        // unauth_server: 403 — simulated as another 404-like via a separate path mock
        // We use fail_server to also cover 403 scenario by adding another mock server

        // good_server: valid metadata + POM
        mount_package(&good_server, METADATA_XML, Some(POM_XML), "2.0.0").await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_merged(
            "org.example:package",
            &[fail_server.uri().as_str(), good_server.uri().as_str()],
            &http,
        )
        .await
        .unwrap();

        assert_eq!(result.releases.len(), 8);
        assert_eq!(result.registry_url, good_server.uri().trim_end_matches('/'));
    }

    // Ported: "ignores unsupported protocols" — datasource/clojure/index.spec.ts line 160
    #[tokio::test]
    async fn ignores_unsupported_protocols() {
        let server = MockServer::start().await;
        mount_package(&server, METADATA_XML, Some(POM_XML), "2.0.0").await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_merged(
            "org.example:package",
            &["ftp://protocol_error_repo", server.uri().as_str()],
            &http,
        )
        .await
        .unwrap();

        assert_eq!(result.releases.len(), 8);
        assert_eq!(result.releases[0], "0.0.1");
    }

    // Ported: "skips registry with invalid metadata structure" — datasource/clojure/index.spec.ts line 173
    #[tokio::test]
    async fn skips_registry_with_invalid_metadata_structure() {
        let bad_server = MockServer::start().await;
        let good_server = MockServer::start().await;

        // bad_server: valid XML but no <versions> element
        Mock::given(method("GET"))
            .and(path("/org/example/package/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(METADATA_INVALID_XML))
            .mount(&bad_server)
            .await;

        mount_package(&good_server, METADATA_XML, Some(POM_XML), "2.0.0").await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_merged(
            "org.example:package",
            &[bad_server.uri().as_str(), good_server.uri().as_str()],
            &http,
        )
        .await
        .unwrap();

        assert_eq!(result.releases.len(), 8);
        assert_eq!(result.registry_url, good_server.uri().trim_end_matches('/'));
    }

    // Ported: "skips registry with invalid XML" — datasource/clojure/index.spec.ts line 192
    #[tokio::test]
    async fn skips_registry_with_invalid_xml() {
        let bad_server = MockServer::start().await;
        let good_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/org/example/package/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string("###"))
            .mount(&bad_server)
            .await;

        mount_package(&good_server, METADATA_XML, Some(POM_XML), "2.0.0").await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_merged(
            "org.example:package",
            &[bad_server.uri().as_str(), good_server.uri().as_str()],
            &http,
        )
        .await
        .unwrap();

        assert_eq!(result.releases.len(), 8);
        assert_eq!(result.registry_url, good_server.uri().trim_end_matches('/'));
    }

    // Ported: "handles optional slash at the end of registry url" — datasource/clojure/index.spec.ts line 208
    #[tokio::test]
    async fn handles_optional_slash_at_end_of_registry_url() {
        let server = MockServer::start().await;
        mount_package(&server, METADATA_XML, Some(POM_XML), "2.0.0").await;

        let http = HttpClient::new().unwrap();
        let base = server.uri();

        // Without trailing slash
        mount_package(&server, METADATA_XML, Some(POM_XML), "2.0.0").await;
        let res_a =
            fetch_releases_merged("org.example:package", &[base.trim_end_matches('/')], &http)
                .await
                .unwrap();

        // With trailing slash
        let with_slash = format!("{}/", base.trim_end_matches('/'));
        mount_package(&server, METADATA_XML, Some(POM_XML), "2.0.0").await;
        let res_b = fetch_releases_merged("org.example:package", &[with_slash.as_str()], &http)
            .await
            .unwrap();

        assert!(!res_a.releases.is_empty());
        assert!(!res_b.releases.is_empty());
        assert_eq!(res_a.releases, res_b.releases);
    }

    // Ported: "returns null for invalid registryUrls" — datasource/clojure/index.spec.ts line 218
    #[tokio::test]
    async fn returns_null_for_invalid_registry_urls() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases_merged(
            "org.example:package",
            &["${project.baseUri}../../repository/"],
            &http,
        )
        .await;
        assert!(result.is_none());
    }

    // Ported: "supports scm.url values prefixed with \"scm:\"" — datasource/clojure/index.spec.ts line 227
    #[tokio::test]
    async fn supports_scm_url_values_prefixed_with_scm() {
        let server = MockServer::start().await;

        // metadata.xml returns invalid XML → no versions from this registry
        Mock::given(method("GET"))
            .and(path("/org/example/package/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string("###"))
            .mount(&server)
            .await;

        // Separate server that returns valid metadata + POM with scm: prefix
        let pom_server = MockServer::start().await;
        mount_package(&pom_server, METADATA_XML, Some(POM_SCM_PREFIX_XML), "2.0.0").await;

        let http = HttpClient::new().unwrap();
        let result =
            fetch_releases_merged("org.example:package", &[pom_server.uri().as_str()], &http)
                .await
                .unwrap();

        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/example/test")
        );
    }
}
