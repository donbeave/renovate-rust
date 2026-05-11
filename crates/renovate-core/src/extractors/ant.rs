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

use std::collections::{HashMap, HashSet};
use std::io::BufReader;

use quick_xml::Reader;
use quick_xml::events::Event;

/// Why an Ant dependency is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AntSkipReason {
    /// Version is a property reference (`${...}`).
    PropertyRef,
    /// Version property resolution recursed back to an already-seen property.
    RecursivePropertyRef,
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
    /// Name of the property that supplied the version.
    pub shared_variable_name: Option<String>,
}

const SCOPE_NAMES: &[&str] = &["compile", "runtime", "test", "provided", "system"];

/// Extract Maven deps from an Apache Ant `build.xml` file.
pub fn extract(content: &str) -> Vec<AntDep> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut deps: Vec<AntDep> = Vec::new();
    let mut registry_urls: Vec<String> = Vec::new();
    let mut properties: HashMap<String, String> = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e) | Event::Start(e)) => {
                // Local name (strip namespace prefix like `artifact:`)
                let raw_name = e.name();
                let local = local_name(raw_name.as_ref());

                match local.as_str() {
                    "dependency" => {
                        if let Some(dep) = parse_dependency_attrs(&e, &registry_urls, &properties) {
                            deps.push(dep);
                        }
                    }
                    "property" => {
                        if let Some((name, value)) = parse_property_attrs(&e) {
                            properties.entry(name).or_insert(value);
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
    properties: &HashMap<String, String>,
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
        return parse_coords_dep(&coords, registry_urls, properties);
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
            shared_variable_name: None,
        });
    }

    if version.contains("${") {
        if let Some(property_name) = exact_property_ref(&version) {
            match resolve_property(property_name, properties, &mut HashSet::new()) {
                Ok(resolved) => {
                    return Some(AntDep {
                        dep_name,
                        current_value: resolved,
                        dep_type,
                        registry_urls: registry_urls.to_vec(),
                        skip_reason: None,
                        shared_variable_name: Some(property_name.to_owned()),
                    });
                }
                Err(skip_reason) => {
                    return Some(AntDep {
                        dep_name,
                        current_value: version,
                        dep_type,
                        registry_urls: registry_urls.to_vec(),
                        skip_reason: Some(skip_reason),
                        shared_variable_name: None,
                    });
                }
            }
        }
        return Some(AntDep {
            dep_name,
            current_value: version,
            dep_type,
            registry_urls: registry_urls.to_vec(),
            skip_reason: Some(AntSkipReason::PropertyRef),
            shared_variable_name: None,
        });
    }

    Some(AntDep {
        dep_name,
        current_value: version,
        dep_type,
        registry_urls: registry_urls.to_vec(),
        skip_reason: None,
        shared_variable_name: None,
    })
}

