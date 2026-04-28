//! Platform client trait and implementations.
//!
//! Each supported platform (GitHub, GitLab, Bitbucket, …) implements
//! [`PlatformClient`]. The trait is intentionally minimal in this initial
//! slice; methods will be added as parity slices land.

pub mod github;

use thiserror::Error;

/// Errors from platform operations.
#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),

    #[error("Authentication failed — check your token")]
    Unauthorized,

    #[error("Unexpected response: {0}")]
    Unexpected(String),
}

/// Minimal authenticated user info returned by the platform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentUser {
    /// Login / username on the platform.
    pub login: String,
}

/// Common interface for all platform integrations.
///
/// All methods are async. Implementations construct the concrete HTTP client
/// and endpoint from the `GlobalConfig` at instantiation time.
pub trait PlatformClient: Send + Sync {
    /// Verify authentication and return the currently-authenticated user.
    ///
    /// Used at startup to confirm the token is valid before doing any
    /// repository work. Maps to Renovate's `initPlatform()` token check.
    fn get_current_user(
        &self,
    ) -> impl std::future::Future<Output = Result<CurrentUser, PlatformError>> + Send;
}
