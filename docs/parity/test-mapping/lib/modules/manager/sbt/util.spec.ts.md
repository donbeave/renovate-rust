# `lib/modules/manager/sbt/util.spec.ts`

[← `manager/sbt`](../../../../_by-module/manager/sbt.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | places build.sbt first | ported | [`crates/renovate-core/src/extractors/sbt.rs:1235`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1235) |
| 20 | does not normalize prior to 2.10 | ported | [`crates/renovate-core/src/extractors/sbt.rs:1253`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1253) |
| 25 | normalizes a scala 2.10 version number | ported | [`crates/renovate-core/src/extractors/sbt.rs:1259`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1259) |
| 30 | normalizes a scala 2.11 version number | ported | [`crates/renovate-core/src/extractors/sbt.rs:1265`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1265) |
| 35 | normalizes a scala 2.12 version number | ported | [`crates/renovate-core/src/extractors/sbt.rs:1271`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1271) |
| 40 | normalizes a scala 2.13 version number | ported | [`crates/renovate-core/src/extractors/sbt.rs:1277`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1277) |
| 45 | normalizes a scala 3 lts version number | ported | [`crates/renovate-core/src/extractors/sbt.rs:1283`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1283) |
| 50 | normalizes a scala 3 current version number | ported | [`crates/renovate-core/src/extractors/sbt.rs:1289`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1289) |

