# `lib/modules/manager/ant/update.spec.ts`

[← `manager/ant`](../../../../_by-module/manager/ant.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | updates inline xml version attribute | ported | [`crates/renovate-core/src/extractors/ant.rs:1704`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1704) |
| 23 | updates single-quoted xml version attribute | ported | [`crates/renovate-core/src/extractors/ant.rs:1722`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1722) |
| 42 | updates .properties file value | ported | [`crates/renovate-core/src/extractors/ant.rs:1740`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1740) |
| 58 | updates .properties value at end of file without trailing newline | ported | [`crates/renovate-core/src/extractors/ant.rs:1758`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1758) |
| 74 | returns filecontent unchanged when already updated | ported | [`crates/renovate-core/src/extractors/ant.rs:1773`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1773) |
| 91 | updates when sharedvariablename is set even if currentvalue differs | ported | [`crates/renovate-core/src/extractors/ant.rs:1788`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1788) |
| 108 | returns null when filereplaceposition is undefined | ported | [`crates/renovate-core/src/extractors/ant.rs:1810`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1810) |
| 122 | updates version within coords attribute | ported | [`crates/renovate-core/src/extractors/ant.rs:1817`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1817) |
| 140 | updates version within 4-part coords attribute | ported | [`crates/renovate-core/src/extractors/ant.rs:1836`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1836) |
| 158 | returns null when value at position does not match | ported | [`crates/renovate-core/src/extractors/ant.rs:1930`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1930) |

