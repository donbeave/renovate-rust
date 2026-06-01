//! Config decryption utilities.
//!
//! Mirrors `lib/config/decrypt.ts`.

pub mod bcpgp;
pub mod openpgp;

use serde::Deserialize;

/// Parsed decrypted object from an encrypted config value.
#[derive(Debug, Deserialize)]
struct DecryptedObject {
    /// Organisation/scope (comma-separated, case-insensitive).
    o: Option<String>,
    /// Specific repository (optional).
    r: Option<String>,
    /// The actual secret value.
    v: Option<String>,
}

fn ensure_trailing_slash(s: &str) -> String {
    if s.ends_with('/') {
        s.to_owned()
    } else {
        format!("{s}/")
    }
}

/// Extract the Azure DevOps collection name from an endpoint URL.
///
/// Returns `None` for non-Azure endpoints or endpoints without a collection name.
/// Mirrors `getAzureCollection()` from `lib/config/decrypt.ts`.
pub fn get_azure_collection(endpoint: &str) -> Option<String> {
    let url = url::Url::parse(endpoint).ok()?;
    let pathname = url.path();
    let trimmed = pathname.trim_matches('/');
    if trimmed.is_empty() {
        return None;
    }
    // Strip "tfs/" prefix for on-premises Azure DevOps Server
    let collection = if trimmed.to_ascii_lowercase().starts_with("tfs/") {
        &trimmed[4..]
    } else {
        trimmed
    };
    if collection.is_empty() {
        None
    } else {
        Some(collection.to_owned())
    }
}

/// Validate that a decrypted value is authorised for the given repository.
///
/// `azure_endpoint` is the Azure DevOps endpoint URL (if platform is Azure).
/// Returns the value when authorised, `None` otherwise.
///
/// Mirrors `validateDecryptedValue` from `lib/config/decrypt.ts`.
pub fn validate_decrypted_value_with_endpoint(
    decrypted_obj_str: &str,
    repository: &str,
    azure_endpoint: Option<&str>,
) -> Option<String> {
    let obj: DecryptedObject = serde_json::from_str(decrypted_obj_str).ok()?;

    let value = obj.v.filter(|v| !v.is_empty())?;
    let org = obj.o.filter(|o| !o.is_empty())?;
    let repo = obj.r.unwrap_or_default();

    let azure_collection = azure_endpoint.and_then(get_azure_collection);
    let azcol = azure_collection
        .as_deref()
        .map(|c| ensure_trailing_slash(&c.to_uppercase()));

    let mut repositories = vec![repository.to_uppercase()];
    if let Some(col) = &azure_collection {
        let col_upper = col.to_uppercase();
        repositories.push(format!("{}/{}", col_upper, repository.to_uppercase()));
        repositories.push(format!("{}/*/", col_upper));
    }

    let org_prefixes: Vec<String> = org
        .split(',')
        .map(|o| ensure_trailing_slash(o.trim()).to_uppercase())
        .collect();

    if !repo.is_empty() {
        let scoped_repos: Vec<String> = org_prefixes
            .iter()
            .map(|prefix| format!("{}{}", prefix, repo.to_uppercase()))
            .collect();
        for rp in &repositories {
            if scoped_repos.iter().any(|r| r == rp) {
                return Some(value);
            }
        }
        None
    } else {
        for rp in &repositories {
            if org_prefixes.iter().any(|prefix| {
                rp.starts_with(prefix.as_str()) && azcol.as_deref() != Some(prefix.as_str())
            }) {
                return Some(value);
            }
        }
        None
    }
}

