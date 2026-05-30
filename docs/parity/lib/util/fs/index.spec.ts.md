# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/fs/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/fs/index.spec.ts
**Total tests:** 56 | **Ported:** 2 | **Actionable:** 56 | **Status:** partial

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
| reads buffer | 112 | pending | — | — | filesystem behavior is in scope |
| reads string | 118 | pending | — | — | filesystem behavior is in scope |
| returns null if file is not found | 124 | pending | — | — | filesystem behavior is in scope |
| logs a warning if hidden Unciode characters are found | 128 | pending | — | — | filesystem behavior is in scope |
| does not log the same hidden Unciode characters if found multiple times | 139 | pending | — | — | filesystem behavior is in scope |
| logs a trace message (not warning) if hidden Unicode characters are found in a binary file | 152 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › readLocalFile › if hidden Byte Order Mark (BOM) Unciode characters are found`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| but no other hidden characters, it logs a trace message | 172 | pending | — | — | filesystem behavior is in scope |
| as well as other hidden characters, it logs a warning | 187 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › writeLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| outputs file | 203 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › deleteLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if platform is local | 213 | pending | — | — | filesystem behavior is in scope |
| deletes file | 218 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › renameLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| renames file | 229 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › ensureDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates directory | 243 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › ensureLocalDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates local directory | 253 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › ensureCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prefers environment variables over global config | 263 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › privateCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cache dir | 272 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › localPathExists`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for file | 279 | pending | — | — | filesystem behavior is in scope |
| returns true for directory | 285 | pending | — | — | filesystem behavior is in scope |
| returns false | 289 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › isLocalPath`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for valid local path | 295 | pending | — | — | filesystem behavior is in scope |
| returns false | 299 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › readLocalSymlink`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads symlink | 305 | pending | — | — | filesystem behavior is in scope |
| return null when link not exists | 317 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › findLocalSiblingOrParent`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns path for file | 331 | pending | — | — | filesystem behavior is in scope |
| immediately returns null when either path is absolute | 355 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › readLocalDirectory`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns dir content | 362 | pending | — | — | filesystem behavior is in scope |
| return empty array for non existing directory | 380 | pending | — | — | filesystem behavior is in scope |
| return empty array for a existing but empty directory | 384 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › createCacheWriteStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates write stream | 393 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › createCacheReadStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates read stream | 410 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › localPathIsFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for file | 433 | pending | — | — | filesystem behavior is in scope |
| returns false for directory | 439 | pending | — | — | filesystem behavior is in scope |
| returns false for non-existing path | 445 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › localPathIsSymbolicLink`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for file | 453 | pending | — | — | filesystem behavior is in scope |
| returns false for directory | 459 | pending | — | — | filesystem behavior is in scope |
| returns false for non-existing path | 465 | pending | — | — | filesystem behavior is in scope |
| returns true for symlink | 470 | pending | — | — | filesystem behavior is in scope |
| unnamed test | 472 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › findUpLocal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns relative path for file | 486 | pending | — | — | filesystem behavior is in scope |
| returns null if nothing found | 492 | pending | — | — | filesystem behavior is in scope |
| returns undefined if found a file outside of localDir | 498 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › chmodLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| changes file mode | 506 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › statLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns stat object | 523 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › statCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns stat object | 534 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › listCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| lists directory | 545 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › rmCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes cache dir | 552 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › cachePathExists`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 561 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › cachePathIsFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if does not exist | 569 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › readCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 575 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › outputCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| outputs file | 585 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › readSystemFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 593 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › writeSystemFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| writes file | 602 | pending | — | — | filesystem behavior is in scope |

### `util/fs/index › getLocalFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads list of files from local fs | 610 | pending | — | — | filesystem behavior is in scope |
| returns null as content if file is not found | 622 | pending | — | — | filesystem behavior is in scope |

---
