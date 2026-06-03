# `lib/modules/manager/gleam/extract.spec.ts`

[← `manager/gleam`](../../../../_by-module/manager/gleam.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | should extract dev and prod dependencies | ported | [`crates/renovate-core/src/extractors/gleam.rs:261`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L261) |
| 41 | should extract dev only dependencies | ported | [`crates/renovate-core/src/extractors/gleam.rs:275`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L275) |
| 65 | should return null when no dependencies are found | ported | [`crates/renovate-core/src/extractors/gleam.rs:308`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L308) |
| 82 | should return null when gleam.toml is invalid | ported | [`crates/renovate-core/src/extractors/gleam.rs:302`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L302) |
| 91 | should return locked versions | ported | [`crates/renovate-core/src/extractors/gleam.rs:362`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L362) |
| 119 | should fail to extract locked version | ported | [`crates/renovate-core/src/extractors/gleam.rs:375`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L375) |
| 138 | should fail to find locked version in range | ported | [`crates/renovate-core/src/extractors/gleam.rs:386`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L386) |
| 166 | should handle invalid versions in lock file | ported | [`crates/renovate-core/src/extractors/gleam.rs:395`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L395) |
| 193 | should handle lock file parsing and extracting errors | ported | [`crates/renovate-core/src/extractors/gleam.rs:404`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L404) |

