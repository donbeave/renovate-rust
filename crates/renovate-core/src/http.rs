//! Shared HTTP client for platform and datasource calls.
//!
//! Wraps `reqwest::Client` with a Renovate-compatible `User-Agent` header and
//! optional bearer-token authentication. All platform and datasource modules
//! should obtain an `HttpClient` from the session context rather than
//! constructing their own `reqwest::Client`.

use reqwest::{Client, RequestBuilder, StatusCode};
use thiserror::Error;

/// Version string embedded in the `User-Agent` header.
const USER_AGENT: &str = concat!("renovate-rust/", env!("CARGO_PKG_VERSION"));

/// Errors from HTTP operations.
#[derive(Debug, Error)]
pub enum HttpError {
    /// The request could not be sent (network error, TLS error, etc.).
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The server returned an unexpected status code.
    #[error("HTTP {status}: {url}")]
    Status { status: StatusCode, url: String },
}

/// Thin wrapper around `reqwest::Client` that adds a shared `User-Agent`
/// and optional bearer-token authentication.
///
/// `HttpClient` is cheap to clone — the underlying `reqwest::Client` uses
/// an `Arc` internally and shares the connection pool.
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: Client,
    token: Option<String>,
}

impl HttpClient {
    /// Create a new client with no authentication token.
    pub fn new() -> Result<Self, HttpError> {
        let inner = Client::builder().user_agent(USER_AGENT).build()?;
        Ok(Self { inner, token: None })
    }

    /// Create a new client with a bearer-token credential.
    pub fn with_token(token: impl Into<String>) -> Result<Self, HttpError> {
        let inner = Client::builder().user_agent(USER_AGENT).build()?;
        Ok(Self {
            inner,
            token: Some(token.into()),
        })
    }

    /// Start a GET request, injecting auth if a token is set.
    pub fn get(&self, url: &str) -> RequestBuilder {
        let rb = self.inner.get(url);
        match &self.token {
            Some(t) => rb.bearer_auth(t),
            None => rb,
        }
    }

    /// Send a GET request and deserialize the JSON response.
    ///
    /// Returns `Err(HttpError::Status)` for non-2xx responses.
    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, HttpError> {
        let resp = self.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(HttpError::Status {
                status: resp.status(),
                url: url.to_owned(),
            });
        }
        let body = resp.json::<T>().await?;
        Ok(body)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("failed to build default HTTP client")
    }
}
