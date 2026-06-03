# `lib/modules/manager/bazel-module/rules-img.spec.ts`

[← `manager/bazel-module`](../../../../_by-module/manager/bazel-module.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | ignores repo rule calls that are not rules_img | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3568`](../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3568) |
| 32 | handles valid rules_img pull call | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3595`](../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3595) |
| 72 | skips repo rule calls without corresponding use_repo_rule | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3628`](../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3628) |
| 91 | skips malformed repo rule calls | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3645`](../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3645) |

