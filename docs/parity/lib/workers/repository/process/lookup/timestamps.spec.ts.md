# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/timestamps.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/timestamps.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `workers/repository/process/lookup/timestamps › calculateLatestReleaseBump`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the timestamp of the latest version | 10 | ported | `util.rs` | `test_timestamps_returns_latest` | — |
| handles releases with missing timestamps | 33 | ported | `util.rs` | `test_timestamps_missing_middle` | — |
| handles latest release with missing timestamp | 53 | ported | `util.rs` | `test_timestamps_latest_no_timestamp` | — |
| handles latest release with deprecation flag | 75 | ported | `util.rs` | `test_timestamps_latest_deprecated` | — |
| handles latest release with invalid version | 99 | ported | `util.rs` | `test_timestamps_invalid_timestamp_for_highest` | — |
| returns undefined mostRecentTimestamp when no valid timestamps exist | 122 | ported | `util.rs` | `test_timestamps_no_valid_timestamps` | — |
| handles empty releases array | 132 | ported | `util.rs` | `test_timestamps_empty_releases` | — |
| preserves other properties in the release result | 138 | ported | `util.rs` | `test_timestamps_single_release` | — |
| handles ancient versions that are higher than the ones recently released | 160 | ported | `util.rs` | `test_timestamps_ancient_high_version` | — |
| handles errors thrown for invalid versions | 180 | ported | `util.rs` | `test_timestamps_invalid_versions_ignored` | — |

---
