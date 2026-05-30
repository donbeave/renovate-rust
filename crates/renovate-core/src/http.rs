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

use chrono::{DateTime, Utc};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde_json::Value;
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

    /// Send a GET request with a custom `Accept` header and return the body as text.
    pub async fn get_raw_with_accept(&self, url: &str, accept: &str) -> Result<String, HttpError> {
        let rb = self.inner.get(url).header("Accept", accept);
        let rb = match &self.token {
            Some(t) => rb.bearer_auth(t),
            None => rb,
        };
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if !resp.status().is_success() {
            return Err(HttpError::Status {
                status: resp.status(),
                url: url.to_owned(),
            });
        }
        let text = resp.text().await.map_err(HttpError::Request)?;
        Ok(text)
    }

    /// Send a GET request with a custom `Accept` header, an additional header, and return the body as text.
    pub async fn get_raw_with_accept_and_header(
        &self,
        url: &str,
        accept: &str,
        header_name: &str,
        header_value: &str,
    ) -> Result<String, HttpError> {
        let rb = self
            .inner
            .get(url)
            .header("Accept", accept)
            .header(header_name, header_value);
        let rb = match &self.token {
            Some(t) => rb.bearer_auth(t),
            None => rb,
        };
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if !resp.status().is_success() {
            return Err(HttpError::Status {
                status: resp.status(),
                url: url.to_owned(),
            });
        }
        let text = resp.text().await.map_err(HttpError::Request)?;
        Ok(text)
    }

    /// Send a POST request with a JSON body and deserialize the JSON response.
    pub async fn post_json<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        body: &str,
    ) -> Result<T, HttpError> {
        let rb = self
            .inner
            .post(url)
            .header("Content-Type", "application/json");
        let rb = match &self.token {
            Some(t) => rb.bearer_auth(t),
            None => rb,
        };
        let resp = rb
            .body(body.to_owned())
            .send()
            .await
            .map_err(HttpError::Request)?;
        if !resp.status().is_success() {
            return Err(HttpError::Status {
                status: resp.status(),
                url: url.to_owned(),
            });
        }
        let result = resp.json::<T>().await?;
        Ok(result)
    }

    /// Send a PATCH request with a JSON body.
    ///
    /// Returns the raw response; callers must check the status code.
    pub async fn patch_json(&self, url: &str, body: &str) -> Result<reqwest::Response, HttpError> {
        let rb = self
            .inner
            .patch(url)
            .header("Content-Type", "application/json");
        let rb = match &self.token {
            Some(t) => rb.bearer_auth(t),
            None => rb,
        };
        let resp = rb
            .body(body.to_owned())
            .send()
            .await
            .map_err(HttpError::Request)?;
        Ok(resp)
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

/// Parse a `Retry-After` header value string into seconds from `now`.
///
/// Supports both the numeric (delay-seconds) and HTTP-date forms per
/// [RFC 7231 §7.1.3](https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.3).
///
/// Returns `None` for missing/invalid values or dates in the past.
pub fn parse_retry_after_value(value: &str, now: DateTime<Utc>) -> Option<u64> {
    let value = value.trim();

    // Numeric form: `Retry-After: 30`
    if let Ok(secs) = value.parse::<u64>() {
        return Some(secs);
    }

    // HTTP-date form: `Retry-After: Wed, 01 Jan 2020 00:00:42 GMT`
    // Strip the " GMT" suffix (always UTC for HTTP dates) and parse as NaiveDateTime.
    let http_date_str = value.strip_suffix(" GMT").unwrap_or(value);
    if let Ok(naive_dt) =
        chrono::NaiveDateTime::parse_from_str(http_date_str, "%a, %d %b %Y %H:%M:%S")
    {
        let date_utc = naive_dt.and_utc();
        let diff = date_utc.signed_duration_since(now).num_seconds();
        if diff >= 0 {
            return Some(diff as u64);
        }
        return None;
    }

    None
}

/// Parse the `Retry-After` response header as a `Duration`.
///
/// Supports both the numeric (delay-seconds) and HTTP-date forms per
/// [RFC 7231 §7.1.3](https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.3).
fn parse_retry_after(resp: &Response) -> Option<Duration> {
    let value = resp.headers().get("Retry-After")?.to_str().ok()?;
    let now = Utc::now();
    parse_retry_after_value(value, now).map(Duration::from_secs)
}

// ── cleanup_http_cache (mirrors lib/util/cache/repository/http-cache.ts) ──────

/// Default HTTP cache TTL in days.
const DEFAULT_HTTP_CACHE_TTL_DAYS: i64 = 90;

/// Remove expired entries from the `httpCache` section of a repository cache.
///
/// - When `ttl_days` is `Some(0)`, removes the entire `httpCache` key.
/// - When `ttl_days` is `None`, uses the default TTL of 90 days.
/// - Entries whose `timestamp` field is older than the TTL are removed.
/// - If `cache` is not an object or has no `httpCache` object, returns without change.
///
/// Mirrors `cleanupHttpCache` from `lib/util/cache/repository/http-cache.ts`.
pub fn cleanup_http_cache(cache: &mut Value, ttl_days: Option<u32>) {
    let Value::Object(cache_map) = cache else {
        return;
    };

    let effective_ttl = ttl_days
        .map(|d| d as i64)
        .unwrap_or(DEFAULT_HTTP_CACHE_TTL_DAYS);
    if effective_ttl == 0 {
        cache_map.remove("httpCache");
        return;
    }

    let Some(Value::Object(http_cache)) = cache_map.get_mut("httpCache") else {
        return;
    };

    let cutoff = Utc::now() - chrono::Duration::days(effective_ttl);
    http_cache.retain(|_, entry| {
        let Value::Object(entry_map) = entry else {
            return false;
        };
        let Some(Value::String(ts)) = entry_map.get("timestamp") else {
            return false;
        };
        DateTime::parse_from_rfc3339(ts)
            .map(|t| t.with_timezone(&Utc) >= cutoff)
            .unwrap_or(false)
    });
}

// ── apply_authorization (mirrors lib/util/http/auth.ts) ───────────────────────

const GITHUB_API_HOST_TYPES: &[&str] = &[
    "github",
    "github-releases",
    "github-release-attachments",
    "github-tags",
    "pod",
    "hermit",
    "github-changelog",
    "conan",
];

const GITEA_API_HOST_TYPES: &[&str] = &["gitea", "gitea-changelog", "gitea-releases", "gitea-tags"];

const FORGEJO_API_HOST_TYPES: &[&str] = &[
    "forgejo",
    "forgejo-changelog",
    "forgejo-releases",
    "forgejo-tags",
];

const GITLAB_API_HOST_TYPES: &[&str] = &[
    "gitlab",
    "gitlab-releases",
    "gitlab-tags",
    "gitlab-packages",
    "gitlab-changelog",
    "pypi",
];

/// Result of `apply_authorization`: which headers to set.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct AppliedAuth {
    /// Value for the `Authorization` header, if any.
    pub authorization: Option<String>,
    /// Value for the `Private-token` header (GitLab PAT), if any.
    pub private_token: Option<String>,
}

