# `lib/modules/datasource/gitlab-packages/index.spec.ts`

[← `datasource/gitlab-packages`](../../../../_by-module/datasource/gitlab-packages.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns package from custom registry | ported | [`crates/renovate-core/src/datasources/gitlab_packages.rs:132`](../../../../../../../crates/renovate-core/src/datasources/gitlab_packages.rs#L132) |
| 48 | returns conan package from custom registry | ported | [`crates/renovate-core/src/datasources/gitlab_packages.rs:173`](../../../../../../../crates/renovate-core/src/datasources/gitlab_packages.rs#L173) |
| 85 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/gitlab_packages.rs:217`](../../../../../../../crates/renovate-core/src/datasources/gitlab_packages.rs#L217) |
| 103 | returns null for empty 200 ok | ported | [`crates/renovate-core/src/datasources/gitlab_packages.rs:206`](../../../../../../../crates/renovate-core/src/datasources/gitlab_packages.rs#L206) |
| 121 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/gitlab_packages.rs:237`](../../../../../../../crates/renovate-core/src/datasources/gitlab_packages.rs#L237) |

