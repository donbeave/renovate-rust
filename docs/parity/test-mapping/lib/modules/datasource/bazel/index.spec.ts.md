# `lib/modules/datasource/bazel/index.spec.ts`

[← `datasource/bazel`](../../../../_by-module/datasource/bazel.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | throws for error | ported | `crates/renovate-core/src/datasources/bazel.rs:192` |
| 33 | returns null for 404 | ported | `crates/renovate-core/src/datasources/bazel.rs:201` |
| 38 | returns null for empty result | ported | `crates/renovate-core/src/datasources/bazel.rs:218` |
| 43 | returns null for empty 200 ok | ported | `crates/renovate-core/src/datasources/bazel.rs:235` |
| 51 | throws for 5xx | ported | `crates/renovate-core/src/datasources/bazel.rs:255` |
| 58 | metadata without yanked versions | ported | `crates/renovate-core/src/datasources/bazel.rs:270` |
| 77 | metadata with yanked versions | ported | `crates/renovate-core/src/datasources/bazel.rs:299` |
| 106 | should handle local file correctly | ported | `crates/renovate-core/src/datasources/bazel.rs:325` |
| 135 | should return null for invalid file path | ported | `crates/renovate-core/src/datasources/bazel.rs:364` |
| 146 | should return null for empty file content | ported | `crates/renovate-core/src/datasources/bazel.rs:374` |