/// Input to `apply_authorization`.
#[derive(Debug, Default)]
pub struct AuthOptions<'a> {
    /// Platform or datasource host type (e.g. "github", "gitlab").
    pub host_type: Option<&'a str>,
    /// Bearer/platform token.
    pub token: Option<&'a str>,
    /// Basic-auth username.
    pub username: Option<&'a str>,
    /// Basic-auth password.
    pub password: Option<&'a str>,
    /// `context.authType` from the request options.
    pub auth_type: Option<&'a str>,
    /// Existing `Authorization` header value.
    pub existing_auth: Option<&'a str>,
    /// If true, skip auth injection.
    pub no_auth: bool,
}

/// Compute the auth header(s) to apply for a request.
///
/// Mirrors `applyAuthorization` from `lib/util/http/auth.ts`.
pub fn apply_authorization(opts: &AuthOptions<'_>) -> AppliedAuth {
    // If there is already an Authorization header or noAuth=true, do nothing.
    if opts.existing_auth.map(|a| !a.is_empty()).unwrap_or(false) || opts.no_auth {
        return AppliedAuth::default();
    }

    if let Some(token) = opts.token {
        // If auth_type is set in context, use it directly.
        if let Some(auth_type) = opts.auth_type {
            let authorization = if auth_type == "Token-Only" {
                token.to_owned()
            } else {
                format!("{auth_type} {token}")
            };
            return AppliedAuth {
                authorization: Some(authorization),
                ..Default::default()
            };
        }

        // GitHub App installation token (x-access-token: prefix).
        if let Some(app_token) = token.strip_prefix("x-access-token:") {
            return AppliedAuth {
                authorization: Some(format!("Bearer {app_token}")),
                ..Default::default()
            };
        }

        // Forgejo → Bearer.
        if opts
            .host_type
            .map(|h| FORGEJO_API_HOST_TYPES.contains(&h))
            .unwrap_or(false)
        {
            return AppliedAuth {
                authorization: Some(format!("Bearer {token}")),
                ..Default::default()
            };
        }

        // Gitea → Bearer.
        if opts
            .host_type
            .map(|h| GITEA_API_HOST_TYPES.contains(&h))
            .unwrap_or(false)
        {
            return AppliedAuth {
                authorization: Some(format!("Bearer {token}")),
                ..Default::default()
            };
        }

        // GitHub → token prefix.
        if opts
            .host_type
            .map(|h| GITHUB_API_HOST_TYPES.contains(&h))
            .unwrap_or(false)
        {
            return AppliedAuth {
                authorization: Some(format!("token {token}")),
                ..Default::default()
            };
        }

        // GitLab → Personal Access Token (20 chars) or Bearer.
        if opts
            .host_type
            .map(|h| GITLAB_API_HOST_TYPES.contains(&h))
            .unwrap_or(false)
        {
            if token.len() == 20 {
                return AppliedAuth {
                    private_token: Some(token.to_owned()),
                    ..Default::default()
                };
            } else {
                return AppliedAuth {
                    authorization: Some(format!("Bearer {token}")),
                    ..Default::default()
                };
            }
        }

        // Default: Bearer.
        return AppliedAuth {
            authorization: Some(format!("Bearer {token}")),
            ..Default::default()
        };
    }

    // Basic auth from password.
    if let Some(password) = opts.password {
        use base64::Engine as _;
        let user = opts.username.unwrap_or("");
        let credentials =
            base64::engine::general_purpose::STANDARD.encode(format!("{user}:{password}"));
        return AppliedAuth {
            authorization: Some(format!("Basic {credentials}")),
            ..Default::default()
        };
    }

    AppliedAuth::default()
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

    // Ported: "works" — util/http/retry-after.spec.ts line 27
    // Ported: "retries" — util/http/retry-after.spec.ts line 44
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

    // Ported: "gives up after max retries" — util/http/retry-after.spec.ts line 59
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

    // Ported: "gives up when delay exceeds maxRetryAfter" — util/http/retry-after.spec.ts line 76
    #[tokio::test]
    async fn gives_up_when_retry_after_exceeds_cap() {
        let server = MockServer::start().await;

        // Retry-After: 61 exceeds MAX_RETRY_AFTER_SECS (60) — client must not retry.
        Mock::given(method("GET"))
            .and(path("/rate-limited-long"))
            .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "61"))
            .expect(1)
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let resp = http
            .get_retrying(&format!("{}/rate-limited-long", server.uri()))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    // Ported: "throws" — util/http/retry-after.spec.ts line 34
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

