# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | parses single url | ported | [`crates/renovate-core/src/util.rs:13037`](../../../../../../../crates/renovate-core/src/util.rs#L13037) |
| 15 | parses multiple urls | ported | [`crates/renovate-core/src/util.rs:13046`](../../../../../../../crates/renovate-core/src/util.rs#L13046) |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | [`crates/renovate-core/src/util.rs:13081`](../../../../../../../crates/renovate-core/src/util.rs#L13081) |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | [`crates/renovate-core/src/util.rs:13097`](../../../../../../../crates/renovate-core/src/util.rs#L13097) |
| 68 | matches on real package prefixes | ported | [`crates/renovate-core/src/util.rs:13122`](../../../../../../../crates/renovate-core/src/util.rs#L13122) |
| 100 | matches on wildcards | ported | [`crates/renovate-core/src/util.rs:13157`](../../../../../../../crates/renovate-core/src/util.rs#L13157) |
| 126 | matches on character ranges | ported | [`crates/renovate-core/src/util.rs:13175`](../../../../../../../crates/renovate-core/src/util.rs#L13175) |
| 131 | caches results | pending | — |

