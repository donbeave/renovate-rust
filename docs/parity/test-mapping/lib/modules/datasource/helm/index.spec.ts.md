# `lib/modules/datasource/helm/index.spec.ts`

[← `datasource/helm`](../../../../_by-module/datasource/helm.md) · [all modules](../../../../README.md)

**14/14 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns null if packagename was not provided | ported | [`crates/renovate-core/src/datasources/helm.rs:485`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L485) |
| 22 | returns null if repository was not provided | ported | [`crates/renovate-core/src/datasources/helm.rs:492`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L492) |
| 37 | returns null for empty response | ported | [`crates/renovate-core/src/datasources/helm.rs:509`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L509) |
| 51 | returns null for missing response body | ported | [`crates/renovate-core/src/datasources/helm.rs:510`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L510) |
| 65 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/helm.rs:527`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L527) |
| 79 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/helm.rs:544`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L544) |
| 93 | returns null for unknown error | ported | [`crates/renovate-core/src/datasources/helm.rs:559`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L559) |
| 107 | returns null if index.yaml in response is empty | ported | [`crates/renovate-core/src/datasources/helm.rs:569`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L569) |
| 120 | returns null if index.yaml in response is invalid | ported | [`crates/renovate-core/src/datasources/helm.rs:575`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L575) |
| 139 | returns null if packagename is not in index.yaml | ported | [`crates/renovate-core/src/datasources/helm.rs:582`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L582) |
| 152 | returns list of versions for normal response | ported | [`crates/renovate-core/src/datasources/helm.rs:588`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L588) |
| 166 | returns list of versions for other packages if one packages has no versions | ported | [`crates/renovate-core/src/datasources/helm.rs:608`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L608) |
| 184 | adds trailing slash to subdirectories | ported | [`crates/renovate-core/src/datasources/helm.rs:635`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L635) |
| 203 | uses undefined as the newdigest when no digest is provided | ported | [`crates/renovate-core/src/datasources/helm.rs:663`](../../../../../../../crates/renovate-core/src/datasources/helm.rs#L663) |

