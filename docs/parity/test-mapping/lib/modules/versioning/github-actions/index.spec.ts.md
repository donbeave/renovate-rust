# `lib/modules/versioning/github-actions/index.spec.ts`

[← `versioning/github-actions`](../../../../_by-module/versioning/github-actions.md) · [all modules](../../../../README.md)

**29/29 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | _(it.each / template — verify manually)_ | ? | — |
| 30 | _(it.each / template — verify manually)_ | ? | — |
| 54 | _(it.each / template — verify manually)_ | ? | — |
| 81 | _(it.each / template — verify manually)_ | ? | — |
| 99 | _(it.each / template — verify manually)_ | ? | — |
| 158 | should not handle invalid range that is not ~latest or valid version | ported | [`crates/renovate-core/src/versioning/github_actions.rs:479`](../../../../../../../crates/renovate-core/src/versioning/github_actions.rs#L479) |
| 166 | _(it.each / template — verify manually)_ | ? | — |
| 202 | _(it.each / template — verify manually)_ | ? | — |
| 226 | _(it.each / template — verify manually)_ | ? | — |
| 260 | _(it.each / template — verify manually)_ | ? | — |
| 287 | _(it.each / template — verify manually)_ | ? | — |
| 302 | _(it.each / template — verify manually)_ | ? | — |
| 316 | _(it.each / template — verify manually)_ | ? | — |
| 330 | _(it.each / template — verify manually)_ | ? | — |
| 364 | _(it.each / template — verify manually)_ | ? | — |
| 394 | _(it.each / template — verify manually)_ | ? | — |
| 422 | _(it.each / template — verify manually)_ | ? | — |
| 436 | _(it.each / template — verify manually)_ | ? | — |
| 502 | does not determine if the proposed newversion exists, if allversions is not set | ported | [`crates/renovate-core/src/versioning/github_actions.rs:701`](../../../../../../../crates/renovate-core/src/versioning/github_actions.rs#L701) |
| 514 | _(it.each / template — verify manually)_ | ? | — |
| 532 | _(it.each / template — verify manually)_ | ? | — |
| 562 | _(it.each / template — verify manually)_ | ? | — |
| 614 | migrates from a floating major to a floating major.minor if the floating major no longer exists | ported | [`crates/renovate-core/src/versioning/github_actions.rs:770`](../../../../../../../crates/renovate-core/src/versioning/github_actions.rs#L770) |
| 625 | _(it.each / template — verify manually)_ | ? | — |
| 658 | when a release candidate version exists, that exact version is used | ported | [`crates/renovate-core/src/versioning/github_actions.rs:802`](../../../../../../../crates/renovate-core/src/versioning/github_actions.rs#L802) |
| 675 | returns newversion when newversion is a floating tag and allversions is not set | ported | [`crates/renovate-core/src/versioning/github_actions.rs:818`](../../../../../../../crates/renovate-core/src/versioning/github_actions.rs#L818) |
| 685 | returns the floating newversion when it exists in allversions | ported | [`crates/renovate-core/src/versioning/github_actions.rs:834`](../../../../../../../crates/renovate-core/src/versioning/github_actions.rs#L834) |
| 698 | newversion is returned anyway | ported | [`crates/renovate-core/src/versioning/github_actions.rs:850`](../../../../../../../crates/renovate-core/src/versioning/github_actions.rs#L850) |
| 709 | debug logs | ported | [`crates/renovate-core/src/versioning/github_actions.rs:866`](../../../../../../../crates/renovate-core/src/versioning/github_actions.rs#L866) |

