# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 26 | **Status:** not-applicable

### `logger/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inits | 44 | not-applicable | — | — | Tests Bunyan logger init via vi.unmock / vi.mock('node:crypto') — JS logger infrastructure |
| uses an auto-generated log context | 48 | not-applicable | — | — | Tests Bunyan logger context generation with mocked crypto.randomUUID |
| sets and gets context | 54 | not-applicable | — | — | Tests Bunyan logger context via spied debug method |
| supports logging with metadata | 65 | not-applicable | — | — | Tests Bunyan logger metadata via spied debug method |
| supports logging with only metadata | 69 | not-applicable | — | — | Tests Bunyan logger via spy |
| supports logging without metadata | 73 | not-applicable | — | — | Tests Bunyan logger via spy |
| sets level | 311 | not-applicable | — | — | Tests Bunyan logger level setter — JS logger infrastructure |
| should create a child logger | 317 | not-applicable | — | — | Tests Bunyan child logger creation |
| saves problems | 329 | not-applicable | — | — | Tests ProblemStream JS infrastructure |
| should contain path or stream parameters | 350 | not-applicable | — | — | Tests Bunyan stream config — JS logger infrastructure |
| doesn't support rotating files | 359 | not-applicable | — | — | Tests Bunyan stream config validation |
| supports file-based logging | 370 | not-applicable | — | — | Tests file stream creation via createDefaultStreams — JS logger infrastructure |
| handles cycles | 393 | not-applicable | — | — | Tests Bunyan circular-reference handling via spy |
| sanitizes secrets | 426 | not-applicable | — | — | Tests Bunyan serializer via spied debug method |
| applies custom serializer while keeping default sanitizers | 480 | not-applicable | — | — | Tests Bunyan serializer infrastructure |
| sanitizes secrets in object keys | 550 | not-applicable | — | — | Tests Bunyan serializer via spy |

### `logger/index › meta functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets meta | 83 | not-applicable | — | — | Tests Bunyan logger meta fields via spied debug method |
| adds meta | 101 | not-applicable | — | — | Tests Bunyan logger meta fields via spy |
| removes meta | 119 | not-applicable | — | — | Tests Bunyan logger meta removal via spy |
| withMeta adds and removes metadata correctly | 153 | not-applicable | — | — | Tests Bunyan logger withMeta via spy |
| withMeta handles cleanup when callback throws | 182 | not-applicable | — | — | Tests Bunyan logger withMeta error handling via spy |

### `logger/index › createDefaultStreams`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates log file stream | 210 | not-applicable | — | — | Tests Bunyan file stream creation — JS logger infrastructure |
| handles log file stream $logFileLevel level | 220 | not-applicable | — | — | Tests Bunyan file stream log level — JS logger infrastructure |
| handles log file stream $logFileFormat format | 248 | not-applicable | — | — | Tests Bunyan file stream format — JS logger infrastructure |
| writes pretty formatted data synchronously to log file | 274 | not-applicable | — | — | Tests synchronous file write via WriteStream spy |
| writes json data synchronously to log file | 293 | not-applicable | — | — | Tests synchronous file write via WriteStream spy |

---
