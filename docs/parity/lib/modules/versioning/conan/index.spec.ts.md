# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/conan/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/conan/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/conan/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $result | 5 | not-applicable | — | — | Renovate's Conan version/range validation API is not implemented in Rust; Rust Conan support is extractor/datasource oriented. |
| isVersion("$version") === $result | 117 | not-applicable | — | — | Renovate's Conan standalone-version classifier is not implemented in Rust; Rust Conan support is extractor/datasource oriented. |
| isCompatible("$version", "$range") === $result | 163 | not-applicable | — | — | Renovate's Conan range compatibility API is not implemented in Rust; Rust Conan support is extractor/datasource oriented. |
| matches("$version", "$range") === $result | 358 | not-applicable | — | — | Renovate's Conan range matcher API is not implemented in Rust; Rust Conan support is extractor/datasource oriented. |
| getSatisfyingVersion("$versions", "$range") === "$result" | 553 | not-applicable | — | — | Renovate's Conan satisfying-version selector is not implemented in Rust; Rust Conan support is extractor/datasource oriented. |
| getSatisfyingVersion("$versions", "$range") === "$result" | 565 | not-applicable | — | — | Renovate's Conan satisfying-version selector for prerelease ranges is not implemented in Rust. |
| getSatisfyingVersion("$versions", "$range") === "$result" | 641 | not-applicable | — | — | Renovate's Conan satisfying-version selector for include-prerelease ranges is not implemented in Rust. |
| minSatisfyingVersion("$versions", "$range") === "$result" | 699 | not-applicable | — | — | Renovate's Conan minimum satisfying-version selector is not implemented in Rust. |
| getMajor("$version") === $major getMinor("$version") === $minor getPatch("$version") === $patch | 720 | not-applicable | — | — | Renovate's Conan component accessor API is not implemented in Rust. |
| getMajor("$version") === "$result" | 743 | not-applicable | — | — | Renovate's Conan major component accessor is not implemented in Rust. |
| getMinor("$version") === "$result" | 752 | not-applicable | — | — | Renovate's Conan minor component accessor is not implemented in Rust. |
| getPatch("$version") === "$result" | 763 | not-applicable | — | — | Renovate's Conan patch component accessor is not implemented in Rust. |
| equals("$version", "$other) === "$result" | 774 | not-applicable | — | — | Renovate's Conan equality helper is not implemented in Rust. |
| isGreaterThan("$version", "$other) === "$result" | 825 | not-applicable | — | — | Renovate's Conan ordering comparator is not implemented in Rust. |
| sortVersions("$version", "$other) === "$result" | 871 | not-applicable | — | — | Renovate's Conan sort comparator is not implemented in Rust. |
| isLessThanRange("$version", "$range") === "$result" | 886 | not-applicable | — | — | Renovate's Conan less-than-range helper is not implemented in Rust. |

---

