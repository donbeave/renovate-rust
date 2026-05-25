# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/releases-goproxy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/releases-goproxy.spec.ts
**Total tests:** 28 | **Ported:** 1 | **Actionable:** 28 | **Status:** partial

### `modules/datasource/go/releases-goproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodeCase | 27 | ported | `gomod.rs` | `encode_module_path_all_lowercase`, `encode_module_path_capital_letters` | Rust verifies Go proxy uppercase escaping, including all-uppercase path segments. |

### `modules/datasource/go/releases-goproxy › requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| listVersions | 37 | pending | — | — | — |
| versionInfo | 49 | pending | — | — | — |

### `modules/datasource/go/releases-goproxy › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles direct | 78 | pending | — | — | — |
| skips GONOPROXY and GOPRIVATE packages | 102 | pending | — | — | — |
| fetches release data from goproxy | 127 | pending | — | — | — |
| handles timestamp fetch errors | 171 | pending | — | — | — |
| handles pipe fallback when abortOnError is $abortOnError | 204 | pending | — | — | — |
| handles comma fallback | 253 | pending | — | — | — |
| short-circuits for errors other than 404 or 410 | 303 | pending | — | — | — |
| supports "direct" keyword | 332 | pending | — | — | — |
| supports "off" keyword | 370 | pending | — | — | — |
| handles soureUrl fetch errors | 392 | pending | — | — | — |
| handles major releases with abortOnError is $abortOnError | 423 | pending | — | — | — |
| handles major releases with 403 status (Artifactory) | 479 | pending | — | — | — |
| handles gopkg.in major releases | 527 | pending | — | — | — |
| handles gopkg.in major releases from v0 | 570 | pending | — | — | — |
| handles baseURL with slash at the end | 607 | pending | — | — | — |
| continues if package returns no releases | 644 | pending | — | — | — |
| uses latest if package has no releases | 661 | pending | — | — | — |

### modules/datasource/go/releases-goproxy › getReleases › looks up `go` directive requirements if constraintsFiltering=strict

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| and returns unfiltered `constraints` in the Release | 689 | pending | — | — | — |
| handles major version updates | 779 | pending | — | — | — |
| handles HTTP errors by omitting constraints on failed HTTP requests | 899 | pending | — | — | — |
| does not set constraints if no `go` directive | 956 | pending | — | — | — |
| normalises constraints if not full SemVer `go` directive: %s | 998 | pending | — | — | — |
| converts minor-only version numbers to include patch of .0 | 1053 | pending | — | — | — |
| skips `toolchain` directive | 1100 | pending | — | — | — |
| does not look up `go` directive requirements if constraintsFiltering=none | 1148 | pending | — | — | — |

---

