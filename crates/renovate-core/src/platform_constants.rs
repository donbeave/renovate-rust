//! Platform host-type groupings.
//!
//! Renovate reference: `lib/constants/platforms.ts`.

pub const PLATFORM_HOST_TYPES: &[&str] = &[
    "azure",
    "bitbucket",
    "bitbucket-server",
    "codecommit",
    "forgejo",
    "gerrit",
    "gitea",
    "github",
    "gitlab",
    "local",
    "scm-manager",
];

pub const AZURE_API_USING_HOST_TYPES: &[&str] = &["azure", "azure-tags"];

pub const GITEA_API_USING_HOST_TYPES: &[&str] =
    &["gitea", "gitea-changelog", "gitea-releases", "gitea-tags"];

pub const FORGEJO_API_USING_HOST_TYPES: &[&str] = &[
    "forgejo",
    "forgejo-changelog",
    "forgejo-releases",
    "forgejo-tags",
];

pub const GITHUB_API_USING_HOST_TYPES: &[&str] = &[
    "github",
    "github-releases",
    "github-release-attachments",
    "github-tags",
    "pod",
    "hermit",
    "github-changelog",
    "conan",
];

pub const GITLAB_API_USING_HOST_TYPES: &[&str] = &[
    "gitlab",
    "gitlab-releases",
    "gitlab-tags",
    "gitlab-packages",
    "gitlab-changelog",
    "pypi",
];

pub const BITBUCKET_API_USING_HOST_TYPES: &[&str] =
    &["bitbucket", "bitbucket-changelog", "bitbucket-tags"];

pub const BITBUCKET_SERVER_API_USING_HOST_TYPES: &[&str] = &[
    "bitbucket-server",
    "bitbucket-server-changelog",
    "bitbucket-server-tags",
];

#[cfg(test)]
mod tests {
    use super::{
        BITBUCKET_API_USING_HOST_TYPES, BITBUCKET_SERVER_API_USING_HOST_TYPES,
        FORGEJO_API_USING_HOST_TYPES, GITEA_API_USING_HOST_TYPES, GITHUB_API_USING_HOST_TYPES,
        GITLAB_API_USING_HOST_TYPES,
    };

    // Ported: "should be part of the GITEA_API_USING_HOST_TYPES" — constants/platform.spec.ts line 23
    #[test]
    fn gitea_api_using_host_types_include_gitea_tags_and_platform() {
        assert!(GITEA_API_USING_HOST_TYPES.contains(&"gitea-tags"));
        assert!(GITEA_API_USING_HOST_TYPES.contains(&"gitea"));
    }

    // Ported: "should be part of the FORGEJO_API_USING_HOST_TYPES" — constants/platform.spec.ts line 30
    #[test]
    fn forgejo_api_using_host_types_include_expected_host_types() {
        assert!(FORGEJO_API_USING_HOST_TYPES.contains(&"forgejo"));
        assert!(FORGEJO_API_USING_HOST_TYPES.contains(&"forgejo-tags"));
        assert!(FORGEJO_API_USING_HOST_TYPES.contains(&"forgejo-releases"));
        assert!(FORGEJO_API_USING_HOST_TYPES.contains(&"forgejo-changelog"));
        assert_eq!(FORGEJO_API_USING_HOST_TYPES.len(), 4);
    }

    // Ported: "should be part of the GITLAB_API_USING_HOST_TYPES" — constants/platform.spec.ts line 42
    #[test]
    fn gitlab_api_using_host_types_include_expected_datasources_and_platform() {
        assert!(GITLAB_API_USING_HOST_TYPES.contains(&"gitlab-tags"));
        assert!(GITLAB_API_USING_HOST_TYPES.contains(&"gitlab-releases"));
        assert!(GITLAB_API_USING_HOST_TYPES.contains(&"gitlab-packages"));
        assert!(GITLAB_API_USING_HOST_TYPES.contains(&"gitlab-changelog"));
        assert!(GITLAB_API_USING_HOST_TYPES.contains(&"gitlab"));
    }

    // Ported: "should be not part of the GITLAB_API_USING_HOST_TYPES" — constants/platform.spec.ts line 58
    #[test]
    fn gitlab_api_using_host_types_do_not_include_github() {
        assert!(!GITLAB_API_USING_HOST_TYPES.contains(&"github"));
    }

    // Ported: "should be part of the GITHUB_API_USING_HOST_TYPES" — constants/platform.spec.ts line 62
    #[test]
    fn github_api_using_host_types_include_expected_datasources_and_platform() {
        assert!(GITHUB_API_USING_HOST_TYPES.contains(&"github-tags"));
        assert!(GITHUB_API_USING_HOST_TYPES.contains(&"github-releases"));
        assert!(GITHUB_API_USING_HOST_TYPES.contains(&"pod"));
        assert!(GITHUB_API_USING_HOST_TYPES.contains(&"hermit"));
        assert!(GITHUB_API_USING_HOST_TYPES.contains(&"github-changelog"));
        assert!(GITHUB_API_USING_HOST_TYPES.contains(&"github"));
    }

    // Ported: "should be not part of the GITHUB_API_USING_HOST_TYPES" — constants/platform.spec.ts line 79
    #[test]
    fn github_api_using_host_types_do_not_include_gitlab() {
        assert!(!GITHUB_API_USING_HOST_TYPES.contains(&"gitlab"));
    }

    // Ported: "should be part of the BITBUCKET_API_USING_HOST_TYPES" — constants/platform.spec.ts line 83
    #[test]
    fn bitbucket_api_using_host_types_include_bitbucket_tags_and_platform() {
        assert!(BITBUCKET_API_USING_HOST_TYPES.contains(&"bitbucket-tags"));
        assert!(BITBUCKET_API_USING_HOST_TYPES.contains(&"bitbucket"));
    }

    // Ported: "should be part of the BITBUCKET_SERVER_API_USING_HOST_TYPES" — constants/platform.spec.ts line 90
    #[test]
    fn bitbucket_server_api_using_host_types_include_server_tags_and_platform() {
        assert!(BITBUCKET_SERVER_API_USING_HOST_TYPES.contains(&"bitbucket-server-tags"));
        assert!(BITBUCKET_SERVER_API_USING_HOST_TYPES.contains(&"bitbucket-server"));
    }
}
