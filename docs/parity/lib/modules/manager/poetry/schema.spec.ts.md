# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/poetry/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/poetry/schema.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses project version | 4 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |

### `PoetrySources`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 13 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| parses unordered sources | 18 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| implicit use of PyPI source | 89 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| source with priority="default" | 133 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| PyPI source with priority="default" | 151 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| source with priority="primary" | 168 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| source with implicit priority="primary" | 191 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| sources with priority="secondary" | 213 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| unordered sources and implicit PyPI priority="primary" | 246 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| unordered sources with implicit PyPI priority="secondary" | 290 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| source with priority="supplemental" | 322 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| source with priority="explicit" | 345 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |

### `PoetryPyProject`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters out invalid build-system requirements | 370 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |
| handles build-system without poetry requirement | 384 | not-applicable | — | — | TS-library-specific; tests Zod schema parsing; Rust uses serde |

---

