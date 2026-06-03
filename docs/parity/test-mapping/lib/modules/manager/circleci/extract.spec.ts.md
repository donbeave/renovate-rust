# `lib/modules/manager/circleci/extract.spec.ts`

[← `manager/circleci`](../../../../_by-module/manager/circleci.md) · [all modules](../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns null for empty | ported | [`crates/renovate-core/src/extractors/circleci.rs:469`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L469) |
| 16 | handles registry alias | ported | [`crates/renovate-core/src/extractors/circleci.rs:475`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L475) |
| 48 | extracts multiple image and resolves yaml anchors | ported | [`crates/renovate-core/src/extractors/circleci.rs:372`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L372) |
| 93 | extracts orbs too | ported | [`crates/renovate-core/src/extractors/circleci.rs:515`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L515) |
| 200 | extracts image without leading dash | ported | [`crates/renovate-core/src/extractors/circleci.rs:438`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L438) |
| 226 | extracts and exclude android images | ported | [`crates/renovate-core/src/extractors/circleci.rs:461`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L461) |
| 237 | extracts orbs without jobs | ported | [`crates/renovate-core/src/extractors/circleci.rs:507`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L507) |
| 251 | extracts executors | ported | [`crates/renovate-core/src/extractors/circleci.rs:563`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L563) |
| 273 | extracts orb definitions | ported | [`crates/renovate-core/src/extractors/circleci.rs:535`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L535) |
| 336 | extracts deps from configs with multiple merge keys per mapping | ported | [`crates/renovate-core/src/extractors/circleci.rs:713`](../../../../../../../crates/renovate-core/src/extractors/circleci.rs#L713) |

