# `lib/modules/manager/bazel/extract.spec.ts`

[← `manager/bazel`](../../../../_by-module/manager/bazel.md) · [all modules](../../../../README.md)

**12/12 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | returns empty if fails to parse | ported | [`crates/renovate-core/src/extractors/bazel.rs:507`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L507) |
| 15 | returns empty if cannot parse dependency | ported | [`crates/renovate-core/src/extractors/bazel.rs:534`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L534) |
| 20 | returns empty for incomplete dependency | ported | [`crates/renovate-core/src/extractors/bazel.rs:655`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L655) |
| 25 | extracts multiple types of dependencies | ported | [`crates/renovate-core/src/extractors/bazel.rs:781`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L781) |
| 31 | extracts github tags | ported | [`crates/renovate-core/src/extractors/bazel.rs:443`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L443) |
| 42 | handle comments and strings | ported | [`crates/renovate-core/src/extractors/bazel.rs:622`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L622) |
| 47 | extracts dependencies from *.bzl files | ported | [`crates/renovate-core/src/extractors/bazel.rs:851`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L851) |
| 65 | extracts dependencies for container_pull deptype | ported | [`crates/renovate-core/src/extractors/bazel.rs:664`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L664) |
| 90 | extracts dependencies for oci_pull deptype | ported | [`crates/renovate-core/src/extractors/bazel.rs:698`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L698) |
| 113 | check remote option in go_repository | ported | [`crates/renovate-core/src/extractors/bazel.rs:730`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L730) |
| 166 | sequential http_archive | ported | [`crates/renovate-core/src/extractors/bazel.rs:487`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L487) |
| 190 | http_archive with gitlab url | ported | [`crates/renovate-core/src/extractors/bazel.rs:566`](../../../../../../../crates/renovate-core/src/extractors/bazel.rs#L566) |

