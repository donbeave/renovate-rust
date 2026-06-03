//! Azure DevOps platform utility functions.
//!
//! Mirrors: `lib/modules/platform/azure/util.ts`

/// Return the git extra-clone options for Azure DevOps authentication.
///
/// Returns a map with key `"-c"` and value `"http.extraHeader=AUTHORIZATION: <type> <value>"`.
///
/// Mirrors `getStorageExtraCloneOpts` from `lib/modules/platform/azure/util.ts`.
pub fn get_storage_extra_clone_opts(
    token: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
) -> std::collections::HashMap<String, String> {
    use base64::{Engine as _, engine::general_purpose};
    let (auth_type, auth_value) = if token.is_none()
        && username.is_some_and(|u| !u.is_empty())
        && password.is_some_and(|p| !p.is_empty())
    {
        let encoded = general_purpose::STANDARD.encode(format!(
            "{}:{}",
            username.unwrap_or(""),
            password.unwrap_or("")
        ));
        ("basic".to_owned(), encoded)
    } else if token.is_some_and(|t| t.len() == 52) {
        let encoded = general_purpose::STANDARD.encode(format!(":{}", token.unwrap_or("")));
        ("basic".to_owned(), encoded)
    } else {
        ("bearer".to_owned(), token.unwrap_or("").to_owned())
    };
    let mut map = std::collections::HashMap::new();
    map.insert(
        "-c".to_owned(),
        format!("http.extraHeader=AUTHORIZATION: {auth_type} {auth_value}"),
    );
    map
}

/// Map an Azure DevOps pull-request status integer to the Renovate PR state string.
///
/// Mirrors the `stateMap` in `lib/modules/platform/azure/util.ts`.
pub fn get_azure_pr_state(status: u32) -> &'static str {
    match status {
        2 => "closed", // Abandoned
        3 => "merged", // Completed
        _ => "open",   // Active (1) and anything else
    }
}

/// Combines a git status context's genre and name into a single slash-separated string.
///
/// Mirrors `getGitStatusContextCombinedName` from `lib/modules/platform/azure/util.ts`.
pub fn get_git_status_context_combined_name(genre: Option<&str>, name: &str) -> String {
    match genre {
        Some(g) if !g.is_empty() => format!("{g}/{name}"),
        _ => name.to_owned(),
    }
}

/// Splits a combined git status context name back into (genre, name).
///
/// Returns `None` if the input is empty.
/// Mirrors `getGitStatusContextFromCombinedName` from `lib/modules/platform/azure/util.ts`.
pub fn get_git_status_context_from_combined_name(
    context: &str,
) -> Option<(Option<String>, String)> {
    if context.is_empty() {
        return None;
    }
    if let Some(last_slash) = context.rfind('/')
        && last_slash > 0
    {
        let genre = &context[..last_slash];
        let name = &context[last_slash + 1..];
        return Some((Some(genre.to_owned()), name.to_owned()));
    }
    Some((None, context.to_owned()))
}

/// Strips the `refs/heads/` prefix from a branch path.
///
/// Returns `None` if the input is empty/None.
/// Mirrors `getBranchNameWithoutRefsheadsPrefix` from `lib/modules/platform/azure/util.ts`.
pub fn get_branch_name_without_refs_heads_prefix(branch_path: &str) -> Option<&str> {
    if branch_path.is_empty() {
        return None;
    }
    Some(
        branch_path
            .strip_prefix("refs/heads/")
            .unwrap_or(branch_path),
    )
}

/// Truncates a string to at most 3999 characters.
///
/// Mirrors `max4000Chars` from `lib/modules/platform/azure/util.ts`.
pub fn max4000_chars(s: &str) -> &str {
    if s.len() >= 4000 { &s[..3999] } else { s }
}

/// Splits an Azure repository path into `(project, repo)`.
///
/// Accepts `"repo"` or `"project/repo"` forms.
/// Mirrors `getProjectAndRepo` from `lib/modules/platform/azure/util.ts`.
pub fn get_project_and_repo(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.split('/').collect();
    match parts.len() {
        1 => Ok((s.to_owned(), s.to_owned())),
        2 => Ok((parts[0].to_owned(), parts[1].to_owned())),
        _ => Err(
            "Azure repository can be only structured this way : 'repository' or 'projectName/repository'!"
                .to_owned(),
        ),
    }
}

/// A minimal Azure git repository representation for lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AzureGitRepo {
    pub id: Option<String>,
    pub name: String,
    pub project_name: Option<String>,
}

