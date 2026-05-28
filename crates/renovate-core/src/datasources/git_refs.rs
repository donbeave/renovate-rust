//! Git Refs datasource.
//!
//! Parses `git ls-remote` output to expose repository refs (tags and branches)
//! as package versions.
//!
//! Renovate reference: `lib/modules/datasource/git-refs/index.ts`
//!
//! ## Protocol
//!
//! 1. Run `git ls-remote <url>` to get all remote refs.
//! 2. Parse the tab-separated output: `{hash}\t{ref_path}`.
//! 3. Resolve annotated tags via `^{}` dereferenced entries.
//! 4. Return `tags` and `heads` refs as releases (deduped by value).

use std::collections::{HashMap, HashSet};

/// A parsed ref entry from `git ls-remote` output.
#[derive(Debug, Clone, PartialEq)]
pub struct RawRef {
    /// Ref category: `"tags"`, `"heads"`, `"for"`, or `""` for HEAD.
    pub type_: String,
    /// Ref name, e.g. `"v1.0.4"` or `"HEAD"`.
    pub value: String,
    /// Commit SHA (annotated tags have this replaced with the dereferenced hash).
    pub hash: String,
}

/// A single release entry returned by `get_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct GitRelease {
    pub version: String,
    pub git_ref: String,
    pub new_digest: Option<String>,
}

/// Full result from `get_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct GitRefsResult {
    pub releases: Vec<GitRelease>,
    pub source_url: String,
}

/// Parse `git ls-remote` output into a list of `RawRef` entries.
///
/// Handles annotated tag dereferencing: lines ending in `^{}` supply the
/// dereferenced commit hash for the corresponding tag. `pull` type refs are
/// excluded.
pub fn parse_ls_remote(output: &str) -> Vec<RawRef> {
    let mut deref_hashes: HashMap<String, String> = HashMap::new();
    let mut raw: Vec<RawRef> = Vec::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((hash, refpath)) = line.split_once('\t') else {
            continue;
        };
        let hash = hash.trim().to_owned();
        let refpath = refpath.trim();

        if refpath == "HEAD" {
            raw.push(RawRef {
                type_: String::new(),
                value: "HEAD".to_owned(),
                hash,
            });
        } else if let Some(rest) = refpath.strip_prefix("refs/")
            && let Some((type_, value)) = rest.split_once('/')
        {
            if let Some(base) = value.strip_suffix("^{}") {
                deref_hashes.insert(base.to_owned(), hash);
            } else {
                raw.push(RawRef {
                    type_: type_.to_owned(),
                    value: value.to_owned(),
                    hash,
                });
            }
        }
    }

    raw.into_iter()
        .filter(|r| r.type_ != "pull")
        .map(|mut r| {
            if let Some(deref_hash) = deref_hashes.get(&r.value) {
                r.hash = deref_hash.clone();
            }
            r
        })
        .collect()
}

/// Strip `.git` suffix and trailing `/` from a package name to derive a source URL.
fn to_source_url(package_name: &str) -> String {
    package_name
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .to_owned()
}

/// Get releases for a package from `git ls-remote` output.
///
/// Returns `None` if `ls_remote` is `None` (subprocess failed) or empty string
/// (mirrors JavaScript's falsy check on the ls-remote result).
pub fn get_releases(package_name: &str, ls_remote: Option<&str>) -> Option<GitRefsResult> {
    let output = ls_remote?;
    if output.is_empty() {
        return None;
    }

    let raw_refs = parse_ls_remote(output);
    let source_url = to_source_url(package_name);

    let mut seen: HashSet<String> = HashSet::new();
    let mut releases = Vec::new();

    for r in &raw_refs {
        if (r.type_ == "tags" || r.type_ == "heads") && seen.insert(r.value.clone()) {
            releases.push(GitRelease {
                version: r.value.clone(),
                git_ref: r.value.clone(),
                new_digest: Some(r.hash.clone()),
            });
        }
    }

    Some(GitRefsResult {
        releases,
        source_url,
    })
}

