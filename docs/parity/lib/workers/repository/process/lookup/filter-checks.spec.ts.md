# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/filter-checks.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/filter-checks.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** not-applicable

### `workers/repository/process/lookup/filter-checks › .filterInternalChecks()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns latest release if internalChecksFilter=none | 74 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| uses datasource-level interception mechanism | 87 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| returns non-pending latest release if internalChecksFilter=flexible and none pass checks | 121 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| returns pending latest release if internalChecksFilter=strict and none pass checks | 135 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| returns non-latest release if internalChecksFilter=strict and some pass checks | 149 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| returns non-latest release if internalChecksFilter=flexible and some pass checks | 163 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| picks up minimumReleaseAge settings from packageRules | 177 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| picks up minimumReleaseAge settings from updateType | 194 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|

### `workers/repository/process/lookup/filter-checks › .filterInternalChecks() › if internalChecksFilter=strict, minimumReleaseAge is specified, and the latest release does not have a releaseTimestamp`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not return the latest release, if minimumReleaseAgeBehaviour=timestamp-required | 218 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| returns the latest release, if minimumReleaseAgeBehaviour=timestamp-optional | 252 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| returns latest release, if minimumReleaseAgeBehaviour=timestamp-required but minimumReleaseAge=0 days | 286 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|
| picks up minimumConfidence settings from updateType | 321 | not-applicable | — | — | mocking framework internals — vi.spyOn on version/config lookups; TypeScript filter-checks pipeline|

---

