# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/instrumentation/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** not-applicable

### `instrumentation/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use NoopTraceProvider if not activated | 40 | not-applicable | — | — | Requires OpenTelemetry SDK infrastructure (ProxyTracerProvider, api.trace.disable) — no Rust equivalent |
| activate console logger | 48 | not-applicable | — | — | Requires OpenTelemetry SDK infrastructure (NodeTracerProvider, SimpleSpanProcessor, process.env OTEL vars) |
| registers GitOperationSpanProcessor, GetDatasourceReleasesSpanProcessor regardless of tracing being enabled | 69 | not-applicable | — | — | Requires OpenTelemetry SDK span processor registration infrastructure |
| activate remote logger | 89 | not-applicable | — | — | Requires OpenTelemetry remote exporter infrastructure |
| activate console logger and remote logger | 122 | not-applicable | — | — | Requires OpenTelemetry SDK infrastructure |

### `instrumentation/index › BunyanInstrumentation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| patches bunyan Logger._emit when tracing is enabled | 161 | not-applicable | — | — | Requires Bunyan JS logger patching infrastructure — no Rust equivalent |

### `instrumentation/index › instrument`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return result | 175 | not-applicable | — | — | Tests TypeScript OpenTelemetry span wrapper — no Rust equivalent |
| should rethrow exception | 183 | not-applicable | — | — | Tests TypeScript OpenTelemetry span wrapper |
| should return result for async fn | 192 | not-applicable | — | — | Tests TypeScript OpenTelemetry span wrapper |
| should rethrow exception for async fn | 202 | not-applicable | — | — | Tests TypeScript OpenTelemetry span wrapper |

---
