# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/fs/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/fs/util.spec.ts
**Total tests:** 5 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `util/fs/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensureLocalPath('$path', '$fullPath') | 14 | not-applicable | — | — | depends on GlobalConfig.localDir TypeScript infrastructure; not portable as unit test |
| ensureLocalPath('$path', '${localDir}') - throws | 22 | not-applicable | — | — | depends on GlobalConfig.localDir TypeScript infrastructure; not portable as unit test |
| ensureCachePath('$path', '$fullPath') | 33 | not-applicable | — | — | depends on GlobalConfig.cacheDir TypeScript infrastructure; not portable as unit test |
| ensureCachePath('$path', '${cacheDir}') - throws | 41 | not-applicable | — | — | depends on GlobalConfig.cacheDir TypeScript infrastructure; not portable as unit test |
| isValidPath($value) == $expected | 53 | ported | `fs.rs` | `is_valid_path_cases` | — |

---

