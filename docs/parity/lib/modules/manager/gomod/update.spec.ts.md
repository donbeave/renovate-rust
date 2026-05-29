# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gomod/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/update.spec.ts
**Total tests:** 33 | **Ported:** 33 | **Actionable:** 33 | **Status:** done

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces existing value | 12 | ported | `extractors/gomod.rs` | `gomod_update_replace_value` | — |
| replaces golang version update | 28 | ported | `extractors/gomod.rs` | `gomod_update_golang_version` | — |
| replaces go toolchain | 44 | ported | `extractors/gomod.rs` | `gomod_update_toolchain` | — |
| replaces two values in one file | 60 | ported | `extractors/gomod.rs` | `gomod_update_two_values` | — |
| returns same | 90 | ported | `extractors/gomod.rs` | `gomod_update_returns_same_if_no_change` | — |
| bumps major v0 > v1 | 104 | ported | `extractors/gomod.rs` | `gomod_update_major_v0_to_v1` | — |
| replaces major updates > 1 | 123 | ported | `extractors/gomod.rs` | `gomod_update_major_adds_v2_suffix` | — |
| bumps major with single package name component | 142 | ported | `extractors/gomod.rs` | `gomod_update_major_single_component` | — |
| bumps major with multiple package name components | 161 | ported | `extractors/gomod.rs` | `gomod_update_major_multiple_components` | — |
| replaces major gopkg.in updates | 182 | ported | `extractors/gomod.rs` | `gomod_update_major_gopkg_in` | — |
| skip replacing incompatible major updates | 202 | ported | `extractors/gomod.rs` | `gomod_update_major_skip_incompatible` | — |
| returns null if mismatch | 223 | ported | `extractors/gomod.rs` | `gomod_update_returns_null_if_mismatch` | — |
| returns null if error | 237 | ported | `extractors/gomod.rs` | `gomod_update_returns_null_on_empty` | — |
| replaces multiline | 247 | ported | `extractors/gomod.rs` | `gomod_update_multiline` | — |
| replaces quoted multiline | 263 | ported | `extractors/gomod.rs` | `gomod_update_quoted_multiline` | — |
| replaces major multiline | 280 | ported | `extractors/gomod.rs` | `gomod_update_major_multiline` | — |
| bumps major multiline | 299 | ported | `extractors/gomod.rs` | `gomod_update_major_multiline_bump` | — |
| bumps major v0 > v1 multiline | 317 | ported | `extractors/gomod.rs` | `gomod_update_major_v0_v1_multiline` | — |
| update multiline digest | 335 | ported | `extractors/gomod.rs` | `gomod_update_multiline_digest` | — |
| skips already-updated multiline digest | 360 | ported | `extractors/gomod.rs` | `gomod_update_skips_already_updated_digest` | — |
| updates pseudo-version with digest updateType | 377 | ported | `extractors/gomod.rs` | `gomod_update_pseudo_version_digest` | — |
| handles multiline mismatch | 395 | ported | `extractors/gomod.rs` | `gomod_update_multiline_mismatch` | — |
| handles +incompatible tag | 412 | ported | `extractors/gomod.rs` | `gomod_update_incompatible_tag_preserved` | — |
| handles +incompatible tag without duplicating it | 433 | ported | `extractors/gomod.rs` | `gomod_update_incompatible_no_duplicate` | — |
| handles replace line with minor version update | 454 | ported | `extractors/gomod.rs` | `gomod_update_replace_minor` | — |
| handles replace line with major version update | 472 | ported | `extractors/gomod.rs` | `gomod_update_replace_major` | — |
| handles replace line with major version update that bumps both sides of the replace | 490 | ported | `extractors/gomod.rs` | `gomod_update_replace_both_sides_major` | — |
| handles replace line with digest | 521 | ported | `extractors/gomod.rs` | `gomod_update_replace_with_digest` | — |
| handles no pinned version to latest available version | 538 | ported | `extractors/gomod.rs` | `gomod_update_no_pinned_version` | — |
| handles multiline replace update | 554 | ported | `extractors/gomod.rs` | `gomod_update_multiline_replace` | — |
| should return null for replacement | 575 | ported | `extractors/gomod.rs` | `gomod_update_null_for_replacement` | — |
| should perform indirect upgrades when top-level | 583 | ported | `extractors/gomod.rs` | `gomod_update_indirect_top_level` | — |
| should perform indirect upgrades when in require blocks | 601 | ported | `extractors/gomod.rs` | `gomod_update_indirect_in_block` | — |

---
