# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 26 | **Status:** pending

### `logger/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inits | 44 | pending | — | — | — |
| uses an auto-generated log context | 48 | pending | — | — | — |
| sets and gets context | 54 | pending | — | — | — |
| supports logging with metadata | 65 | pending | — | — | — |
| supports logging with only metadata | 69 | pending | — | — | — |
| supports logging without metadata | 73 | pending | — | — | — |
| sets level | 311 | pending | — | — | — |
| should create a child logger | 317 | pending | — | — | — |
| saves problems | 329 | pending | — | — | — |
| should contain path or stream parameters | 350 | pending | — | — | — |
| doesn't support rotating files | 359 | pending | — | — | — |
| supports file-based logging | 370 | pending | — | — | — |
| handles cycles | 393 | pending | — | — | — |
| sanitizes secrets | 426 | pending | — | — | — |
| applies custom serializer while keeping default sanitizers | 480 | pending | — | — | — |
| sanitizes secrets in object keys | 550 | pending | — | — | — |

### `logger/index › meta functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets meta | 83 | pending | — | — | — |
| adds meta | 101 | pending | — | — | — |
| removes meta | 119 | pending | — | — | — |
| withMeta adds and removes metadata correctly | 153 | pending | — | — | — |
| withMeta handles cleanup when callback throws | 182 | pending | — | — | — |

### `logger/index › createDefaultStreams`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates log file stream | 210 | pending | — | — | — |
| handles log file stream $logFileLevel level | 220 | pending | — | — | — |
| handles log file stream $logFileFormat format | 248 | pending | — | — | — |
| writes pretty formatted data synchronously to log file | 274 | pending | — | — | — |
| writes json data synchronously to log file | 293 | pending | — | — | — |

---

