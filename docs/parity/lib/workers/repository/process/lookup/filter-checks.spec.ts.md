# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/filter-checks.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/filter-checks.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** done

### `workers/repository/process/lookup/filter-checks › .filterInternalChecks()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns latest release if internalChecksFilter=none | 74 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| uses datasource-level interception mechanism | 87 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| returns non-pending latest release if internalChecksFilter=flexible and none pass checks | 121 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| returns pending latest release if internalChecksFilter=strict and none pass checks | 135 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| returns non-latest release if internalChecksFilter=strict and some pass checks | 149 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| returns non-latest release if internalChecksFilter=flexible and some pass checks | 163 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| picks up minimumReleaseAge settings from packageRules | 177 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| picks up minimumReleaseAge settings from updateType | 194 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |

### `workers/repository/process/lookup/filter-checks › .filterInternalChecks() › if internalChecksFilter=strict, minimumReleaseAge is specified, and the latest release does not have a releaseTimestamp`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not return the latest release, if minimumReleaseAgeBehaviour=timestamp-required | 218 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| returns the latest release, if minimumReleaseAgeBehaviour=timestamp-optional | 252 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| returns latest release, if minimumReleaseAgeBehaviour=timestamp-required but minimumReleaseAge=0 days | 286 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |
| picks up minimumConfidence settings from updateType | 321 | not-applicable | — | — | Requires vi.mock datasource/host-rules mock infrastructure |

---

