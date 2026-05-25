# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/instrumentation/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** pending

### `instrumentation/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use NoopTraceProvider if not activated | 40 | pending | — | — | — |
| activate console logger | 48 | pending | — | — | — |
| registers GitOperationSpanProcessor, GetDatasourceReleasesSpanProcessor regardless of tracing being enabled | 69 | pending | — | — | — |
| activate remote logger | 89 | pending | — | — | — |
| activate console logger and remote logger | 122 | pending | — | — | — |

### `instrumentation/index › BunyanInstrumentation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| patches bunyan Logger._emit when tracing is enabled | 161 | pending | — | — | — |

### `instrumentation/index › instrument`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return result | 175 | pending | — | — | — |
| should rethrow exception | 183 | pending | — | — | — |
| should return result for async fn | 192 | pending | — | — | — |
| should rethrow exception for async fn | 202 | pending | — | — | — |

---

