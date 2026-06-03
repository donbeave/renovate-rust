# `lib/modules/platform/github/issue.spec.ts`

[← `platform/github`](../../../../_by-module/platform/github.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | returns null for empty cache | ported | `crates/renovate-core/src/platform/github_api_cache.rs:485` |
| 20 | stores issues to the cache | ported | `crates/renovate-core/src/platform/github_api_cache.rs:492` |
| 64 | returns issues from the cache in the correct order | ported | `crates/renovate-core/src/platform/github_api_cache.rs:506` |
| 120 | updates particular issue in the cache | ported | `crates/renovate-core/src/platform/github_api_cache.rs:521` |
| 162 | removes particular issue from the cache | ported | `crates/renovate-core/src/platform/github_api_cache.rs:545` |
| 188 | reconciles cache | ported | `crates/renovate-core/src/platform/github_api_cache.rs:560` |
| 246 | resets cache during failed reconciliation | ported | `crates/renovate-core/src/platform/github_api_cache.rs:585` |

