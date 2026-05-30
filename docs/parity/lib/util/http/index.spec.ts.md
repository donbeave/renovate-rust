# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/index.spec.ts
**Total tests:** 52 | **Ported:** 5 | **Actionable:** 52 | **Status:** partial

### `util/http/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 29 | pending | — | — | —|
| returns 429 error | 40 | pending | — | — | —|
| returns 401 error | 48 | ported | `http.rs` | `get_returns_401_error` | Verifies status code and www-authenticate header |
| converts 404 error to ExternalHostError | 84 | pending | — | — | `ExternalHostError` concept not implemented in Rust |
| disables hosts | 93 | pending | — | — | Host disabling not implemented in Rust HTTP client |
| ignores 404 error and does not throw ExternalHostError | 100 | pending | — | — | `ExternalHostError` concept not implemented in Rust |
| does not pass auth on redirects | 109 | pending | — | — | Redirect auth stripping not implemented in Rust |
| getJson | 127 | ported | `http.rs` | `get_json_parses_json_body` | Rust does not send `Accept: application/json` header automatically |
| postJson | 151 | ported | `http.rs` | `post_json_sends_body_and_parses_response` | — |
| putJson | 166 | pending | — | — | —|
| patchJson | 181 | pending | — | — | —|
| deleteJson | 196 | pending | — | — | —|
| headJson | 211 | pending | — | — | —|
| stream | 226 | pending | — | — | —|
| disables hosts for stream | 251 | pending | — | — | —|
| limits concurrency by host | 259 | pending | — | — | —|
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
| parses yaml response without schema | 427 | pending | — | — | —|
| parses yaml with options | 434 | pending | — | — | —|
| throws on invalid yaml | 447 | pending | — | — | —|

### `util/http/index › Schema support › getYaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yaml with schema validation | 457 | pending | — | — | —|
| parses yaml with options and schema | 464 | pending | — | — | —|
| throws on schema validation failure | 479 | pending | — | — | —|
| throws on invalid yaml | 487 | pending | — | — | —|

### `util/http/index › Schema support › getYamlSafe`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns successful result with schema validation | 497 | pending | — | — | —|
| returns schema error result | 508 | pending | — | — | —|
| returns error result for invalid yaml | 522 | pending | — | — | —|
| returns error result for network errors | 533 | pending | — | — | —|
| works with options and schema | 547 | pending | — | — | —|

### `util/http/index › Schema support › getJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 568 | pending | — | — | —|
| throws on schema mismatch | 588 | pending | — | — | —|

### `util/http/index › Schema support › getJsonSafe`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 605 | pending | — | — | —|
| returns schema error result | 619 | pending | — | — | —|
| returns error result | 633 | pending | — | — | —|

### `util/http/index › Schema support › postJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 646 | pending | — | — | —|
| throws on schema mismatch | 661 | pending | — | — | —|

### `util/http/index › Throttling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works without throttling | 679 | pending | — | — | —|
| limits request rate by host | 691 | pending | — | — | —|

### `util/http/index › getToml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses toml with schema validation | 711 | pending | — | — | —|
| parses toml with options and schema | 718 | pending | — | — | —|
| throws on schema validation failure | 737 | pending | — | — | —|
| throws on invalid toml | 752 | pending | — | — | —|

| sets default user-agent | 36 | pending | — | — | —|
| uses userAgent when set as a plain string | 46 | pending | — | — | —|
| interpolates {{renovateVersion}} in a custom userAgent template | 55 | pending | — | — | —|
| renders unknown template variables as empty string | 68 | pending | — | — | —|
| supports Handlebars helpers in userAgent template | 77 | pending | — | — | —|
| supports conditional Handlebars syntax in userAgent template | 88 | pending | — | — | —|
| preserves existing headers | 100 | pending | — | — | —|
---
