# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/instrumentation/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `instrumentation/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use NoopTraceProvider if not activated | 40 | not-applicable | — | — | OpenTelemetry JavaScript tracer-provider setup; Rust currently uses `tracing` logging and does not expose Renovate's JS OpenTelemetry initialization API. |
| activate console logger | 48 | not-applicable | — | — | OpenTelemetry JavaScript tracer-provider setup; Rust currently uses `tracing` logging and does not expose Renovate's JS OpenTelemetry initialization API. |
| registers GitOperationSpanProcessor, GetDatasourceReleasesSpanProcessor regardless of tracing being enabled | 69 | not-applicable | — | — | OpenTelemetry JavaScript span processor wiring; Rust does not implement Renovate's JS Git/datasource span processors. |
| activate remote logger | 89 | not-applicable | — | — | OpenTelemetry JavaScript OTLP exporter setup; Rust does not expose Renovate's JS OpenTelemetry exporter pipeline. |
| activate console logger and remote logger | 122 | not-applicable | — | — | OpenTelemetry JavaScript exporter setup; Rust does not expose Renovate's JS OpenTelemetry exporter pipeline. |

### `instrumentation/index › BunyanInstrumentation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| patches bunyan Logger._emit when tracing is enabled | 161 | not-applicable | — | — | JavaScript Bunyan OpenTelemetry instrumentation; Rust logging uses `tracing` and has no Bunyan logger to patch. |

### `instrumentation/index › instrument`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return result | 175 | not-applicable | — | — | JavaScript OpenTelemetry span wrapper helper; Rust does not expose Renovate's JS `instrument()` API. |
| should rethrow exception | 183 | not-applicable | — | — | JavaScript OpenTelemetry span wrapper helper; Rust does not expose Renovate's JS `instrument()` API. |
| should return result for async fn | 192 | not-applicable | — | — | JavaScript OpenTelemetry span wrapper helper; Rust does not expose Renovate's JS `instrument()` API. |
| should rethrow exception for async fn | 202 | not-applicable | — | — | JavaScript OpenTelemetry span wrapper helper; Rust does not expose Renovate's JS `instrument()` API. |

---

