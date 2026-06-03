# `lib/modules/manager/droneci/extract.spec.ts`

[← `manager/droneci`](../../../../_by-module/manager/droneci.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns null for empty | ported | `crates/renovate-core/src/extractors/droneci.rs:243` |
| 12 | extracts multiple image lines | ported | `crates/renovate-core/src/extractors/droneci.rs:174` |
| 19 | extracts image and replaces registry | ported | `crates/renovate-core/src/extractors/droneci.rs:288` |
| 42 | extracts image but no replacement | ported | `crates/renovate-core/src/extractors/droneci.rs:234` |
| 65 | extracts image and no double replacement | ported | `crates/renovate-core/src/extractors/droneci.rs:327` |