// ── WWW-Authenticate header parser ───────────────────────────────────────────

/// Parsed params for a WWW-Authenticate challenge.
#[derive(Debug, Clone, PartialEq)]
pub enum WwwAuthParams {
    /// Key=value pairs (bearer, digest, basic with quoted values).
    Map(std::collections::BTreeMap<String, String>),
    /// Single token argument (negotiate with GSSAPI token, etc.).
    Token(String),
}

/// A single parsed WWW-Authenticate challenge.
#[derive(Debug, Clone, PartialEq)]
pub struct WwwAuthChallenge {
    pub scheme: String,
    pub params: Option<WwwAuthParams>,
}

/// Check whether an HTTP response status is "OK".
///
/// Ports `isResponseOk` from `lib/util/http/hooks.ts`.
///
/// - `follow_redirect = true`: success range is 200–299 or 304
/// - `follow_redirect = false`: success range is 200–399 or 304
pub fn is_response_ok(status_code: u16, follow_redirect: bool) -> bool {
    let limit = if follow_redirect { 299 } else { 399 };
    (status_code >= 200 && status_code <= limit) || status_code == 304
}

/// Parse a `WWW-Authenticate` header (or slice of headers).
///
/// Mirrors `lib/util/http/www-authenticate.ts` `parse()`.
/// Returns `Err` if the input contains invalid tokens.
pub fn parse_www_authenticate(headers: &[&str]) -> Result<Vec<WwwAuthChallenge>, String> {
    let mut result = Vec::new();
    for &header in headers {
        parse_single_header(header, &mut result)?;
    }
    Ok(result)
}

