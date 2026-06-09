# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | parses single url | ported | [`crates/renovate-core/src/util.rs:13038`](../../../../../../../crates/renovate-core/src/util.rs#L13038) |
| 15 | parses multiple urls | ported | [`crates/renovate-core/src/util.rs:13047`](../../../../../../../crates/renovate-core/src/util.rs#L13047) |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | [`crates/renovate-core/src/util.rs:13082`](../../../../../../../crates/renovate-core/src/util.rs#L13082) |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | [`crates/renovate-core/src/util.rs:13098`](../../../../../../../crates/renovate-core/src/util.rs#L13098) |
| 68 | matches on real package prefixes | ported | [`crates/renovate-core/src/util.rs:13123`](../../../../../../../crates/renovate-core/src/util.rs#L13123) |
| 100 | matches on wildcards | ported | [`crates/renovate-core/src/util.rs:13158`](../../../../../../../crates/renovate-core/src/util.rs#L13158) |
| 126 | matches on character ranges | ported | [`crates/renovate-core/src/util.rs:13176`](../../../../../../../crates/renovate-core/src/util.rs#L13176) |
| 131 | caches results | pending | — |

