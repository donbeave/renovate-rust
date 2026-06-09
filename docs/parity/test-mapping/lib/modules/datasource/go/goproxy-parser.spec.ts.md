# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | parses single url | ported | [`crates/renovate-core/src/util.rs:13039`](../../../../../../../crates/renovate-core/src/util.rs#L13039) |
| 15 | parses multiple urls | ported | [`crates/renovate-core/src/util.rs:13048`](../../../../../../../crates/renovate-core/src/util.rs#L13048) |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | [`crates/renovate-core/src/util.rs:13083`](../../../../../../../crates/renovate-core/src/util.rs#L13083) |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | [`crates/renovate-core/src/util.rs:13099`](../../../../../../../crates/renovate-core/src/util.rs#L13099) |
| 68 | matches on real package prefixes | ported | [`crates/renovate-core/src/util.rs:13124`](../../../../../../../crates/renovate-core/src/util.rs#L13124) |
| 100 | matches on wildcards | ported | [`crates/renovate-core/src/util.rs:13159`](../../../../../../../crates/renovate-core/src/util.rs#L13159) |
| 126 | matches on character ranges | ported | [`crates/renovate-core/src/util.rs:13177`](../../../../../../../crates/renovate-core/src/util.rs#L13177) |
| 131 | caches results | pending | — |