/// Validate that a decrypted value is authorised for the given repository.
/// Non-Azure version (no endpoint context).
pub fn validate_decrypted_value(decrypted_obj_str: &str, repository: &str) -> Option<String> {
    validate_decrypted_value_with_endpoint(decrypted_obj_str, repository, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "equals("$str", "$repo") === $expected" (platforms non azure) — config/decrypt.spec.ts line 68
    #[test]
    fn validate_decrypted_value_platforms_non_azure() {
        let cases: &[(&str, &str, Option<&str>)] = &[
            // Malformed JSON → None
            (
                r#"{"o":"abcd",         "r":"",     "v":"123#"#,
                "abcd/edf",
                None,
            ),
            // Empty value → None
            (
                r#"{"o":"abcd",         "r":"",     "v":""}"#,
                "abcd/edf",
                None,
            ),
            // Empty org → None
            (
                r#"{"o":"",             "r":"",     "v":"val"}"#,
                "abcd/edf",
                None,
            ),
            // Exact org+repo match
            (
                r#"{"o":"abcd",         "r":"edf",  "v":"val-1"}"#,
                "abcd/edf",
                Some("val-1"),
            ),
            // Org prefix match (no specific repo)
            (
                r#"{"o":"abcd",         "r":"",     "v":"val-2"}"#,
                "abcd/edf",
                Some("val-2"),
            ),
            // Nested org/repo match
            (
                r#"{"o":"abcd/fgh",     "r":"ef",   "v":"val-3"}"#,
                "abcd/fgh/ef",
                Some("val-3"),
            ),
            // Nested org prefix match
            (
                r#"{"o":"abcd/fgh",     "r":"",     "v":"val-4"}"#,
                "abcd/fgh/ef",
                Some("val-4"),
            ),
            // Deep org/repo match
            (
                r#"{"o":"a/b/c/d",      "r":"ef",   "v":"val-5"}"#,
                "a/b/c/d/ef",
                Some("val-5"),
            ),
            // Scoped to different repo → None
            (
                r#"{"o":"abcd/fgh",     "r":"any",  "v":"val-6"}"#,
                "abcd/fgh/ef",
                None,
            ),
            // Org mismatch → None
            (
                r#"{"o":"abcd/xy",      "r":"",     "v":"val-7"}"#,
                "abcd/fgh/ef",
                None,
            ),
            // Org mismatch → None
            (
                r#"{"o":"xy",           "r":"",     "v":"val-8"}"#,
                "abcd/fgh/ef",
                None,
            ),
            // Comma-separated org list, second matches
            (
                r#"{"o":"xy, abcd/fgh", "r":"ef",   "v":"val-9"}"#,
                "abcd/fgh/ef",
                Some("val-9"),
            ),
            // Comma-separated org list, second matches
            (
                r#"{"o":"xy ,abcd",     "r":"ef",   "v":"val-10"}"#,
                "abcd/ef",
                Some("val-10"),
            ),
            // Comma-separated org list, first matches (prefix)
            (
                r#"{"o":"abcd, xy",     "r":"",     "v":"val-11"}"#,
                "abcd/fgh/ef",
                Some("val-11"),
            ),
            // Comma-separated org list
            (
                r#"{"o":"abcd,xy ",     "r":"",     "v":"val-12"}"#,
                "abcd/ef",
                Some("val-12"),
            ),
            // First org in list is " xy", trimmed to "xy" — doesn't match abcd/...
            (
                r#"{"o":" xy,abc",      "r":"",     "v":"val-13"}"#,
                "abcd/fgh/ef",
                None,
            ),
        ];
        for &(str_, repo, expected) in cases {
            let result = validate_decrypted_value(str_, repo);
            assert_eq!(
                result.as_deref(),
                expected,
                "validate_decrypted_value({str_:?}, {repo:?})"
            );
        }
    }
}

// Ported: "equals("$str", "$repo") === $expected" (azure with dev.azure.com) — config/decrypt.spec.ts line 93
#[test]
fn validate_decrypted_value_azure_dev() {
    let endpoint = "https://dev.azure.com/az123";
    let cases: &[(&str, &str, Option<&str>)] = &[
        (r#"{"o":"any","r":"","v":"wrong-123#"#, "fgh/rp1", None),
        (r#"{"o":"any","r":"","v":""}"#, "fgh/rp1", None),
        (r#"{"o":"","r":"","v":"any"}"#, "fgh/rp1", None),
        (
            r#"{"o":"fgh","r":"rp1","v":"zv-1"}"#,
            "fgh/rp1",
            Some("zv-1"),
        ),
        (r#"{"o":"fgh","r":"","v":"zv-2"}"#, "fgh/rp1", Some("zv-2")),
        (
            r#"{"o":"az123/fgh","r":"rp1","v":"zv-3"}"#,
            "fgh/rp1",
            Some("zv-3"),
        ),
        (
            r#"{"o":"az123/fgh","r":"","v":"zv-4"}"#,
            "fgh/rp1",
            Some("zv-4"),
        ),
        (
            r#"{"o":"az123/*","r":"","v":"zv-5"}"#,
            "fgh/rp1",
            Some("zv-5"),
        ),
        (r#"{"o":"az123/","r":"","v":"zv-6"}"#, "fgh/rp1", None),
        (r#"{"o":"az123","r":"","v":"zv-7"}"#, "fgh/rp1", None),
        (r#"{"o":"az1","r":"","v":"zv-8"}"#, "fgh/rp1", None),
        (r#"{"o":"az123/any","r":"rp1","v":"zv-9"}"#, "fgh/rp1", None),
        (r#"{"o":"az123/any","r":"","v":"zv-10"}"#, "fgh/rp1", None),
        (r#"{"o":"any/*","r":"","v":"zv-11"}"#, "fgh/rp1", None),
        (
            r#"{"o":"az123/*,any/*","r":"","v":"zv-12"}"#,
            "fgh/rp1",
            Some("zv-12"),
        ),
        (
            r#"{"o":"fgh,any/*","r":"","v":"zv-13"}"#,
            "fgh/rp1",
            Some("zv-13"),
        ),
        (
            r#"{"o":"az123/,any/*","r":"","v":"zv-14"}"#,
            "fgh/rp1",
            None,
        ),
        (
            r#"{"o":"any/*,fgh/","r":"","v":"zv-15"}"#,
            "fgh/rp1",
            Some("zv-15"),
        ),
        (r#"{"o":"any/*,az123","r":"","v":"zv-16"}"#, "fgh/rp1", None),
        (r#"{"o":"any/*,az12","r":"","v":"zv-17"}"#, "fgh/rp1", None),
        (r#"{"o":"az12,any/*","r":"","v":"zv-18"}"#, "fgh/rp1", None),
    ];
    for &(str_, repo, expected) in cases {
        let result = validate_decrypted_value_with_endpoint(str_, repo, Some(endpoint));
        assert_eq!(
            result.as_deref(),
            expected,
            "validate_azure_dev({str_:?}, {repo:?})"
        );
    }
}

// Ported: "equals("$str", "$repo") === $expected" (azure with tfs self-hosted) — config/decrypt.spec.ts line 129
#[test]
fn validate_decrypted_value_azure_tfs() {
    let endpoint = "http://your-server-name:8080/tfs/az123";
    let cases: &[(&str, &str, Option<&str>)] = &[
        (
            r#"{"o":"fgh","r":"rp1","v":"zv-1"}"#,
            "fgh/rp1",
            Some("zv-1"),
        ),
        (r#"{"o":"fgh","r":"","v":"zv-2"}"#, "fgh/rp1", Some("zv-2")),
        (
            r#"{"o":"az123/fgh","r":"rp1","v":"zv-3"}"#,
            "fgh/rp1",
            Some("zv-3"),
        ),
        (
            r#"{"o":"az123/fgh","r":"","v":"zv-4"}"#,
            "fgh/rp1",
            Some("zv-4"),
        ),
        (
            r#"{"o":"az123/*","r":"","v":"zv-5"}"#,
            "fgh/rp1",
            Some("zv-5"),
        ),
    ];
    for &(str_, repo, expected) in cases {
        let result = validate_decrypted_value_with_endpoint(str_, repo, Some(endpoint));
        assert_eq!(
            result.as_deref(),
            expected,
            "validate_azure_tfs({str_:?}, {repo:?})"
        );
    }
}

// Ported: "endpoint URL invalid" — config/decrypt.spec.ts line 164
#[test]
fn validate_decrypted_value_azure_invalid_endpoint() {
    let endpoint = "ht tps://dev.az ure.com/az123"; // invalid URL
    assert_eq!(
        validate_decrypted_value_with_endpoint(
            r#"{"o":"proj","r":"repo","v":"any-1"}"#,
            "proj/repo",
            Some(endpoint)
        )
        .as_deref(),
        Some("any-1")
    );
    assert_eq!(
        validate_decrypted_value_with_endpoint(
            r#"{"o":"proj","r":"","v":"any-2"}"#,
            "proj/repo",
            Some(endpoint)
        )
        .as_deref(),
        Some("any-2")
    );
    assert_eq!(
        validate_decrypted_value_with_endpoint(
            r#"{"o":"col/proj","r":"","v":"any"}"#,
            "proj/repo",
            Some(endpoint)
        ),
        None
    );
    assert_eq!(
        validate_decrypted_value_with_endpoint(
            r#"{"o":"col/*","r":"","v":"any"}"#,
            "proj/repo",
            Some(endpoint)
        ),
        None
    );
}

// Ported: "endpoint URL without collection" — config/decrypt.spec.ts line 196
#[test]
fn validate_decrypted_value_azure_no_collection() {
    let endpoint = "https://dev.azure.com/";
    assert_eq!(
        validate_decrypted_value_with_endpoint(
            r#"{"o":"proj","r":"repo","v":"any-3"}"#,
            "proj/repo",
            Some(endpoint)
        )
        .as_deref(),
        Some("any-3")
    );
    assert_eq!(
        validate_decrypted_value_with_endpoint(
            r#"{"o":"proj","r":"","v":"any-4"}"#,
            "proj/repo",
            Some(endpoint)
        )
        .as_deref(),
        Some("any-4")
    );
    assert_eq!(
        validate_decrypted_value_with_endpoint(
            r#"{"o":"col/proj","r":"","v":"any"}"#,
            "proj/repo",
            Some(endpoint)
        ),
        None
    );
    assert_eq!(
        validate_decrypted_value_with_endpoint(
            r#"{"o":"col/*","r":"","v":"any"}"#,
            "proj/repo",
            Some(endpoint)
        ),
        None
    );
}

// Ported: "no pathname and url ends with slash" — config/decrypt.spec.ts line 235
#[test]
fn get_azure_collection_no_pathname_slash() {
    assert_eq!(get_azure_collection("https://dev.azure.com/"), None);
}

// Ported: "no pathname and no slash at end of URL" — config/decrypt.spec.ts line 243
#[test]
fn get_azure_collection_no_pathname_no_slash() {
    assert_eq!(get_azure_collection("https://dev.azure.com"), None);
}

// Ported: "pathname no slash at end" — config/decrypt.spec.ts line 251
#[test]
fn get_azure_collection_pathname_no_slash() {
    assert_eq!(
        get_azure_collection("https://dev.azure.com/aaa").as_deref(),
        Some("aaa")
    );
}

// Ported: "pathname with slash at end" — config/decrypt.spec.ts line 259
#[test]
fn get_azure_collection_pathname_with_slash() {
    assert_eq!(
        get_azure_collection("https://dev.azure.com/aaa/").as_deref(),
        Some("aaa")
    );
}

// Ported: "pathname 2 levels no slash at end" — config/decrypt.spec.ts line 267
#[test]
fn get_azure_collection_pathname_2_levels_no_slash() {
    assert_eq!(
        get_azure_collection("https://dev.azure.com/aaa/bbb").as_deref(),
        Some("aaa/bbb")
    );
}

// Ported: "pathname 2 levels with slash at end" — config/decrypt.spec.ts line 275
#[test]
fn get_azure_collection_pathname_2_levels_with_slash() {
    assert_eq!(
        get_azure_collection("https://dev.azure.com/aaa/bbb/").as_deref(),
        Some("aaa/bbb")
    );
}
