# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/apk/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/apk/index.spec.ts
**Total tests:** 53 | **Ported:** 0 | **Actionable:** 53 | **Status:** pending

### `modules/versioning/apk/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid($version) === $expected | 5 | pending | — | — | — |
| isStable($version) === $expected | 19 | pending | — | — | — |
| getMajor($version) === $expected | 41 | pending | — | — | — |
| getMinor($version) === $expected | 51 | pending | — | — | — |
| getPatch($version) === $expected | 61 | pending | — | — | — |
| compare($a, $b) === $expected | 74 | pending | — | — | — |
| isGreaterThan($a, $b) === $expected | 102 | pending | — | — | — |
| equals($a, $b) === $expected | 115 | pending | — | — | — |
| getSatisfyingVersion with exact match ($range) === $expected | 136 | pending | — | — | — |
| getSatisfyingVersion with range operator ($range) === $expected | 149 | pending | — | — | — |
| getSatisfyingVersion with tilde range ($range) === $expected | 164 | pending | — | — | — |
| should return null for invalid range operators | 175 | pending | — | — | — |
| should return null for empty versions array | 179 | pending | — | — | — |
| should filter out invalid versions | 183 | pending | — | — | — |
| isSingleVersion($version) === $expected | 192 | pending | — | — | — |
| should return false for empty versions | 202 | pending | — | — | — |
| isLessThanRange($version, $range) === $expected | 210 | pending | — | — | — |
| should sort versions correctly | 225 | pending | — | — | — |
| should compare release numbers when version parts are equal | 236 | pending | — | — | — |
| should parse complex versions ($version) === $expected | 246 | pending | — | — | — |
| should identify stable versions ($version) === $expected | 261 | pending | — | — | — |
| should compare versions with prerelease identifiers ($a, $b) === $expected | 278 | pending | — | — | — |
| should handle invalid version parsing gracefully | 295 | pending | — | — | — |
| should handle null/undefined inputs | 305 | pending | — | — | — |
| should return false for unstable versions with prerelease | 315 | pending | — | — | — |
| should return false for empty versions in isStable | 321 | pending | — | — | — |
| should handle versions with different major versions in tilde range | 329 | pending | — | — | — |
| should handle versions with different minor versions in tilde range | 335 | pending | — | — | — |
| should handle invalid target versions in ranges | 340 | pending | — | — | — |
| should handle versions with prerelease identifiers in ranges | 346 | pending | — | — | — |
| should return null for versions with _p package fix suffix | 358 | pending | — | — | — |
| should return null for invalid versions | 364 | pending | — | — | — |
| should return patch version for non-_p patterns | 370 | pending | — | — | — |
| should handle versions with operators | 376 | pending | — | — | — |
| should strip revision from newVersion when currentValue has no revision | 384 | pending | — | — | — |
| should keep revision in newVersion when currentValue has revision | 394 | pending | — | — | — |
| should handle newVersion without revision when currentValue has no revision | 404 | pending | — | — | — |
| should handle newVersion without revision when currentValue has revision | 414 | pending | — | — | — |
| should handle complex prerelease identifier comparisons | 426 | pending | — | — | — |
| should handle versions with different prerelease patterns | 438 | pending | — | — | — |
| should handle unknown range operators | 445 | pending | — | — | — |
| should handle unhandled range operators that match regex | 456 | pending | — | — | — |
| should handle tilde range with invalid target version | 467 | pending | — | — | — |
| should handle tilde range with invalid version in list | 474 | pending | — | — | — |
| should handle major-only versions without minor/patch | 485 | pending | — | — | — |
| should handle letter vs number at same position in version parts | 494 | pending | — | — | — |
| should handle number vs letter comparison in version parts | 499 | pending | — | — | — |
| should handle extra numeric parts in remaining segments | 504 | pending | — | — | — |
| should handle lexicographic string comparison in version parts | 509 | pending | — | — | — |
| should handle equal letter parts continuing to next segment | 514 | pending | — | — | — |
| should handle trailing letter in remaining segments | 519 | pending | — | — | — |
| should return 0 for numerically equal but string-different versions | 524 | pending | — | — | — |
| should handle versions with different extra segment lengths | 528 | pending | — | — | — |

---

