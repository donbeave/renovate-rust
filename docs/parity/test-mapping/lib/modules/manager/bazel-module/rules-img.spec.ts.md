# `lib/modules/manager/bazel-module/rules-img.spec.ts`

[← `manager/bazel-module`](../../../../_by-module/manager/bazel-module.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | ignores repo rule calls that are not rules_img | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3568` |
| 32 | handles valid rules_img pull call | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3595` |
| 72 | skips repo rule calls without corresponding use_repo_rule | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3628` |
| 91 | skips malformed repo rule calls | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3645` |

