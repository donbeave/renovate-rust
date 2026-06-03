# `lib/modules/datasource/jsr/index.spec.ts`

[← `datasource/jsr`](../../../../_by-module/datasource/jsr.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 24 | should return null for invalid package name | ported | [`crates/renovate-core/src/datasources/jsr.rs:276`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L276) |
| 32 | should return null for no versions | ported | [`crates/renovate-core/src/datasources/jsr.rs:286`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L286) |
| 46 | should fetch package info from jsr | ported | [`crates/renovate-core/src/datasources/jsr.rs:305`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L305) |
| 74 | contains yanked versions | ported | [`crates/renovate-core/src/datasources/jsr.rs:341`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L341) |
| 102 | should return null if lookup fails | ported | [`crates/renovate-core/src/datasources/jsr.rs:370`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L370) |
| 115 | should throw error for unparseable | ported | [`crates/renovate-core/src/datasources/jsr.rs:386`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L386) |

