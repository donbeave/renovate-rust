# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inits | 44 | not-applicable | — | — | JavaScript global Bunyan logger facade initialization; Rust initializes tracing directly and does not expose Renovate's JS logger singleton API. |
| uses an auto-generated log context | 48 | not-applicable | — | — | JavaScript Bunyan log-context metadata behavior; Rust tracing output does not expose Renovate's JS logger context mutators. |
| sets and gets context | 54 | not-applicable | — | — | JavaScript Bunyan log-context metadata behavior; Rust tracing output does not expose Renovate's JS logger context mutators. |
| supports logging with metadata | 65 | not-applicable | — | — | JavaScript Bunyan logger facade call shape; Rust uses `tracing` macros rather than Renovate's JS logger object API. |
| supports logging with only metadata | 69 | not-applicable | — | — | JavaScript Bunyan logger facade call shape; Rust uses `tracing` macros rather than Renovate's JS logger object API. |
| supports logging without metadata | 73 | not-applicable | — | — | JavaScript Bunyan logger facade call shape; Rust uses `tracing` macros rather than Renovate's JS logger object API. |
| sets level | 311 | not-applicable | — | — | Runtime Bunyan stream level mutation API; Rust logging level is configured through tracing initialization, not Renovate's JS `levels()` helper. |
| should create a child logger | 317 | not-applicable | — | — | JavaScript Bunyan child logger behavior; Rust tracing logging does not expose Renovate's JS `childLogger()` API. |
| saves problems | 329 | not-applicable | — | — | JavaScript `ProblemStream` capture and sanitizer behavior; Rust logging does not expose Renovate's JS problem stream. |
| should contain path or stream parameters | 350 | not-applicable | — | — | JavaScript Bunyan stream validation; Rust tracing setup does not expose Renovate's JS `addStream()` API. |
| doesn't support rotating files | 359 | not-applicable | — | — | JavaScript Bunyan stream validation; Rust tracing setup does not expose Renovate's JS `addStream()` API. |
| supports file-based logging | 370 | not-applicable | — | — | JavaScript Bunyan file stream behavior; Rust logging does not expose Renovate's JS `addStream()` logfile API. |
| handles cycles | 393 | not-applicable | — | — | JavaScript Bunyan serializer cycle handling; Rust logging does not serialize arbitrary JS object graphs. |
| sanitizes secrets | 426 | not-applicable | — | — | JavaScript Bunyan sanitizer integration for arbitrary object values; Rust logging does not expose Renovate's JS sanitizer pipeline. |
| applies custom serializer while keeping default sanitizers | 480 | not-applicable | — | — | JavaScript Bunyan custom serializer integration; Rust logging does not expose Renovate's JS serializer pipeline. |
| sanitizes secrets in object keys | 550 | not-applicable | — | — | JavaScript Bunyan sanitizer integration for arbitrary object keys; Rust logging does not expose Renovate's JS sanitizer pipeline. |

### `logger/index › meta functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets meta | 83 | not-applicable | — | — | JavaScript global logger metadata mutator behavior; Rust tracing logging does not expose Renovate's JS `setMeta` API. |
| adds meta | 101 | not-applicable | — | — | JavaScript global logger metadata mutator behavior; Rust tracing logging does not expose Renovate's JS `addMeta` API. |
| removes meta | 119 | not-applicable | — | — | JavaScript global logger metadata mutator behavior; Rust tracing logging does not expose Renovate's JS `removeMeta` API. |
| withMeta adds and removes metadata correctly | 153 | not-applicable | — | — | JavaScript scoped logger metadata mutator behavior; Rust tracing logging does not expose Renovate's JS `withMeta` API. |
| withMeta handles cleanup when callback throws | 182 | not-applicable | — | — | JavaScript scoped logger metadata cleanup behavior; Rust tracing logging does not expose Renovate's JS `withMeta` API. |

### `logger/index › createDefaultStreams`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates log file stream | 210 | not-applicable | — | — | JavaScript Bunyan default stream construction; Rust tracing setup does not expose Renovate's JS `createDefaultStreams()` API. |
| handles log file stream $logFileLevel level | 220 | not-applicable | — | — | JavaScript Bunyan logfile stream level configuration; Rust tracing setup does not expose Renovate's JS logfile stream model. |
| handles log file stream $logFileFormat format | 248 | not-applicable | — | — | JavaScript Bunyan logfile stream format configuration; Rust tracing setup does not expose Renovate's JS logfile stream model. |
| writes pretty formatted data synchronously to log file | 274 | not-applicable | — | — | JavaScript Bunyan pretty logfile stream behavior; Rust tracing setup does not expose Renovate's JS logfile stream model. |
| writes json data synchronously to log file | 293 | not-applicable | — | — | JavaScript Bunyan JSON logfile stream behavior; Rust tracing setup does not expose Renovate's JS logfile stream model. |

---

