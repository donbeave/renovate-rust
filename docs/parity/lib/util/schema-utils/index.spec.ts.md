# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/schema-utils/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/schema-utils/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable-applicable-applicable

### `util/schema-utils/index › LooseArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses array | 23 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| drops wrong items | 28 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| runs callback for wrong elements | 33 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › LooseRecord`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses record | 59 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| drops wrong items | 64 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| supports key schema | 69 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| reports key schema errors | 80 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| runs callback for wrong elements | 108 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › Json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses json | 148 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › Json5`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses JSON5 | 214 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › Jsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses JSONC | 280 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › UtcDate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses date | 346 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| rejects invalid date | 352 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › Yaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 362 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| throws error for non-string | 368 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| throws error for invalid yaml | 385 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › MultidocYaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 410 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| throws error for non-string | 420 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| throws error for invalid yaml | 437 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › multidocYaml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 462 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › Toml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid toml | 478 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| throws error for invalid schema | 488 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| throws error for invalid toml | 508 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › Ini`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid ini | 529 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |
| throws error for invalid schema | 539 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API with Zod-specific error formats that have no direct Rust serde equivalent |

### `util/schema-utils/index › logging utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs debug message and returns fallback value | 556 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|
| logs trace message and returns fallback value | 571 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|

### `util/schema-utils/index › NotCircular`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| allows non-circular primitive values | 588 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|
| allows non-circular arrays | 598 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|
| allows non-circular objects | 614 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|
| allows objects reuse | 624 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|
| rejects circular objects | 639 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|
| rejects circular arrays | 659 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|
| rejects deeply nested circular references | 679 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|
| can be combined with other schema types | 708 | not-applicable | — | — | TS-library-specific; tests Zod v3 schema API (LooseArray, LooseRecord, pipe, safeParse) with Zod-specific error formats that have no direct Rust serde equivalent|

---

