//! Apache Ant `build.xml` Maven dependency extractor.
//!
//! Scans `build.xml` files for `<dependency>` elements in
//! `<artifact:dependencies>` sections and extracts Maven GAV coordinates.
//!
//! Renovate reference:
//! - `lib/modules/manager/ant/extract.ts`
//! - Pattern: `**/build.xml`
//! - Datasource: Maven
//!
//! ## Supported forms
//!
//! ```xml
//! <artifact:dependencies>
//!   <!-- Inline groupId/artifactId/version attributes -->
//!   <dependency groupId="junit" artifactId="junit" version="4.13.2" scope="test" />
//!   <!-- coords attribute: groupId:artifactId:version[:scope] -->
//!   <dependency coords="org.slf4j:slf4j-api:1.7.36" />
//!   <!-- remoteRepository for custom registries -->
//!   <remoteRepository url="https://repo.example.com/" />
//! </artifact:dependencies>
//! ```

use std::io::BufReader;

use quick_xml::Reader;
use quick_xml::events::Event;

/// Why an Ant dependency is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AntSkipReason {
    /// Version is a property reference (`${...}`).
    PropertyRef,
    /// Required attributes missing.
    MissingVersion,
}

/// A single extracted Ant/Maven dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AntDep {
    /// `groupId:artifactId`
    pub dep_name: String,
    /// Version string.
    pub current_value: String,
    /// Scope (compile, test, runtime, provided, system).
    pub dep_type: String,
    /// Registry URLs from `<remoteRepository>` elements, if any.
    pub registry_urls: Vec<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<AntSkipReason>,
}

const SCOPE_NAMES: &[&str] = &["compile", "runtime", "test", "provided", "system"];

/// Extract Maven deps from an Apache Ant `build.xml` file.
pub fn extract(content: &str) -> Vec<AntDep> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut deps: Vec<AntDep> = Vec::new();
    let mut registry_urls: Vec<String> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e) | Event::Start(e)) => {
                // Local name (strip namespace prefix like `artifact:`)
                let raw_name = e.name();
                let local = local_name(raw_name.as_ref());

                match local.as_str() {
                    "dependency" => {
                        if let Some(dep) = parse_dependency_attrs(&e, &registry_urls) {
                            deps.push(dep);
                        }
                    }
                    "remoteRepository" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"url"
                                && let Ok(url) = std::str::from_utf8(attr.value.as_ref())
                            {
                                registry_urls.push(url.to_owned());
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    // Attach collected registry URLs to all deps that don't already have them
    if !registry_urls.is_empty() {
        for dep in &mut deps {
            if dep.registry_urls.is_empty() {
                dep.registry_urls.clone_from(&registry_urls);
            }
        }
    }

    deps
}

/// Strip XML namespace prefix: `artifact:dependency` → `dependency`.
fn local_name(raw: &[u8]) -> String {
    let s = std::str::from_utf8(raw).unwrap_or("");
    if let Some(pos) = s.find(':') {
        s[pos + 1..].to_owned()
    } else {
        s.to_owned()
    }
}

fn parse_dependency_attrs(
    e: &quick_xml::events::BytesStart<'_>,
    registry_urls: &[String],
) -> Option<AntDep> {
    let mut group_id = String::new();
    let mut artifact_id = String::new();
    let mut version = String::new();
    let mut scope = String::new();
    let mut coords = String::new();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
        let val = std::str::from_utf8(attr.value.as_ref())
            .unwrap_or("")
            .to_owned();
        match key {
            "groupId" => group_id = val,
            "artifactId" => artifact_id = val,
            "version" => version = val,
            "scope" => scope = val,
            "coords" => coords = val,
            _ => {}
        }
    }

    if !coords.is_empty() {
        return parse_coords_dep(&coords, registry_urls);
    }

    if group_id.is_empty() || artifact_id.is_empty() {
        return None;
    }

    let dep_name = format!("{group_id}:{artifact_id}");
    let dep_type = if SCOPE_NAMES.contains(&scope.as_str()) {
        scope
    } else {
        "compile".to_owned()
    };

    if version.is_empty() {
        return Some(AntDep {
            dep_name,
            current_value: String::new(),
            dep_type,
            registry_urls: registry_urls.to_vec(),
            skip_reason: Some(AntSkipReason::MissingVersion),
        });
    }

    if version.contains("${") {
        return Some(AntDep {
            dep_name,
            current_value: version,
            dep_type,
            registry_urls: registry_urls.to_vec(),
            skip_reason: Some(AntSkipReason::PropertyRef),
        });
    }

    Some(AntDep {
        dep_name,
        current_value: version,
        dep_type,
        registry_urls: registry_urls.to_vec(),
        skip_reason: None,
    })
}

fn parse_coords_dep(coords: &str, registry_urls: &[String]) -> Option<AntDep> {
    // coords: groupId:artifactId:version[:type[:classifier[:scope]]]
    let normalized = coords.replace('/', ":");
    let parts: Vec<&str> = normalized.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    let dep_name = format!("{}:{}", parts[0], parts[1]);
    let version = parts[2].to_owned();

    // Scope is the last element if it's a known scope name.
    let dep_type = parts
        .last()
        .filter(|&&s| SCOPE_NAMES.contains(&s))
        .map(|&s| s.to_owned())
        .unwrap_or_else(|| "compile".to_owned());

    if version.contains("${") {
        return Some(AntDep {
            dep_name,
            current_value: version,
            dep_type,
            registry_urls: registry_urls.to_vec(),
            skip_reason: Some(AntSkipReason::PropertyRef),
        });
    }

    if version.is_empty() {
        return Some(AntDep {
            dep_name,
            current_value: String::new(),
            dep_type,
            registry_urls: registry_urls.to_vec(),
            skip_reason: Some(AntSkipReason::MissingVersion),
        });
    }

    Some(AntDep {
        dep_name,
        current_value: version,
        dep_type,
        registry_urls: registry_urls.to_vec(),
        skip_reason: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_inline_dependency() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency groupId="junit" artifactId="junit" version="4.13.2" scope="test" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[0].current_value, "4.13.2");
        assert_eq!(deps[0].dep_type, "test");
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn extracts_coords_form() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency coords="org.slf4j:slf4j-api:1.7.36" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.slf4j:slf4j-api");
        assert_eq!(deps[0].current_value, "1.7.36");
    }

    #[test]
    fn property_ref_skipped() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency groupId="org.foo" artifactId="bar" version="${bar.version}" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(AntSkipReason::PropertyRef));
    }

    #[test]
    fn multiple_deps_extracted() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency groupId="junit" artifactId="junit" version="4.13.2" scope="test" />
    <dependency groupId="org.slf4j" artifactId="slf4j-api" version="1.7.36" scope="compile" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[1].dep_name, "org.slf4j:slf4j-api");
    }

    #[test]
    fn remote_repository_collected() {
        let content = r#"
<project>
  <artifact:dependencies>
    <remoteRepository url="https://repo.example.com/" />
    <dependency groupId="com.example" artifactId="mylib" version="1.0.0" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].registry_urls, vec!["https://repo.example.com/"]);
    }

    #[test]
    fn empty_xml_returns_empty() {
        assert!(extract("<project />").is_empty());
    }
}
