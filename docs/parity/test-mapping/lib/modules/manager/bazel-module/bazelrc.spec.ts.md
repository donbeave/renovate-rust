# `lib/modules/manager/bazel-module/bazelrc.spec.ts`

[← `manager/bazel-module`](../../../../_by-module/manager/bazel-module.md) · [all modules](../../../../README.md)

**19/19 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 35 | _(it.each / template — verify manually)_ | ? | — |
| 51 | getoption | ported | `crates/renovate-core/src/extractors/bazel_module.rs:1969` |
| 62 | parse | ported | `crates/renovate-core/src/extractors/bazel_module.rs:1982` |
| 103 | when .bazelrc does not exist | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2117` |
| 110 | when .bazelrc has invalid lines | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2126` |
| 128 | when .bazelrc has no imports | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2147` |
| 148 | when .bazelrc has import and try-import, try-import exists | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2170` |
| 173 | when .bazelrc has import and try-import, try-import does not exist | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2192` |
| 188 | when .bazelrc multi-level import | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2207` |
| 213 | when bazlerc files recursively import each other | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2229` |
| 239 | when .bazelrc refers to a non-local file | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2248` |
| 255 | when bazelrc has %workspace% paths in options | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2260` |
| 274 | when bazelrc has %workspace% paths in imported files | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2277` |
| 304 | should return original value if no workspace path | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2033` |
| 310 | should expand valid workspace path | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2042` |
| 320 | should throw error for invalid workspace path | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2053` |
| 328 | should handle options without values | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2062` |
| 333 | should expand valid workspace paths | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2072` |
| 352 | should throw error for invalid workspace paths | ported | `crates/renovate-core/src/extractors/bazel_module.rs:2093` |

