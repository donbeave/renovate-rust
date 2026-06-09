# `lib/util/stats.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**14/31 in-scope tests ported** (17 pending, 2 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 21 | supports empty data | ported | [`crates/renovate-core/src/util.rs:7339`](../../../../../crates/renovate-core/src/util.rs#L7339) |
| 32 | supports single data point | ported | [`crates/renovate-core/src/util.rs:7355`](../../../../../crates/renovate-core/src/util.rs#L7355) |
| 43 | supports multiple data points | ported | [`crates/renovate-core/src/util.rs:7371`](../../../../../crates/renovate-core/src/util.rs#L7371) |
| 64 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7384`](../../../../../crates/renovate-core/src/util.rs#L7384) |
| 69 | writes data points | ported | [`crates/renovate-core/src/util.rs:7396`](../../../../../crates/renovate-core/src/util.rs#L7396) |
| 95 | wraps a function | ported | [`crates/renovate-core/src/util.rs:7742`](../../../../../crates/renovate-core/src/util.rs#L7742) |
| 113 | logs report | ported | [`crates/renovate-core/src/util.rs:7756`](../../../../../crates/renovate-core/src/util.rs#L7756) |
| 152 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7384`](../../../../../crates/renovate-core/src/util.rs#L7384) |
| 166 | writes data points | ported | [`crates/renovate-core/src/util.rs:7396`](../../../../../crates/renovate-core/src/util.rs#L7396) |
| 308 | wraps a function | ported | [`crates/renovate-core/src/util.rs:7742`](../../../../../crates/renovate-core/src/util.rs#L7742) |
| 362 | logs report | ported | [`crates/renovate-core/src/util.rs:7756`](../../../../../crates/renovate-core/src/util.rs#L7756) |
| 578 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7384`](../../../../../crates/renovate-core/src/util.rs#L7384) |
| 586 | writes data points | ported | [`crates/renovate-core/src/util.rs:7396`](../../../../../crates/renovate-core/src/util.rs#L7396) |
| 612 | wraps get function | ported | [`crates/renovate-core/src/util.rs:7784`](../../../../../crates/renovate-core/src/util.rs#L7784) |
| 625 | wraps set function | ported | [`crates/renovate-core/src/util.rs:7794`](../../../../../crates/renovate-core/src/util.rs#L7794) |
| 637 | logs report | ported | [`crates/renovate-core/src/util.rs:7756`](../../../../../crates/renovate-core/src/util.rs#L7756) |
| 668 | collects data points | ported | [`crates/renovate-core/src/util.rs:7422`](../../../../../crates/renovate-core/src/util.rs#L7422) |
| 708 | reports | ported | [`crates/renovate-core/src/util.rs:7485`](../../../../../crates/renovate-core/src/util.rs#L7485) |
| 722 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7384`](../../../../../crates/renovate-core/src/util.rs#L7384) |
| 733 | writes data points | ported | [`crates/renovate-core/src/util.rs:7396`](../../../../../crates/renovate-core/src/util.rs#L7396) |
| 839 | logs report | ported | [`crates/renovate-core/src/util.rs:7756`](../../../../../crates/renovate-core/src/util.rs#L7756) |
| 954 | returns empty data | ported | [`crates/renovate-core/src/util.rs:7533`](../../../../../crates/renovate-core/src/util.rs#L7533) |
| 959 | ignores wrong url | ported | [`crates/renovate-core/src/util.rs:7540`](../../../../../crates/renovate-core/src/util.rs#L7540) |
| 964 | writes data points | ported | [`crates/renovate-core/src/util.rs:7396`](../../../../../crates/renovate-core/src/util.rs#L7396) |
| 989 | prints report | opt-out | asserts TypeScript logger.debug spy behavior (exact call with 'HTTP cache statistics' message and specific aggregated data object for HttpCacheStats.report()); the core stats tracking (inc* methods) and report generation are covered by other ported tests like 'logs report'; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing) or test harness |
| 1016 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7384`](../../../../../crates/renovate-core/src/util.rs#L7384) |
| 1021 | writes data points | ported | [`crates/renovate-core/src/util.rs:7396`](../../../../../crates/renovate-core/src/util.rs#L7396) |
| 1069 | logs report | ported | [`crates/renovate-core/src/util.rs:7756`](../../../../../crates/renovate-core/src/util.rs#L7756) |
| 1096 | does not log report when no data | opt-out | asserts TypeScript logger.debug spy not called (via .not.toHaveBeenCalled) for AbandonedPackageStats.report() when no data; the conditional logging guard (only report when has data) and empty-report path are covered by already-ported tests ('logs report', 'returns empty report', AbandonedPackageStats tests in util.rs); this test is a pure spy assertion with no Rust logger equivalent (Rust stats use internal counters + tracing for reports); matches prior opt-out for sibling 'prints report' spy in same spec. |
| 1112 | returns empty report | ported | [`crates/renovate-core/src/util.rs:7384`](../../../../../crates/renovate-core/src/util.rs#L7384) |
| 1117 | writes data points | ported | [`crates/renovate-core/src/util.rs:7396`](../../../../../crates/renovate-core/src/util.rs#L7396) |
| 1141 | rounds total towards ceiling when preparing report | ported | [`crates/renovate-core/src/util.rs:7664`](../../../../../crates/renovate-core/src/util.rs#L7664) |
| 1161 | logs report | ported | [`crates/renovate-core/src/util.rs:7756`](../../../../../crates/renovate-core/src/util.rs#L7756) |

