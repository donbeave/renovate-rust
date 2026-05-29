# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/stats.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/stats.spec.ts
**Total tests:** 33 | **Ported:** 24 | **Actionable:** 24 | **Status:** ported

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
| wraps a function | 95 | ported | `util.rs` | `test_lookup_stats_wraps_function` | — |
| logs report | 113 | not-applicable | — | — | mocking framework internals — tests logger.debug spy only; data computation covered by writes/report tests |

### `util/stats › GetDatasourceReleasesStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 152 | ported | `util.rs` | `test_get_datasource_releases_stats_empty` | — |
| writes data points | 166 | ported | `util.rs` | `test_get_datasource_releases_stats_writes` | — |
| wraps a function | 308 | ported | `util.rs` | `test_get_datasource_releases_stats_wraps_function` | — |
| logs report | 362 | not-applicable | — | — | mocking framework internals — tests logger.trace spy only |

### `util/stats › PackageCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 578 | ported | `util.rs` | `test_package_cache_stats_empty_report` | — |
| writes data points | 586 | ported | `util.rs` | `test_package_cache_stats_writes_data_points` | — |
| wraps get function | 612 | ported | `util.rs` | `test_package_cache_stats_wraps_get_function` | — |
| wraps set function | 625 | ported | `util.rs` | `test_package_cache_stats_wraps_set_function` | — |
| logs report | 637 | not-applicable | — | — | mocking framework internals — tests logger.debug spy only |

### `util/stats › DatasourceCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| collects data points | 668 | ported | `util.rs` | `test_datasource_cache_stats_collects` | — |
| reports | 708 | not-applicable | — | — | mocking framework internals — tests logger.trace spy only |

### `util/stats › HttpStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 722 | ported | `util.rs` | `test_http_stats_empty_report` | — |
| writes data points | 733 | ported | `util.rs` | `test_http_stats_writes_data_points` | — |
| logs report | 839 | not-applicable | — | — | mocking framework internals — tests logger.debug spy only |

### `util/stats › HttpCacheStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty data | 954 | ported | `util.rs` | `test_http_cache_stats_empty` | — |
| ignores wrong url | 959 | ported | `util.rs` | `test_http_cache_stats_ignores_invalid_url` | — |
| writes data points | 964 | ported | `util.rs` | `test_http_cache_stats_writes_data_points` | — |
| prints report | 989 | not-applicable | — | — | mocking framework internals — tests logger.debug spy only |

### `util/stats › AbandonedPackageStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 1016 | ported | `util.rs` | `test_abandoned_package_stats_empty_report` | — |
| writes data points | 1021 | ported | `util.rs` | `test_abandoned_package_stats_writes_data_points` | — |
| logs report | 1069 | not-applicable | — | — | mocking framework internals — tests logger.debug spy only |
| does not log report when no data | 1096 | not-applicable | — | — | mocking framework internals — tests logger.debug spy only |

### `util/stats › GitOperationsStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty report | 1112 | ported | `util.rs` | `test_git_operation_stats_empty_report` | — |
| writes data points | 1117 | ported | `util.rs` | `test_git_operation_stats_writes_data_points` | — |
| rounds total towards ceiling when preparing report | 1141 | ported | `util.rs` | `test_git_operation_stats_ceils_total` | — |
| logs report | 1161 | not-applicable | — | — | mocking framework internals — tests logger.debug spy only |

---