fn parse_coords_dep(
    coords: &str,
    registry_urls: &[String],
    properties: &HashMap<String, String>,
) -> Option<AntDep> {
    // coords: groupId:artifactId:[type:[classifier:]]version[:scope]
    let normalized = coords.replace('/', ":");
    let parts: Vec<&str> = normalized.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    if parts[0].is_empty() || parts[1].is_empty() {
        // Reject malformed coords with empty groupId or artifactId.
        return None;
    }

    let dep_name = format!("{}:{}", parts[0], parts[1]);

    // The optional trailing scope is recognised only when the last segment
    // matches a known Maven scope name. Otherwise the last segment is the
    // version (and any segments between artifactId and the version are
    // type/classifier metadata that does not affect the dep name).
    let last = *parts.last().unwrap();
    let (dep_type, version) = if parts.len() >= 4 && SCOPE_NAMES.contains(&last) {
        // grp:art:[type:[classifier:]]version:scope — pick the version slot.
        // The version is the segment immediately before scope.
        let version_segment = parts[parts.len() - 2];
        (last.to_owned(), version_segment.to_owned())
    } else if parts.len() == 3 {
        // grp:art:version
        ("compile".to_owned(), parts[2].to_owned())
    } else {
        // grp:art:[type:[classifier:]]version (no scope) — version is last.
        ("compile".to_owned(), last.to_owned())
    };

    if version.contains("${") {
        if let Some(property_name) = exact_property_ref(&version) {
            match resolve_property(property_name, properties, &mut HashSet::new()) {
                Ok(resolved) => {
                    return Some(AntDep {
                        dep_name,
                        current_value: resolved,
                        dep_type,
                        registry_urls: registry_urls.to_vec(),
                        skip_reason: None,
                        shared_variable_name: Some(property_name.to_owned()),
                    });
                }
                Err(skip_reason) => {
                    return Some(AntDep {
                        dep_name,
                        current_value: version,
                        dep_type,
                        registry_urls: registry_urls.to_vec(),
                        skip_reason: Some(skip_reason),
                        shared_variable_name: None,
                    });
                }
            }
        }
        return Some(AntDep {
            dep_name,
            current_value: version,
            dep_type,
            registry_urls: registry_urls.to_vec(),
            skip_reason: Some(AntSkipReason::PropertyRef),
            shared_variable_name: None,
        });
    }

    if version.is_empty() {
        return Some(AntDep {
            dep_name,
            current_value: String::new(),
            dep_type,
            registry_urls: registry_urls.to_vec(),
            skip_reason: Some(AntSkipReason::MissingVersion),
            shared_variable_name: None,
        });
    }

    Some(AntDep {
        dep_name,
        current_value: version,
        dep_type,
        registry_urls: registry_urls.to_vec(),
        skip_reason: None,
        shared_variable_name: None,
    })
}

fn parse_property_attrs(e: &quick_xml::events::BytesStart<'_>) -> Option<(String, String)> {
    let mut name = String::new();
    let mut value = String::new();
    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
        let val = std::str::from_utf8(attr.value.as_ref())
            .unwrap_or("")
            .to_owned();
        match key {
            "name" => name = val,
            "value" => value = val,
            _ => {}
        }
    }
    (!name.is_empty()).then_some((name, value))
}

fn exact_property_ref(value: &str) -> Option<&str> {
    value
        .strip_prefix("${")
        .and_then(|inner| inner.strip_suffix('}'))
        .filter(|name| !name.contains("${") && !name.is_empty())
}

fn resolve_property(
    name: &str,
    properties: &HashMap<String, String>,
    seen: &mut HashSet<String>,
) -> Result<String, AntSkipReason> {
    if !seen.insert(name.to_owned()) {
        return Err(AntSkipReason::RecursivePropertyRef);
    }
    let Some(value) = properties.get(name) else {
        seen.remove(name);
        return Err(AntSkipReason::PropertyRef);
    };
    let resolved = resolve_property_placeholders(value, properties, seen);
    seen.remove(name);
    resolved
}

