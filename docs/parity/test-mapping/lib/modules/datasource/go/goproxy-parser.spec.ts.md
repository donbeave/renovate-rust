# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | parses single url | ported | [`crates/renovate-core/src/util.rs:13048`](../../../../../../../crates/renovate-core/src/util.rs#L13048) |
| 15 | parses multiple urls | ported | [`crates/renovate-core/src/util.rs:13057`](../../../../../../../crates/renovate-core/src/util.rs#L13057) |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | [`crates/renovate-core/src/util.rs:13092`](../../../../../../../crates/renovate-core/src/util.rs#L13092) |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | [`crates/renovate-core/src/util.rs:13108`](../../../../../../../crates/renovate-core/src/util.rs#L13108) |
| 68 | matches on real package prefixes | ported | [`crates/renovate-core/src/util.rs:13133`](../../../../../../../crates/renovate-core/src/util.rs#L13133) |
| 100 | matches on wildcards | ported | [`crates/renovate-core/src/util.rs:13168`](../../../../../../../crates/renovate-core/src/util.rs#L13168) |
| 126 | matches on character ranges | ported | [`crates/renovate-core/src/util.rs:13186`](../../../../../../../crates/renovate-core/src/util.rs#L13186) |
| 131 | caches results | pending | — |

