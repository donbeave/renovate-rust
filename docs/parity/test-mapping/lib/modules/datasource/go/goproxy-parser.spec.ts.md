# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | parses single url | ported | [`crates/renovate-core/src/util.rs:13036`](../../../../../../../crates/renovate-core/src/util.rs#L13036) |
| 15 | parses multiple urls | ported | [`crates/renovate-core/src/util.rs:13045`](../../../../../../../crates/renovate-core/src/util.rs#L13045) |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | [`crates/renovate-core/src/util.rs:13080`](../../../../../../../crates/renovate-core/src/util.rs#L13080) |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | [`crates/renovate-core/src/util.rs:13096`](../../../../../../../crates/renovate-core/src/util.rs#L13096) |
| 68 | matches on real package prefixes | ported | [`crates/renovate-core/src/util.rs:13121`](../../../../../../../crates/renovate-core/src/util.rs#L13121) |
| 100 | matches on wildcards | ported | [`crates/renovate-core/src/util.rs:13156`](../../../../../../../crates/renovate-core/src/util.rs#L13156) |
| 126 | matches on character ranges | ported | [`crates/renovate-core/src/util.rs:13174`](../../../../../../../crates/renovate-core/src/util.rs#L13174) |
| 131 | caches results | pending | — |

