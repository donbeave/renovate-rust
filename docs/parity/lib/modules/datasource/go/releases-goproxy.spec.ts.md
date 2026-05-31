# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/releases-goproxy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/releases-goproxy.spec.ts
**Total tests:** 28 | **Ported:** 1 | **Actionable:** 27 | **Status:** pending

### `modules/datasource/go/releases-goproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodeCase | 27 | ported | `gomod.rs` | `encode_module_path_all_lowercase`, `encode_module_path_capital_letters` | Rust verifies Go proxy uppercase escaping, including all-uppercase path segments. |

### `modules/datasource/go/releases-goproxy › requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| listVersions | 37 | pending | — | — | No corresponding Rust source|
| versionInfo | 49 | pending | — | — | No corresponding Rust source|

### `modules/datasource/go/releases-goproxy › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles direct | 78 | pending | — | — | No corresponding Rust source|
| skips GONOPROXY and GOPRIVATE packages | 102 | pending | — | — | No corresponding Rust source|
| fetches release data from goproxy | 127 | pending | — | — | No corresponding Rust source|
| handles timestamp fetch errors | 171 | pending | — | — | No corresponding Rust source|
| handles pipe fallback when abortOnError is $abortOnError | 204 | pending | — | — | No corresponding Rust source|
| handles comma fallback | 253 | pending | — | — | No corresponding Rust source|
| short-circuits for errors other than 404 or 410 | 303 | pending | — | — | No corresponding Rust source|
| supports "direct" keyword | 332 | pending | — | — | No corresponding Rust source|
| supports "off" keyword | 370 | pending | — | — | No corresponding Rust source|
| handles soureUrl fetch errors | 392 | pending | — | — | No corresponding Rust source|
| handles major releases with abortOnError is $abortOnError | 423 | pending | — | — | No corresponding Rust source|
| handles major releases with 403 status (Artifactory) | 479 | pending | — | — | No corresponding Rust source|
| handles gopkg.in major releases | 527 | pending | — | — | No corresponding Rust source|
| handles gopkg.in major releases from v0 | 570 | pending | — | — | No corresponding Rust source|
| handles baseURL with slash at the end | 607 | pending | — | — | No corresponding Rust source|
| continues if package returns no releases | 644 | pending | — | — | No corresponding Rust source|
| uses latest if package has no releases | 661 | pending | — | — | No corresponding Rust source|

### modules/datasource/go/releases-goproxy › getReleases › looks up `go` directive requirements if constraintsFiltering=strict

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| and returns unfiltered `constraints` in the Release | 689 | pending | — | — | No corresponding Rust source|
| handles major version updates | 779 | pending | — | — | No corresponding Rust source|
| handles HTTP errors by omitting constraints on failed HTTP requests | 899 | pending | — | — | No corresponding Rust source|
| does not set constraints if no `go` directive | 956 | pending | — | — | No corresponding Rust source|
| normalises constraints if not full SemVer `go` directive: %s | 998 | pending | — | — | No corresponding Rust source|
| converts minor-only version numbers to include patch of .0 | 1053 | pending | — | — | No corresponding Rust source|
| skips `toolchain` directive | 1100 | pending | — | — | No corresponding Rust source|
| does not look up `go` directive requirements if constraintsFiltering=none | 1148 | pending | — | — | No corresponding Rust source|

---
