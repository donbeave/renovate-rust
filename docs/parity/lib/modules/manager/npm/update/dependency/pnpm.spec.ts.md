# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/dependency/pnpm.spec.ts
**Total tests:** 24 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null on invalid input | 8 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| handles implicit default catalog dependency | 19 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| handles explicit default catalog dependency | 46 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| handles explicit named catalog dependency | 75 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| does nothing if the new and old values match | 111 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| replaces package | 132 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| replaces a github dependency value | 160 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| replaces a npm package alias | 189 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| replaces a github short hash | 219 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| replaces a github fully specified version | 248 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| returns null if the dependency is not present in the target catalog | 277 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| returns null if catalogs are missing | 298 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| returns null if empty file | 316 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| preserves literal whitespace | 330 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| preserves single quote style | 357 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| preserves comments | 384 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| preserves double quote style | 415 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| preserves anchors, replacing only the value | 442 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| preserves whitespace with anchors | 474 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| preserves quotation style with anchors | 501 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| preserves formatting in flow style syntax | 528 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| does not replace aliases in the value position | 559 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| does not replace aliases in the key position | 587 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |
| handles workspace overrides | 611 | not-applicable | — | — | tests pnpm-lock.yaml update via lockfile manipulation; complex lockfile format out of scope |

---

