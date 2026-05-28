//! Git Tags datasource.
//!
//! Like `git_refs` but `get_releases` returns only `refs/tags/` entries,
//! not branches.
//!
//! Renovate reference: `lib/modules/datasource/git-tags/index.ts`

use crate::datasources::git_refs::{self, GitRefsResult, GitRelease};

/// Get releases containing only `refs/tags/` entries for a package.
///
/// Returns `None` if `ls_remote` is `None` (subprocess failed) or empty.
pub fn get_releases(package_name: &str, ls_remote: Option<&str>) -> Option<GitRefsResult> {
    let output = ls_remote?;
    if output.is_empty() {
        return None;
    }

    let raw_refs = git_refs::parse_ls_remote(output);
    let source_url = package_name
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .to_owned();

    let releases: Vec<GitRelease> = raw_refs
        .into_iter()
        .filter(|r| r.type_ == "tags")
        .map(|r| GitRelease {
            version: r.value.clone(),
            git_ref: r.value,
            new_digest: Some(r.hash),
        })
        .collect();

    Some(GitRefsResult {
        releases,
        source_url,
    })
}

/// Get the commit digest for a specific ref value (or HEAD) from ls-remote output.
///
/// Searches all ref types (unlike `git_refs::get_digest` which restricts to
/// `heads` and `tags`).  HEAD is looked up by the literal value `"HEAD"`.
pub fn get_digest(ls_remote: Option<&str>, new_value: Option<&str>) -> Option<String> {
    let output = ls_remote?;
    if output.is_empty() {
        return None;
    }

    let raw_refs = git_refs::parse_ls_remote(output);
    let find_value = new_value.unwrap_or("HEAD");

    raw_refs
        .iter()
        .find(|r| r.value == find_value)
        .map(|r| r.hash.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LS_REMOTE_1: &str = "\
a9920c014aebc28dc1b23e7efcc006d0455cc710\tHEAD
46fd703d4738905cd55e1c5c36a70e5d43432b9c\trefs/for/master
2e24e927538bbc03cc3cd946834c8f5fe333f32c\trefs/heads/feat/slim-image
a9920c014aebc28dc1b23e7efcc006d0455cc710\trefs/heads/master
a9920c014aebc28dc1b23e7efcc006d045512345\trefs/heads/v1.0.0
c152f7c4675575e498113873fd5538ec475b1dba\trefs/pull/11/head
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

    // Ported: "returns nil if response is wrong" — datasource/git-tags/index.spec.ts line 38
    #[test]
    fn returns_nil_if_response_is_wrong() {
        let result = get_releases("https://github.com/example/example.git", Some(""));
        assert!(result.is_none());
    }

    // Ported: "returns nil if remote call throws exception" — datasource/git-tags/index.spec.ts line 45
    #[test]
    fn returns_nil_if_remote_call_throws_exception() {
        let result = get_releases("https://github.com/example/example.git", None);
        assert!(result.is_none());
    }

    // Ported: "returns versions filtered from tags" — datasource/git-tags/index.spec.ts line 52
    #[test]
    fn returns_versions_filtered_from_tags() {
        let result =
            get_releases("https://github.com/example/example.git", Some(LS_REMOTE_1)).unwrap();

        assert_eq!(result.source_url, "https://github.com/example/example");
        // Only tags — no heads entries
        assert_eq!(result.releases.len(), 6);

        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert_eq!(
            versions,
            vec!["v1.0.0", "v1.0.1", "v1.0.2", "v1.0.3", "v1.0.4", "v1.0.5"]
        );

        // Annotated tags use dereferenced hashes
        let v1_0_0 = result
            .releases
            .iter()
            .find(|r| r.version == "v1.0.0")
            .unwrap();
        assert_eq!(
            v1_0_0.new_digest.as_deref(),
            Some("7b756026fb2de270240a889a413e7e3a9d4d4d85")
        );
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

    // Ported: "returns null if not found" — datasource/git-tags/index.spec.ts line 64
    #[test]
    fn returns_null_if_not_found() {
        let digest = get_digest(Some(LS_REMOTE_1), Some("notfound"));
        assert!(digest.is_none());
    }

    // Ported: "returns digest for tag" — datasource/git-tags/index.spec.ts line 74
    #[test]
    fn returns_digest_for_tag() {
        let digest = get_digest(Some(LS_REMOTE_1), Some("v1.0.2")).unwrap();
        assert_eq!(digest, "3936a6bced3587dc9fd464b0a910e0dfd4cfe10d");
    }

    // Ported: "returns digest for HEAD" — datasource/git-tags/index.spec.ts line 84
    #[test]
    fn returns_digest_for_head() {
        let digest = get_digest(Some(LS_REMOTE_1), None).unwrap();
        assert_eq!(digest, "a9920c014aebc28dc1b23e7efcc006d0455cc710");
    }

    // "returns digest for HEAD with authentication environment variables" (line 94),
    // "returns digest for HEAD with authentication environment variables for datasource type git-tags" (line 121)
    // → not-applicable: tests mock TypeScript-internal simpleGit infrastructure;
    //   no equivalent Rust mock surface.
}
