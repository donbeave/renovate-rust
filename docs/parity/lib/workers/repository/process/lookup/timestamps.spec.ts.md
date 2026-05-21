# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/timestamps.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/timestamps.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** not-applicable

### `workers/repository/process/lookup/timestamps › calculateLatestReleaseBump`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the timestamp of the latest version | 10 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| handles releases with missing timestamps | 33 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| handles latest release with missing timestamp | 53 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| handles latest release with deprecation flag | 75 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| handles latest release with invalid version | 99 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| returns undefined mostRecentTimestamp when no valid timestamps exist | 122 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| handles empty releases array | 132 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| preserves other properties in the release result | 138 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| handles ancient versions that are higher than the ones recently released | 160 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |
| handles errors thrown for invalid versions | 180 | not-applicable | — | — | tests calculateMostRecentTimestamp with TypeScript semver versioning API |

---