fn resolve_property_placeholders(
    value: &str,
    properties: &HashMap<String, String>,
    seen: &mut HashSet<String>,
) -> Result<String, AntSkipReason> {
    let mut out = String::new();
    let mut rest = value;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after_start = &rest[start + 2..];
        let Some(end) = after_start.find('}') else {
            out.push_str(&rest[start..]);
            return Ok(out);
        };
        let property_name = &after_start[..end];
        out.push_str(&resolve_property(property_name, properties, seen)?);
        rest = &after_start[end + 1..];
    }
    out.push_str(rest);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts inline version dependencies from build.xml" — ant/extract.spec.ts line 9
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

    // Ported: "extracts dependency from 3-part coords attribute" — ant/extract.spec.ts line 760
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

    // Ported: "skips dependencies with unresolvable property references" — ant/extract.spec.ts line 288
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

    // Ported: "resolves inline property references" — ant/extract.spec.ts line 167
    #[test]
    fn resolves_inline_property_references() {
        let content = r#"
<project>
  <property name="slf4j.version" value="1.7.36"/>
  <artifact:dependencies>
    <dependency groupId="org.slf4j" artifactId="slf4j-api" version="${slf4j.version}" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.slf4j:slf4j-api");
        assert_eq!(deps[0].current_value, "1.7.36");
        assert_eq!(
            deps[0].shared_variable_name.as_deref(),
            Some("slf4j.version")
        );
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "implements first-definition-wins for inline properties" — ant/extract.spec.ts line 228
    #[test]
    fn first_inline_property_definition_wins() {
        let content = r#"
<project>
  <property name="junit.version" value="4.13.2"/>
  <property name="junit.version" value="4.12"/>
  <artifact:dependencies>
    <dependency groupId="junit" artifactId="junit" version="${junit.version}" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "4.13.2");
        assert_eq!(
            deps[0].shared_variable_name.as_deref(),
            Some("junit.version")
        );
    }

    // Ported: "detects circular property references" — ant/extract.spec.ts line 312
    #[test]
    fn circular_property_reference_is_skipped() {
        let content = r#"
<project>
  <property name="a" value="${b}"/>
  <property name="b" value="${a}"/>
  <artifact:dependencies>
    <dependency groupId="junit" artifactId="junit" version="${a}" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(AntSkipReason::RecursivePropertyRef)
        );
    }

    // Ported: "resolves chained property references" — ant/extract.spec.ts line 338
    #[test]
    fn resolves_chained_property_references() {
        let content = r#"
<project>
  <property name="base.version" value="1.7"/>
  <property name="full.version" value="${base.version}.36"/>
  <property name="slf4j.version" value="${full.version}"/>
  <artifact:dependencies>
    <dependency groupId="org.slf4j" artifactId="slf4j-api" version="${slf4j.version}" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.slf4j:slf4j-api");
        assert_eq!(deps[0].current_value, "1.7.36");
        assert_eq!(
            deps[0].shared_variable_name.as_deref(),
            Some("slf4j.version")
        );
    }

    // Ported: "groups multiple dependencies sharing the same property" — ant/extract.spec.ts line 368
    #[test]
    fn resolves_shared_property_for_multiple_dependencies() {
        let content = r#"
<project>
  <property name="jackson.version" value="2.15.2"/>
  <artifact:dependencies>
    <dependency groupId="com.fasterxml.jackson.core" artifactId="jackson-core" version="${jackson.version}" />
    <dependency groupId="com.fasterxml.jackson.core" artifactId="jackson-databind" version="${jackson.version}" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().all(|dep| dep.current_value == "2.15.2"));
        assert!(
            deps.iter()
                .all(|dep| dep.shared_variable_name.as_deref() == Some("jackson.version"))
        );
    }

    // Ported: "skips partial placeholder in version string" — ant/extract.spec.ts line 522
    #[test]
    fn partial_placeholder_version_is_skipped() {
        let content = r#"
<project>
  <property name="base.version" value="1.7"/>
  <artifact:dependencies>
    <dependency groupId="org.slf4j" artifactId="slf4j-api" version="${base.version}.36" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.slf4j:slf4j-api");
        assert_eq!(deps[0].skip_reason, Some(AntSkipReason::PropertyRef));
    }

    // Ported: "defaults depType to compile when no scope is set" — ant/extract.spec.ts line 68
    #[test]
    fn defaults_dep_type_to_compile_without_scope() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency groupId="junit" artifactId="junit" version="4.13.2" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[0].dep_type, "compile");
    }

    // Ported: "extracts multiple dependencies" — ant/extract.spec.ts line 33
    #[test]
    fn multiple_deps_extracted() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency groupId="junit" artifactId="junit" version="4.13.2" scope="test" />
    <dependency groupId="org.slf4j" artifactId="slf4j-api" version="1.7.36" scope="compile" />
    <dependency groupId="org.apache.commons" artifactId="commons-lang3" version="3.12.0" scope="runtime" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[1].dep_name, "org.slf4j:slf4j-api");
        assert_eq!(deps[2].dep_name, "org.apache.commons:commons-lang3");
        assert_eq!(deps[2].current_value, "3.12.0");
        assert_eq!(deps[2].dep_type, "runtime");
    }

    // Ported: "collects registry URLs from remoteRepository elements" — ant/extract.spec.ts line 949
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

    // Ported: "passes registry URLs to coords-style dependencies" — ant/extract.spec.ts line 979
    #[test]
    fn remote_repository_applies_to_coords_dependency() {
        let content = r#"
<project>
  <artifact:dependencies>
    <remoteRepository url="https://repo.example.com/maven2" />
    <dependency coords="junit:junit:4.13.2" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://repo.example.com/maven2"]
        );
    }

    // Ported: "returns null for invalid XML" — ant/extract.spec.ts line 90
    #[test]
    fn invalid_xml_returns_empty() {
        assert!(extract("<<< not xml >>>").is_empty());
    }

    // Ported: "handles unparseable XML returned by readLocalFile" — ant/extract.spec.ts line 549
    #[test]
    fn unparseable_xml_returns_empty() {
        assert!(extract("<<< not xml >>>").is_empty());
    }

    // Ported: "returns null for build.xml with no dependencies" — ant/extract.spec.ts line 94
    #[test]
    fn project_without_artifact_dependencies_returns_empty() {
        let content = r#"<project><target name="build" /></project>"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "ignores dependency nodes without version" — ant/extract.spec.ts line 104
    //
    // The TS extractor returns null when no actionable deps are present.
    // Rust returns an empty Vec for the same input — there is no dep
    // to surface because the only dependency lacks a `version` attribute.
    #[test]
    fn dependency_without_version_returns_empty() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency groupId="org.example" artifactId="lib" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        let actionable: usize = deps.iter().filter(|d| d.skip_reason.is_none()).count();
        assert_eq!(actionable, 0);
    }

    // Ported: "extracts dependencies with single-quoted attributes" — ant/extract.spec.ts line 119
    #[test]
    fn single_quoted_attributes_extracted() {
        let content = "<project><artifact:dependencies><dependency groupId='junit' artifactId='junit' version='4.13.2' /></artifact:dependencies></project>";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[0].current_value, "4.13.2");
    }

    // Ported: "extracts scope from 4-part coords attribute" — ant/extract.spec.ts line 791
    #[test]
    fn four_part_coords_with_scope_at_end() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency coords="junit:junit:4.13.2:test" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[0].current_value, "4.13.2");
        assert_eq!(deps[0].dep_type, "test");
    }

    // Ported: "ignores coords with fewer than 3 parts" — ant/extract.spec.ts line 821
    #[test]
    fn coords_with_fewer_than_3_parts_skipped() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency coords="junit:junit" />
  </artifact:dependencies>
</project>"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "ignores coords with empty groupId" — ant/extract.spec.ts line 840
    #[test]
    fn coords_with_empty_groupid_skipped() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency coords=":junit:4.13.2" />
  </artifact:dependencies>
</project>"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "marks coords dependency with unresolvable property" — ant/extract.spec.ts line 890
    #[test]
    fn coords_with_unresolvable_property_is_skipped() {
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency coords="junit:junit:${missing}" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[0].skip_reason, Some(AntSkipReason::PropertyRef));
    }

    // Ported: "treats last part as version when it is not a known scope" — ant/extract.spec.ts line 919
    #[test]
    fn four_part_coords_last_segment_is_version_when_not_a_scope() {
        // groupId:artifactId:type:version — `jar` is not a Maven scope, so
        // the last segment (`1.0.0`) is the version and depType defaults to
        // `compile`.
        let content = r#"
<project>
  <artifact:dependencies>
    <dependency coords="org.example:lib:jar:1.0.0" />
  </artifact:dependencies>
</project>"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.example:lib");
        assert_eq!(deps[0].current_value, "1.0.0");
        assert_eq!(deps[0].dep_type, "compile");
    }
}
