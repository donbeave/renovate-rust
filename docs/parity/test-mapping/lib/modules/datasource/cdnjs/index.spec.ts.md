# `lib/modules/datasource/cdnjs/index.spec.ts`

[← `datasource/cdnjs`](../../../../_by-module/datasource/cdnjs.md) · [all modules](../../../../README.md)

**13/14 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 18 | throws for empty result | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:167`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L167) |
| 28 | throws for error | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:182`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L182) |
| 38 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:190`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L190) |
| 48 | returns null for empty 200 ok | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:207`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L207) |
| 61 | throws for 401 | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:224`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L224) |
| 71 | throws for 429 | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:239`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L239) |
| 81 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:254`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L254) |
| 91 | throws for unknown error | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:269`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L269) |
| 101 | processes real data | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:277`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L277) |
| 115 | returs null for no result | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:304`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L304) |
| 131 | returs null for empty sri object | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:322`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L322) |
| 147 | returs null if file not found | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:340`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L340) |
| 163 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:190`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L190) |
| 176 | returns digest | ported | [`crates/renovate-core/src/datasources/cdnjs.rs:376`](../../../../../../../crates/renovate-core/src/datasources/cdnjs.rs#L376) |

