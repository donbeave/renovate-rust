# `lib/modules/datasource/go/goproxy-parser.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**7/9 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | parses single url | ported | [`crates/renovate-core/src/util.rs:13052`](../../../../../../../crates/renovate-core/src/util.rs#L13052) |
| 15 | parses multiple urls | ported | [`crates/renovate-core/src/util.rs:13061`](../../../../../../../crates/renovate-core/src/util.rs#L13061) |
| 25 | ignores everything starting from "direct" and "off" keywords | ported | [`crates/renovate-core/src/util.rs:13096`](../../../../../../../crates/renovate-core/src/util.rs#L13096) |
| 43 | caches results | pending | — |
| 49 | produces regex | ported | [`crates/renovate-core/src/util.rs:13112`](../../../../../../../crates/renovate-core/src/util.rs#L13112) |
| 68 | matches on real package prefixes | ported | [`crates/renovate-core/src/util.rs:13137`](../../../../../../../crates/renovate-core/src/util.rs#L13137) |
| 100 | matches on wildcards | ported | [`crates/renovate-core/src/util.rs:13172`](../../../../../../../crates/renovate-core/src/util.rs#L13172) |
| 126 | matches on character ranges | ported | [`crates/renovate-core/src/util.rs:13190`](../../../../../../../crates/renovate-core/src/util.rs#L13190) |
| 131 | caches results | pending | — |

