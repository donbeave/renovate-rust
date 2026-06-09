# `lib/modules/platform/gitlab/pr-cache.spec.ts`

[← `platform/gitlab`](../../../../_by-module/platform/gitlab.md) · [all modules](../../../../README.md)

**4/8 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 81 | fetches cache initially | ported | [`crates/renovate-core/src/platform/gitlab.rs:1616`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1616) |
| 110 | fetches cache with ignoreprauthor=true | ported | [`crates/renovate-core/src/platform/gitlab.rs:1664`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1664) |
| 128 | resets cache for not matching authors | pending | — |
| 169 | resets cache for older format with milliseconds | pending | — |
| 210 | syncs cache with updated_after parameter | pending | — |
| 251 | handles empty response | ported | [`crates/renovate-core/src/platform/gitlab.rs:2026`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2026) |
| 267 | returns items in reverse order (most recent first) | ported | [`crates/renovate-core/src/platform/gitlab.rs:1695`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1695) |
| 280 | normalizes timestamps by removing milliseconds | pending | — |

