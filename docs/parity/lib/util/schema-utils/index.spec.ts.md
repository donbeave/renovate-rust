# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/schema-utils/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/schema-utils/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/schema-utils/index › LooseArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses array | 23 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| drops wrong items | 28 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| runs callback for wrong elements | 33 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › LooseRecord`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses record | 59 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| drops wrong items | 64 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| supports key schema | 69 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| reports key schema errors | 80 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| runs callback for wrong elements | 108 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › Json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses json | 148 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › Json5`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses JSON5 | 214 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › Jsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses JSONC | 280 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › UtcDate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses date | 346 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| rejects invalid date | 352 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › Yaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 362 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| throws error for non-string | 368 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| throws error for invalid yaml | 385 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › MultidocYaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 410 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| throws error for non-string | 420 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| throws error for invalid yaml | 437 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › multidocYaml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 462 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › Toml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid toml | 478 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| throws error for invalid schema | 488 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| throws error for invalid toml | 508 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › Ini`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid ini | 529 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| throws error for invalid schema | 539 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › logging utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs debug message and returns fallback value | 556 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| logs trace message and returns fallback value | 571 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

### `util/schema-utils/index › NotCircular`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| allows non-circular primitive values | 588 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| allows non-circular arrays | 598 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| allows non-circular objects | 614 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| allows objects reuse | 624 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| rejects circular objects | 639 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| rejects circular arrays | 659 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| rejects deeply nested circular references | 679 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |
| can be combined with other schema types | 708 | not-applicable | — | — | tests Zod schema utility helpers (LooseArray, LooseRecord, Toml); TypeScript Zod-specific, Rust uses serde |

---

