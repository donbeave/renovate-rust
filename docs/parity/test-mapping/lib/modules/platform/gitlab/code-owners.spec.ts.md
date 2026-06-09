# `lib/modules/platform/gitlab/code-owners.spec.ts`

[← `platform/gitlab`](../../../../_by-module/platform/gitlab.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | should extract an owner rule from a line | ported | [`crates/renovate-core/src/platform/gitlab.rs:2543`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2543) |
| 20 | should extract an owner rule from a line with no usernames | ported | [`crates/renovate-core/src/platform/gitlab.rs:2553`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2553) |
| 33 | should extract an owner rule from a line after a section header | ported | [`crates/renovate-core/src/platform/gitlab.rs:2563`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2563) |
| 47 | should extract an owner rule from a line after a section header with no usernames | ported | [`crates/renovate-core/src/platform/gitlab.rs:2574`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2574) |
| 61 | should extract an owner rule from a line after a section header with spaces | ported | [`crates/renovate-core/src/platform/gitlab.rs:2584`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2584) |
| 75 | should extract an owner rule from a line after a section header with spaces and no usernames | ported | [`crates/renovate-core/src/platform/gitlab.rs:2595`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2595) |
| 89 | should extract an owner rule from a line after a section header with spaces and multiple usernames | ported | [`crates/renovate-core/src/platform/gitlab.rs:2605`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2605) |
| 103 | should extract an owner rule from a line after an optional section header with spaces | ported | [`crates/renovate-core/src/platform/gitlab.rs:2651`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2651) |
| 117 | should extract an owner rule from a line after a section header with approval count and spaces | ported | [`crates/renovate-core/src/platform/gitlab.rs:2662`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2662) |

