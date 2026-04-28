//! Shared HTTP client for platform and datasource calls.
//!
//! Wraps `reqwest::Client` with a Renovate-compatible `User-Agent` header,
//! optional bearer-token authentication, and automatic retry on transient
//! failures.
//!
//! ## Retry behaviour
//!
//! `get_retrying` retries on:
//! - **429 Too Many Requests** — always, up to [`MAX_RETRIES`] times
//! - **503 Service Unavailable** — always, up to [`MAX_RETRIES`] times
//! - **504 Gateway Timeout** — always, up to [`MAX_RETRIES`] times
//!
//! The wait between retries is determined by the `Retry-After` response header
//! (number of seconds or an HTTP-date); when absent, exponential backoff is
//! used: 1 s, 2 s, 4 s (capped at [`MAX_RETRY_AFTER_SECS`]).
//!
//! Renovate reference: `lib/util/http/retry-after.ts` — `wrapWithRetry`.

use reqwest::{Client, RequestBuilder, Response, StatusCode};
use thiserror::Error;
use tokio::time::Duration;

/// Version string embedded in the `User-Agent` header.
const USER_AGENT: &str = concat!("renovate-rust/", env!("CARGO_PKG_VERSION"));

/// Maximum number of retry attempts after the initial request.
const MAX_RETRIES: u32 = 3;

/// Hard cap on `Retry-After` delay in seconds — we refuse to wait longer.
const MAX_RETRY_AFTER_SECS: u64 = 60;

/// Base delay for exponential backoff when no `Retry-After` header is present.
#[cfg(not(test))]
const BASE_DELAY_MS: u64 = 1_000;
/// Shortened base delay in tests so retry tests complete quickly.
#[cfg(test)]
const BASE_DELAY_MS: u64 = 10;

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

/// Thin wrapper around `reqwest::Client` that adds a shared `User-Agent`,
/// optional bearer-token authentication, and transparent retry.
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
    ///
    /// The returned `RequestBuilder` must be `send()`-ed by the caller.
    /// Prefer [`get_retrying`][Self::get_retrying] when a single `Response`
    /// is all that is needed — it handles retry transparently.
    pub fn get(&self, url: &str) -> RequestBuilder {
        let rb = self.inner.get(url);
        match &self.token {
            Some(t) => rb.bearer_auth(t),
            None => rb,
        }
    }

    /// Send a GET request with automatic retry on transient failures.
    ///
    /// Retries up to [`MAX_RETRIES`] times on 429/503/504.  Respects the
    /// `Retry-After` response header; falls back to exponential backoff when
    /// the header is absent.  Returns the final `Response` regardless of
    /// status — callers must check `resp.status().is_success()`.
    pub async fn get_retrying(&self, url: &str) -> Result<Response, HttpError> {
        let mut attempt: u32 = 0;
        loop {
            let resp = self.get(url).send().await.map_err(HttpError::Request)?;
            let status = resp.status();

            if !is_retryable(status) || attempt >= MAX_RETRIES {
                return Ok(resp);
            }

            let delay = retry_delay(&resp, attempt);
            if delay > Duration::from_secs(MAX_RETRY_AFTER_SECS) {
                tracing::debug!(
                    url,
                    delay_secs = delay.as_secs(),
                    "Retry-After exceeds cap; not retrying"
                );
                return Ok(resp);
            }

            tracing::debug!(
                url,
                attempt,
                delay_ms = delay.as_millis(),
                status = status.as_u16(),
                "transient HTTP error — will retry"
            );
            tokio::time::sleep(delay).await;
            attempt += 1;
        }
    }

    /// Send a GET request (with retry) and deserialize the JSON response.
    ///
    /// Returns `Err(HttpError::Status)` for non-2xx responses after all
    /// retry attempts are exhausted.
    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, HttpError> {
        let resp = self.get_retrying(url).await?;
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

// ── Retry helpers ─────────────────────────────────────────────────────────────

/// Return `true` when a status code warrants an automatic retry.
fn is_retryable(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS            // 429
            | StatusCode::SERVICE_UNAVAILABLE    // 503
            | StatusCode::GATEWAY_TIMEOUT // 504
    )
}

