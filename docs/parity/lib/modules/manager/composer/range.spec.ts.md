# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/range.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/range.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns same if not auto | 5 | ported | `composer.rs` | `composer_range_returns_same_if_not_auto` | — |
| replaces require-dev | 10 | ported | `composer.rs` | `composer_range_auto_require_dev_returns_update_lockfile` | — |
| replaces project require | 18 | ported | `composer.rs` | `composer_range_auto_project_returns_update_lockfile` | — |
| widens complex ranges | 27 | ported | `composer.rs` | `composer_range_auto_complex_returns_widen` | — |
| widens complex bump | 36 | ported | `composer.rs` | `composer_range_bump_complex_returns_widen` | — |
| defaults to update-lockfile | 45 | ported | `composer.rs` | `composer_range_auto_defaults_to_update_lockfile` | — |
| defaults to widen for TYPO3 extensions | 50 | ported | `composer.rs` | `composer_range_auto_typo3_returns_widen` | — |

---

