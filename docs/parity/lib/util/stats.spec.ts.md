# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/stats.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/stats.spec.ts
**Total tests:** 33 | **Ported:** 3 | **Actionable:** 33 | **Status:** partial

### `util/stats › makeTimingReport`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports empty data | 21 | ported | `util.rs` | `test_make_timing_report_empty` | — |
| supports single data point | 32 | ported | `util.rs` | `test_make_timing_report_single` | — |
| supports multiple data points | 43 | ported | `util.rs` | `test_make_timing_report_multiple` | — |

### `util/stats › LookupStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 64 | pending | — | — | — |
| writes data points | 69 | pending | — | — | — |
| wraps a function | 95 | pending | — | — | — |
| logs report | 113 | pending | — | — | — |

### `util/stats › GetDatasourceReleasesStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 152 | pending | — | — | — |
| writes data points | 166 | pending | — | — | — |
| wraps a function | 308 | pending | — | — | — |
| logs report | 362 | pending | — | — | — |

### `util/stats › PackageCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 578 | pending | — | — | — |
| writes data points | 586 | pending | — | — | — |
| wraps get function | 612 | pending | — | — | — |
| wraps set function | 625 | pending | — | — | — |
| logs report | 637 | pending | — | — | — |

### `util/stats › DatasourceCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| collects data points | 668 | pending | — | — | — |
| reports | 708 | pending | — | — | — |

### `util/stats › HttpStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 722 | pending | — | — | — |
| writes data points | 733 | pending | — | — | — |
| logs report | 839 | pending | — | — | — |

### `util/stats › HttpCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty data | 954 | pending | — | — | — |
| ignores wrong url | 959 | pending | — | — | — |
| writes data points | 964 | pending | — | — | — |
| prints report | 989 | pending | — | — | — |

### `util/stats › AbandonedPackageStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 1016 | pending | — | — | — |
| writes data points | 1021 | pending | — | — | — |
| logs report | 1069 | pending | — | — | — |
| does not log report when no data | 1096 | pending | — | — | — |

### `util/stats › GitOperationsStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 1112 | pending | — | — | — |
| writes data points | 1117 | pending | — | — | — |
| rounds total towards ceiling when preparing report | 1141 | pending | — | — | — |
| logs report | 1161 | pending | — | — | — |

---

