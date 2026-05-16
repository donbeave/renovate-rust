# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/stats.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/stats.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/stats › makeTimingReport`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports empty data | 21 | not-applicable | — | — | Renovate's TypeScript global timing/statistics utility is not implemented as a Rust API; Rust CLI output stats are a separate reporting surface. |
| supports single data point | 32 | not-applicable | — | — | Renovate's TypeScript global timing/statistics utility is not implemented as a Rust API; Rust CLI output stats are a separate reporting surface. |
| supports multiple data points | 43 | not-applicable | — | — | Renovate's TypeScript global timing/statistics utility is not implemented as a Rust API; Rust CLI output stats are a separate reporting surface. |

### `util/stats › LookupStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 64 | not-applicable | — | — | Renovate's TypeScript global lookup-stat accumulator is not implemented as a Rust API. |
| writes data points | 69 | not-applicable | — | — | Renovate's TypeScript global lookup-stat accumulator is not implemented as a Rust API. |
| wraps a function | 95 | not-applicable | — | — | Renovate's TypeScript async timing wrapper is not implemented as a Rust API. |
| logs report | 113 | not-applicable | — | — | Renovate's TypeScript global stats logger side effect is not implemented as a Rust API. |

### `util/stats › GetDatasourceReleasesStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 152 | not-applicable | — | — | Renovate's TypeScript global datasource-release stats accumulator is not implemented as a Rust API. |
| writes data points | 166 | not-applicable | — | — | Renovate's TypeScript global datasource-release stats accumulator is not implemented as a Rust API. |
| wraps a function | 308 | not-applicable | — | — | Renovate's TypeScript async timing wrapper is not implemented as a Rust API. |
| logs report | 362 | not-applicable | — | — | Renovate's TypeScript global stats logger side effect is not implemented as a Rust API. |

### `util/stats › PackageCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 578 | not-applicable | — | — | Renovate's TypeScript package-cache stats accumulator is not implemented as a Rust API. |
| writes data points | 586 | not-applicable | — | — | Renovate's TypeScript package-cache stats accumulator is not implemented as a Rust API. |
| wraps get function | 612 | not-applicable | — | — | Renovate's TypeScript package-cache async timing wrapper is not implemented as a Rust API. |
| wraps set function | 625 | not-applicable | — | — | Renovate's TypeScript package-cache async timing wrapper is not implemented as a Rust API. |
| logs report | 637 | not-applicable | — | — | Renovate's TypeScript package-cache stats logger side effect is not implemented as a Rust API. |

### `util/stats › DatasourceCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| collects data points | 668 | not-applicable | — | — | Renovate's TypeScript datasource-cache stats accumulator is not implemented as a Rust API. |
| reports | 708 | not-applicable | — | — | Renovate's TypeScript datasource-cache stats logger side effect is not implemented as a Rust API. |

### `util/stats › HttpStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 722 | not-applicable | — | — | Renovate's TypeScript HTTP stats accumulator is not implemented as a Rust API. |
| writes data points | 733 | not-applicable | — | — | Renovate's TypeScript HTTP stats accumulator is not implemented as a Rust API. |
| logs report | 839 | not-applicable | — | — | Renovate's TypeScript HTTP stats logger side effect is not implemented as a Rust API. |

### `util/stats › HttpCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty data | 954 | not-applicable | — | — | Renovate's TypeScript HTTP-cache stats accumulator is not implemented as a Rust API. |
| ignores wrong url | 959 | not-applicable | — | — | Renovate's TypeScript HTTP-cache stats accumulator is not implemented as a Rust API. |
| writes data points | 964 | not-applicable | — | — | Renovate's TypeScript HTTP-cache stats accumulator is not implemented as a Rust API. |
| prints report | 989 | not-applicable | — | — | Renovate's TypeScript HTTP-cache stats logger side effect is not implemented as a Rust API. |

### `util/stats › AbandonedPackageStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 1016 | not-applicable | — | — | Renovate's TypeScript abandoned-package stats accumulator is not implemented as a Rust API. |
| writes data points | 1021 | not-applicable | — | — | Renovate's TypeScript abandoned-package stats accumulator is not implemented as a Rust API. |
| logs report | 1069 | not-applicable | — | — | Renovate's TypeScript abandoned-package stats logger side effect is not implemented as a Rust API. |
| does not log report when no data | 1096 | not-applicable | — | — | Renovate's TypeScript abandoned-package stats logger side effect is not implemented as a Rust API. |

### `util/stats › GitOperationsStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 1112 | not-applicable | — | — | Renovate's TypeScript git-operation stats accumulator is not implemented as a Rust API. |
| writes data points | 1117 | not-applicable | — | — | Renovate's TypeScript git-operation stats accumulator is not implemented as a Rust API. |
| rounds total towards ceiling when preparing report | 1141 | not-applicable | — | — | Renovate's TypeScript git-operation stats accumulator is not implemented as a Rust API. |
| logs report | 1161 | not-applicable | — | — | Renovate's TypeScript git-operation stats logger side effect is not implemented as a Rust API. |

---

