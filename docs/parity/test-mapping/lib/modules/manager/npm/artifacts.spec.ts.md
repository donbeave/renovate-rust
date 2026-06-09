# `lib/modules/manager/npm/artifacts.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**10/23 in-scope tests ported** (13 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | returns null if no packagemanager updates present | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:178`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L178) |
| 68 | returns null if currentvalue is undefined | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:327`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L327) |
| 79 | returns null if currentvalue has no hash | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:400`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L400) |
| 90 | returns null if unchanged | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:348`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L348) |
| 105 | returns updated package.json | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:145`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L145) |
| 131 | supports docker mode | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:421`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L421) |
| 180 | supports install mode | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:185`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L185) |
| 221 | catches errors | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:369`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L369) |
| 243 | returns null if no security updates are found | ported | [`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs:428`](../../../../../../../crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs#L428) |
| 254 | returns null if pnpm workspace file does not exist | ported | [`crates/renovate-core/src/extractors/npm.rs:8403`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8403) |
| 274 | returns null if the pnpmshrinkwrap file is not found | pending | — |
| 305 | returns null if no minimumreleaseage setting found | pending | — |
| 326 | returns null if minimumreleaseageexclude excludes all versions of updated dep | pending | — |
| 359 | updates pnpm workspace - adds minimumreleaseageexclude block if not found | pending | — |
| 390 | updates pnpm workspace - appends new minimumreleaseageexclude setting | pending | — |
| 423 | updates pnpm workspace - expands existing minimumreleaseageexclude setting | pending | — |
| 466 | updates pnpm workspace - handles comment with version already present on an inner minimumreleaseageexclude setting | pending | — |
| 497 | updates pnpm workspace - handles comment on an inner minimumreleaseageexclude setting | pending | — |
| 537 | updates pnpm workspace - uses newversion over newvalue in minimumreleaseageexclude | pending | — |
| 573 | handles multiple security upgrades of the same package (at different versions) in a monorepo | pending | — |
| 644 | handles multiple security upgrades of the same package (at the same version) in a monorepo | pending | — |
| 707 | preserves catalog changes in pnpm-workspace.yaml when adding minimumreleaseageexclude | pending | — |
| 747 | handles multiple security upgrades correctly (bug fix test) | pending | — |

