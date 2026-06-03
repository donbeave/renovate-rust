# `lib/modules/platform/gitlab/code-owners.spec.ts`

[← `platform/gitlab`](../../../../_by-module/platform/gitlab.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | should extract an owner rule from a line | ported | `crates/renovate-core/src/platform/gitlab.rs:2487` |
| 20 | should extract an owner rule from a line with no usernames | ported | `crates/renovate-core/src/platform/gitlab.rs:2497` |
| 33 | should extract an owner rule from a line after a section header | ported | `crates/renovate-core/src/platform/gitlab.rs:2507` |
| 47 | should extract an owner rule from a line after a section header with no usernames | ported | `crates/renovate-core/src/platform/gitlab.rs:2518` |
| 61 | should extract an owner rule from a line after a section header with spaces | ported | `crates/renovate-core/src/platform/gitlab.rs:2528` |
| 75 | should extract an owner rule from a line after a section header with spaces and no usernames | ported | `crates/renovate-core/src/platform/gitlab.rs:2539` |
| 89 | should extract an owner rule from a line after a section header with spaces and multiple usernames | ported | `crates/renovate-core/src/platform/gitlab.rs:2549` |
| 103 | should extract an owner rule from a line after an optional section header with spaces | ported | `crates/renovate-core/src/platform/gitlab.rs:2595` |
| 117 | should extract an owner rule from a line after a section header with approval count and spaces | ported | `crates/renovate-core/src/platform/gitlab.rs:2606` |

