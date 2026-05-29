# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/schema-utils/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/schema-utils/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 35 | **Status:** partial

### `util/schema-utils/index › LooseArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses array | 23 | pending | — | — | — |
| drops wrong items | 28 | pending | — | — | — |
| runs callback for wrong elements | 33 | pending | — | — | — |

### `util/schema-utils/index › LooseRecord`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses record | 59 | pending | — | — | — |
| drops wrong items | 64 | pending | — | — | — |
| supports key schema | 69 | pending | — | — | — |
| reports key schema errors | 80 | pending | — | — | — |
| runs callback for wrong elements | 108 | pending | — | — | — |

### `util/schema-utils/index › Json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses json | 148 | pending | — | — | — |

### `util/schema-utils/index › Json5`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses JSON5 | 214 | pending | — | — | — |

### `util/schema-utils/index › Jsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses JSONC | 280 | pending | — | — | — |

### `util/schema-utils/index › UtcDate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses date | 346 | pending | — | — | — |
| rejects invalid date | 352 | pending | — | — | — |

### `util/schema-utils/index › Yaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 362 | pending | — | — | — |
| throws error for non-string | 368 | pending | — | — | — |
| throws error for invalid yaml | 385 | pending | — | — | — |

### `util/schema-utils/index › MultidocYaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 410 | pending | — | — | — |
| throws error for non-string | 420 | pending | — | — | — |
| throws error for invalid yaml | 437 | pending | — | — | — |

### `util/schema-utils/index › multidocYaml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid yaml | 462 | pending | — | — | — |

### `util/schema-utils/index › Toml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid toml | 478 | pending | — | — | — |
| throws error for invalid schema | 488 | pending | — | — | — |
| throws error for invalid toml | 508 | pending | — | — | — |

### `util/schema-utils/index › Ini`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses valid ini | 529 | pending | — | — | — |
| throws error for invalid schema | 539 | pending | — | — | — |

### `util/schema-utils/index › logging utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs debug message and returns fallback value | 556 | not-applicable | — | — | Asserts expect(logger.logger.debug).toHaveBeenCalledWith — logger spy |
| logs trace message and returns fallback value | 571 | not-applicable | — | — | Asserts expect(logger.logger.trace).toHaveBeenCalledWith — logger spy |

### `util/schema-utils/index › NotCircular`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| allows non-circular primitive values | 588 | not-applicable | — | — | JavaScript-specific circular reference detection via Zod; safe Rust code cannot produce circular references |
| allows non-circular arrays | 598 | not-applicable | — | — | JavaScript-specific circular reference detection; impossible in safe Rust |
| allows non-circular objects | 614 | not-applicable | — | — | JavaScript-specific circular reference detection; impossible in safe Rust |
| allows objects reuse | 624 | not-applicable | — | — | JavaScript-specific circular reference detection; impossible in safe Rust |
| rejects circular objects | 639 | not-applicable | — | — | JavaScript-specific circular reference detection; impossible in safe Rust |
| rejects circular arrays | 659 | not-applicable | — | — | JavaScript-specific circular reference detection; impossible in safe Rust |
| rejects deeply nested circular references | 679 | not-applicable | — | — | JavaScript-specific circular reference detection; impossible in safe Rust |
| can be combined with other schema types | 708 | not-applicable | — | — | JavaScript-specific circular reference detection; impossible in safe Rust |

---

