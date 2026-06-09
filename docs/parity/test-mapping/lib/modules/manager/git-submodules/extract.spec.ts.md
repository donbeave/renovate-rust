# `lib/modules/manager/git-submodules/extract.spec.ts`

[← `manager/git-submodules`](../../../../_by-module/manager/git-submodules.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | empty submodule returns null | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:365`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L365) |
| 50 | currentvalue is unset when no branch is specified | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:371`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L371) |
| 56 | given branch is used when branch is specified | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:387`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L387) |
| 62 | submodule packagename is constructed from relative path | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:439`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L439) |
| 71 | when using ssh clone url | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:561`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L561) |
| 78 | when using a relative path | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:490`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L490) |
| 87 | fallback to current branch if special value is detected | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:400`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L400) |
| 125 | given semver version is extracted from branch and versioning is set to semver | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:505`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L505) |

