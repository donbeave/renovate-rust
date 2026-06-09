# `lib/util/stats.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**14/32 in-scope tests ported** (18 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 21 | supports empty data | ported | [`crates/renovate-core/src/util.rs:7336`](../../../../../crates/renovate-core/src/util.rs#L7336) |
| 32 | supports single data point | ported | [`crates/renovate-core/src/util.rs:7352`](../../../../../crates/renovate-core/src/util.rs#L7352) |
| 43 | supports multiple data points | ported | [`crates/renovate-core/src/util.rs:7368`](../../../../../crates/renovate-core/src/util.rs#L7368) |
| 64 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7381`](../../../../../crates/renovate-core/src/util.rs#L7381) |
| 69 | writes data points | ported | [`crates/renovate-core/src/util.rs:7393`](../../../../../crates/renovate-core/src/util.rs#L7393) |
| 95 | wraps a function | ported | [`crates/renovate-core/src/util.rs:7739`](../../../../../crates/renovate-core/src/util.rs#L7739) |
| 113 | logs report | ported | [`crates/renovate-core/src/util.rs:7753`](../../../../../crates/renovate-core/src/util.rs#L7753) |
| 152 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7381`](../../../../../crates/renovate-core/src/util.rs#L7381) |
| 166 | writes data points | ported | [`crates/renovate-core/src/util.rs:7393`](../../../../../crates/renovate-core/src/util.rs#L7393) |
| 308 | wraps a function | ported | [`crates/renovate-core/src/util.rs:7739`](../../../../../crates/renovate-core/src/util.rs#L7739) |
| 362 | logs report | ported | [`crates/renovate-core/src/util.rs:7753`](../../../../../crates/renovate-core/src/util.rs#L7753) |
| 578 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7381`](../../../../../crates/renovate-core/src/util.rs#L7381) |
| 586 | writes data points | ported | [`crates/renovate-core/src/util.rs:7393`](../../../../../crates/renovate-core/src/util.rs#L7393) |
| 612 | wraps get function | ported | [`crates/renovate-core/src/util.rs:7781`](../../../../../crates/renovate-core/src/util.rs#L7781) |
| 625 | wraps set function | ported | [`crates/renovate-core/src/util.rs:7791`](../../../../../crates/renovate-core/src/util.rs#L7791) |
| 637 | logs report | ported | [`crates/renovate-core/src/util.rs:7753`](../../../../../crates/renovate-core/src/util.rs#L7753) |
| 668 | collects data points | ported | [`crates/renovate-core/src/util.rs:7419`](../../../../../crates/renovate-core/src/util.rs#L7419) |
| 708 | reports | ported | [`crates/renovate-core/src/util.rs:7482`](../../../../../crates/renovate-core/src/util.rs#L7482) |
| 722 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7381`](../../../../../crates/renovate-core/src/util.rs#L7381) |
| 733 | writes data points | ported | [`crates/renovate-core/src/util.rs:7393`](../../../../../crates/renovate-core/src/util.rs#L7393) |
| 839 | logs report | ported | [`crates/renovate-core/src/util.rs:7753`](../../../../../crates/renovate-core/src/util.rs#L7753) |
| 954 | returns empty data | ported | [`crates/renovate-core/src/util.rs:7530`](../../../../../crates/renovate-core/src/util.rs#L7530) |
| 959 | ignores wrong url | ported | [`crates/renovate-core/src/util.rs:7537`](../../../../../crates/renovate-core/src/util.rs#L7537) |
| 964 | writes data points | ported | [`crates/renovate-core/src/util.rs:7393`](../../../../../crates/renovate-core/src/util.rs#L7393) |
| 989 | prints report | opt-out | asserts TypeScript logger.debug spy behavior (exact call with 'HTTP cache statistics' message and specific aggregated data object for HttpCacheStats.report()); the core stats tracking (inc* methods) and report generation are covered by other ported tests like 'logs report'; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing) or test harness |
| 1016 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7381`](../../../../../crates/renovate-core/src/util.rs#L7381) |
| 1021 | writes data points | ported | [`crates/renovate-core/src/util.rs:7393`](../../../../../crates/renovate-core/src/util.rs#L7393) |
| 1069 | logs report | ported | [`crates/renovate-core/src/util.rs:7753`](../../../../../crates/renovate-core/src/util.rs#L7753) |
| 1096 | does not log report when no data | pending | — |
| 1112 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7381`](../../../../../crates/renovate-core/src/util.rs#L7381) |
| 1117 | writes data points | ported | [`crates/renovate-core/src/util.rs:7393`](../../../../../crates/renovate-core/src/util.rs#L7393) |
| 1141 | rounds total towards ceiling when preparing report | ported | [`crates/renovate-core/src/util.rs:7661`](../../../../../crates/renovate-core/src/util.rs#L7661) |
| 1161 | logs report | ported | [`crates/renovate-core/src/util.rs:7753`](../../../../../crates/renovate-core/src/util.rs#L7753) |

