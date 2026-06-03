# `lib/modules/versioning/exact/index.spec.ts`

[← `versioning/exact`](../../../../_by-module/versioning/exact.md) · [all modules](../../../../README.md)

**13/14 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | _(it.each / template — verify manually)_ | ? | — |
| 19 | _(it.each / template — verify manually)_ | ? | — |
| 32 | returns true for any valid version | ported | [`crates/renovate-core/src/versioning/exact.rs:104`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L104) |
| 39 | returns true for any version | ported | [`crates/renovate-core/src/versioning/exact.rs:111`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L111) |
| 46 | returns true when version equals current | ported | [`crates/renovate-core/src/versioning/exact.rs:118`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L118) |
| 50 | returns false when version differs from current | ported | [`crates/renovate-core/src/versioning/exact.rs:124`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L124) |
| 56 | returns null for all | ported | [`crates/renovate-core/src/versioning/exact.rs:130`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L130) |
| 64 | _(it.each / template — verify manually)_ | ? | — |
| 76 | _(it.each / template — verify manually)_ | ? | — |
| 87 | _(it.each / template — verify manually)_ | ? | — |
| 102 | returns exact match only | ported | [`crates/renovate-core/src/versioning/exact.rs:180`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L180) |
| 110 | returns exact match only | ported | [`crates/renovate-core/src/versioning/exact.rs:180`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L180) |
| 118 | returns currentvalue unchanged | ported | [`crates/renovate-core/src/versioning/exact.rs:196`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L196) |
| 131 | returns 0 for any comparison | ported | [`crates/renovate-core/src/versioning/exact.rs:202`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L202) |

