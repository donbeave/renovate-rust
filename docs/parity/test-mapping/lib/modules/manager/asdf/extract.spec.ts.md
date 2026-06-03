# `lib/modules/manager/asdf/extract.spec.ts`

[← `manager/asdf`](../../../../_by-module/manager/asdf.md) · [all modules](../../../../README.md)

**13/13 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | returns a result | ported | [`crates/renovate-core/src/extractors/asdf.rs:1571`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1571) |
| 19 | provides skipreason for lines with unsupported tooling | ported | [`crates/renovate-core/src/extractors/asdf.rs:1585`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1585) |
| 31 | only captures the first version | ported | [`crates/renovate-core/src/extractors/asdf.rs:1616`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1616) |
| 44 | can handle multiple tools in one file | ported | [`crates/renovate-core/src/extractors/asdf.rs:1519`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1519) |
| 890 | can handle multiple tools with indented versions in one file | ported | [`crates/renovate-core/src/extractors/asdf.rs:1651`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1651) |
| 923 | can handle flutter version channel | ported | [`crates/renovate-core/src/extractors/asdf.rs:1679`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1679) |
| 946 | can handle java jre / jdk | ported | [`crates/renovate-core/src/extractors/asdf.rs:1696`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1696) |
| 1004 | can handle scala v 2 & 3 | ported | [`crates/renovate-core/src/extractors/asdf.rs:1749`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1749) |
| 1054 | entry: '${data.entry}' | ported | [`crates/renovate-core/src/extractors/asdf.rs:1594`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1594) |
| 1069 | invalid comment placements fail to parse | ported | [`crates/renovate-core/src/extractors/asdf.rs:1795`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1795) |
| 1076 | ignores lines that are just comments | ported | [`crates/renovate-core/src/extractors/asdf.rs:1609`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1609) |
| 1081 | ignores comments across multiple lines | ported | [`crates/renovate-core/src/extractors/asdf.rs:1783`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1783) |
| 1096 | ignores supported tooling with a renovate:ignore comment | ported | [`crates/renovate-core/src/extractors/asdf.rs:1635`](../../../../../../../crates/renovate-core/src/extractors/asdf.rs#L1635) |

