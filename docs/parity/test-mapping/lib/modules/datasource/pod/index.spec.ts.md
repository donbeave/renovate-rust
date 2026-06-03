# `lib/modules/datasource/pod/index.spec.ts`

[← `datasource/pod`](../../../../_by-module/datasource/pod.md) · [all modules](../../../../README.md)

**2/19 in-scope tests ported** (17 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | returns null for invalid inputs | pending | — |
| 41 | returns null disabled host | pending | — |
| 51 | returns null for empty result | pending | — |
| 60 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/cocoapods.rs:220`](../../../../../../../crates/renovate-core/src/datasources/cocoapods.rs#L220) |
| 78 | returns null for 404 github enterprise | pending | — |
| 99 | returns null for 404 github enterprise with different url style | pending | — |
| 117 | returns null for 401 | ported | [`crates/renovate-core/src/datasources/cocoapods.rs:237`](../../../../../../../crates/renovate-core/src/datasources/cocoapods.rs#L237) |
| 125 | throws for 429 | pending | — |
| 133 | throws for 500 | pending | — |
| 141 | returns null for unknown error | pending | — |
| 149 | processes real data from cdn | pending | — |
| 169 | processes real data from github with shard with specs | pending | — |
| 188 | processes real data from github with shard without specs | pending | — |
| 209 | processes real data from github with specs without shard | pending | — |
| 232 | processes real data from github without specs without shard | pending | — |
| 257 | processes real data from github enterprise with shard with specs | pending | — |
| 276 | processes real data from github enterprise with shard without specs | pending | — |
| 297 | processes real data from github enterprise with specs without shard | pending | — |
| 320 | processes real data from github enterprise without specs without shard | pending | — |

