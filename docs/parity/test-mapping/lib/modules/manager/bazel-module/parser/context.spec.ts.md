# `lib/modules/manager/bazel-module/parser/context.spec.ts`

[← `manager/bazel-module`](../../../../../_by-module/manager/bazel-module.md) · [all modules](../../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | throws if there is no current | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2450`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2450) |
| 13 | throws if the current is not a prepared extension tag | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2460`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2460) |
| 23 | throws if the current is not an extension tag | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2473`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2473) |
| 30 | throws on missing current | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2486`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2486) |
| 37 | throws on unbalanced endrule | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2496`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2496) |
| 44 | throws on unbalanced endarray | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2510`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2510) |
| 51 | throws if add an attribute without a parent | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2524`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2524) |
| 60 | throws if current use repo rule does not exist | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2538`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2538) |
| 67 | throws if current repo rule call does not exist | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2551`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2551) |
| 74 | creates ctxprocessingerror with parent type | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:2564`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L2564) |

