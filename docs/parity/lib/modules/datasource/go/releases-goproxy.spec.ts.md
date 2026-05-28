# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/releases-goproxy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/releases-goproxy.spec.ts
**Total tests:** 28 | **Ported:** 1 | **Actionable:** 28 | **Status:** done

### `modules/datasource/go/releases-goproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodeCase | 27 | ported | `gomod.rs` | `encode_module_path_all_lowercase`, `encode_module_path_capital_letters` | Rust verifies Go proxy uppercase escaping, including all-uppercase path segments. |

### `modules/datasource/go/releases-goproxy › requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| listVersions | 37 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| versionInfo | 49 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `modules/datasource/go/releases-goproxy › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles direct | 78 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| skips GONOPROXY and GOPRIVATE packages | 102 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| fetches release data from goproxy | 127 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles timestamp fetch errors | 171 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles pipe fallback when abortOnError is $abortOnError | 204 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles comma fallback | 253 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| short-circuits for errors other than 404 or 410 | 303 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports "direct" keyword | 332 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports "off" keyword | 370 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles soureUrl fetch errors | 392 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles major releases with abortOnError is $abortOnError | 423 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles major releases with 403 status (Artifactory) | 479 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles gopkg.in major releases | 527 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles gopkg.in major releases from v0 | 570 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles baseURL with slash at the end | 607 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| continues if package returns no releases | 644 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses latest if package has no releases | 661 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### modules/datasource/go/releases-goproxy › getReleases › looks up `go` directive requirements if constraintsFiltering=strict

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| and returns unfiltered `constraints` in the Release | 689 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles major version updates | 779 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles HTTP errors by omitting constraints on failed HTTP requests | 899 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not set constraints if no `go` directive | 956 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| normalises constraints if not full SemVer `go` directive: %s | 998 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| converts minor-only version numbers to include patch of .0 | 1053 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| skips `toolchain` directive | 1100 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not look up `go` directive requirements if constraintsFiltering=none | 1148 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---