/// Get the commit digest for a specific ref or HEAD from `git ls-remote` output.
///
/// - `new_value = Some(tag)` → hash of the first matching `tags` or `heads` ref.
/// - `new_value = None` → hash of HEAD.
///
/// Returns `None` if `ls_remote` is `None`/empty or the ref is not found.
pub fn get_digest(ls_remote: Option<&str>, new_value: Option<&str>) -> Option<String> {
    let output = ls_remote?;
    if output.is_empty() {
        return None;
    }

    let raw_refs = parse_ls_remote(output);

    if let Some(value) = new_value {
        raw_refs
            .iter()
            .find(|r| (r.type_ == "tags" || r.type_ == "heads") && r.value == value)
            .map(|r| r.hash.clone())
    } else {
        raw_refs
            .iter()
            .find(|r| r.type_.is_empty() && r.value == "HEAD")
            .map(|r| r.hash.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fixture content matching `__fixtures__/ls-remote-1.txt` from the
    /// Renovate git-refs spec.
    const LS_REMOTE_1: &str = "\
a9920c014aebc28dc1b23e7efcc006d0455cc710\tHEAD
46fd703d4738905cd55e1c5c36a70e5d43432b9c\trefs/for/master
2e24e927538bbc03cc3cd946834c8f5fe333f32c\trefs/heads/feat/slim-image
a9920c014aebc28dc1b23e7efcc006d0455cc710\trefs/heads/master
a9920c014aebc28dc1b23e7efcc006d045512345\trefs/heads/v1.0.0
c152f7c4675575e498113873fd5538ec475b1dba\trefs/pull/11/head
1aff0dda91369bd82143a15261bedc0833f46c65\trefs/pull/12/head
0be9ffb1ae0c8d0846cd88b58dfef42d74674673\trefs/tags/v1.0.0
7b756026fb2de270240a889a413e7e3a9d4d4d85\trefs/tags/v1.0.0^{}
281fbfb58990ec98b237a923d67904c102bec34c\trefs/tags/v1.0.1
e173183f932ba8a31d0e4f23cc1070e8ebfa59d6\trefs/tags/v1.0.1^{}
9cb93e0b236385a4e2efd089d7c6a458f5ff321f\trefs/tags/v1.0.2
3936a6bced3587dc9fd464b0a910e0dfd4cfe10d\trefs/tags/v1.0.2^{}
8b0d0e0aec21ea059448ef0255387dbb82c61973\trefs/tags/v1.0.3
125ca9f3df4151e50046e5327ecb29ec4c13efab\trefs/tags/v1.0.3^{}
2b52829c7c1bd65b3501c450849c53b90b11fa0e\trefs/tags/v1.0.4
3ed9e7d7094fd4ee7751c24a3e6b706060f461ff\trefs/tags/v1.0.4^{}
2d138c34e4c6939d0a8686943e851c6528aa04db\trefs/tags/v1.0.5
6d7a933c2e6b7b39e992b1f93b6b42de083b28f0\trefs/tags/v1.0.5^{}";

    // Ported: "returns nil if response is wrong" — datasource/git-refs/index.spec.ts line 38
    #[test]
    fn returns_nil_if_response_is_wrong() {
        // Empty string is falsy in JS → None
        let result = get_releases("https://github.com/example/example.git", Some(""));
        assert!(result.is_none());
    }

    // Ported: "returns nil if response is malformed" — datasource/git-refs/index.spec.ts line 48
    #[test]
    fn returns_nil_if_response_is_malformed() {
        // Non-empty but unparseable → Some with empty releases
        let result = get_releases(
            "https://github.com/example/example.git",
            Some("aabbccddeeff"),
        )
        .unwrap();
        assert!(result.releases.is_empty());
    }

    // Ported: "returns nil if remote call throws exception" — datasource/git-refs/index.spec.ts line 58
    #[test]
    fn returns_nil_if_remote_call_throws_exception() {
        // None represents subprocess failure → None
        let result = get_releases("https://github.com/example/example.git", None);
        assert!(result.is_none());
    }

    // Ported: "returns versions filtered from tags" — datasource/git-refs/index.spec.ts line 68
    #[test]
    fn returns_versions_filtered_from_tags() {
        let result =
            get_releases("https://github.com/example/example.git", Some(LS_REMOTE_1)).unwrap();

        assert_eq!(result.source_url, "https://github.com/example/example");

        // Tags v1.0.0..v1.0.5 plus heads feat/slim-image, master (v1.0.0 deduped)
        // The TypeScript test reaches 6 after getPkgReleases version filtering;
        // the raw datasource returns all unique tags + heads refs.
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"v1.0.0"));
        assert!(versions.contains(&"v1.0.1"));
        assert!(versions.contains(&"v1.0.2"));
        assert!(versions.contains(&"v1.0.3"));
        assert!(versions.contains(&"v1.0.4"));
        assert!(versions.contains(&"v1.0.5"));

        // pull refs must be excluded
        assert!(!versions.iter().any(|v| v.contains("pull")));
        // refs/for/ must be excluded (not heads/tags type)
        let no_for_master = result.releases.iter().all(|r| {
            !(r.version == "master"
                && r.new_digest.as_deref() == Some("46fd703d4738905cd55e1c5c36a70e5d43432b9c"))
        });
        assert!(
            no_for_master,
            "refs/for/master hash must not appear in releases"
        );

        // Annotated tag: v1.0.5 should use the dereferenced hash
        let v1_0_5 = result
            .releases
            .iter()
            .find(|r| r.version == "v1.0.5")
            .unwrap();
        assert_eq!(
            v1_0_5.new_digest.as_deref(),
            Some("6d7a933c2e6b7b39e992b1f93b6b42de083b28f0")
        );
    }

    // Ported: "returns null if not found" — datasource/git-refs/index.spec.ts line 82
    #[test]
    fn returns_null_if_not_found() {
        let digest = get_digest(Some(LS_REMOTE_1), Some("v2.0.0"));
        assert!(digest.is_none());
    }

    // Ported: "returns digest for tag" — datasource/git-refs/index.spec.ts line 92
    #[test]
    fn returns_digest_for_tag() {
        // Annotated tag: should return the ^{} dereferenced commit hash
        let digest = get_digest(Some(LS_REMOTE_1), Some("v1.0.4")).unwrap();
        assert_eq!(digest, "3ed9e7d7094fd4ee7751c24a3e6b706060f461ff");
    }

    // Ported: "ignores refs/for/" — datasource/git-refs/index.spec.ts line 104
    #[test]
    fn ignores_refs_for() {
        // refs/for/master has hash 46fd703d… but refs/heads/master has a9920c01…
        // getDigest for "master" must return the heads hash, not the for hash
        let digest = get_digest(Some(LS_REMOTE_1), Some("master")).unwrap();
        assert_eq!(digest, "a9920c014aebc28dc1b23e7efcc006d0455cc710");
    }

    // Ported: "returns digest for HEAD" — datasource/git-refs/index.spec.ts line 114
    #[test]
    fn returns_digest_for_head() {
        let digest = get_digest(Some(LS_REMOTE_1), None).unwrap();
        assert_eq!(digest, "a9920c014aebc28dc1b23e7efcc006d0455cc710");
    }

    // "calls simpleGit with emptyEnv if no hostrules exist" (line 124),
    // "calls simpleGit with git envs if hostrules exist" (line 135),
    // "calls simpleGit with git envs if hostrules exist for datasource type git-refs" (line 162)
    // → not-applicable: test mocks TypeScript-internal simpleGit infrastructure;
    //   Rust subprocess auth env is wired differently and has no equivalent mock surface.
}