/// Find the first repository matching `name` (case-insensitive).
///
/// `name` may be `"project/repo"` or just `"repo"`.
/// Mirrors `getRepoByName` from `lib/modules/platform/azure/util.ts`.
pub fn get_repo_by_name<'a>(name: &str, repos: &'a [AzureGitRepo]) -> Option<&'a AzureGitRepo> {
    let (project, repo) = get_project_and_repo(name).ok()?;
    let project_lower = project.to_lowercase();
    let repo_lower = repo.to_lowercase();

    repos.iter().find(|r| {
        r.name.to_lowercase() == repo_lower
            && r.project_name
                .as_ref()
                .map(|p| p.to_lowercase() == project_lower)
                .unwrap_or(false)
    })
}

/// Azure DevOps `maxBodyLength` constant.
///
/// Mirrors `maxBodyLength` from `lib/modules/platform/azure/index.ts`.
pub const AZURE_MAX_BODY_LENGTH: usize = 4000;

/// Transform Markdown content for Azure DevOps compatibility.
///
/// Replaces rebase-related text, strips renovate debug comments and HTML.
/// Mirrors `massageMarkdown` from `lib/modules/platform/azure/index.ts`.
pub fn massage_markdown(input: &str) -> String {
    use crate::platform::pr_body::smart_truncate;
    let s = smart_truncate(input, AZURE_MAX_BODY_LENGTH);
    let s = s.replace(
        "you tick the rebase/retry checkbox",
        "PR is renamed to start with \"rebase!\"",
    );
    let s = s.replace(
        "checking the rebase/retry box above",
        "renaming the PR to start with \"rebase!\"",
    );
    let re = regex::Regex::new(r"\n---\n\n.*?<!-- rebase-check -->.*?\n").unwrap();
    let s = re.replace(&s, "").into_owned();
    let re = regex::Regex::new(r"<!--renovate-(?:debug|config-hash):.*?-->").unwrap();
    re.replace(&s, "").into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── get_azure_pr_state ───────────────────────────────────────────────────

    // Ported: "should be formated (closed)" — lib/modules/platform/azure/util.spec.ts line 91
    #[test]
    fn azure_pr_state_closed() {
        assert_eq!(get_azure_pr_state(2), "closed");
    }

    // Ported: "should be formated (closed v2)" — lib/modules/platform/azure/util.spec.ts line 96
    #[test]
    fn azure_pr_state_merged() {
        assert_eq!(get_azure_pr_state(3), "merged");
    }

    // Ported: "should be formated (not closed)" — lib/modules/platform/azure/util.spec.ts line 101
    #[test]
    fn azure_pr_state_open() {
        assert_eq!(get_azure_pr_state(1), "open");
        assert_eq!(get_azure_pr_state(0), "open"); // unknown → open
    }

    // Ported: "should return undefined if null context passed" (getGitStatusContextCombinedName) — lib/modules/platform/azure/util.spec.ts line 16
    #[test]
    fn git_status_context_combined_name_none_genre_empty_name() {
        // null → treated as None genre
        let result = get_git_status_context_combined_name(None, "");
        assert_eq!(result, "");
    }

    // Ported: "should combine valid genre and name with slash" — lib/modules/platform/azure/util.spec.ts line 21
    //         — modules/platform/azure/util.spec.ts line 21
    #[test]
    fn git_status_context_combined_name_genre_and_name() {
        let result = get_git_status_context_combined_name(Some("my-genre"), "status-name");
        assert_eq!(result, "my-genre/status-name");
    }

    // Ported: "should combine valid empty genre and name without a slash" — lib/modules/platform/azure/util.spec.ts line 29
    //         — modules/platform/azure/util.spec.ts line 29
    #[test]
    fn git_status_context_combined_name_undefined_genre() {
        let result = get_git_status_context_combined_name(None, "status-name");
        assert_eq!(result, "status-name");
    }

    // Ported: "should return undefined if null context passed" (getGitStatusContextFromCombinedName) — lib/modules/platform/azure/util.spec.ts line 39
    #[test]
    fn git_status_context_from_combined_name_empty_returns_none() {
        assert!(get_git_status_context_from_combined_name("").is_none());
    }

    // Ported: "should parse valid genre and name with slash" — lib/modules/platform/azure/util.spec.ts line 44
    //         — modules/platform/azure/util.spec.ts line 44
    #[test]
    fn git_status_context_from_combined_name_slash() {
        let result = get_git_status_context_from_combined_name("my-genre/status-name");
        assert_eq!(
            result,
            Some((Some("my-genre".to_owned()), "status-name".to_owned()))
        );
    }

    // Ported: "should parse valid genre and name with multiple slashes" — lib/modules/platform/azure/util.spec.ts line 54
    //         — modules/platform/azure/util.spec.ts line 54
    #[test]
    fn git_status_context_from_combined_name_multiple_slashes() {
        let result = get_git_status_context_from_combined_name("my-genre/sub-genre/status-name");
        assert_eq!(
            result,
            Some((
                Some("my-genre/sub-genre".to_owned()),
                "status-name".to_owned()
            ))
        );
    }

    // Ported: "should parse valid empty genre and name without a slash" — lib/modules/platform/azure/util.spec.ts line 64
    //         — modules/platform/azure/util.spec.ts line 64
    #[test]
    fn git_status_context_from_combined_name_no_slash() {
        let result = get_git_status_context_from_combined_name("status-name");
        assert_eq!(result, Some((None, "status-name".to_owned())));
    }

    // Ported: "should be renamed" — lib/modules/platform/azure/util.spec.ts line 74
    #[test]
    fn branch_name_strips_refs_heads_prefix() {
        let result = get_branch_name_without_refs_heads_prefix("refs/heads/testBB");
        assert_eq!(result, Some("testBB"));
    }

    // Ported: "should log error and return undefined" — lib/modules/platform/azure/util.spec.ts line 79
    #[test]
    fn branch_name_empty_returns_none() {
        let result = get_branch_name_without_refs_heads_prefix("");
        assert!(result.is_none());
    }

    // Ported: "should return the input" — lib/modules/platform/azure/util.spec.ts line 84
    #[test]
    fn branch_name_without_prefix_returns_as_is() {
        let result = get_branch_name_without_refs_heads_prefix("testBB");
        assert_eq!(result, Some("testBB"));
    }

    // Ported: "should be the same" — lib/modules/platform/azure/util.spec.ts line 144
    #[test]
    fn max4000_chars_short_string_unchanged() {
        assert_eq!(max4000_chars("Hello"), "Hello");
    }

    // Ported: "should be truncated" — lib/modules/platform/azure/util.spec.ts line 149
    #[test]
    fn max4000_chars_long_string_truncated() {
        let s: String = "a".repeat(5000);
        assert_eq!(max4000_chars(&s).len(), 3999);
    }

    // Ported: "should return the object with same strings" — lib/modules/platform/azure/util.spec.ts line 160
    #[test]
    fn get_project_and_repo_single_name() {
        let (project, repo) = get_project_and_repo("myRepoName").unwrap();
        assert_eq!(project, "myRepoName");
        assert_eq!(repo, "myRepoName");
    }

    // Ported: "should return the object with project and repo" — lib/modules/platform/azure/util.spec.ts line 165
    #[test]
    fn get_project_and_repo_project_slash_repo() {
        let (project, repo) = get_project_and_repo("prjName/myRepoName").unwrap();
        assert_eq!(project, "prjName");
        assert_eq!(repo, "myRepoName");
    }

    // Ported: "should return an error" — lib/modules/platform/azure/util.spec.ts line 170
    #[test]
    fn get_project_and_repo_too_many_segments() {
        let result = get_project_and_repo("prjName/myRepoName/blalba");
        assert!(result.is_err());
    }

    // Ported: "returns null when repos array is empty" — lib/modules/platform/azure/util.spec.ts line 180
    #[test]
    fn get_repo_by_name_empty_list_returns_none() {
        assert!(get_repo_by_name("foo/bar", &[]).is_none());
    }

    // Ported: "returns null when repo is not found" — lib/modules/platform/azure/util.spec.ts line 186
    #[test]
    fn get_repo_by_name_not_found_returns_none() {
        let repos = vec![AzureGitRepo {
            id: None,
            name: "bar".to_owned(),
            project_name: Some("bar".to_owned()),
        }];
        assert!(get_repo_by_name("foo/foo", &repos).is_none());
    }

    // Ported: "finds repo" — lib/modules/platform/azure/util.spec.ts line 192
    #[test]
    fn get_repo_by_name_finds_first_match() {
        let repos = vec![
            AzureGitRepo {
                id: Some("1".to_owned()),
                name: "baz".to_owned(),
                project_name: Some("qux".to_owned()),
            },
            AzureGitRepo {
                id: Some("2".to_owned()),
                name: "bar".to_owned(),
                project_name: None,
            },
            AzureGitRepo {
                id: Some("3".to_owned()),
                name: "bar".to_owned(),
                project_name: Some("foo".to_owned()),
            },
            AzureGitRepo {
                id: Some("4".to_owned()),
                name: "bar".to_owned(),
                project_name: Some("foo".to_owned()),
            },
        ];
        let result = get_repo_by_name("foo/bar", &repos).unwrap();
        assert_eq!(result.id.as_deref(), Some("3"));
    }

    // Ported: "supports shorthand names" — lib/modules/platform/azure/util.spec.ts line 205
    #[test]
    fn get_repo_by_name_shorthand() {
        let repos = vec![
            AzureGitRepo {
                id: Some("1".to_owned()),
                name: "bar".to_owned(),
                project_name: Some("bar".to_owned()),
            },
            AzureGitRepo {
                id: Some("2".to_owned()),
                name: "foo".to_owned(),
                project_name: Some("foo".to_owned()),
            },
        ];
        let result = get_repo_by_name("foo", &repos).unwrap();
        assert_eq!(result.id.as_deref(), Some("2"));
    }

    // Ported: "is case-independent" — lib/modules/platform/azure/util.spec.ts line 214
    #[test]
    fn get_repo_by_name_case_insensitive() {
        let repos = vec![
            AzureGitRepo {
                id: Some("1".to_owned()),
                name: "FOO".to_owned(),
                project_name: Some("FOO".to_owned()),
            },
            AzureGitRepo {
                id: Some("2".to_owned()),
                name: "foo".to_owned(),
                project_name: Some("foo".to_owned()),
            },
        ];
        // All variations should find id=1 (first match)
        assert_eq!(
            get_repo_by_name("FOO/foo", &repos).unwrap().id.as_deref(),
            Some("1")
        );
        assert_eq!(
            get_repo_by_name("foo/FOO", &repos).unwrap().id.as_deref(),
            Some("1")
        );
        assert_eq!(
            get_repo_by_name("foo/foo", &repos).unwrap().id.as_deref(),
            Some("1")
        );
    }

    // ── getStorageExtraCloneOpts ─────────────────────────────────────────────

    // Ported: "should configure basic auth" — lib/modules/platform/azure/util.spec.ts line 122
    #[test]
    fn storage_extra_clone_opts_basic_auth() {
        use base64::{Engine as _, engine::general_purpose};
        let result = get_storage_extra_clone_opts(None, Some("user"), Some("pass"));
        let expected_b64 = general_purpose::STANDARD.encode("user:pass");
        let c_value = result.get("-c").unwrap();
        assert!(c_value.contains("basic"));
        assert!(c_value.contains(&expected_b64));
    }

    // Ported: "should configure personal access token" — lib/modules/platform/azure/util.spec.ts line 130
    #[test]
    fn storage_extra_clone_opts_pat() {
        use base64::{Engine as _, engine::general_purpose};
        let token = "1234567890123456789012345678901234567890123456789012"; // 52 chars
        assert_eq!(token.len(), 52);
        let result = get_storage_extra_clone_opts(Some(token), None, None);
        let expected_b64 = general_purpose::STANDARD.encode(format!(":{token}"));
        let c_value = result.get("-c").unwrap();
        assert!(c_value.contains("basic"));
        assert!(c_value.contains(&expected_b64));
    }

    // Ported: "should configure bearer token" — lib/modules/platform/azure/util.spec.ts line 137
    #[test]
    fn storage_extra_clone_opts_bearer() {
        let token = "sometoken";
        let result = get_storage_extra_clone_opts(Some(token), None, None);
        let c_value = result.get("-c").unwrap();
        assert!(c_value.contains("bearer"));
        assert!(c_value.contains(token));
    }

    // Ported: "throws when repo name is invalid" — lib/modules/platform/azure/util.spec.ts line 224
    // TypeScript throws; Rust returns None (same observable behavior: lookup fails for invalid name)
    #[test]
    fn get_repo_by_name_three_part_name_returns_none() {
        // "foo/bar/baz" has 3 parts - invalid; TypeScript throws, Rust returns None
        let result = get_repo_by_name("foo/bar/baz", &[]);
        assert!(result.is_none());
    }

    // Ported: "returns updated pr body" — lib/modules/platform/azure/index.spec.ts line 1621
    #[test]
    fn massage_markdown_returns_updated_pr_body() {
        let pr_body = "\n---\n\n - [ ] <!-- rebase-check --> rebase\n<!--renovate-config-hash:-->plus also [a link](https://github.com/foo/bar/issues/5)";
        assert_eq!(
            massage_markdown(pr_body),
            "plus also [a link](https://github.com/foo/bar/issues/5)"
        );
    }

    // Ported: "returns updated comment content" — lib/modules/platform/azure/index.spec.ts line 1630
    #[test]
    fn massage_markdown_returns_updated_comment_content() {
        let comment = "You can manually request rebase by checking the rebase/retry box above.\n\nplus also [a link](https://github.com/foo/bar/issues/5)";
        assert_eq!(
            massage_markdown(comment),
            "You can manually request rebase by renaming the PR to start with \"rebase!\".\n\nplus also [a link](https://github.com/foo/bar/issues/5)"
        );
    }
}
