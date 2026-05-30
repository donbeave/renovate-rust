# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/filter-checks.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/filter-checks.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/process/lookup/filter-checks › .filterInternalChecks()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns latest release if internalChecksFilter=none | 74 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| uses datasource-level interception mechanism | 87 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| returns non-pending latest release if internalChecksFilter=flexible and none pass checks | 121 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| returns pending latest release if internalChecksFilter=strict and none pass checks | 135 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| returns non-latest release if internalChecksFilter=strict and some pass checks | 149 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| returns non-latest release if internalChecksFilter=flexible and some pass checks | 163 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| picks up minimumReleaseAge settings from packageRules | 177 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| picks up minimumReleaseAge settings from updateType | 194 | not-applicable | — | — | Lookup filter checks not implemented in Rust |

### `workers/repository/process/lookup/filter-checks › .filterInternalChecks() › if internalChecksFilter=strict, minimumReleaseAge is specified, and the latest release does not have a releaseTimestamp`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not return the latest release, if minimumReleaseAgeBehaviour=timestamp-required | 218 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| returns the latest release, if minimumReleaseAgeBehaviour=timestamp-optional | 252 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| returns latest release, if minimumReleaseAgeBehaviour=timestamp-required but minimumReleaseAge=0 days | 286 | not-applicable | — | — | Lookup filter checks not implemented in Rust |
| picks up minimumConfidence settings from updateType | 321 | not-applicable | — | — | Lookup filter checks not implemented in Rust |

---

