# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | parses single url | ported | [`crates/renovate-core/src/util.rs:13134`](../../../../../../../crates/renovate-core/src/util.rs#L13134) |
| 15 | parses multiple urls | ported | [`crates/renovate-core/src/util.rs:13143`](../../../../../../../crates/renovate-core/src/util.rs#L13143) |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | [`crates/renovate-core/src/util.rs:13178`](../../../../../../../crates/renovate-core/src/util.rs#L13178) |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | [`crates/renovate-core/src/util.rs:13194`](../../../../../../../crates/renovate-core/src/util.rs#L13194) |
| 68 | matches on real package prefixes | ported | [`crates/renovate-core/src/util.rs:13219`](../../../../../../../crates/renovate-core/src/util.rs#L13219) |
| 100 | matches on wildcards | ported | [`crates/renovate-core/src/util.rs:13254`](../../../../../../../crates/renovate-core/src/util.rs#L13254) |
| 126 | matches on character ranges | ported | [`crates/renovate-core/src/util.rs:13272`](../../../../../../../crates/renovate-core/src/util.rs#L13272) |
| 131 | caches results | pending | — |

