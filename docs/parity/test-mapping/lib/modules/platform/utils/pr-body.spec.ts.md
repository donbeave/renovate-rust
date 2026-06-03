# `lib/modules/platform/utils/pr-body.spec.ts`

[← `platform/utils`](../../../../_by-module/platform/utils.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | truncates to 1000 | ported | [`crates/renovate-core/src/platform/pr_body.rs:249`](../../../../../../../crates/renovate-core/src/platform/pr_body.rs#L249) |
| 18 | truncates to 300 not smart | ported | [`crates/renovate-core/src/platform/pr_body.rs:257`](../../../../../../../crates/renovate-core/src/platform/pr_body.rs#L257) |
| 27 | includes truncation notice at end of truncated content (when "not smart") | ported | [`crates/renovate-core/src/platform/pr_body.rs:264`](../../../../../../../crates/renovate-core/src/platform/pr_body.rs#L264) |
| 33 | includes truncation notice before configuration section (when "smart") | ported | [`crates/renovate-core/src/platform/pr_body.rs:272`](../../../../../../../crates/renovate-core/src/platform/pr_body.rs#L272) |
| 43 | truncates content without release notes structure when notice fits | ported | [`crates/renovate-core/src/platform/pr_body.rs:284`](../../../../../../../crates/renovate-core/src/platform/pr_body.rs#L284) |
| 49 | truncates to below notice length with release notes structure | ported | [`crates/renovate-core/src/platform/pr_body.rs:292`](../../../../../../../crates/renovate-core/src/platform/pr_body.rs#L292) |
| 55 | truncates to 10 | ported | [`crates/renovate-core/src/platform/pr_body.rs:300`](../../../../../../../crates/renovate-core/src/platform/pr_body.rs#L300) |
| 63 | does not truncate | ported | [`crates/renovate-core/src/platform/pr_body.rs:307`](../../../../../../../crates/renovate-core/src/platform/pr_body.rs#L307) |

