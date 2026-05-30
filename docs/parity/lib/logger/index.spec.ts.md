# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `logger/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inits | 44 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| uses an auto-generated log context | 48 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| sets and gets context | 54 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| supports logging with metadata | 65 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| supports logging with only metadata | 69 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| supports logging without metadata | 73 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| sets level | 311 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| should create a child logger | 317 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| saves problems | 329 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| should contain path or stream parameters | 350 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| doesn't support rotating files | 359 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| supports file-based logging | 370 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| handles cycles | 393 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| sanitizes secrets | 426 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| applies custom serializer while keeping default sanitizers | 480 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| sanitizes secrets in object keys | 550 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|

### `logger/index › meta functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets meta | 83 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| adds meta | 101 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| removes meta | 119 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| withMeta adds and removes metadata correctly | 153 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| withMeta handles cleanup when callback throws | 182 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|

### `logger/index › createDefaultStreams`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates log file stream | 210 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| handles log file stream $logFileLevel level | 220 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| handles log file stream $logFileFormat format | 248 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| writes pretty formatted data synchronously to log file | 274 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|
| writes json data synchronously to log file | 293 | not-applicable | — | — | TypeScript Bunyan logger infrastructure; tests TypeScript-specific Bunyan logger init/context/streams/serialization/child-logger OOP API|

---
