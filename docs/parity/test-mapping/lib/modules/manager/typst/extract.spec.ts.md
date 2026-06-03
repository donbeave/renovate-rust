# `lib/modules/manager/typst/extract.spec.ts`

[← `manager/typst`](../../../../_by-module/manager/typst.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | returns empty deps for empty content | ported | `crates/renovate-core/src/extractors/typst.rs:178` |
| 10 | returns empty deps when no imports found | ported | `crates/renovate-core/src/extractors/typst.rs:151` |
| 21 | extracts single import | ported | `crates/renovate-core/src/extractors/typst.rs:91` |
| 36 | extracts multiple imports | ported | `crates/renovate-core/src/extractors/typst.rs:138` |
| 67 | handles imports with different version formats | ported | `crates/renovate-core/src/extractors/typst.rs:184` |
| 98 | strips json comments before parsing | ported | `crates/renovate-core/src/extractors/typst.rs:131` |
| 125 | handles multiple imports on same line | ported | `crates/renovate-core/src/extractors/typst.rs:202` |
| 147 | ignores invalid import formats | ported | `crates/renovate-core/src/extractors/typst.rs:157` |
| 167 | adds skipreason for non-preview namespaces | ported | `crates/renovate-core/src/extractors/typst.rs:113` |

