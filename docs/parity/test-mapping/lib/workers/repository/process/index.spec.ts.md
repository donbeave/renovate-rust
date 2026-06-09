# `lib/workers/repository/process/index.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**1/13 in-scope tests ported** (12 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 28 | processes single branches | ported | [`crates/renovate-core/src/workers/repository/process/index.rs:272`](../../../../../../../crates/renovate-core/src/workers/repository/process/index.rs#L272) |
| 33 | processes basebranchpatterns | pending | — |
| 49 | reads config from default branch if usebasebranchconfig not specified | pending | — |
| 68 | reads config from branches in basebranchpatterns if usebasebranchconfig specified | pending | — |
| 92 | throws if base branch config is invalid | pending | — |
| 107 | handles config name mismatch between basebranches if usebasebranchconfig specified | pending | — |
| 127 | processes basebranchpatterns dryrun extract | pending | — |
| 140 | finds basebranches via regular expressions | pending | — |
| 191 | maps $default to defaultbranch | pending | — |
| 212 | adds base branch name to branchprefix if multiple base branches expected - more than one base branch configured | pending | — |
| 222 | adds base branch name to branchprefix if multiple base branches expected - base branch regex configured | pending | — |
| 232 | does not add base branch name to branchprefix if multiple base branches are not expected - only one base branch configured | pending | — |
| 242 | does not add base branch name to branchprefix if multiple base branches are not expected - basebranchpatterns undefined | pending | — |

