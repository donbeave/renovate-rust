# `lib/modules/manager/circleci/extract.spec.ts`

[← `manager/circleci`](../../../../_by-module/manager/circleci.md) · [all modules](../../../../README.md)

**9/10 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | returns null for empty | ported | `crates/renovate-core/src/extractors/circleci.rs:469` |
| 16 | handles registry alias | ported | `crates/renovate-core/src/extractors/circleci.rs:475` |
| 48 | extracts multiple image and resolves yaml anchors | ported | `crates/renovate-core/src/extractors/circleci.rs:372` |
| 93 | extracts orbs too | ported | `crates/renovate-core/src/extractors/circleci.rs:515` |
| 200 | extracts image without leading dash | ported | `crates/renovate-core/src/extractors/circleci.rs:438` |
| 226 | extracts and exclude android images | ported | `crates/renovate-core/src/extractors/circleci.rs:461` |
| 237 | extracts orbs without jobs | ported | `crates/renovate-core/src/extractors/circleci.rs:507` |
| 251 | extracts executors | ported | `crates/renovate-core/src/extractors/circleci.rs:563` |
| 273 | extracts orb definitions | ported | `crates/renovate-core/src/extractors/circleci.rs:535` |
| 336 | extracts deps from configs with multiple merge keys per mapping | pending | — |

