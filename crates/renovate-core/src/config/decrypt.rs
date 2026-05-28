//! Config decryption utilities.
//!
//! Mirrors `lib/config/decrypt.ts`.

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
    if s.ends_with('/') { s.to_owned() } else { format!("{s}/") }
}

/// Validate that a decrypted value is authorised for the given repository.
///
/// Returns the value when authorised, `None` otherwise.
///
/// Mirrors `validateDecryptedValue` from `lib/config/decrypt.ts`.
/// Supports the "platforms non azure" case; the Azure `getAzureCollection`
/// integration is not yet implemented.
pub fn validate_decrypted_value(decrypted_obj_str: &str, repository: &str) -> Option<String> {
    let obj: DecryptedObject = serde_json::from_str(decrypted_obj_str).ok()?;

    let value = obj.v.filter(|v| !v.is_empty())?;
    let org = obj.o.filter(|o| !o.is_empty())?;
    let repo = obj.r.unwrap_or_default();

    let repo_upper = repository.to_uppercase();
    let repositories = vec![repo_upper];

    let org_prefixes: Vec<String> = org
        .split(',')
        .map(|o| ensure_trailing_slash(o.trim()).to_uppercase())
        .collect();

    if !repo.is_empty() {
        // Scoped to a specific org/repo
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
        // Scoped to org only — any repo under the org is allowed
        for rp in &repositories {
            if org_prefixes.iter().any(|prefix| rp.starts_with(prefix.as_str())) {
                return Some(value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "equals("$str", "$repo") === $expected" (platforms non azure)
    //        — config/decrypt.spec.ts line 68
    #[test]
    fn validate_decrypted_value_platforms_non_azure() {
        let cases: &[(&str, &str, Option<&str>)] = &[
            // Malformed JSON → None
            (r#"{"o":"abcd",         "r":"",     "v":"123#"#, "abcd/edf", None),
            // Empty value → None
            (r#"{"o":"abcd",         "r":"",     "v":""}"#, "abcd/edf", None),
            // Empty org → None
            (r#"{"o":"",             "r":"",     "v":"val"}"#, "abcd/edf", None),
            // Exact org+repo match
            (r#"{"o":"abcd",         "r":"edf",  "v":"val-1"}"#, "abcd/edf", Some("val-1")),
            // Org prefix match (no specific repo)
            (r#"{"o":"abcd",         "r":"",     "v":"val-2"}"#, "abcd/edf", Some("val-2")),
            // Nested org/repo match
            (r#"{"o":"abcd/fgh",     "r":"ef",   "v":"val-3"}"#, "abcd/fgh/ef", Some("val-3")),
            // Nested org prefix match
            (r#"{"o":"abcd/fgh",     "r":"",     "v":"val-4"}"#, "abcd/fgh/ef", Some("val-4")),
            // Deep org/repo match
            (r#"{"o":"a/b/c/d",      "r":"ef",   "v":"val-5"}"#, "a/b/c/d/ef", Some("val-5")),
            // Scoped to different repo → None
            (r#"{"o":"abcd/fgh",     "r":"any",  "v":"val-6"}"#, "abcd/fgh/ef", None),
            // Org mismatch → None
            (r#"{"o":"abcd/xy",      "r":"",     "v":"val-7"}"#, "abcd/fgh/ef", None),
            // Org mismatch → None
            (r#"{"o":"xy",           "r":"",     "v":"val-8"}"#, "abcd/fgh/ef", None),
            // Comma-separated org list, second matches
            (r#"{"o":"xy, abcd/fgh", "r":"ef",   "v":"val-9"}"#, "abcd/fgh/ef", Some("val-9")),
            // Comma-separated org list, second matches
            (r#"{"o":"xy ,abcd",     "r":"ef",   "v":"val-10"}"#, "abcd/ef", Some("val-10")),
            // Comma-separated org list, first matches (prefix)
            (r#"{"o":"abcd, xy",     "r":"",     "v":"val-11"}"#, "abcd/fgh/ef", Some("val-11")),
            // Comma-separated org list
            (r#"{"o":"abcd,xy ",     "r":"",     "v":"val-12"}"#, "abcd/ef", Some("val-12")),
            // First org in list is " xy", trimmed to "xy" — doesn't match abcd/...
            (r#"{"o":" xy,abc",      "r":"",     "v":"val-13"}"#, "abcd/fgh/ef", None),
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
