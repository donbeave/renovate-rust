# `lib/modules/manager/homebrew/update.spec.ts`

[← `manager/homebrew`](../../../../_by-module/manager/homebrew.md) · [all modules](../../../../README.md)

**7/19 ported** (12 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | updates "releases" github dependency | pending | — |
| 49 | updates "archive" github dependency | pending | — |
| 86 | updates "archive" github dependency from old url format | pending | — |
| 132 | returns unchanged content if fromstream promise rejects | pending | — |
| 165 | returns unchanged content if url field in upgrade object is invalid | ported | [`crates/renovate-core/src/extractors/homebrew.rs:1311`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L1311) |
| 190 | returns unchanged content if reponame in upgrade object is invalid | pending | — |
| 215 | returns unchanged content if reponame in upgrade object is wrong | pending | — |
| 240 | returns unchanged content if url field in formula file is invalid | ported | [`crates/renovate-core/src/extractors/homebrew.rs:1329`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L1329) |
| 280 | returns unchanged content if url field in formula file is missing | ported | [`crates/renovate-core/src/extractors/homebrew.rs:1348`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L1348) |
| 319 | returns unchanged content if sha256 field in formula file is invalid | ported | [`crates/renovate-core/src/extractors/homebrew.rs:1366`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L1366) |
| 359 | returns unchanged content if sha256 field in formula file is missing | ported | [`crates/renovate-core/src/extractors/homebrew.rs:1380`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L1380) |
| 398 | returns unchanged content if both got requests fail | pending | — |
| 429 | returns unchanged content if managerdata is missing required fields | pending | — |
| 452 | returns unchanged content for unknown handler type | ported | [`crates/renovate-core/src/extractors/homebrew.rs:1394`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L1394) |
| 476 | returns unchanged content if newvalue is missing | ported | [`crates/renovate-core/src/extractors/homebrew.rs:1412`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L1412) |
| 500 | returns unchanged content if handler buildarchiveurls returns null | pending | — |
| 542 | updates npm scoped package dependency | pending | — |
| 586 | updates npm unscoped package dependency | pending | — |
| 630 | returns unchanged content if npm tarball download fails | pending | — |

