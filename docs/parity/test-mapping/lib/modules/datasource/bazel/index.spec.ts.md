# `lib/modules/datasource/bazel/index.spec.ts`

[← `datasource/bazel`](../../../../_by-module/datasource/bazel.md) · [all modules](../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | throws for error | ported | [`crates/renovate-core/src/datasources/bazel.rs:192`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L192) |
| 33 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/bazel.rs:201`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L201) |
| 38 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/bazel.rs:218`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L218) |
| 43 | returns null for empty 200 ok | ported | [`crates/renovate-core/src/datasources/bazel.rs:235`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L235) |
| 51 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/bazel.rs:255`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L255) |
| 58 | metadata without yanked versions | ported | [`crates/renovate-core/src/datasources/bazel.rs:270`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L270) |
| 77 | metadata with yanked versions | ported | [`crates/renovate-core/src/datasources/bazel.rs:299`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L299) |
| 106 | should handle local file correctly | ported | [`crates/renovate-core/src/datasources/bazel.rs:325`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L325) |
| 135 | should return null for invalid file path | ported | [`crates/renovate-core/src/datasources/bazel.rs:364`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L364) |
| 146 | should return null for empty file content | ported | [`crates/renovate-core/src/datasources/bazel.rs:374`](../../../../../../../crates/renovate-core/src/datasources/bazel.rs#L374) |

