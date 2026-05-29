# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/versioning/distro.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/distro.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `modules/versioning/distro`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isCodename("$version") === $expected | 12 | ported | `ubuntu.rs` | `distro_is_codename` | — |
| getVersionByCodename("$version") === $expected | 27 | ported | `ubuntu.rs` | `distro_get_version_by_codename` | — |
| getCodenameByVersion("$version") === $expected | 44 | ported | `ubuntu.rs` | `distro_get_codename_by_version` | — |
| exists("$version") === $expected | 61 | ported | `ubuntu.rs` | `distro_exists` | — |
| isEolLts("$version") === $expected | 80 | ported | `ubuntu.rs` | `distro_is_eol_lts` | Fixed date 2021-03-20 |
| isReleased("$version") === $expected | 98 | ported | `ubuntu.rs` | `distro_is_released` | — |
| retrieves schedule of the previous previous release | 115 | ported | `ubuntu.rs` | `distro_get_n_latest` | — |
| retrieves schedule of the previous release | 122 | ported | `ubuntu.rs` | `distro_get_n_latest` | — |
| retrieves schedule of the most recent release | 129 | ported | `ubuntu.rs` | `distro_get_n_latest` | — |
| sends a float as an argument | 136 | ported | `ubuntu.rs` | `distro_get_n_latest` | — |
| sends an out of bound argument | 143 | ported | `ubuntu.rs` | `distro_get_n_latest` | — |
| sends another out of bound argument | 147 | ported | `ubuntu.rs` | `distro_get_n_latest` | — |
| retrieves focal release schedule | 151 | ported | `ubuntu.rs` | `distro_get_schedule` | — |
| retrieves non-existent release schedule | 158 | ported | `ubuntu.rs` | `distro_get_schedule` | — |
| works with debian | 162 | ported | `ubuntu.rs` | `distro_works_with_debian` | Uses `ignore_eol=true` to simulate TypeScript's `delete schedule.eol` |

---

