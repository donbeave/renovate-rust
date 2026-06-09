# `lib/modules/manager/git-submodules/update.spec.ts`

[← `manager/git-submodules`](../../../../_by-module/manager/git-submodules.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 49 | returns null on error | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:774`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L774) |
| 60 | returns content on update | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:633`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L633) |
| 72 | returns content on update and uses git environment variables | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:651`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L651) |
| 107 | update gitmodule branch value if value changed | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:578`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L578) |
| 136 | do not update gitmodule branch value if value not changed | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:599`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L599) |
| 154 | returns content on update and uses git environment variables for git-tags/git-refs | ported | [`crates/renovate-core/src/extractors/git_submodules.rs:701`](../../../../../../../crates/renovate-core/src/extractors/git_submodules.rs#L701) |

