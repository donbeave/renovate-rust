# `lib/modules/manager/helmsman/extract.spec.ts`

[← `manager/helmsman`](../../../../_by-module/manager/helmsman.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if empty | ported | [`crates/renovate-core/src/extractors/helmsman.rs:317`](../../../../../../../crates/renovate-core/src/extractors/helmsman.rs#L317) |
| 16 | returns null if extracting non helmsman yaml file | ported | [`crates/renovate-core/src/extractors/helmsman.rs:329`](../../../../../../../crates/renovate-core/src/extractors/helmsman.rs#L329) |
| 23 | returns null if apps not defined | ported | [`crates/renovate-core/src/extractors/helmsman.rs:323`](../../../../../../../crates/renovate-core/src/extractors/helmsman.rs#L323) |
| 29 | extract deps | ported | [`crates/renovate-core/src/extractors/helmsman.rs:279`](../../../../../../../crates/renovate-core/src/extractors/helmsman.rs#L279) |

