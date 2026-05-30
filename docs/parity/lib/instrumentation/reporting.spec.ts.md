# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/instrumentation/reporting.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/reporting.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable-applicable-applicable

### `instrumentation/reporting`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return empty report if no stats have been added | 74 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| return report if reportType is set to logging | 93 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| log report if reportType is set to logging | 105 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| write report if reportType is set to file | 122 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| write formatted report if reportFormatting is enabled | 139 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| send report to an S3 bucket if reportType is s3 | 157 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| handle failed parsing of S3 url | 179 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| catch exception | 199 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| reports nothing when reportType=null | 213 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| should add problems to report | 226 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| should handle libyears addition | 271 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|

---

