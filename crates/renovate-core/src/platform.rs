//! Platform client trait and implementations.
//!
//! Each supported platform (GitHub, GitLab, Bitbucket, …) implements
//! [`PlatformClient`]. The trait grows as parity slices land.
//!
//! ## Object safety and dispatch
//!
//! `PlatformClient` uses `-> impl Future` RPIT which is not object-safe, so
//! `Box<dyn PlatformClient>` does not compile. Use [`AnyPlatformClient`] for
//! runtime dispatch — it delegates to each concrete client with zero
//! allocation overhead.

pub mod github;

use thiserror::Error;

use crate::config::{GlobalConfig, Platform};
use crate::http::HttpError;
use github::GithubClient;

/// Errors from platform operations.
#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),

    #[error("Authentication failed — check your token")]
    Unauthorized,

    /// The platform is not yet implemented in this slice.
    #[error("Platform not supported yet: {0}")]
    NotSupported(String),

    #[error("Unexpected response: {0}")]
    Unexpected(String),
}

/// Minimal authenticated user info returned by the platform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentUser {
    /// Login / username on the platform.
    pub login: String,
}

/// A raw (text) file fetched from the platform.
#[derive(Debug, Clone)]
pub struct RawFile {
    /// Path within the repository.
    pub path: String,
    /// Decoded UTF-8 content.
    pub content: String,
}

/// Common interface for all platform integrations.
///
/// All methods are async. Use [`AnyPlatformClient`] when you need runtime
/// dispatch across platforms.
pub trait PlatformClient: Send + Sync {
    /// Verify authentication and return the currently-authenticated user.
    fn get_current_user(
        &self,
    ) -> impl std::future::Future<Output = Result<CurrentUser, PlatformError>> + Send;

    /// Fetch a single file from the repository at the default branch.
    ///
    /// Returns `Ok(None)` when the file does not exist (404), and `Err` for
    /// other failures. This is a simplified implementation that uses the
    /// platform's REST API; a later slice will replace it with git-based
    /// reading after a repo clone.
    fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Option<RawFile>, PlatformError>> + Send;
}

/// Enum dispatch wrapper covering all supported platform clients.
///
/// Constructed via [`AnyPlatformClient::create`] from a [`GlobalConfig`].
#[derive(Debug, Clone)]
pub enum AnyPlatformClient {
    Github(GithubClient),
}

impl AnyPlatformClient {
    /// Build the right platform client from a resolved [`GlobalConfig`].
    pub fn create(config: &GlobalConfig) -> Result<Self, PlatformError> {
        match config.platform {
            Platform::Github => {
                let token = config.token.as_deref().unwrap_or_default();
                let client = match config.endpoint.as_deref() {
                    Some(ep) => GithubClient::with_endpoint(token, ep),
                    None => GithubClient::new(token),
                }
                .map_err(PlatformError::Http)?;
                Ok(Self::Github(client))
            }
            other => Err(PlatformError::NotSupported(other.to_string())),
        }
    }

    /// Verify authentication and return the currently-authenticated user.
    pub async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        match self {
            Self::Github(c) => c.get_current_user().await,
        }
    }

    /// Fetch a single file from the repository.
    pub async fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        match self {
            Self::Github(c) => c.get_raw_file(owner, repo, path).await,
        }
    }
}
