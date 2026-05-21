# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/fs/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/fs/index.spec.ts
**Total tests:** 56 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

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
| reads buffer | 112 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| reads string | 118 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns null if file is not found | 124 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| logs a warning if hidden Unciode characters are found | 128 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| does not log the same hidden Unciode characters if found multiple times | 139 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| logs a trace message (not warning) if hidden Unicode characters are found in a binary file | 152 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › readLocalFile › if hidden Byte Order Mark (BOM) Unciode characters are found`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| but no other hidden characters, it logs a trace message | 172 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| as well as other hidden characters, it logs a warning | 187 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › writeLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| outputs file | 203 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › deleteLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if platform is local | 213 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| deletes file | 218 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › renameLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| renames file | 229 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › ensureDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates directory | 243 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › ensureLocalDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates local directory | 253 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › ensureCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prefers environment variables over global config | 263 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › privateCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cache dir | 272 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › localPathExists`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for file | 279 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns true for directory | 285 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns false | 289 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › isLocalPath`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for valid local path | 295 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns false | 299 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › readLocalSymlink`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads symlink | 305 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| return null when link not exists | 317 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › findLocalSiblingOrParent`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns path for file | 331 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| immediately returns null when either path is absolute | 355 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › readLocalDirectory`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns dir content | 362 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| return empty array for non existing directory | 380 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| return empty array for a existing but empty directory | 384 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › createCacheWriteStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates write stream | 393 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › createCacheReadStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates read stream | 410 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › localPathIsFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for file | 433 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns false for directory | 439 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns false for non-existing path | 445 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › localPathIsSymbolicLink`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for file | 453 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns false for directory | 459 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns false for non-existing path | 465 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns true for symlink | 470 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| unnamed test | 472 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › findUpLocal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns relative path for file | 486 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns null if nothing found | 492 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns undefined if found a file outside of localDir | 498 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › chmodLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| changes file mode | 506 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › statLocalFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns stat object | 523 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › statCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns stat object | 534 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › listCacheDir`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| lists directory | 545 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › rmCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes cache dir | 552 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › cachePathExists`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 561 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › cachePathIsFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if does not exist | 569 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › readCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 575 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › outputCacheFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| outputs file | 585 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › readSystemFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads file | 593 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › writeSystemFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| writes file | 602 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

### `util/fs/index › getLocalFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads list of files from local fs | 610 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |
| returns null as content if file is not found | 622 | not-applicable | — | — | tests Node.js fs-extra/tmp-promise filesystem operations; most ops need GlobalConfig.localDir infrastructure |

---

