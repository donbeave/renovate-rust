# `lib/modules/datasource/helm/index.spec.ts`

[← `datasource/helm`](../../../../_by-module/datasource/helm.md) · [all modules](../../../../README.md)

**14/14 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | returns null if packagename was not provided | ported | `crates/renovate-core/src/datasources/helm.rs:485` |
| 22 | returns null if repository was not provided | ported | `crates/renovate-core/src/datasources/helm.rs:492` |
| 37 | returns null for empty response | ported | `crates/renovate-core/src/datasources/helm.rs:509` |
| 51 | returns null for missing response body | ported | `crates/renovate-core/src/datasources/helm.rs:510` |
| 65 | returns null for 404 | ported | `crates/renovate-core/src/datasources/helm.rs:527` |
| 79 | throws for 5xx | ported | `crates/renovate-core/src/datasources/helm.rs:544` |
| 93 | returns null for unknown error | ported | `crates/renovate-core/src/datasources/helm.rs:559` |
| 107 | returns null if index.yaml in response is empty | ported | `crates/renovate-core/src/datasources/helm.rs:569` |
| 120 | returns null if index.yaml in response is invalid | ported | `crates/renovate-core/src/datasources/helm.rs:575` |
| 139 | returns null if packagename is not in index.yaml | ported | `crates/renovate-core/src/datasources/helm.rs:582` |
| 152 | returns list of versions for normal response | ported | `crates/renovate-core/src/datasources/helm.rs:588` |
| 166 | returns list of versions for other packages if one packages has no versions | ported | `crates/renovate-core/src/datasources/helm.rs:608` |
| 184 | adds trailing slash to subdirectories | ported | `crates/renovate-core/src/datasources/helm.rs:635` |
| 203 | uses undefined as the newdigest when no digest is provided | ported | `crates/renovate-core/src/datasources/helm.rs:663` |

