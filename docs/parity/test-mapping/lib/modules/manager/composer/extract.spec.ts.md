# `lib/modules/manager/composer/extract.spec.ts`

[← `manager/composer`](../../../../_by-module/manager/composer.md) · [all modules](../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 24 | returns null for invalid json | ported | [`crates/renovate-core/src/extractors/composer.rs:1099`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1099) |
| 28 | returns null for empty deps | ported | [`crates/renovate-core/src/extractors/composer.rs:1092`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1092) |
| 32 | extracts dependencies with no lock file | ported | [`crates/renovate-core/src/extractors/composer.rs:698`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L698) |
| 38 | extracts registryurls | ported | [`crates/renovate-core/src/extractors/composer.rs:876`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L876) |
| 81 | extracts object registryurls | ported | [`crates/renovate-core/src/extractors/composer.rs:907`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L907) |
| 186 | extracts repositories and registryurls | ported | [`crates/renovate-core/src/extractors/composer.rs:949`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L949) |
| 219 | extracts bitbucket repositories and registryurls | ported | [`crates/renovate-core/src/extractors/composer.rs:1001`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1001) |
| 248 | extracts object repositories and registryurls with lock file | ported | [`crates/renovate-core/src/extractors/composer.rs:1026`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1026) |
| 284 | skips path dependencies | ported | [`crates/renovate-core/src/extractors/composer.rs:852`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L852) |
| 313 | extracts dependencies with lock file | ported | [`crates/renovate-core/src/extractors/composer.rs:1083`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1083) |

