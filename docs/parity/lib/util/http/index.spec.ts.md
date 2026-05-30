# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/index.spec.ts
**Total tests:** 52 | **Ported:** 20 | **Actionable:** 4 | **Status:** partial

### `util/http/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 29 | ported | `http.rs` | `get_sends_request_and_receives_response` | — |
| returns 429 error | 40 | ported | `http.rs` | `get_returns_429_error_after_retries_exhausted` | — |
| returns 401 error | 48 | ported | `http.rs` | `get_returns_401_error` | Verifies status code and www-authenticate header |
| converts 404 error to ExternalHostError | 84 | not-applicable | — | — | `ExternalHostError` concept not implemented in Rust |
| disables hosts | 93 | not-applicable | — | — | Host disabling not implemented in Rust HTTP client |
| ignores 404 error and does not throw ExternalHostError | 100 | not-applicable | — | — | `ExternalHostError` concept not implemented in Rust |
| does not pass auth on redirects | 109 | not-applicable | — | — | Redirect auth stripping not implemented in Rust |
| getJson | 127 | ported | `http.rs` | `get_json_parses_json_body` | Rust does not send `Accept: application/json` header automatically |
| postJson | 151 | ported | `http.rs` | `post_json_sends_body_and_parses_response` | — |
| putJson | 166 | ported | `http.rs` | `put_json_sends_body_and_parses_response` | — |
| patchJson | 181 | ported | `http.rs` | `patch_json_sends_body_and_returns_response` | — |
| deleteJson | 196 | ported | `http.rs` | `delete_json_sends_request_and_parses_response` | — |
| headJson | 211 | ported | `http.rs` | `head_json_sends_request_and_returns_response` | — |
| stream | 226 | pending | — | — | —|
| disables hosts for stream | 251 | not-applicable | — | — | Host disabling not implemented in Rust HTTP client |
| limits concurrency by host | 259 | not-applicable | — | — | Concurrency limiting not implemented in Rust HTTP client |
| getBuffer | 349 | pending | — | — | —|

### `util/http/index › retry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 369 | ported | `http.rs` | `retries_on_429_then_succeeds` | — |

### `util/http/index › Schema support › getPlain`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets plain text with correct headers | 402 | ported | `http.rs` | `get_raw_with_accept_returns_body` | — |
| works with custom options | 412 | pending | — | — | Custom options (timeout, auth) not tested via wiremock |

### `util/http/index › Schema support › getYamlUnchecked`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yaml response without schema | 427 | ported | `http.rs` | `get_yaml_parses_response_body` | — |
| parses yaml with options | 434 | pending | — | — | Custom options not implemented |
| throws on invalid yaml | 447 | ported | `http.rs` | `get_yaml_throws_on_invalid_yaml` | — |

### `util/http/index › Schema support › getYaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yaml with schema validation | 457 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| parses yaml with options and schema | 464 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| throws on schema validation failure | 479 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| throws on invalid yaml | 487 | ported | `http.rs` | `get_yaml_throws_on_invalid_yaml` | — |

### `util/http/index › Schema support › getYamlSafe`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns successful result with schema validation | 497 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| returns schema error result | 508 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| returns error result for invalid yaml | 522 | ported | `http.rs` | `get_yaml_returns_error_for_invalid_yaml` | — |
| returns error result for network errors | 533 | ported | `http.rs` | `get_yaml_returns_error_for_network_errors` | — |
| works with options and schema | 547 | not-applicable | — | — | Zod schema validation not applicable to Rust |

### `util/http/index › Schema support › getJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 568 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| throws on schema mismatch | 588 | not-applicable | — | — | Zod schema validation not applicable to Rust |

### `util/http/index › Schema support › getJsonSafe`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 605 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| returns schema error result | 619 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| returns error result | 633 | ported | `http.rs` | `get_returns_429_error_after_retries_exhausted` | Rust get_json returns Result<T, HttpError> natively; error handling covered by status code tests |

### `util/http/index › Schema support › postJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 646 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| throws on schema mismatch | 661 | not-applicable | — | — | Zod schema validation not applicable to Rust |

### `util/http/index › Throttling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works without throttling | 679 | not-applicable | — | — | Throttling not implemented in Rust HTTP client |
| limits request rate by host | 691 | not-applicable | — | — | Throttling not implemented in Rust HTTP client |

### `util/http/index › getToml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses toml with schema validation | 711 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| parses toml with options and schema | 718 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| throws on schema validation failure | 737 | not-applicable | — | — | Zod schema validation not applicable to Rust |
| throws on invalid toml | 752 | ported | `http.rs` | `get_toml_throws_on_invalid_toml` | — |

| sets default user-agent | 36 | ported | `http.rs` | `default_user_agent_is_set_on_requests` | — |
| uses userAgent when set as a plain string | 46 | not-applicable | — | — | Custom user-agent not supported in Rust HTTP client |
| interpolates {{renovateVersion}} in a custom userAgent template | 55 | not-applicable | — | — | Handlebars templating not applicable to Rust |
| renders unknown template variables as empty string | 68 | not-applicable | — | — | Handlebars templating not applicable to Rust |
| supports Handlebars helpers in userAgent template | 77 | not-applicable | — | — | Handlebars templating not applicable to Rust |
| supports conditional Handlebars syntax in userAgent template | 88 | not-applicable | — | — | Handlebars templating not applicable to Rust |
| preserves existing headers | 100 | ported | `http.rs` | `get_preserves_existing_headers` | — |
---
