# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | parses single url | ported | `crates/renovate-core/src/util.rs:11313` |
| 15 | parses multiple urls | ported | `crates/renovate-core/src/util.rs:11322` |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | `crates/renovate-core/src/util.rs:11357` |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | `crates/renovate-core/src/util.rs:11373` |
| 68 | matches on real package prefixes | ported | `crates/renovate-core/src/util.rs:11398` |
| 100 | matches on wildcards | ported | `crates/renovate-core/src/util.rs:11433` |
| 126 | matches on character ranges | ported | `crates/renovate-core/src/util.rs:11451` |
| 131 | caches results | pending | — |

