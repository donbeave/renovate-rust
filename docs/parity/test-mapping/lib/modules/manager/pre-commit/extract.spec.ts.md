# `lib/modules/manager/pre-commit/extract.spec.ts`

[← `manager/pre-commit`](../../../../_by-module/manager/pre-commit.md) · [all modules](../../../../README.md)

**12/12 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 52 | returns null for invalid yaml file content | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:499`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L499) |
| 57 | returns null for empty yaml file content | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:439`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L439) |
| 62 | returns null for no file content | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:518`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L518) |
| 68 | returns null for no repos | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:445`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L445) |
| 73 | returns null for empty repos | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:505`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L505) |
| 78 | returns null for invalid repo | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:511`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L511) |
| 83 | extracts from values.yaml correctly with same structure as "pre-commit sample-config" | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:451`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L451) |
| 105 | extracts from complex config file correctly | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:377`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L377) |
| 161 | can handle private git repos | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:471`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L471) |
| 183 | can handle invalid private git repos | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:459`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L459) |
| 200 | can handle unknown private git repos | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:484`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L484) |
| 220 | can handle pinned repo versions | ported | [`crates/renovate-core/src/extractors/pre_commit.rs:525`](../../../../../../../crates/renovate-core/src/extractors/pre_commit.rs#L525) |

