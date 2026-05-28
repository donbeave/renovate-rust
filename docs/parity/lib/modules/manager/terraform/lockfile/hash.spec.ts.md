# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/lockfile/hash.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/lockfile/hash.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if getBuilds returns null | 43 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| return null if requesting a version which is not available | 58 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| backend index throws error | 72 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| returns null for no builds | 86 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| fail to create hashes | 99 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| full walkthrough | 128 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| full walkthrough on terraform cloud | 162 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| full walkthrough with different shasum per build | 227 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| full walkthrough without ziphashes available | 332 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |
| does not add any ziphashes when the shasums endpoint fails` | 385 | not-applicable | — | — | Uses httpMock + GlobalConfig + tmp-promise cacheDir; HTTP mock infrastructure not portable to Rust |

### `hashOfZipContent`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return hash for content with subfolders | 451 | not-applicable | — | — | Uses tmp-promise cacheDir + real zip fixture; requires temporary filesystem infrastructure |

---

