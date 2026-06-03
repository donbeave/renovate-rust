# `lib/modules/manager/bazel-module/parser/index.spec.ts`

[← `manager/bazel-module`](../../../../../_by-module/manager/bazel-module.md) · [all modules](../../../../../README.md)

**12/12 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | returns empty string if invalid content | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3699`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3699) |
| 17 | finds simple bazel_dep | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3707`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3707) |
| 44 | finds the git_override | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3718`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3718) |
| 85 | finds archive_override | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3733`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3733) |
| 119 | finds local_path_override | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3749`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3749) |
| 148 | finds single_version_override | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3765`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3765) |
| 179 | finds maven.artifact | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3782`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3782) |
| 248 | finds maven.install and maven.artifact | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3798`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3798) |
| 335 | finds oci.pull | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3826`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3826) |
| 376 | finds the git_repository | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3843`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3843) |
| 408 | finds use_repo_rule and repo rule call | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3856`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3856) |
| 420 | ignores use_repo_rule with insufficient args | ported | [`crates/renovate-core/src/extractors/bazel_module.rs:3871`](../../../../../../../../crates/renovate-core/src/extractors/bazel_module.rs#L3871) |

