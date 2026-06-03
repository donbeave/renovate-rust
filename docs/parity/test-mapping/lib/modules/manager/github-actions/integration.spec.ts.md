# `lib/modules/manager/github-actions/integration.spec.ts`

[← `manager/github-actions`](../../../../_by-module/manager/github-actions.md) · [all modules](../../../../README.md)

**0/17 ported** (17 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 33 | proposes major update when using tagged major, if a major is available | pending | — |
| 87 | switches major-only version to major.minor if no major is available | pending | — |
| 138 | proposes major and minor updates for tagged major.minor | pending | — |
| 203 | proposes minor update for full semver | pending | — |
| 252 | proposes updates for sha-pinned action with major-only comment | pending | — |
| 312 | proposes updates for sha-pinned action with major.minor comment | pending | — |
| 386 | proposes updates for sha-pinned action with full semver comment | pending | — |
| 458 | proposes minor and major updates for floating minor tag | pending | — |
| 522 | proposes no update for major, when only newer patch/minor releases exist | pending | — |
| 557 | proposes minor+major+digest updates for sha-pinned with floating major comment | pending | — |
| 617 | proposes no update for sha-pinned when only patch version available and digest unchanged | pending | — |
| 652 | preserves floating major tag when newer patch/minor versions exist with full semver | pending | — |
| 702 | preserves floating major tag when only floating minor tags exist | pending | — |
| 733 | migrates floating major tag to major.minor when only floating minor tags exist | pending | — |
| 780 | proposes minor update for floating minor tag without returning less-specific floating major | pending | — |
| 828 | handles multiple deps in one workflow | pending | — |
| 904 | proposes minor and major updates for semver tag | pending | — |