fn parse_single_header(input: &str, out: &mut Vec<WwwAuthChallenge>) -> Result<(), String> {
    if input.is_empty() {
        return Ok(());
    }

    let tokens = tokenize(input)?;
    let tokens = group_pairs(tokens);
    let challenges = group_challenges(tokens);

    for c in challenges {
        let mut args: Vec<String> = Vec::new();
        let mut params: std::collections::BTreeMap<String, String> = Default::default();

        for t in c.tokens {
            match t {
                Token::Value(v) => args.push(v),
                Token::Pair(k, v) => {
                    params.insert(k, v);
                }
                Token::Comma => {}
                Token::Equals => {}
            }
        }

        let ch = if !args.is_empty() {
            WwwAuthChallenge {
                scheme: c.scheme.to_lowercase(),
                params: Some(WwwAuthParams::Token(args[0].clone())),
            }
        } else if !params.is_empty() {
            WwwAuthChallenge {
                scheme: c.scheme.to_lowercase(),
                params: Some(WwwAuthParams::Map(params)),
            }
        } else {
            WwwAuthChallenge {
                scheme: c.scheme.to_lowercase(),
                params: None,
            }
        };
        out.push(ch);
    }
    Ok(())
}

#[derive(Debug, Clone)]
enum Token {
    Value(String),
    Equals,
    Comma,
    Pair(String, String),
}

struct Challenge {
    scheme: String,
    tokens: Vec<Token>,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    use std::sync::LazyLock;
    static TOKEN_RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r#"^([a-zA-Z0-9!#$%&'*+.^_`|~-]+)"#).unwrap());
    static QUOTED_RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r#"^"((?:[^"\\]|\\\\|\\")*)""#).unwrap());
    static WS_RE: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"^\s+").unwrap());

    let mut result = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        if let Some(m) = WS_RE.find(remaining) {
            remaining = &remaining[m.end()..];
        } else if let Some(c) = QUOTED_RE.captures(remaining) {
            let v = c.get(1).unwrap().as_str().to_owned();
            remaining = &remaining[c.get(0).unwrap().end()..];
            result.push(Token::Value(v));
        } else if let Some(c) = TOKEN_RE.captures(remaining) {
            let v = c.get(1).unwrap().as_str().to_owned();
            remaining = &remaining[c.get(0).unwrap().end()..];
            result.push(Token::Value(v));
        } else if remaining.starts_with('=') {
            remaining = &remaining[1..];
            result.push(Token::Equals);
        } else if remaining.starts_with(',') {
            remaining = &remaining[1..];
            result.push(Token::Comma);
        } else {
            return Err("Failed to parse value".to_owned());
        }
    }
    Ok(result)
}

