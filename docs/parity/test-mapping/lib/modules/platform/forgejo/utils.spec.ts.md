# `lib/modules/platform/forgejo/utils.spec.ts`

[← `platform/forgejo`](../../../../_by-module/platform/forgejo.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | trimtrailingapipath | ported | [`crates/renovate-core/src/platform/gitea_forgejo_utils.rs:121`](../../../../../../../crates/renovate-core/src/platform/gitea_forgejo_utils.rs#L121) |
| 45 | should abort when endpoint is not valid | ported | [`crates/renovate-core/src/platform/gitea_forgejo_utils.rs:147`](../../../../../../../crates/renovate-core/src/platform/gitea_forgejo_utils.rs#L147) |
| 53 | _(it.each / template — verify manually)_ | ? | — |
| 66 | should return true when repo is usable | ported | [`crates/renovate-core/src/platform/gitea_forgejo_utils.rs:173`](../../../../../../../crates/renovate-core/src/platform/gitea_forgejo_utils.rs#L173) |
| 70 | should return false when repo lacks permissions | ported | [`crates/renovate-core/src/platform/gitea_forgejo_utils.rs:179`](../../../../../../../crates/renovate-core/src/platform/gitea_forgejo_utils.rs#L179) |
| 85 | should return false when repo has disabled pull requests | ported | [`crates/renovate-core/src/platform/gitea_forgejo_utils.rs:202`](../../../../../../../crates/renovate-core/src/platform/gitea_forgejo_utils.rs#L202) |

