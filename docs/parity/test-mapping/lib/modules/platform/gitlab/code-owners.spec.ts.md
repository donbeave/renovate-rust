# `lib/modules/platform/gitlab/code-owners.spec.ts`

[← `platform/gitlab`](../../../../_by-module/platform/gitlab.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | should extract an owner rule from a line | ported | [`crates/renovate-core/src/platform/gitlab.rs:2862`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2862) |
| 20 | should extract an owner rule from a line with no usernames | ported | [`crates/renovate-core/src/platform/gitlab.rs:2872`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2872) |
| 33 | should extract an owner rule from a line after a section header | ported | [`crates/renovate-core/src/platform/gitlab.rs:2882`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2882) |
| 47 | should extract an owner rule from a line after a section header with no usernames | ported | [`crates/renovate-core/src/platform/gitlab.rs:2893`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2893) |
| 61 | should extract an owner rule from a line after a section header with spaces | ported | [`crates/renovate-core/src/platform/gitlab.rs:2903`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2903) |
| 75 | should extract an owner rule from a line after a section header with spaces and no usernames | ported | [`crates/renovate-core/src/platform/gitlab.rs:2914`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2914) |
| 89 | should extract an owner rule from a line after a section header with spaces and multiple usernames | ported | [`crates/renovate-core/src/platform/gitlab.rs:2924`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2924) |
| 103 | should extract an owner rule from a line after an optional section header with spaces | ported | [`crates/renovate-core/src/platform/gitlab.rs:2970`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2970) |
| 117 | should extract an owner rule from a line after a section header with approval count and spaces | ported | [`crates/renovate-core/src/platform/gitlab.rs:2981`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2981) |

