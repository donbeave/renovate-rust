# `lib/util/stats.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**14/32 in-scope tests ported** (18 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 21 | supports empty data | ported | [`crates/renovate-core/src/util.rs:7335`](../../../../../crates/renovate-core/src/util.rs#L7335) |
| 32 | supports single data point | ported | [`crates/renovate-core/src/util.rs:7351`](../../../../../crates/renovate-core/src/util.rs#L7351) |
| 43 | supports multiple data points | ported | [`crates/renovate-core/src/util.rs:7367`](../../../../../crates/renovate-core/src/util.rs#L7367) |
| 64 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7380`](../../../../../crates/renovate-core/src/util.rs#L7380) |
| 69 | writes data points | ported | [`crates/renovate-core/src/util.rs:7392`](../../../../../crates/renovate-core/src/util.rs#L7392) |
| 95 | wraps a function | ported | [`crates/renovate-core/src/util.rs:7738`](../../../../../crates/renovate-core/src/util.rs#L7738) |
| 113 | logs report | ported | [`crates/renovate-core/src/util.rs:7752`](../../../../../crates/renovate-core/src/util.rs#L7752) |
| 152 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7380`](../../../../../crates/renovate-core/src/util.rs#L7380) |
| 166 | writes data points | ported | [`crates/renovate-core/src/util.rs:7392`](../../../../../crates/renovate-core/src/util.rs#L7392) |
| 308 | wraps a function | ported | [`crates/renovate-core/src/util.rs:7738`](../../../../../crates/renovate-core/src/util.rs#L7738) |
| 362 | logs report | ported | [`crates/renovate-core/src/util.rs:7752`](../../../../../crates/renovate-core/src/util.rs#L7752) |
| 578 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7380`](../../../../../crates/renovate-core/src/util.rs#L7380) |
| 586 | writes data points | ported | [`crates/renovate-core/src/util.rs:7392`](../../../../../crates/renovate-core/src/util.rs#L7392) |
| 612 | wraps get function | ported | [`crates/renovate-core/src/util.rs:7780`](../../../../../crates/renovate-core/src/util.rs#L7780) |
| 625 | wraps set function | ported | [`crates/renovate-core/src/util.rs:7790`](../../../../../crates/renovate-core/src/util.rs#L7790) |
| 637 | logs report | ported | [`crates/renovate-core/src/util.rs:7752`](../../../../../crates/renovate-core/src/util.rs#L7752) |
| 668 | collects data points | ported | [`crates/renovate-core/src/util.rs:7418`](../../../../../crates/renovate-core/src/util.rs#L7418) |
| 708 | reports | ported | [`crates/renovate-core/src/util.rs:7481`](../../../../../crates/renovate-core/src/util.rs#L7481) |
| 722 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7380`](../../../../../crates/renovate-core/src/util.rs#L7380) |
| 733 | writes data points | ported | [`crates/renovate-core/src/util.rs:7392`](../../../../../crates/renovate-core/src/util.rs#L7392) |
| 839 | logs report | ported | [`crates/renovate-core/src/util.rs:7752`](../../../../../crates/renovate-core/src/util.rs#L7752) |
| 954 | returns empty data | ported | [`crates/renovate-core/src/util.rs:7529`](../../../../../crates/renovate-core/src/util.rs#L7529) |
| 959 | ignores wrong url | ported | [`crates/renovate-core/src/util.rs:7536`](../../../../../crates/renovate-core/src/util.rs#L7536) |
| 964 | writes data points | ported | [`crates/renovate-core/src/util.rs:7392`](../../../../../crates/renovate-core/src/util.rs#L7392) |
| 989 | prints report | opt-out | asserts TypeScript logger.debug spy behavior (exact call with 'HTTP cache statistics' message and specific aggregated data object for HttpCacheStats.report()); the core stats tracking (inc* methods) and report generation are covered by other ported tests like 'logs report'; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing) or test harness |
| 1016 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7380`](../../../../../crates/renovate-core/src/util.rs#L7380) |
| 1021 | writes data points | ported | [`crates/renovate-core/src/util.rs:7392`](../../../../../crates/renovate-core/src/util.rs#L7392) |
| 1069 | logs report | ported | [`crates/renovate-core/src/util.rs:7752`](../../../../../crates/renovate-core/src/util.rs#L7752) |
| 1096 | does not log report when no data | pending | — |
| 1112 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7380`](../../../../../crates/renovate-core/src/util.rs#L7380) |
| 1117 | writes data points | ported | [`crates/renovate-core/src/util.rs:7392`](../../../../../crates/renovate-core/src/util.rs#L7392) |
| 1141 | rounds total towards ceiling when preparing report | ported | [`crates/renovate-core/src/util.rs:7660`](../../../../../crates/renovate-core/src/util.rs#L7660) |
| 1161 | logs report | ported | [`crates/renovate-core/src/util.rs:7752`](../../../../../crates/renovate-core/src/util.rs#L7752) |