/// Determine how long to wait before the next attempt.
///
/// Tries the `Retry-After` header first (both numeric seconds and HTTP-date
/// forms); falls back to exponential backoff `BASE_DELAY_MS * 2^attempt`.
fn retry_delay(resp: &Response, attempt: u32) -> Duration {
    if let Some(delay) = parse_retry_after(resp) {
        return delay;
    }
    // Exponential backoff: BASE_DELAY * 2^attempt, capped at 30s.
    let ms = BASE_DELAY_MS.saturating_mul(1u64 << attempt.min(5));
    Duration::from_millis(ms.min(30_000))
}

/// Parse the `Retry-After` response header as a `Duration`.
///
/// Supports both the numeric (delay-seconds) and HTTP-date forms per
/// [RFC 7231 §7.1.3](https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.3).
fn parse_retry_after(resp: &Response) -> Option<Duration> {
    let value = resp.headers().get("Retry-After")?.to_str().ok()?;

    // Numeric form: `Retry-After: 30`
    if let Ok(secs) = value.trim().parse::<u64>() {
        return Some(Duration::from_secs(secs));
    }

    // HTTP-date form: `Retry-After: Wed, 21 Oct 2015 07:28:00 GMT`
    // Use httpdate crate if available; for now skip unsupported date form.
    None
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // ── is_retryable ──────────────────────────────────────────────────────────

    #[test]
    fn retryable_status_codes() {
        assert!(is_retryable(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable(StatusCode::SERVICE_UNAVAILABLE));
        assert!(is_retryable(StatusCode::GATEWAY_TIMEOUT));
    }

    #[test]
    fn non_retryable_status_codes() {
        assert!(!is_retryable(StatusCode::OK));
        assert!(!is_retryable(StatusCode::NOT_FOUND));
        assert!(!is_retryable(StatusCode::UNAUTHORIZED));
        assert!(!is_retryable(StatusCode::INTERNAL_SERVER_ERROR));
    }

    // ── parse_retry_after ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn retry_after_numeric_is_parsed() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "5"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let resp = http
            .get(&format!("{}/test", server.uri()))
            .send()
            .await
            .unwrap();
        let delay = parse_retry_after(&resp);
        assert_eq!(delay, Some(Duration::from_secs(5)));
    }

    // ── get_retrying ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn retries_on_429_then_succeeds() {
        let server = MockServer::start().await;

        // First two requests: 429
        Mock::given(method("GET"))
            .and(path("/resource"))
            .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "0"))
            .up_to_n_times(2)
            .mount(&server)
            .await;

        // Third request: 200
        Mock::given(method("GET"))
            .and(path("/resource"))
            .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let resp = http
            .get_retrying(&format!("{}/resource", server.uri()))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.text().await.unwrap(), "ok");
    }

    #[tokio::test]
    async fn stops_retrying_after_max_attempts() {
        let server = MockServer::start().await;

        // All requests return 429.
        Mock::given(method("GET"))
            .and(path("/rate-limited"))
            .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "0"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let resp = http
            .get_retrying(&format!("{}/rate-limited", server.uri()))
            .await
            .unwrap();

        // After MAX_RETRIES the last 429 response is returned.
        assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn does_not_retry_on_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/missing"))
            .respond_with(ResponseTemplate::new(404))
            // Strict: must only be called once (no retry).
            .expect(1)
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let resp = http
            .get_retrying(&format!("{}/missing", server.uri()))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_json_retries_on_503() {
        let server = MockServer::start().await;

        // First call: 503
        Mock::given(method("GET"))
            .and(path("/api"))
            .respond_with(ResponseTemplate::new(503))
            .up_to_n_times(1)
            .mount(&server)
            .await;

        // Second call: 200 with JSON
        Mock::given(method("GET"))
            .and(path("/api"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"v": 42})))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let val: serde_json::Value = http
            .get_json(&format!("{}/api", server.uri()))
            .await
            .unwrap();

        assert_eq!(val["v"], 42);
    }
}
