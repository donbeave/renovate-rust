# `lib/modules/manager/git-submodules/extract.spec.ts`

[← `manager/git-submodules`](../../../../_by-module/manager/git-submodules.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 46 | empty submodule returns null | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:339`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L339) |
| 50 | currentvalue is unset when no branch is specified | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:345`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L345) |
| 56 | given branch is used when branch is specified | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:361`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L361) |
| 62 | submodule packagename is constructed from relative path | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:413`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L413) |
| 71 | when using ssh clone url | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:535`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L535) |
| 78 | when using a relative path | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:464`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L464) |
| 87 | fallback to current branch if special value is detected | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:374`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L374) |
| 125 | given semver version is extracted from branch and versioning is set to semver | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:479`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L479) |

