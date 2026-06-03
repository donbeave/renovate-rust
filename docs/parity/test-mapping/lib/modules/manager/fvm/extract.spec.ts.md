# `lib/modules/manager/fvm/extract.spec.ts`

[← `manager/fvm`](../../../../_by-module/manager/fvm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | returns null for invalid json | ported | [`crates/renovate-core/src/extractors/fvm.rs:68`](../../../../../../../crates/renovate-core/src/extractors/fvm.rs#L68) |
| 13 | returns null for empty flutter sdk version | ported | [`crates/renovate-core/src/extractors/fvm.rs:62`](../../../../../../../crates/renovate-core/src/extractors/fvm.rs#L62) |
| 17 | returns null for non string flutter sdk version | ported | [`crates/renovate-core/src/extractors/fvm.rs:74`](../../../../../../../crates/renovate-core/src/extractors/fvm.rs#L74) |
| 26 | returns a result for .fvm/fvm_config.json | ported | [`crates/renovate-core/src/extractors/fvm.rs:48`](../../../../../../../crates/renovate-core/src/extractors/fvm.rs#L48) |
| 41 | returns a result for .fvmrc | ported | [`crates/renovate-core/src/extractors/fvm.rs:41`](../../../../../../../crates/renovate-core/src/extractors/fvm.rs#L41) |
| 53 | supports non range for .fvm/fvm_config.json | ported | [`crates/renovate-core/src/extractors/fvm.rs:80`](../../../../../../../crates/renovate-core/src/extractors/fvm.rs#L80) |
| 68 | supports non range for .fvmrc | ported | [`crates/renovate-core/src/extractors/fvm.rs:87`](../../../../../../../crates/renovate-core/src/extractors/fvm.rs#L87) |

