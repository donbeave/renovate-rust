# `lib/modules/manager/gitlabci/extract.spec.ts`

[← `manager/gitlabci`](../../../../_by-module/manager/gitlabci.md) · [all modules](../../../../README.md)

**13/14 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 22 | extracts from empty file | ported | `crates/renovate-core/src/extractors/gitlabci.rs:770` |
| 28 | returns null for empty | ported | `crates/renovate-core/src/extractors/gitlabci.rs:1172` |
| 36 | extracts from multidoc yaml | ported | `crates/renovate-core/src/extractors/gitlabci.rs:785` |
| 46 | extracts multiple included image lines | pending | — |
| 57 | extracts named services | ported | `crates/renovate-core/src/extractors/gitlabci.rs:749` |
| 66 | extracts multiple named services | ported | `crates/renovate-core/src/extractors/gitlabci.rs:1178` |
| 75 | extracts multiple image lines | ported | `crates/renovate-core/src/extractors/gitlabci.rs:724` |
| 94 | extracts multiple image lines with comments | ported | `crates/renovate-core/src/extractors/gitlabci.rs:793` |
| 110 | catches errors | ported | `crates/renovate-core/src/extractors/gitlabci.rs:1224` |
| 118 | skips images with variables | ported | `crates/renovate-core/src/extractors/gitlabci.rs:776` |
| 172 | extract images from dependency proxy | ported | `crates/renovate-core/src/extractors/gitlabci.rs:816` |
| 229 | extract images via registry aliases | ported | `crates/renovate-core/src/extractors/gitlabci.rs:838` |
| 299 | extracts component references via registry aliases | ported | `crates/renovate-core/src/extractors/gitlabci.rs:890` |
| 377 | extracts component references | ported | `crates/renovate-core/src/extractors/gitlabci.rs:946` |

