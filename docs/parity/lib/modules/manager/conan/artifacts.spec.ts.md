# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/conan/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/conan/artifacts.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if updatedDeps are empty and lockFileMaintenance is turned off | 30 | not-applicable | — | — | Subprocess artifact generation |
| returns null if conan.lock was not found | 45 | not-applicable | — | — | Subprocess artifact generation |
| returns null if conan.lock read operation failed | 64 | not-applicable | — | — | Subprocess artifact generation |
| returns null if read operation failed for new conan.lock | 87 | not-applicable | — | — | Subprocess artifact generation |
| returns null if original and updated conan.lock files are the same | 118 | not-applicable | — | — | Subprocess artifact generation |
| returns updated conan.lock for conanfile.txt | 148 | not-applicable | — | — | Subprocess artifact generation |
| supports install mode | 184 | not-applicable | — | — | Subprocess artifact generation |
| returns updated conan.lock when updateType are not empty | 232 | not-applicable | — | — | Subprocess artifact generation |
| returns updated conan.lock when updateType are empty, but isLockFileMaintenance is true | 268 | not-applicable | — | — | Subprocess artifact generation |
| rethrows temporary error | 299 | not-applicable | — | — | Subprocess artifact generation |
| returns an artifact error when conan.lock update fails | 320 | not-applicable | — | — | Subprocess artifact generation |

---

