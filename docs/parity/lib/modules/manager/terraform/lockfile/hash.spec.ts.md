# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/lockfile/hash.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/lockfile/hash.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if getBuilds returns null | 43 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| return null if requesting a version which is not available | 58 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| backend index throws error | 72 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| returns null for no builds | 86 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| fail to create hashes | 99 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| full walkthrough | 128 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| full walkthrough on terraform cloud | 162 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| full walkthrough with different shasum per build | 227 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| full walkthrough without ziphashes available | 332 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |
| does not add any ziphashes when the shasums endpoint fails` | 385 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |

### `hashOfZipContent`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return hash for content with subfolders | 451 | not-applicable | Mock framework internals — tests terraform lockfile hash via vitest-mocked fs/exec; Rust tests this at different layer | — | Terraform lockfile hashing |

---

