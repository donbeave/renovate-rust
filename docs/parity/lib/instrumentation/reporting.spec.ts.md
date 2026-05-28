# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/instrumentation/reporting.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/reporting.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** done

### `instrumentation/reporting`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return empty report if no stats have been added | 74 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| return report if reportType is set to logging | 93 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| log report if reportType is set to logging | 105 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| write report if reportType is set to file | 122 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| write formatted report if reportFormatting is enabled | 139 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| send report to an S3 bucket if reportType is s3 | 157 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| handle failed parsing of S3 url | 179 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| catch exception | 199 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| reports nothing when reportType=null | 213 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| should add problems to report | 226 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |
| should handle libyears addition | 271 | not-applicable | — | — | Requires S3 mock + vi.mock logger/fs mock infrastructure |

---

