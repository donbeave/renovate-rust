# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/apk/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/apk/index.spec.ts
**Total tests:** 53 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/apk/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid($version) === $expected | 5 | not-applicable | — | — | Renovate's APK versioning scheme is not implemented as a Rust versioning API. |
| isStable($version) === $expected | 19 | not-applicable | — | — | Renovate's APK versioning stability classifier is not implemented as a Rust versioning API. |
| getMajor($version) === $expected | 41 | not-applicable | — | — | Renovate's APK version component parser is not implemented as a Rust versioning API. |
| getMinor($version) === $expected | 51 | not-applicable | — | — | Renovate's APK version component parser is not implemented as a Rust versioning API. |
| getPatch($version) === $expected | 61 | not-applicable | — | — | Renovate's APK version component parser is not implemented as a Rust versioning API. |
| compare($a, $b) === $expected | 74 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| isGreaterThan($a, $b) === $expected | 102 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| equals($a, $b) === $expected | 115 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| getSatisfyingVersion with exact match ($range) === $expected | 136 | not-applicable | — | — | Renovate's APK range satisfying-version helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion with range operator ($range) === $expected | 149 | not-applicable | — | — | Renovate's APK range satisfying-version helper is not implemented as a Rust versioning API. |
| getSatisfyingVersion with tilde range ($range) === $expected | 164 | not-applicable | — | — | Renovate's APK range satisfying-version helper is not implemented as a Rust versioning API. |
| should return null for invalid range operators | 175 | not-applicable | — | — | Renovate's APK range satisfying-version helper is not implemented as a Rust versioning API. |
| should return null for empty versions array | 179 | not-applicable | — | — | Renovate's APK range satisfying-version helper is not implemented as a Rust versioning API. |
| should filter out invalid versions | 183 | not-applicable | — | — | Renovate's APK range satisfying-version helper is not implemented as a Rust versioning API. |
| isSingleVersion($version) === $expected | 192 | not-applicable | — | — | Renovate's APK single-version classifier is not implemented as a Rust versioning API. |
| should return false for empty versions | 202 | not-applicable | — | — | Renovate's APK single-version classifier is not implemented as a Rust versioning API. |
| isLessThanRange($version, $range) === $expected | 210 | not-applicable | — | — | Renovate's APK range comparison helper is not implemented as a Rust versioning API. |
| should sort versions correctly | 225 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should compare release numbers when version parts are equal | 236 | not-applicable | — | — | Renovate's APK release-number comparator is not implemented as a Rust versioning API. |
| should parse complex versions ($version) === $expected | 246 | not-applicable | — | — | Renovate's APK version parser is not implemented as a Rust versioning API. |
| should identify stable versions ($version) === $expected | 261 | not-applicable | — | — | Renovate's APK versioning stability classifier is not implemented as a Rust versioning API. |
| should compare versions with prerelease identifiers ($a, $b) === $expected | 278 | not-applicable | — | — | Renovate's APK prerelease comparator is not implemented as a Rust versioning API. |
| should handle invalid version parsing gracefully | 295 | not-applicable | — | — | Renovate's APK version parser is not implemented as a Rust versioning API. |
| should handle null/undefined inputs | 305 | not-applicable | — | — | Renovate's APK version parser nullish-input behavior is not implemented as a Rust versioning API. |
| should return false for unstable versions with prerelease | 315 | not-applicable | — | — | Renovate's APK versioning stability classifier is not implemented as a Rust versioning API. |
| should return false for empty versions in isStable | 321 | not-applicable | — | — | Renovate's APK versioning stability classifier is not implemented as a Rust versioning API. |
| should handle versions with different major versions in tilde range | 329 | not-applicable | — | — | Renovate's APK tilde-range matcher is not implemented as a Rust versioning API. |
| should handle versions with different minor versions in tilde range | 335 | not-applicable | — | — | Renovate's APK tilde-range matcher is not implemented as a Rust versioning API. |
| should handle invalid target versions in ranges | 340 | not-applicable | — | — | Renovate's APK range matcher is not implemented as a Rust versioning API. |
| should handle versions with prerelease identifiers in ranges | 346 | not-applicable | — | — | Renovate's APK range matcher is not implemented as a Rust versioning API. |
| should return null for versions with _p package fix suffix | 358 | not-applicable | — | — | Renovate's APK suffix-stripping helper is not implemented as a Rust versioning API. |
| should return null for invalid versions | 364 | not-applicable | — | — | Renovate's APK suffix-stripping helper is not implemented as a Rust versioning API. |
| should return patch version for non-_p patterns | 370 | not-applicable | — | — | Renovate's APK suffix-stripping helper is not implemented as a Rust versioning API. |
| should handle versions with operators | 376 | not-applicable | — | — | Renovate's APK suffix-stripping helper is not implemented as a Rust versioning API. |
| should strip revision from newVersion when currentValue has no revision | 384 | not-applicable | — | — | Renovate's APK update-value helper is not implemented as a Rust versioning API. |
| should keep revision in newVersion when currentValue has revision | 394 | not-applicable | — | — | Renovate's APK update-value helper is not implemented as a Rust versioning API. |
| should handle newVersion without revision when currentValue has no revision | 404 | not-applicable | — | — | Renovate's APK update-value helper is not implemented as a Rust versioning API. |
| should handle newVersion without revision when currentValue has revision | 414 | not-applicable | — | — | Renovate's APK update-value helper is not implemented as a Rust versioning API. |
| should handle complex prerelease identifier comparisons | 426 | not-applicable | — | — | Renovate's APK prerelease comparator is not implemented as a Rust versioning API. |
| should handle versions with different prerelease patterns | 438 | not-applicable | — | — | Renovate's APK prerelease comparator is not implemented as a Rust versioning API. |
| should handle unknown range operators | 445 | not-applicable | — | — | Renovate's APK range matcher is not implemented as a Rust versioning API. |
| should handle unhandled range operators that match regex | 456 | not-applicable | — | — | Renovate's APK range matcher is not implemented as a Rust versioning API. |
| should handle tilde range with invalid target version | 467 | not-applicable | — | — | Renovate's APK tilde-range matcher is not implemented as a Rust versioning API. |
| should handle tilde range with invalid version in list | 474 | not-applicable | — | — | Renovate's APK tilde-range matcher is not implemented as a Rust versioning API. |
| should handle major-only versions without minor/patch | 485 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should handle letter vs number at same position in version parts | 494 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should handle number vs letter comparison in version parts | 499 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should handle extra numeric parts in remaining segments | 504 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should handle lexicographic string comparison in version parts | 509 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should handle equal letter parts continuing to next segment | 514 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should handle trailing letter in remaining segments | 519 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should return 0 for numerically equal but string-different versions | 524 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |
| should handle versions with different extra segment lengths | 528 | not-applicable | — | — | Renovate's APK version comparator is not implemented as a Rust versioning API. |

---

