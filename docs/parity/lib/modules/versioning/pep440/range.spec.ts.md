# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/pep440/range.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/pep440/range.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** done

### `modules/versioning/pep440/range`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checkRange("$rangeInput, "$newVersion"") === "$expected" | 8 | ported | `pep440.rs` | `check_range_table` | — |
| returns null without warning if new version is excluded from range | 24 | ported | `pep440.rs` | `excluded_version_returns_none` | — |
| handles v-prefixed version as currentValue | 39 | ported | `pep440.rs` | `v_prefix_preserved` | — |
| handles bare version that differs from currentVersion without v-prefix | 49 | ported | `pep440.rs` | `bare_version_differs_from_current` | — |

---

