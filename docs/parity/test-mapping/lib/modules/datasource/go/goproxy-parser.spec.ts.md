# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | parses single url | ported | [`crates/renovate-core/src/util.rs:13045`](../../../../../../../crates/renovate-core/src/util.rs#L13045) |
| 15 | parses multiple urls | ported | [`crates/renovate-core/src/util.rs:13054`](../../../../../../../crates/renovate-core/src/util.rs#L13054) |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | [`crates/renovate-core/src/util.rs:13089`](../../../../../../../crates/renovate-core/src/util.rs#L13089) |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | [`crates/renovate-core/src/util.rs:13105`](../../../../../../../crates/renovate-core/src/util.rs#L13105) |
| 68 | matches on real package prefixes | ported | [`crates/renovate-core/src/util.rs:13130`](../../../../../../../crates/renovate-core/src/util.rs#L13130) |
| 100 | matches on wildcards | ported | [`crates/renovate-core/src/util.rs:13165`](../../../../../../../crates/renovate-core/src/util.rs#L13165) |
| 126 | matches on character ranges | ported | [`crates/renovate-core/src/util.rs:13183`](../../../../../../../crates/renovate-core/src/util.rs#L13183) |
| 131 | caches results | pending | — |

