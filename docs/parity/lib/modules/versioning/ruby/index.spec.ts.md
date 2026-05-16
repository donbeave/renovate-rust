# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/ruby/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/ruby/index.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/ruby/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$a", "$b") === $expected | 4 | not-applicable | - | - | Renovate's RubyGems version comparator is not implemented as a Rust versioning API. |
| getMajor, getMinor, getPatch for "$version" | 21 | not-applicable | - | - | Renovate's RubyGems version parser/component API is not implemented as a Rust versioning API. |
| isVersion("$version") === $expected | 38 | not-applicable | - | - | Renovate's RubyGems version validator is not implemented as a Rust versioning API. |
| isGreaterThan("$a", "$b") === $expected | 62 | not-applicable | - | - | Renovate's RubyGems version comparator is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 106 | not-applicable | - | - | Renovate's RubyGems prerelease stability classifier is not implemented as a Rust versioning API. |
| $versions -> sortVersions -> $expected | 122 | not-applicable | - | - | Renovate's RubyGems sorting comparator is not implemented as a Rust versioning API. |
| minSatisfyingVersion($versions, "$range") === "$expected" | 129 | not-applicable | - | - | Renovate's RubyGems range parser and satisfying-version selector are not implemented as a Rust versioning API. |
| getSatisfyingVersion($versions, "$range") === "$expected" | 147 | not-applicable | - | - | Renovate's RubyGems range parser and satisfying-version selector are not implemented as a Rust versioning API. |
| matches("$version", "$range") === "$expected" | 165 | not-applicable | - | - | Renovate's RubyGems range matching API is not implemented as a Rust versioning API. |
| isLessThanRange("$version", "$range") === "$expected" | 185 | not-applicable | - | - | Renovate's RubyGems range comparison API is not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 209 | not-applicable | - | - | Renovate's RubyGems range/version validation API is not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 224 | not-applicable | - | - | Renovate's RubyGems range/operator validation API is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $expected | 247 | not-applicable | - | - | Renovate's RubyGems single-version/range discriminator is not implemented as a Rust versioning API. |
| returns a pinned value | 276 | not-applicable | - | - | Renovate's RubyGems pinned-value normalizer is not implemented as a Rust versioning API. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 281 | not-applicable | - | - | Renovate's RubyGems range strategy rewrite engine is not implemented as a Rust versioning API. |

---

