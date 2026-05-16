# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/semver/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/semver/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/semver/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 4 | not-applicable | — | — | Renovate's full SemVer `VersioningApi` validation contract is not implemented as a Rust API; Rust currently exposes narrower generic semver update-summary helpers. |
| isSingleVersion("$version") === $expected | 22 | not-applicable | — | — | Renovate's SemVer single-version classifier is not implemented as a Rust API; Rust currently exposes narrower generic semver update-summary helpers. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 34 | not-applicable | — | — | Renovate's SemVer update-value helper is not implemented as a Rust API; Rust currently exposes narrower generic semver update-summary helpers. |
| isBreaking("$currentVersion", "$newVersion") === $expected | 51 | not-applicable | — | — | Renovate's SemVer breaking-change helper is not implemented as a Rust API; Rust currently exposes narrower generic semver update classification helpers. |
| isCompatible("$version") === $expected | 72 | not-applicable | — | — | Renovate's SemVer compatibility helper is not implemented as a Rust API; Rust currently exposes narrower generic semver update-summary helpers. |

---

