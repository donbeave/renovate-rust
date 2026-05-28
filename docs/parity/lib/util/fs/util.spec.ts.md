# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/fs/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/fs/util.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `util/fs/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensureLocalPath('$path', '$fullPath') | 14 | ported | `fs.rs` | `ensure_base_path_resolves_relative_paths` | — |
| ensureLocalPath('$path', '${localDir}') - throws | 22 | ported | `fs.rs` | `ensure_base_path_rejects_escaping_paths` | — |
| ensureCachePath('$path', '$fullPath') | 33 | ported | `fs.rs` | `ensure_base_path_resolves_relative_paths` | — |
| ensureCachePath('$path', '${cacheDir}') - throws | 41 | ported | `fs.rs` | `ensure_base_path_rejects_escaping_paths` | — |
| isValidPath($value) == $expected | 53 | ported | `fs.rs` | `is_valid_path_cases` | — |

---

