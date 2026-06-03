# `lib/modules/manager/terraform/lockfile/hash.spec.ts`

[← `manager/terraform`](../../../../../_by-module/manager/terraform.md) · [all modules](../../../../../README.md)

**10/11 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 43 | returns null if getbuilds returns null | ported | [`crates/renovate-core/src/extractors/terraform.rs:3785`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3785) |
| 58 | return null if requesting a version which is not available | ported | [`crates/renovate-core/src/extractors/terraform.rs:3794`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3794) |
| 72 | backend index throws error | ported | [`crates/renovate-core/src/extractors/terraform.rs:3806`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3806) |
| 86 | returns null for no builds | ported | [`crates/renovate-core/src/extractors/terraform.rs:3818`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3818) |
| 99 | fail to create hashes | ported | [`crates/renovate-core/src/extractors/terraform.rs:3843`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3843) |
| 128 | full walkthrough | ported | [`crates/renovate-core/src/extractors/terraform.rs:3830`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3830) |
| 162 | full walkthrough on terraform cloud | ported | [`crates/renovate-core/src/extractors/terraform.rs:3855`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3855) |
| 227 | full walkthrough with different shasum per build | ported | [`crates/renovate-core/src/extractors/terraform.rs:3868`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3868) |
| 332 | full walkthrough without ziphashes available | ported | [`crates/renovate-core/src/extractors/terraform.rs:3881`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3881) |
| 385 | does not add any ziphashes when the shasums endpoint fails` | ported | [`crates/renovate-core/src/extractors/terraform.rs:3894`](../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3894) |
| 451 | return hash for content with subfolders | pending | — |

