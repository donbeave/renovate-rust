# `lib/modules/manager/gomod/artifacts-extra.spec.ts`

[← `manager/gomod`](../../../../_by-module/manager/gomod.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 34 | detects extra dependencies | ported | `crates/renovate-core/src/extractors/gomod.rs:1899` |
| 55 | generates a table | ported | `crates/renovate-core/src/extractors/gomod.rs:1913` |
| 83 | returns null when one of files is missing | ported | `crates/renovate-core/src/extractors/gomod.rs:1939` |
| 88 | returns null when all dependencies are excluded | ported | `crates/renovate-core/src/extractors/gomod.rs:1946` |
| 94 | returns a notice when there is an extra dependency | ported | `crates/renovate-core/src/extractors/gomod.rs:1954` |
| 117 | returns a notice when there are extra dependencies | ported | `crates/renovate-core/src/extractors/gomod.rs:1974` |
| 141 | adds special notice for updated `go` version | ported | `crates/renovate-core/src/extractors/gomod.rs:1997` |
| 166 | correctly identifies toolchain updates vs go version updates | ported | `crates/renovate-core/src/extractors/gomod.rs:2021` |
| 215 | correctly identifies and distinguishes toolchain updates vs go version updates when both are present | ported | `crates/renovate-core/src/extractors/gomod.rs:2065` |
| 266 | correctly handles the introduction of a toolchain directive by not indicating a change | ported | `crates/renovate-core/src/extractors/gomod.rs:2112` |

