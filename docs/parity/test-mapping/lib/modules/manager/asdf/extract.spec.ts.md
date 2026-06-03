# `lib/modules/manager/asdf/extract.spec.ts`

[← `manager/asdf`](../../../../_by-module/manager/asdf.md) · [all modules](../../../../README.md)

**13/13 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | returns a result | ported | `crates/renovate-core/src/extractors/asdf.rs:1571` |
| 19 | provides skipreason for lines with unsupported tooling | ported | `crates/renovate-core/src/extractors/asdf.rs:1585` |
| 31 | only captures the first version | ported | `crates/renovate-core/src/extractors/asdf.rs:1616` |
| 44 | can handle multiple tools in one file | ported | `crates/renovate-core/src/extractors/asdf.rs:1519` |
| 890 | can handle multiple tools with indented versions in one file | ported | `crates/renovate-core/src/extractors/asdf.rs:1651` |
| 923 | can handle flutter version channel | ported | `crates/renovate-core/src/extractors/asdf.rs:1679` |
| 946 | can handle java jre / jdk | ported | `crates/renovate-core/src/extractors/asdf.rs:1696` |
| 1004 | can handle scala v 2 & 3 | ported | `crates/renovate-core/src/extractors/asdf.rs:1749` |
| 1054 | entry: '${data.entry}' | ported | `crates/renovate-core/src/extractors/asdf.rs:1594` |
| 1069 | invalid comment placements fail to parse | ported | `crates/renovate-core/src/extractors/asdf.rs:1795` |
| 1076 | ignores lines that are just comments | ported | `crates/renovate-core/src/extractors/asdf.rs:1609` |
| 1081 | ignores comments across multiple lines | ported | `crates/renovate-core/src/extractors/asdf.rs:1783` |
| 1096 | ignores supported tooling with a renovate:ignore comment | ported | `crates/renovate-core/src/extractors/asdf.rs:1635` |

