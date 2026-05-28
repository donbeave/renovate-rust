# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/stats.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/stats.spec.ts
**Total tests:** 33 | **Ported:** 16 | **Actionable:** 33 | **Status:** partial

### `util/stats › makeTimingReport`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports empty data | 21 | ported | `util.rs` | `test_make_timing_report_empty` | — |
| supports single data point | 32 | ported | `util.rs` | `test_make_timing_report_single` | — |
| supports multiple data points | 43 | ported | `util.rs` | `test_make_timing_report_multiple` | — |

### `util/stats › LookupStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 64 | ported | `util.rs` | `test_lookup_stats_empty_report` | — |
| writes data points | 69 | ported | `util.rs` | `test_lookup_stats_writes_data_points` | — |
| wraps a function | 95 | not-applicable | — | — | Uses vi.useFakeTimers() to advance time; not portable without fake-timer test infrastructure |
| logs report | 113 | not-applicable | — | — | Checks logger.logger.debug spy; not portable without tracing test infrastructure |

### `util/stats › GetDatasourceReleasesStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 152 | pending | — | — | — |
| writes data points | 166 | pending | — | — | — |
| wraps a function | 308 | not-applicable | — | — | Uses vi.useFakeTimers() to advance time; not portable without fake-timer test infrastructure |
| logs report | 362 | not-applicable | — | — | Checks logger.logger.debug spy; not portable without tracing test infrastructure |

### `util/stats › PackageCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 578 | ported | `util.rs` | `test_package_cache_stats_empty_report` | — |
| writes data points | 586 | ported | `util.rs` | `test_package_cache_stats_writes_data_points` | — |
| wraps get function | 612 | not-applicable | — | — | Uses vi.useFakeTimers() to advance time; not portable without fake-timer test infrastructure |
| wraps set function | 625 | not-applicable | — | — | Uses vi.useFakeTimers() to advance time; not portable without fake-timer test infrastructure |
| logs report | 637 | not-applicable | — | — | Checks logger.logger.debug spy; not portable without tracing test infrastructure |

### `util/stats › DatasourceCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| collects data points | 668 | pending | — | — | — |
| reports | 708 | not-applicable | — | — | Checks logger.logger.trace/debug spy; not portable without tracing test infrastructure |

### `util/stats › HttpStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 722 | ported | `util.rs` | `test_http_stats_empty_report` | — |
| writes data points | 733 | ported | `util.rs` | `test_http_stats_writes_data_points` | — |
| logs report | 839 | not-applicable | — | — | Checks logger.logger.debug spy; not portable without tracing test infrastructure |

### `util/stats › HttpCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty data | 954 | ported | `util.rs` | `test_http_cache_stats_empty` | — |
| ignores wrong url | 959 | ported | `util.rs` | `test_http_cache_stats_ignores_invalid_url` | — |
| writes data points | 964 | ported | `util.rs` | `test_http_cache_stats_writes_data_points` | — |
| prints report | 989 | not-applicable | — | — | Checks logger.logger.debug spy; not portable without tracing test infrastructure |

### `util/stats › AbandonedPackageStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 1016 | ported | `util.rs` | `test_abandoned_package_stats_empty_report` | — |
| writes data points | 1021 | ported | `util.rs` | `test_abandoned_package_stats_writes_data_points` | — |
| logs report | 1069 | not-applicable | — | — | Checks logger.logger.debug spy; not portable without tracing test infrastructure |
| does not log report when no data | 1096 | not-applicable | — | — | Checks logger.logger.debug spy; not portable without tracing test infrastructure |

### `util/stats › GitOperationsStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 1112 | ported | `util.rs` | `test_git_operation_stats_empty_report` | — |
| writes data points | 1117 | ported | `util.rs` | `test_git_operation_stats_writes_data_points` | — |
| rounds total towards ceiling when preparing report | 1141 | pending | — | — | — |
| logs report | 1161 | not-applicable | — | — | Checks logger.logger.debug spy; not portable without tracing test infrastructure |

---
