# `lib/modules/datasource/endoflife-date/index.spec.ts`

[← `datasource/endoflife-date`](../../../../_by-module/datasource/endoflife-date.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 22 | processes real data | ported | [`crates/renovate-core/src/datasources/endoflife.rs:174`](../../../../../../../crates/renovate-core/src/datasources/endoflife.rs#L174) |
| 83 | returns null without registryurl | ported | [`crates/renovate-core/src/datasources/endoflife.rs:213`](../../../../../../../crates/renovate-core/src/datasources/endoflife.rs#L213) |
| 92 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/endoflife.rs:221`](../../../../../../../crates/renovate-core/src/datasources/endoflife.rs#L221) |
| 102 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/endoflife.rs:238`](../../../../../../../crates/renovate-core/src/datasources/endoflife.rs#L238) |
| 112 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/endoflife.rs:255`](../../../../../../../crates/renovate-core/src/datasources/endoflife.rs#L255) |
| 122 | detects boolean discontinuation | ported | [`crates/renovate-core/src/datasources/endoflife.rs:270`](../../../../../../../crates/renovate-core/src/datasources/endoflife.rs#L270) |
| 158 | detects date discontinuation | ported | [`crates/renovate-core/src/datasources/endoflife.rs:322`](../../../../../../../crates/renovate-core/src/datasources/endoflife.rs#L322) |

