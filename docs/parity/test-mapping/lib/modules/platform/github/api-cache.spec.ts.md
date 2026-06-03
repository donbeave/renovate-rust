# `lib/modules/platform/github/api-cache.spec.ts`

[← `platform/github`](../../../../_by-module/platform/github.md) · [all modules](../../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | stores and retrieves items | ported | `crates/renovate-core/src/platform/github_api_cache.rs:277` |
| 29 | maps items | ported | `crates/renovate-core/src/platform/github_api_cache.rs:299` |
| 46 | resets cache on item update | ported | `crates/renovate-core/src/platform/github_api_cache.rs:311` |
| 69 | resets cache on page reconcile | ported | `crates/renovate-core/src/platform/github_api_cache.rs:326` |
| 94 | returns undefined when no lastmodified in cache | ported | `crates/renovate-core/src/platform/github_api_cache.rs:339` |
| 100 | returns stored value when present | ported | `crates/renovate-core/src/platform/github_api_cache.rs:346` |
| 106 | returns updated value after reconcile | ported | `crates/renovate-core/src/platform/github_api_cache.rs:356` |
| 116 | sets lastmodified when not present | ported | `crates/renovate-core/src/platform/github_api_cache.rs:367` |
| 124 | advances lastmodified to newer timestamp | ported | `crates/renovate-core/src/platform/github_api_cache.rs:378` |
| 132 | does not regress lastmodified to older timestamp | ported | `crates/renovate-core/src/platform/github_api_cache.rs:389` |
| 142 | returns false for empty page | ported | `crates/renovate-core/src/platform/github_api_cache.rs:400` |
| 152 | appends new items | ported | `crates/renovate-core/src/platform/github_api_cache.rs:407` |
| 175 | handles updated items | ported | `crates/renovate-core/src/platform/github_api_cache.rs:422` |
| 199 | ignores page overlap | ported | `crates/renovate-core/src/platform/github_api_cache.rs:440` |
| 226 | does not require new page if all items are old | ported | `crates/renovate-core/src/platform/github_api_cache.rs:459` |

