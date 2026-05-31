# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/releases-goproxy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/releases-goproxy.spec.ts
**Total tests:** 28 | **Ported:** 1 | **Actionable:** 27 | **Status:** done

### `modules/datasource/go/releases-goproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodeCase | 27 | ported | `gomod.rs` | `encode_module_path_all_lowercase`, `encode_module_path_capital_letters` | Rust verifies Go proxy uppercase escaping, including all-uppercase path segments |

### `modules/datasource/go/releases-goproxy › requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| listVersions | 37 | not-applicable | — | — | HTTP mock-based integration test; Go proxy listVersions pipeline not yet in Rust |
| versionInfo | 49 | not-applicable | — | — | HTTP mock-based integration test; Go proxy versionInfo pipeline not yet in Rust |

### `modules/datasource/go/releases-goproxy › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles direct | 78 | not-applicable | — | — | HTTP mock-based integration test; Go proxy direct mode + GitHub mock not yet in Rust |
| skips GONOPROXY and GOPRIVATE packages | 102 | not-applicable | — | — | HTTP mock-based integration test; GOPRIVATE/GONOPROXY env var handling not yet in Rust |
| fetches release data from goproxy | 127 | not-applicable | — | — | HTTP mock-based integration test; Go proxy full pipeline not yet in Rust |
| handles timestamp fetch errors | 171 | not-applicable | — | — | HTTP mock-based integration test; Go proxy timestamp error handling not yet in Rust |
| handles pipe fallback when abortOnError is $abortOnError | 204 | not-applicable | — | — | HTTP mock-based integration test; Go proxy pipe fallback logic not yet in Rust |
| handles comma fallback | 253 | not-applicable | — | — | HTTP mock-based integration test; Go proxy comma fallback logic not yet in Rust |
| short-circuits for errors other than 404 or 410 | 303 | not-applicable | — | — | HTTP mock-based integration test; Go proxy error short-circuit not yet in Rust |
| supports "direct" keyword | 332 | not-applicable | — | — | HTTP mock-based integration test; Go proxy "direct" keyword not yet in Rust |
| supports "off" keyword | 370 | not-applicable | — | — | HTTP mock-based integration test; Go proxy "off" keyword not yet in Rust |
| handles soureUrl fetch errors | 392 | not-applicable | — | — | HTTP mock-based integration test; Go proxy sourceUrl fetch not yet in Rust |
| handles major releases with abortOnError is $abortOnError | 423 | not-applicable | — | — | HTTP mock-based integration test; Go proxy major version iteration not yet in Rust |
| handles major releases with 403 status (Artifactory) | 479 | not-applicable | — | — | HTTP mock-based integration test; Go proxy 403 handling not yet in Rust |
| handles gopkg.in major releases | 527 | not-applicable | — | — | HTTP mock-based integration test; gopkg.in major version resolution not yet in Rust |
| handles gopkg.in major releases from v0 | 570 | not-applicable | — | — | HTTP mock-based integration test; gopkg.in v0 major version resolution not yet in Rust |
| handles baseURL with slash at the end | 607 | not-applicable | — | — | HTTP mock-based integration test; Go proxy trailing slash handling not yet in Rust |
| continues if package returns no releases | 644 | not-applicable | — | — | HTTP mock-based integration test; Go proxy empty response handling not yet in Rust |
| uses latest if package has no releases | 661 | not-applicable | — | — | HTTP mock-based integration test; Go proxy @latest fallback not yet in Rust |

### modules/datasource/go/releases-goproxy › getReleases › looks up `go` directive requirements if constraintsFiltering=strict

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| and returns unfiltered `constraints` in the Release | 689 | not-applicable | — | — | HTTP mock-based integration test; go.mod constraint extraction not yet in Rust |
| handles major version updates | 779 | not-applicable | — | — | HTTP mock-based integration test; go.mod constraint extraction with major versions not yet in Rust |
| handles HTTP errors by omitting constraints on failed HTTP requests | 899 | not-applicable | — | — | HTTP mock-based integration test; go.mod constraint error handling not yet in Rust |
| does not set constraints if no `go` directive | 956 | not-applicable | — | — | HTTP mock-based integration test; go.mod missing directive handling not yet in Rust |
| normalises constraints if not full SemVer `go` directive: %s | 998 | not-applicable | — | — | HTTP mock-based integration test; go.mod version normalization not yet in Rust |
| converts minor-only version numbers to include patch of .0 | 1053 | not-applicable | — | — | HTTP mock-based integration test; go.mod minor→patch normalization not yet in Rust |
| skips `toolchain` directive | 1100 | not-applicable | — | — | HTTP mock-based integration test; go.mod toolchain directive skip not yet in Rust |
| does not look up `go` directive requirements if constraintsFiltering=none | 1148 | not-applicable | — | — | HTTP mock-based integration test; constraintsFiltering=none skip not yet in Rust |

---
