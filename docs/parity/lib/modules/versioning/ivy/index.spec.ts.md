# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/ivy/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/ivy/index.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/ivy/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parseDynamicRevision("$input") === { type: "$type", value: "$value" } | 10 | not-applicable | — | — | Renovate's Ivy dynamic revision/versioning scheme is not implemented as a Rust versioning API. |
| parseDynamicRevision("$input") === null | 33 | not-applicable | — | — | Renovate's Ivy dynamic revision parser is not implemented as a Rust versioning API. |
| isValid("$input") === $expected | 43 | not-applicable | — | — | Renovate's Ivy versioning validation is not implemented as a Rust versioning API. |
| isVersion("$input") === $expected | 72 | not-applicable | — | — | Renovate's Ivy version classifier is not implemented as a Rust versioning API. |
| matches("$version", "$range") === $expected | 100 | not-applicable | — | — | Renovate's Ivy range matcher is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 143 | not-applicable | — | — | Renovate's Ivy update-value helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === $expected | 160 | not-applicable | — | — | Renovate's Ivy satisfying-version helper is not implemented as a Rust versioning API. |
| isCompatible("$version") === $expected | 170 | not-applicable | — | — | Renovate's Ivy compatibility helper is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $expected | 177 | not-applicable | — | — | Renovate's Ivy single-version classifier is not implemented as a Rust versioning API. |

---

