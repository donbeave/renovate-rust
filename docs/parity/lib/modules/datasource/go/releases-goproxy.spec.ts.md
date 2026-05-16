# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/releases-goproxy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/releases-goproxy.spec.ts
**Total tests:** 28 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/go/releases-goproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodeCase | 27 | ported | `gomod.rs` | `encode_module_path_all_lowercase`, `encode_module_path_capital_letters` | Rust verifies Go proxy uppercase escaping, including all-uppercase path segments. |

### `modules/datasource/go/releases-goproxy › requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| listVersions | 37 | not-applicable | — | — | Renovate's Go proxy `@v/list` release-list request helper is not implemented in Rust; Rust currently queries only `@latest`. |
| versionInfo | 49 | not-applicable | — | — | Renovate's per-version `.info` request helper is not implemented in Rust; Rust parses metadata from the `@latest` response. |

### `modules/datasource/go/releases-goproxy › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles direct | 78 | not-applicable | — | — | Renovate's GOPROXY `direct` mode and direct tag datasource fallback are not implemented in Rust. |
| skips GONOPROXY and GOPRIVATE packages | 102 | not-applicable | — | — | Renovate's GONOPROXY/GOPRIVATE matching and proxy bypass are not implemented in Rust. |
| fetches release data from goproxy | 127 | not-applicable | — | — | Renovate's full Go proxy release-list assembly from `@v/list`, `.info`, `@latest`, pseudo-version digest extraction, and source URL discovery is not implemented in Rust. |
| handles timestamp fetch errors | 171 | not-applicable | — | — | Renovate's per-version timestamp fallback from `.info` fetch failures is not implemented in Rust. |
| handles pipe fallback when abortOnError is $abortOnError | 204 | not-applicable | — | — | Renovate's multi-proxy pipe fallback and hostRules abort behavior are not implemented in Rust. |
| handles comma fallback | 253 | not-applicable | — | — | Renovate's multi-proxy comma fallback behavior is not implemented in Rust. |
| short-circuits for errors other than 404 or 410 | 303 | not-applicable | — | — | Renovate's multi-proxy error short-circuiting around `@v/list` is not implemented in Rust. |
| supports "direct" keyword | 332 | not-applicable | — | — | Renovate's GOPROXY `direct` keyword fallback to tag datasources is not implemented in Rust. |
| supports "off" keyword | 370 | not-applicable | — | — | Renovate's GOPROXY `off` keyword handling is not implemented in Rust. |
| handles soureUrl fetch errors | 392 | not-applicable | — | — | Renovate's Go source URL discovery from `go-get=1` HTML is not implemented in Rust. |
| handles major releases with abortOnError is $abortOnError | 423 | not-applicable | — | — | Renovate's v2+ Go proxy release-list scanning with hostRules abort behavior is not implemented in Rust. |
| handles major releases with 403 status (Artifactory) | 479 | not-applicable | — | — | Renovate's v2+ release-list scanning stop condition for Artifactory 403 responses is not implemented in Rust. |
| handles gopkg.in major releases | 527 | not-applicable | — | — | Renovate's gopkg.in major-version proxy path scanning is not implemented in Rust. |
| handles gopkg.in major releases from v0 | 570 | not-applicable | — | — | Renovate's gopkg.in v0-to-v1 major-version proxy path scanning is not implemented in Rust. |
| handles baseURL with slash at the end | 607 | not-applicable | — | — | Renovate's GOPROXY base URL normalization for release-list scanning is not implemented in Rust; Rust callers pass the proxy base URL directly for `@latest`. |
| continues if package returns no releases | 644 | not-applicable | — | — | Renovate's empty `@v/list` handling inside release-list assembly is not implemented in Rust. |
| uses latest if package has no releases | 661 | not-applicable | — | — | Renovate's fallback from empty `@v/list` to `@latest` as a synthetic release is not implemented in Rust; Rust only fetches the latest summary. |

### modules/datasource/go/releases-goproxy › getReleases › looks up `go` directive requirements if constraintsFiltering=strict

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| and returns unfiltered `constraints` in the Release | 689 | not-applicable | — | — | Renovate's per-release `.mod` parsing and `%goMod` constraints extraction are not implemented in Rust. |
| handles major version updates | 779 | not-applicable | — | — | Renovate's v2+ release-list scanning with per-release `.mod` constraints extraction is not implemented in Rust. |
| handles HTTP errors by omitting constraints on failed HTTP requests | 899 | not-applicable | — | — | Renovate's `.mod` fetch error fallback for release constraints is not implemented in Rust. |
| does not set constraints if no `go` directive | 956 | not-applicable | — | — | Renovate's `.mod` parsing and missing-go-directive behavior are not implemented in Rust. |
| normalises constraints if not full SemVer `go` directive: %s | 998 | not-applicable | — | — | Renovate's Go directive SemVer normalization for release constraints is not implemented in Rust. |
| converts minor-only version numbers to include patch of .0 | 1053 | not-applicable | — | — | Renovate's Go directive minor-version normalization for release constraints is not implemented in Rust. |
| skips `toolchain` directive | 1100 | not-applicable | — | — | Renovate's `.mod` toolchain-directive skipping while extracting Go constraints is not implemented in Rust. |
| does not look up `go` directive requirements if constraintsFiltering=none | 1148 | not-applicable | — | — | Renovate's constraintsFiltering switch for `.mod` lookups is not implemented in Rust. |

---

