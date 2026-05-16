# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/instrumentation/reporting.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/reporting.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `instrumentation/reporting`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return empty report if no stats have been added | 74 | not-applicable | — | — | JavaScript instrumentation report accumulator for `reportType`; Rust CLI has a separate report output model and does not expose Renovate's JS reporting module. |
| return report if reportType is set to logging | 93 | not-applicable | — | — | JavaScript instrumentation report accumulator for `reportType`; Rust CLI has a separate report output model and does not expose Renovate's JS reporting module. |
| log report if reportType is set to logging | 105 | not-applicable | — | — | JavaScript instrumentation report export via logger; Rust CLI has a separate report output model and no JS `reportType=logging` exporter. |
| write report if reportType is set to file | 122 | not-applicable | — | — | JavaScript instrumentation report export via `writeSystemFile`; Rust CLI has a separate report output model and no JS `reportType=file` exporter. |
| write formatted report if reportFormatting is enabled | 139 | not-applicable | — | — | JavaScript instrumentation report formatting via Prettier; Rust CLI has a separate report output model and no JS report formatter. |
| send report to an S3 bucket if reportType is s3 | 157 | not-applicable | — | — | JavaScript instrumentation report export via AWS S3 client; Rust CLI has no equivalent JS `reportType=s3` exporter. |
| handle failed parsing of S3 url | 179 | not-applicable | — | — | JavaScript instrumentation report export via AWS S3 client; Rust CLI has no equivalent JS `reportType=s3` exporter. |
| catch exception | 199 | not-applicable | — | — | JavaScript instrumentation report exporter failure handling; Rust CLI has a separate report output model and does not expose this exporter. |
| reports nothing when reportType=null | 213 | not-applicable | — | — | JavaScript instrumentation `reportType` gating; Rust CLI has a separate report output model and does not expose Renovate's JS reporting module. |
| should add problems to report | 226 | not-applicable | — | — | JavaScript `ProblemStream` integration for instrumentation reports; Rust logging does not expose Renovate's JS problem stream. |
| should handle libyears addition | 271 | not-applicable | — | — | JavaScript instrumentation report libyears accumulator; Rust CLI has a separate report output model and does not expose this JS reporting module. |

---

