# `lib/modules/versioning/exact/index.spec.ts`

[← `versioning/exact`](../../../../_by-module/versioning/exact.md) · [all modules](../../../../README.md)

**13/14 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | _(it.each / template — verify manually)_ | ? | — |
| 19 | _(it.each / template — verify manually)_ | ? | — |
| 32 | returns true for any valid version | ported | [`crates/renovate-core/src/versioning/exact.rs:103`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L103) |
| 39 | returns true for any version | ported | [`crates/renovate-core/src/versioning/exact.rs:110`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L110) |
| 46 | returns true when version equals current | ported | [`crates/renovate-core/src/versioning/exact.rs:117`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L117) |
| 50 | returns false when version differs from current | ported | [`crates/renovate-core/src/versioning/exact.rs:123`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L123) |
| 56 | returns null for all | ported | [`crates/renovate-core/src/versioning/exact.rs:129`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L129) |
| 64 | _(it.each / template — verify manually)_ | ? | — |
| 76 | _(it.each / template — verify manually)_ | ? | — |
| 87 | _(it.each / template — verify manually)_ | ? | — |
| 102 | returns exact match only | ported | [`crates/renovate-core/src/versioning/exact.rs:179`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L179) |
| 110 | returns exact match only | ported | [`crates/renovate-core/src/versioning/exact.rs:179`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L179) |
| 118 | returns currentvalue unchanged | ported | [`crates/renovate-core/src/versioning/exact.rs:195`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L195) |
| 131 | returns 0 for any comparison | ported | [`crates/renovate-core/src/versioning/exact.rs:201`](../../../../../../../crates/renovate-core/src/versioning/exact.rs#L201) |

