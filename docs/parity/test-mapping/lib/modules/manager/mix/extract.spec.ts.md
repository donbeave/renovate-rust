# `lib/modules/manager/mix/extract.spec.ts`

[← `manager/mix`](../../../../_by-module/manager/mix.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty for invalid dependency file | ported | [`crates/renovate-core/src/extractors/mix.rs:427`](../../../../../../../crates/renovate-core/src/extractors/mix.rs#L427) |
| 16 | extracts all dependencies when no lockfile | ported | [`crates/renovate-core/src/extractors/mix.rs:249`](../../../../../../../crates/renovate-core/src/extractors/mix.rs#L249) |
| 139 | extracts all dependencies and adds the locked version if lockfile present | ported | [`crates/renovate-core/src/extractors/mix.rs:388`](../../../../../../../crates/renovate-core/src/extractors/mix.rs#L388) |

