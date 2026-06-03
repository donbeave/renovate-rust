# `lib/modules/manager/bazel-module/extract.spec.ts`

[← `manager/bazel-module`](../../../../_by-module/manager/bazel-module.md) · [all modules](../../../../README.md)

**35/35 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 25 | returns null if fails to parse | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3536` |
| 33 | returns null if something throws an error | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3542` |
| 41 | returns null if file is empty | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3530` |
| 46 | returns null if file has unrecognized declarations | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3550` |
| 54 | returns bazel_dep and git_override dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2579` |
| 95 | returns bazel_dep with no version and git_override | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2630` |
| 125 | returns dependencies and custom registry urls when specified in a bazelrc | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3029` |
| 148 | returns bazel_dep and archive_override dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3325` |
| 179 | returns bazel_dep with no version and archive_override dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3351` |
| 209 | returns bazel_dep and local_path_override dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3377` |
| 238 | returns bazel_dep with no version and local_path_override dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3401` |
| 266 | returns bazel_dep and single_version_override dependencies if a version is specified | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3425` |
| 299 | returns bazel_dep with no version and single_version_override dependencies if a version is specified | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3456` |
| 331 | returns bazel_dep dependency if single_version_override does not have a version | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3483` |
| 355 | returns bazel_dep with no version dependency if single_version_override does not have a version | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3505` |
| 377 | returns crate.spec dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2642` |
| 453 | returns maven.install and maven.artifact dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2697` |
| 507 | returns oci.pull dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2745` |
| 544 | returns oci.pull dependencies without tags | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2770` |
| 578 | returns oci.pull dependencies with tag only (no digest) | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2792` |
| 611 | returns oci.pull dependencies without tag or digest | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2811` |
| 641 | returns oci.pull dependencies with registryaliases | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2829` |
| 682 | returns oci.pull dependencies with registryaliases with multiple segments | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2858` |
| 723 | returns maven.install and bazel_dep dependencies together | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2884` |
| 772 | returns git_repository dependencies with digest | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2929` |
| 796 | returns git_repository dependencies with tag | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2952` |
| 820 | returns new_git_repository dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2972` |
| 846 | handles a real-world module.bazel file (rules_sh) | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2996` |
| 887 | handles every method available in module.bazel files | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3232` |
| 1005 | returns rules_img pull dependencies | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3088` |
| 1051 | returns rules_img pull dependencies with custom registry | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3121` |
| 1086 | returns rules_img pull dependencies with multiple pulls | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3142` |
| 1141 | ignores rules_img pull without required fields | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3175` |
| 1161 | handles rules_img with renamed variable | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3194` |
| 1193 | ignores non-rules_img repo rules | ported | `crates/renovate-core/src/extractors/bazel_module.rs:3212` |

