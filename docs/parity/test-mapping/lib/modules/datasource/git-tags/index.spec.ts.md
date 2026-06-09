# `lib/modules/datasource/git-tags/index.spec.ts`

[← `datasource/git-tags`](../../../../_by-module/datasource/git-tags.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns nil if response is wrong | ported | [`crates/renovate-core/src/datasources/git_tags.rs:84`](../../../../../../../crates/renovate-core/src/datasources/git_tags.rs#L84) |
| 45 | returns nil if remote call throws exception | ported | [`crates/renovate-core/src/datasources/git_tags.rs:91`](../../../../../../../crates/renovate-core/src/datasources/git_tags.rs#L91) |
| 52 | returns versions filtered from tags | ported | [`crates/renovate-core/src/datasources/git_tags.rs:98`](../../../../../../../crates/renovate-core/src/datasources/git_tags.rs#L98) |
| 64 | returns null if not found | ported | [`crates/renovate-core/src/datasources/git_tags.rs:135`](../../../../../../../crates/renovate-core/src/datasources/git_tags.rs#L135) |
| 74 | returns digest for tag | ported | [`crates/renovate-core/src/datasources/git_tags.rs:142`](../../../../../../../crates/renovate-core/src/datasources/git_tags.rs#L142) |
| 84 | returns digest for head | ported | [`crates/renovate-core/src/datasources/git_tags.rs:149`](../../../../../../../crates/renovate-core/src/datasources/git_tags.rs#L149) |
| 94 | returns digest for head with authentication environment variables | opt-out | mocks TypeScript-internal simpleGit infrastructure; no equivalent Rust mock surface |
| 121 | returns digest for head with authentication environment variables for datasource type git-tags | ported | [`crates/renovate-core/src/util.rs:9387`](../../../../../../../crates/renovate-core/src/util.rs#L9387) |

