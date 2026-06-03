# `lib/modules/datasource/gitlab-tags/index.spec.ts`

[← `datasource/gitlab-tags`](../../../../_by-module/datasource/gitlab-tags.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | returns tags from custom registry | ported | [`crates/renovate-core/src/datasources/gitlab_tags.rs:352`](../../../../../../../crates/renovate-core/src/datasources/gitlab_tags.rs#L352) |
| 38 | returns tags from custom registry in sub path | ported | [`crates/renovate-core/src/datasources/gitlab_tags.rs:383`](../../../../../../../crates/renovate-core/src/datasources/gitlab_tags.rs#L383) |
| 67 | returns tags with default registry | ported | [`crates/renovate-core/src/datasources/gitlab_tags.rs:409`](../../../../../../../crates/renovate-core/src/datasources/gitlab_tags.rs#L409) |
| 83 | returns commits from gitlab installation | ported | [`crates/renovate-core/src/datasources/gitlab_tags.rs:432`](../../../../../../../crates/renovate-core/src/datasources/gitlab_tags.rs#L432) |
| 102 | returns commits from gitlab installation for a specific branch | ported | [`crates/renovate-core/src/datasources/gitlab_tags.rs:451`](../../../../../../../crates/renovate-core/src/datasources/gitlab_tags.rs#L451) |
| 122 | returns null from gitlab installation with no commits | ported | [`crates/renovate-core/src/datasources/gitlab_tags.rs:472`](../../../../../../../crates/renovate-core/src/datasources/gitlab_tags.rs#L472) |
| 135 | returns null from gitlab installation with unknown branch | ported | [`crates/renovate-core/src/datasources/gitlab_tags.rs:489`](../../../../../../../crates/renovate-core/src/datasources/gitlab_tags.rs#L489) |

