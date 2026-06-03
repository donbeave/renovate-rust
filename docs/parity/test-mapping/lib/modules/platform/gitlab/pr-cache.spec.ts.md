# `lib/modules/platform/gitlab/pr-cache.spec.ts`

[← `platform/gitlab`](../../../../_by-module/platform/gitlab.md) · [all modules](../../../../README.md)

**3/8 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 81 | fetches cache initially | ported | [`crates/renovate-core/src/platform/gitlab.rs:1612`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1612) |
| 110 | fetches cache with ignoreprauthor=true | ported | [`crates/renovate-core/src/platform/gitlab.rs:1660`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1660) |
| 128 | resets cache for not matching authors | pending | — |
| 169 | resets cache for older format with milliseconds | pending | — |
| 210 | syncs cache with updated_after parameter | pending | — |
| 251 | handles empty response | ported | [`crates/renovate-core/src/platform/gitlab.rs:1970`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1970) |
| 267 | returns items in reverse order (most recent first) | pending | — |
| 280 | normalizes timestamps by removing milliseconds | pending | — |