fn group_pairs(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut i = 0;
    while i + 2 < tokens.len() {
        if matches!(
            (&tokens[i], &tokens[i + 1], &tokens[i + 2]),
            (Token::Value(_), Token::Equals, Token::Value(_))
        ) {
            let Token::Value(val) = tokens.remove(i + 2) else {
                unreachable!()
            };
            tokens.remove(i + 1); // equals
            let Token::Value(key) = std::mem::replace(&mut tokens[i], Token::Comma) else {
                unreachable!()
            };
            tokens[i] = Token::Pair(key, val);
        } else {
            i += 1;
        }
    }
    tokens
}

fn group_challenges(mut tokens: Vec<Token>) -> Vec<Challenge> {
    let mut result = Vec::new();
    while !tokens.is_empty() {
        let Token::Value(scheme) = tokens.remove(0) else {
            break;
        };
        let mut j = 0;
        if tokens.is_empty() || matches!(tokens[0], Token::Comma) {
            // nothing
        } else if matches!(tokens[0], Token::Value(_)) {
            j = 1;
        } else {
            while j < tokens.len() && matches!(tokens[j], Token::Pair(_, _)) {
                j += 2;
            }
            j = j.saturating_sub(1);
        }
        let ch_tokens: Vec<Token> = tokens.drain(0..j).collect();
        if !tokens.is_empty() {
            tokens.remove(0);
        } // comma
        result.push(Challenge {
            scheme,
            tokens: ch_tokens,
        });
    }
    result
}

#[cfg(test)]
mod www_auth_tests {
    use super::*;

    fn bearer_params(realm: &str, service: &str, scope: &str) -> WwwAuthParams {
        let mut m = std::collections::BTreeMap::new();
        m.insert("realm".into(), realm.into());
        m.insert("scope".into(), scope.into());
        m.insert("service".into(), service.into());
        WwwAuthParams::Map(m)
    }

