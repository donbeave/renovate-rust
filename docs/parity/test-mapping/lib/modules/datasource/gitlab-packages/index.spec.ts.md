# `lib/modules/datasource/gitlab-packages/index.spec.ts`

[← `datasource/gitlab-packages`](../../../../_by-module/datasource/gitlab-packages.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns package from custom registry | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs:132` |
| 48 | returns conan package from custom registry | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs:173` |
| 85 | returns null for 404 | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs:217` |
| 103 | returns null for empty 200 ok | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs:206` |
| 121 | throws for 5xx | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs:237` |

