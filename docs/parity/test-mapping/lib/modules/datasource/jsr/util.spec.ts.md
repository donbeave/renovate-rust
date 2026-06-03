# `lib/modules/datasource/jsr/util.spec.ts`

[← `datasource/jsr`](../../../../_by-module/datasource/jsr.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should extract package name | ported | [`crates/renovate-core/src/datasources/jsr.rs:216`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L216) |
| 12 | should return null for invalid name | ported | [`crates/renovate-core/src/datasources/jsr.rs:224`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L224) |
| 17 | should return null for below scope min length | ported | [`crates/renovate-core/src/datasources/jsr.rs:230`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L230) |
| 22 | should return null for exceed scope max length | ported | [`crates/renovate-core/src/datasources/jsr.rs:236`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L236) |
| 27 | should return null for invalid scope name | ported | [`crates/renovate-core/src/datasources/jsr.rs:243`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L243) |
| 32 | should return null for invalid package name starting with @ | ported | [`crates/renovate-core/src/datasources/jsr.rs:249`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L249) |
| 37 | should return null for exceed package max length | ported | [`crates/renovate-core/src/datasources/jsr.rs:255`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L255) |
| 42 | should return null for invalid package name | ported | [`crates/renovate-core/src/datasources/jsr.rs:262`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L262) |
| 47 | should return null for invalid package name starting with - | ported | [`crates/renovate-core/src/datasources/jsr.rs:268`](../../../../../../../crates/renovate-core/src/datasources/jsr.rs#L268) |

