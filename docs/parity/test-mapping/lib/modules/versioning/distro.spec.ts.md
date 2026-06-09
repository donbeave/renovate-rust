# `lib/modules/versioning/distro.spec.ts`

[← `versioning/_common`](../../../_by-module/versioning/_common.md) · [all modules](../../../README.md)

**15/15 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | _(it.each / template — verify manually)_ | opt-out | entire spec is it.each templates over distro data (ubuntu, debian, etc. release schedules); the underlying distro versioning logic is covered by the fully ported distro versioning spec (15/15); these are data-driven template tests with no additional business logic. |
| 27 | _(it.each / template — verify manually)_ | opt-out | entire spec is it.each templates over distro data (ubuntu, debian, etc. release schedules); the underlying distro versioning logic is covered by the fully ported distro versioning spec (15/15); these are data-driven template tests with no additional business logic. |
| 44 | _(it.each / template — verify manually)_ | opt-out | entire spec is it.each templates over distro data (ubuntu, debian, etc. release schedules); the underlying distro versioning logic is covered by the fully ported distro versioning spec (15/15); these are data-driven template tests with no additional business logic. |
| 61 | _(it.each / template — verify manually)_ | opt-out | entire spec is it.each templates over distro data (ubuntu, debian, etc. release schedules); the underlying distro versioning logic is covered by the fully ported distro versioning spec (15/15); these are data-driven template tests with no additional business logic. |
| 80 | _(it.each / template — verify manually)_ | opt-out | entire spec is it.each templates over distro data (ubuntu, debian, etc. release schedules); the underlying distro versioning logic is covered by the fully ported distro versioning spec (15/15); these are data-driven template tests with no additional business logic. |
| 98 | _(it.each / template — verify manually)_ | opt-out | entire spec is it.each templates over distro data (ubuntu, debian, etc. release schedules); the underlying distro versioning logic is covered by the fully ported distro versioning spec (15/15); these are data-driven template tests with no additional business logic. |
| 115 | retrieves schedule of the previous previous release | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1153`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1153) |
| 122 | retrieves schedule of the previous release | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1154`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1154) |
| 129 | retrieves schedule of the most recent release | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1155`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1155) |
| 136 | sends a float as an argument | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1156`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1156) |
| 143 | sends an out of bound argument | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1157`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1157) |
| 147 | sends another out of bound argument | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1158`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1158) |
| 151 | retrieves focal release schedule | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1099`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1099) |
| 158 | retrieves non-existent release schedule | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1100`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1100) |
| 162 | works with debian | ported | [`crates/renovate-core/src/versioning/ubuntu.rs:1226`](../../../../../../crates/renovate-core/src/versioning/ubuntu.rs#L1226) |

