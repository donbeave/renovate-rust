//! Platform type.
//!
//! Source: `PLATFORM_HOST_TYPES` in `lib/constants/platforms.ts`.

/// Identifies the type of source-code platform hosting the target repositories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    Azure,
    Bitbucket,
    BitbucketServer,
    Codecommit,
    Forgejo,
    Gerrit,
    Gitea,
    Github,
    Gitlab,
    Local,
    ScmManager,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Azure => "azure",
            Self::Bitbucket => "bitbucket",
            Self::BitbucketServer => "bitbucket-server",
            Self::Codecommit => "codecommit",
            Self::Forgejo => "forgejo",
            Self::Gerrit => "gerrit",
            Self::Gitea => "gitea",
            Self::Github => "github",
            Self::Gitlab => "gitlab",
            Self::Local => "local",
            Self::ScmManager => "scm-manager",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_display_github() {
        assert_eq!(Platform::Github.to_string(), "github");
    }

    #[test]
    fn platform_display_gitlab() {
        assert_eq!(Platform::Gitlab.to_string(), "gitlab");
    }

    #[test]
    fn platform_display_bitbucket_server() {
        assert_eq!(Platform::BitbucketServer.to_string(), "bitbucket-server");
    }

    #[test]
    fn platform_display_all_variants() {
        assert_eq!(Platform::Azure.to_string(), "azure");
        assert_eq!(Platform::Bitbucket.to_string(), "bitbucket");
        assert_eq!(Platform::Codecommit.to_string(), "codecommit");
        assert_eq!(Platform::Forgejo.to_string(), "forgejo");
        assert_eq!(Platform::Gerrit.to_string(), "gerrit");
        assert_eq!(Platform::Gitea.to_string(), "gitea");
        assert_eq!(Platform::Local.to_string(), "local");
        assert_eq!(Platform::ScmManager.to_string(), "scm-manager");
    }
}
