# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/instrumentation/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `instrumentation/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use NoopTraceProvider if not activated | 40 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| activate console logger | 48 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| registers GitOperationSpanProcessor, GetDatasourceReleasesSpanProcessor regardless of tracing being enabled | 69 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| activate remote logger | 89 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| activate console logger and remote logger | 122 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|

### `instrumentation/index › BunyanInstrumentation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| patches bunyan Logger._emit when tracing is enabled | 161 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|

### `instrumentation/index › instrument`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return result | 175 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| should rethrow exception | 183 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| should return result for async fn | 192 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|
| should rethrow exception for async fn | 202 | not-applicable | — | — | TypeScript/OpenTelemetry infrastructure test; Rust uses tracing crate with different API|

---
