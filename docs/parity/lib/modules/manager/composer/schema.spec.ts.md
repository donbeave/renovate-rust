# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/schema.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `ReposRecord`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 5 | ported | `extractors/composer.rs` | `repos_record_parses_default` | — |
| parses repositories | 9 | ported | `extractors/composer.rs` | `repos_record_parses_repositories` | — |

### `ReposArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 30 | ported | `extractors/composer.rs` | `repos_array_parses_default` | — |
| parses repositories | 34 | ported | `extractors/composer.rs` | `repos_array_parses_repositories` | — |

### `Repos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 58 | ported | `extractors/composer.rs` | `repos_parses_null_default` | — |
| parses repositories | 66 | ported | `extractors/composer.rs` | `repos_parses_array_repos` | — |
| parses repositories with packagist disabled | 92 | ported | `extractors/composer.rs` | `repos_parses_with_packagist_disabled` | — |

---

