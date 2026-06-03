# `lib/modules/datasource/galaxy-collection/index.spec.ts`

[← `datasource/galaxy-collection`](../../../../_by-module/datasource/galaxy-collection.md) · [all modules](../../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 29 | returns null for 404 result | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:263`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L263) |
| 39 | throws for remote host error | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:280`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L280) |
| 49 | returns null for unexpected data at base | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:295`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L295) |
| 62 | returns null for unexpected data at versions | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:315`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L315) |
| 77 | throws error for remote host versions error | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:343`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L343) |
| 92 | throws error for remote host detailed versions error | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:370`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L370) |
| 113 | returns null for empty lookup | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:421`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L421) |
| 122 | returns null for null packagename | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:431`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L431) |
| 131 | returns null for unknown error | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:441`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L441) |
| 144 | processes real data | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:454`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L454) |
| 167 | returns null but matches automation hub url | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:565`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L565) |
| 183 | processes real data with automation hub url | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:582`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L582) |
| 212 | returns ansible url with artifactory url | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:523`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L523) |
| 223 | returns galaxy url with automation hub url | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:537`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L537) |
| 234 | returns galaxy url with other url | ported | [`crates/renovate-core/src/datasources/galaxy_collection.rs:551`](../../../../../../../crates/renovate-core/src/datasources/galaxy_collection.rs#L551) |

