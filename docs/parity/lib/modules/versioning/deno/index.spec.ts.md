# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/deno/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/deno/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/deno/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $isValid | 4 | not-applicable | — | — | Renovate's Deno versioning scheme is not implemented as a Rust versioning API. |
| getSatisfyingVersion("$versions","$range") === $maxSatisfying | 31 | not-applicable | — | — | Renovate's Deno semver-range wrapper is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $isSingle | 47 | not-applicable | — | — | Renovate's Deno versioning scheme is not implemented as a Rust versioning API. |
| subset("$a", "$b") === $expected | 58 | not-applicable | — | — | Renovate's Deno semver-range subset helper is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 72 | not-applicable | — | — | Renovate's Deno range update-value helper is not implemented as a Rust versioning API. |

---