    // Ported: "parses: bearer" (it.each) — util/http/www-authenticate.spec.ts line 4
    #[test]
    fn www_auth_parses_bearer() {
        let parsed = parse_www_authenticate(&[
            "Bearer realm=\"https://renovate.com/v2/token\",service=\"container_registry\",scope=\"*\""
        ]).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].scheme, "bearer");
        assert_eq!(
            parsed[0].params,
            Some(bearer_params(
                "https://renovate.com/v2/token",
                "container_registry",
                "*"
            ))
        );
    }

    // Ported: "parses empty string" — util/http/www-authenticate.spec.ts line 135
    #[test]
    fn www_auth_parses_empty_string() {
        let parsed = parse_www_authenticate(&[""]).unwrap();
        assert!(parsed.is_empty());
    }

    // Ported: "throws on invalid input" — util/http/www-authenticate.spec.ts line 139
    #[test]
    fn www_auth_throws_on_invalid_input() {
        let result = parse_www_authenticate(&[
            "Bearer realm=\"https://renovate.com/v2/token\",service=\"container_registry\",scope=\"*",
        ]);
        assert!(result.is_err());
    }

    // ── apply_authorization ───────────────────────────────────────────────────

    // Ported: "does nothing" — util/http/auth.spec.ts line 3
    #[test]
    fn auth_does_nothing_with_existing_header() {
        let opts = AuthOptions {
            existing_auth: Some("token"),
            ..Default::default()
        };
        assert_eq!(apply_authorization(&opts), AppliedAuth::default());
    }

    // Ported: "gitea password" — util/http/auth.spec.ts line 15
    #[test]
    fn auth_gitea_password_basic() {
        let opts = AuthOptions {
            host_type: Some("gitea"),
            password: Some("XXXX"),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(result.authorization.as_deref(), Some("Basic OlhYWFg="));
    }

    // Ported: "gittea token" — util/http/auth.spec.ts line 28
    #[test]
    fn auth_gitea_token_bearer() {
        let opts = AuthOptions {
            host_type: Some("gitea"),
            token: Some("XXXX"),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(result.authorization.as_deref(), Some("Bearer XXXX"));
    }

    // Ported: "github token" — util/http/auth.spec.ts line 41
    #[test]
    fn auth_github_token_prefix() {
        let opts = AuthOptions {
            host_type: Some("github"),
            token: Some("XXX"),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(result.authorization.as_deref(), Some("token XXX"));
    }

    // Ported: "github token for datasource using github api" — util/http/auth.spec.ts line 56
    #[test]
    fn auth_github_releases_token_prefix() {
        let opts = AuthOptions {
            host_type: Some("github-releases"),
            token: Some("ZZZZ"),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(result.authorization.as_deref(), Some("token ZZZZ"));
    }

    // Ported: "github app token with hostType not in GITHUB_API_USING_HOST_TYPES" — util/http/auth.spec.ts line 71
    #[test]
    fn auth_github_app_token_bearer() {
        let opts = AuthOptions {
            host_type: Some("github-digest"),
            token: Some("x-access-token:ghs_123test"),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(result.authorization.as_deref(), Some("Bearer ghs_123test"));
    }

    // Ported: "gitlab personal access token" — util/http/auth.spec.ts line 85
    #[test]
    fn auth_gitlab_personal_access_token() {
        let opts = AuthOptions {
            host_type: Some("gitlab"),
            token: Some("0123456789012345test"), // 20 chars
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert!(result.authorization.is_none());
        assert_eq!(
            result.private_token.as_deref(),
            Some("0123456789012345test")
        );
    }

    // Ported: "gitlab oauth token" — util/http/auth.spec.ts line 101
    #[test]
    fn auth_gitlab_oauth_token_bearer() {
        let token = "a40bdd925a0c0b9c4cdd19d101c0df3b2bcd063ab7ad6706f03bcffcec01test";
        let opts = AuthOptions {
            host_type: Some("gitlab"),
            token: Some(token),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(
            result.authorization.as_deref(),
            Some(format!("Bearer {token}").as_str())
        );
    }

    // Ported: "npm basic token" — util/http/auth.spec.ts line 117
    #[test]
    fn auth_npm_basic_auth_type() {
        let opts = AuthOptions {
            host_type: Some("npm"),
            token: Some("test"),
            auth_type: Some("Basic"),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(result.authorization.as_deref(), Some("Basic test"));
    }

    // Ported: "bare token" — util/http/auth.spec.ts line 132
    #[test]
    fn auth_token_only_auth_type() {
        let opts = AuthOptions {
            token: Some("test"),
            auth_type: Some("Token-Only"),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(result.authorization.as_deref(), Some("test"));
    }

    // Ported: "honors authType" — util/http/auth.spec.ts line 146
    #[test]
    fn auth_honors_auth_type() {
        let opts = AuthOptions {
            host_type: Some("custom"),
            token: Some("test"),
            auth_type: Some("Bearer"),
            ..Default::default()
        };
        let result = apply_authorization(&opts);
        assert_eq!(result.authorization.as_deref(), Some("Bearer test"));
    }

    // ── cleanup_http_cache ────────────────────────────────────────────────────

    // Ported: "should not throw if cache is not a valid HttpCache" — util/cache/repository/http-cache.spec.ts line 12
    #[test]
    fn cleanup_http_cache_noop_for_empty_object() {
        let mut cache = serde_json::json!({});
        cleanup_http_cache(&mut cache, None);
        assert_eq!(cache, serde_json::json!({}));
    }

    // Ported: "should remove expired items from the cache" — util/cache/repository/http-cache.spec.ts line 16
    #[test]
    fn cleanup_http_cache_removes_expired_entries() {
        // Expired: 91 days ago; fresh: now
        let expired = (Utc::now() - chrono::Duration::days(91))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let fresh = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        let mut cache = serde_json::json!({
            "httpCache": {
                "http://example.com/foo": { "timestamp": expired, "etag": "abc", "httpResponse": {} },
                "http://example.com/bar": { "timestamp": fresh, "etag": "abc", "httpResponse": {} }
            }
        });
        cleanup_http_cache(&mut cache, None);

        let http_cache = &cache["httpCache"];
        assert!(
            http_cache["http://example.com/foo"].is_null(),
            "expired entry should be removed"
        );
        assert!(
            !http_cache["http://example.com/bar"].is_null(),
            "fresh entry should remain"
        );
    }

    // Ported: "should remove all items if ttlDays is not configured" — util/cache/repository/http-cache.spec.ts line 50
    #[test]
    fn cleanup_http_cache_removes_all_when_ttl_is_zero() {
        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let mut cache = serde_json::json!({
            "httpCache": {
                "http://example.com/foo": { "timestamp": now, "etag": "abc", "httpResponse": {} },
                "http://example.com/bar": { "timestamp": now, "etag": "abc", "httpResponse": {} }
            }
        });
        cleanup_http_cache(&mut cache, Some(0));
        assert!(
            cache.get("httpCache").is_none(),
            "httpCache should be removed entirely"
        );
    }

    // ── parse_retry_after_value ───────────────────────────────────────────────

    // Ported: "returns null for non-integer \"retry-after\" header" — util/http/retry-after.spec.ts line 109
    #[test]
    fn retry_after_value_past_date_returns_none() {
        // "Wed, 21 Oct 2015 07:28:00 GMT" is in the past relative to 2026 → None
        let now = Utc::now();
        assert!(parse_retry_after_value("Wed, 21 Oct 2015 07:28:00 GMT", now).is_none());
    }

    // Ported: "returns delay in seconds from date" — util/http/retry-after.spec.ts line 122
    #[test]
    fn retry_after_value_future_date_returns_seconds() {
        // Mock: now = 2020-01-01T00:00:00Z, retry-after = 2020-01-01T00:00:42Z → 42
        let now = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let result = parse_retry_after_value("Wed, 01 Jan 2020 00:00:42 GMT", now);
        assert_eq!(result, Some(42));
    }

    // Ported: "returns delay in seconds from number" — util/http/retry-after.spec.ts line 136
    #[test]
    fn retry_after_value_numeric_returns_seconds() {
        let now = Utc::now();
        assert_eq!(parse_retry_after_value("42", now), Some(42));
    }

    // Ported: "returns null for invalid header value" — util/http/retry-after.spec.ts line 149
    #[test]
    fn retry_after_value_invalid_returns_none() {
        let now = Utc::now();
        assert!(parse_retry_after_value("invalid", now).is_none());
    }

    // Ported: "returns null missing \"retry-after\" header" — util/http/retry-after.spec.ts line 103
    #[tokio::test]
    async fn retry_after_missing_header_returns_none() {
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/no-header"))
            .respond_with(wiremock::ResponseTemplate::new(429))
            .mount(&server)
            .await;
        let http = HttpClient::new().unwrap();
        let resp = http
            .get(&format!("{}/no-header", server.uri()))
            .send()
            .await
            .unwrap();
        assert!(parse_retry_after(&resp).is_none());
    }

    // Ported: "returns $expected for status code $statusCode and followRedirect $followRedirect" — util/http/hooks.spec.ts line 5
    #[test]
    fn test_is_response_ok() {
        // When followRedirect=false, limit is 399; 304 always OK.
        assert!(is_response_ok(200, false));
        assert!(is_response_ok(299, false));
        assert!(is_response_ok(304, false));
        assert!(is_response_ok(302, false));
        assert!(!is_response_ok(400, false));
        // When followRedirect=true, limit is 299; 304 always OK.
        assert!(is_response_ok(304, true));
        assert!(!is_response_ok(302, true));
        assert!(!is_response_ok(400, true));
    }
}
