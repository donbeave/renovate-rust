# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/pretty-stdout.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/pretty-stdout.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/pretty-stdout › getMeta(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if null rec | 9 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| returns empty string if empty rec | 13 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| returns empty string if no meta fields | 17 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| supports single meta | 24 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| supports multi meta | 34 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| returns plain text when colorize is false | 46 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |

### `logger/pretty-stdout › getDetails(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if null rec | 57 | not-applicable | — | — | Bunyan record detail formatter; Rust logging/output does not expose Renovate's JavaScript pretty-stdout detail formatting API. |
| returns empty string if empty rec | 61 | not-applicable | — | — | Bunyan record detail formatter; Rust logging/output does not expose Renovate's JavaScript pretty-stdout detail formatting API. |
| returns empty string if all are meta fields | 67 | not-applicable | — | — | Bunyan record detail formatter; Rust logging/output does not expose Renovate's JavaScript pretty-stdout detail formatting API. |
| supports a config | 75 | not-applicable | — | — | Bunyan record detail formatter; Rust logging/output does not expose Renovate's JavaScript pretty-stdout detail formatting API. |
| formats err.stack as readable multi-line output | 88 | not-applicable | — | — | JavaScript error stack formatting for Bunyan records; Rust errors and tracing output are not rendered through this formatter. |
| formats err.stack without other err fields | 108 | not-applicable | — | — | JavaScript error stack formatting for Bunyan records; Rust errors and tracing output are not rendered through this formatter. |

### `logger/pretty-stdout › formatRecord(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| formats record | 136 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript record formatter. |
| formats record without colors | 155 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript record formatter. |

### `logger/pretty-stdout › PrettyStdoutStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| writes formatted data to stdout | 175 | not-applicable | — | — | JavaScript Writable stream wrapper for Bunyan records; Rust logging/output does not expose Renovate's `PrettyStdoutStream`. |

---

