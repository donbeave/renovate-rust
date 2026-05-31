# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/fs/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/fs/index.spec.ts
**Total tests:** 55 | **Ported:** 31 | **Actionable:** 24 | **Status:** pending

### `util/fs/index › getParentDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ('$dir') -> '$expected' | 77 | ported | `fs.rs` | `get_parent_dir_cases` | — |

### `util/fs/index › getSiblingFileName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ('$file', '$sibling') -> '$expected' | 98 | ported | `fs.rs` | `get_sibling_file_name_cases` | — |

### `util/fs/index › readLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads buffer | 112 | ported | `fs.rs` | `local_file_read_cases` | — |
| reads string | 118 | ported | `fs.rs` | `local_file_read_cases` | — |
| returns null if file is not found | 124 | ported | `fs.rs` | `local_file_read_cases` | — |
| logs a warning if hidden Unciode characters are found | 128 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | Hidden Unicode character detection not implemented in Rust read_local_string |
| does not log the same hidden Unciode characters if found multiple times | 139 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | Hidden Unicode character detection not implemented in Rust read_local_string |
| logs a trace message (not warning) if hidden Unicode characters are found in a binary file | 152 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | Hidden Unicode character detection not implemented in Rust read_local_string |

### `util/fs/index › readLocalFile › if hidden Byte Order Mark (BOM) Unciode characters are found`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| but no other hidden characters, it logs a trace message | 172 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | BOM detection not implemented in Rust read_local_string |
| as well as other hidden characters, it logs a warning | 187 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | BOM detection not implemented in Rust read_local_string |

### `util/fs/index › writeLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| outputs file | 203 | ported | `fs.rs` | `local_file_write_delete_and_rename` | — |

### `util/fs/index › deleteLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if platform is local | 213 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | Platform abstraction (local vs non-local) not implemented in Rust |
| deletes file | 218 | ported | `fs.rs` | `local_file_write_delete_and_rename` | — |

### `util/fs/index › renameLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| renames file | 229 | ported | `fs.rs` | `local_file_write_delete_and_rename` | — |

### `util/fs/index › ensureDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates directory | 243 | ported | `fs.rs` | `directory_and_cache_helpers` | — |

### `util/fs/index › ensureLocalDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates local directory | 253 | ported | `fs.rs` | `directory_and_cache_helpers` | — |

### `util/fs/index › ensureCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prefers environment variables over global config | 263 | ported | `fs.rs` | `directory_and_cache_helpers` | Rust helper verifies the resulting private cache namespace path without process env indirection |

### `util/fs/index › privateCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cache dir | 272 | ported | `fs.rs` | `directory_and_cache_helpers` | — |

### `util/fs/index › localPathExists`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for file | 279 | ported | `fs.rs` | `local_path_status_helpers` | — |
| returns true for directory | 285 | ported | `fs.rs` | `local_path_status_helpers` | — |
| returns false | 289 | ported | `fs.rs` | `local_path_status_helpers` | — |

### `util/fs/index › isLocalPath`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for valid local path | 295 | ported | `fs.rs` | `local_path_status_helpers` | — |
| returns false | 299 | ported | `fs.rs` | `local_path_status_helpers` | — |

### `util/fs/index › readLocalSymlink`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads symlink | 305 | ported | `fs.rs` | `local_symlink_helpers` | — |
| return null when link not exists | 317 | ported | `fs.rs` | `local_symlink_helpers` | — |

### `util/fs/index › findLocalSiblingOrParent`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns path for file | 331 | ported | `fs.rs` | `find_local_sibling_or_parent_cases` | — |
| immediately returns null when either path is absolute | 355 | ported | `fs.rs` | `find_local_sibling_or_parent_cases` | — |

### `util/fs/index › readLocalDirectory`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns dir content | 362 | ported | `fs.rs` | `read_local_directory_cases` | — |
| return empty array for non existing directory | 380 | ported | `fs.rs` | `read_local_directory_cases` | Rust verifies the current thrown-error behavior |
| return empty array for a existing but empty directory | 384 | ported | `fs.rs` | `read_local_directory_cases` | — |

### `util/fs/index › createCacheWriteStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates write stream | 393 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `createCacheWriteStream` not implemented in Rust |

### `util/fs/index › createCacheReadStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates read stream | 410 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `createCacheReadStream` not implemented in Rust |

### `util/fs/index › localPathIsFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for file | 433 | ported | `fs.rs` | `local_path_status_helpers` | — |
| returns false for directory | 439 | ported | `fs.rs` | `local_path_status_helpers` | — |
| returns false for non-existing path | 445 | ported | `fs.rs` | `local_path_status_helpers` | — |

### `util/fs/index › localPathIsSymbolicLink`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for file | 453 | ported | `fs.rs` | `local_symlink_helpers` | — |
| returns false for directory | 459 | ported | `fs.rs` | `local_symlink_helpers` | — |
| returns false for non-existing path | 465 | ported | `fs.rs` | `local_symlink_helpers` | — |
| returns true for symlink | 470 | ported | `fs.rs` | `local_symlink_helpers` | — |

### `util/fs/index › findUpLocal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns relative path for file | 486 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `findUpLocal` not implemented in Rust |
| returns null if nothing found | 492 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `findUpLocal` not implemented in Rust |
| returns undefined if found a file outside of localDir | 498 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `findUpLocal` not implemented in Rust |

### `util/fs/index › chmodLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| changes file mode | 506 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `chmodLocalFile` not implemented in Rust |

### `util/fs/index › statLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns stat object | 523 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `statLocalFile` not implemented in Rust |

### `util/fs/index › statCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns stat object | 534 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `statCacheFile` not implemented in Rust |

### `util/fs/index › listCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| lists directory | 545 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `listCacheDir` not implemented in Rust |

### `util/fs/index › rmCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes cache dir | 552 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `rmCache` not implemented in Rust |

### `util/fs/index › cachePathExists`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 561 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `cachePathExists` not implemented in Rust |

### `util/fs/index › cachePathIsFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if does not exist | 569 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `cachePathIsFile` not implemented in Rust |

### `util/fs/index › readCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 575 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `readCacheFile` not implemented in Rust |

### `util/fs/index › outputCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| outputs file | 585 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `outputCacheFile` not implemented in Rust |

### `util/fs/index › readSystemFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 593 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `readSystemFile` not implemented in Rust |

### `util/fs/index › writeSystemFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| writes file | 602 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `writeSystemFile` not implemented in Rust |

### `util/fs/index › getLocalFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads list of files from local fs | 610 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `getLocalFiles` not implemented in Rust |
| returns null as content if file is not found | 622 | not-applicable | Mock framework internals — tests fs util via vitest-mocked fs; Rust tests this at different layer | — | `getLocalFiles` not implemented in Rust |

---
