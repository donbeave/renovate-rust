//! Azure DevOps platform utility functions.
//!
//! Mirrors: `lib/modules/platform/azure/util.ts`

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

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should return undefined if null context passed" (getGitStatusContextCombinedName)
    //         — modules/platform/azure/util.spec.ts line 17
    #[test]
    fn git_status_context_combined_name_none_genre_empty_name() {
        // null → treated as None genre
        let result = get_git_status_context_combined_name(None, "");
        assert_eq!(result, "");
    }

    // Ported: "should combine valid genre and name with slash"
    //         — modules/platform/azure/util.spec.ts line 22
    #[test]
    fn git_status_context_combined_name_genre_and_name() {
        let result = get_git_status_context_combined_name(Some("my-genre"), "status-name");
        assert_eq!(result, "my-genre/status-name");
    }

    // Ported: "should combine valid empty genre and name without a slash"
    //         — modules/platform/azure/util.spec.ts line 30
    #[test]
    fn git_status_context_combined_name_undefined_genre() {
        let result = get_git_status_context_combined_name(None, "status-name");
        assert_eq!(result, "status-name");
    }

    // Ported: "should return undefined if null context passed" (getGitStatusContextFromCombinedName)
    //         — modules/platform/azure/util.spec.ts line 40
    #[test]
    fn git_status_context_from_combined_name_empty_returns_none() {
        assert!(get_git_status_context_from_combined_name("").is_none());
    }

    // Ported: "should parse valid genre and name with slash"
    //         — modules/platform/azure/util.spec.ts line 45
    #[test]
    fn git_status_context_from_combined_name_slash() {
        let result = get_git_status_context_from_combined_name("my-genre/status-name");
        assert_eq!(
            result,
            Some((Some("my-genre".to_owned()), "status-name".to_owned()))
        );
    }

    // Ported: "should parse valid genre and name with multiple slashes"
    //         — modules/platform/azure/util.spec.ts line 52
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

    // Ported: "should parse valid empty genre and name without a slash"
    //         — modules/platform/azure/util.spec.ts line 60
    #[test]
    fn git_status_context_from_combined_name_no_slash() {
        let result = get_git_status_context_from_combined_name("status-name");
        assert_eq!(result, Some((None, "status-name".to_owned())));
    }

    // Ported: "should be renamed" — modules/platform/azure/util.spec.ts line 69
    #[test]
    fn branch_name_strips_refs_heads_prefix() {
        let result = get_branch_name_without_refs_heads_prefix("refs/heads/testBB");
        assert_eq!(result, Some("testBB"));
    }

    // Ported: "should log error and return undefined" — modules/platform/azure/util.spec.ts line 74
    #[test]
    fn branch_name_empty_returns_none() {
        let result = get_branch_name_without_refs_heads_prefix("");
        assert!(result.is_none());
    }

    // Ported: "should return the input" — modules/platform/azure/util.spec.ts line 79
    #[test]
    fn branch_name_without_prefix_returns_as_is() {
        let result = get_branch_name_without_refs_heads_prefix("testBB");
        assert_eq!(result, Some("testBB"));
    }

    // Ported: "should be the same" — modules/platform/azure/util.spec.ts line 105
    #[test]
    fn max4000_chars_short_string_unchanged() {
        assert_eq!(max4000_chars("Hello"), "Hello");
    }

    // Ported: "should be truncated" — modules/platform/azure/util.spec.ts line 110
    #[test]
    fn max4000_chars_long_string_truncated() {
        let s: String = "a".repeat(5000);
        assert_eq!(max4000_chars(&s).len(), 3999);
    }

    // Ported: "should return the object with same strings" — modules/platform/azure/util.spec.ts line 120
    #[test]
    fn get_project_and_repo_single_name() {
        let (project, repo) = get_project_and_repo("myRepoName").unwrap();
        assert_eq!(project, "myRepoName");
        assert_eq!(repo, "myRepoName");
    }

    // Ported: "should return the object with project and repo" — modules/platform/azure/util.spec.ts line 125
    #[test]
    fn get_project_and_repo_project_slash_repo() {
        let (project, repo) = get_project_and_repo("prjName/myRepoName").unwrap();
        assert_eq!(project, "prjName");
        assert_eq!(repo, "myRepoName");
    }

    // Ported: "should return an error" — modules/platform/azure/util.spec.ts line 130
    #[test]
    fn get_project_and_repo_too_many_segments() {
        let result = get_project_and_repo("prjName/myRepoName/blalba");
        assert!(result.is_err());
    }

    // Ported: "returns null when repos array is empty" — modules/platform/azure/util.spec.ts line 151
    #[test]
    fn get_repo_by_name_empty_list_returns_none() {
        assert!(get_repo_by_name("foo/bar", &[]).is_none());
    }

    // Ported: "returns null when repo is not found" — modules/platform/azure/util.spec.ts line 157
    #[test]
    fn get_repo_by_name_not_found_returns_none() {
        let repos = vec![AzureGitRepo {
            id: None,
            name: "bar".to_owned(),
            project_name: Some("bar".to_owned()),
        }];
        assert!(get_repo_by_name("foo/foo", &repos).is_none());
    }

    // Ported: "finds repo" — modules/platform/azure/util.spec.ts line 163
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

    // Ported: "supports shorthand names" — modules/platform/azure/util.spec.ts line 181
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

    // Ported: "is case-independent" — modules/platform/azure/util.spec.ts line 189
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
}
