# `lib/modules/platform/gitlab/pr-cache.spec.ts`

[← `platform/gitlab`](../../../../_by-module/platform/gitlab.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 81 | fetches cache initially | ported | [`crates/renovate-core/src/platform/gitlab.rs:1727`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1727) |
| 110 | fetches cache with ignoreprauthor=true | ported | [`crates/renovate-core/src/platform/gitlab.rs:1777`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1777) |
| 128 | resets cache for not matching authors | ported | [`crates/renovate-core/src/platform/gitlab.rs:1862`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1862) |
| 169 | resets cache for older format with milliseconds | ported | [`crates/renovate-core/src/platform/gitlab.rs:1920`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1920) |
| 210 | syncs cache with updated_after parameter | ported | [`crates/renovate-core/src/platform/gitlab.rs:1973`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1973) |
| 251 | handles empty response | ported | [`crates/renovate-core/src/platform/gitlab.rs:2344`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2344) |
| 267 | returns items in reverse order (most recent first) | ported | [`crates/renovate-core/src/platform/gitlab.rs:1809`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1809) |
| 280 | normalizes timestamps by removing milliseconds | ported | [`crates/renovate-core/src/platform/gitlab.rs:2029`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2029) |

