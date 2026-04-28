//! Platform client trait and implementations.
//!
//! Each supported platform (GitHub, GitLab, Bitbucket, …) implements
//! [`PlatformClient`]. The trait is intentionally minimal; methods are added
//! as parity slices land.
//!
//! ## Object safety and dispatch
//!
//! `PlatformClient` uses `-> impl Future` RPIT which is not object-safe, so
//! `Box<dyn PlatformClient>` does not compile. Use [`AnyPlatformClient`] for
//! runtime dispatch — it is an enum that delegates to each concrete client
//! with no heap allocation and full static dispatch inside each arm.

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

/// Common interface for all platform integrations.
///
/// All methods are async. Use [`AnyPlatformClient`] when you need runtime
/// dispatch across platforms.
pub trait PlatformClient: Send + Sync {
    /// Verify authentication and return the currently-authenticated user.
    ///
    /// Used at startup to confirm the token is valid before any repository
    /// work. Maps to Renovate's `initPlatform()` token check.
    fn get_current_user(
        &self,
    ) -> impl std::future::Future<Output = Result<CurrentUser, PlatformError>> + Send;
}

/// Enum dispatch wrapper covering all supported platform clients.
///
/// Constructed via [`AnyPlatformClient::create`] from a [`GlobalConfig`].
/// Prefer this over `Box<dyn PlatformClient>` — it avoids heap allocation
/// and keeps full monomorphization inside each match arm.
#[derive(Debug, Clone)]
pub enum AnyPlatformClient {
    Github(GithubClient),
}

impl AnyPlatformClient {
    /// Build the right platform client from a resolved [`GlobalConfig`].
    ///
    /// Returns `Err(PlatformError::NotSupported)` for platforms that do not
    /// have a Rust implementation yet.
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
}
