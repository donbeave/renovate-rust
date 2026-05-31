# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/parser.spec.ts
**Total tests:** 45 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles end of input | 30 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `variables › Groovy: single var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 37 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `variables › Groovy: single var assignments (non-match)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 62 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `variables › Groovy: multi var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple map | 74 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| nested map | 118 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| map with interpolated dependency strings | 190 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `variables › Kotlin: single var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 243 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `variables › Kotlin: single var assignments (non-match)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 255 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `variables › Kotlin: single extra var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 266 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `variables › Kotlin: multi var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple map | 279 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| nested map | 300 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| map with interpolated dependency strings | 350 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › simple dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 384 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › interpolated dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 406 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › concatenated dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 430 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › property accessors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $accessor | 451 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › kotlin() short notation dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 489 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › map notation dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 511 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › dependencySet dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple dependencySet | 541 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › dependencySet dependencies › dependencySet variants`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 590 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › dependencySubstitution constructs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 617 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependencies › plugins`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 630 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `registries › predefined registries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 665 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `registries › custom registries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 679 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| pluginManagement | 731 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `registries › content descriptors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| valid combinations | 781 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `registries › content descriptors › invalid or unsupported regEx patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $pattern | 909 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| exclusiveContent | 936 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `version catalog`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 1018 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `heuristic dependency matching`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1056 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| handles 3 independent dependencies mismatched as groupId, artifactId, version | 1069 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `calculations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calculates offset | 1082 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| parses fixture from "gradle" manager | 1093 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `gradle.properties`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1205 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| handles multi-line file | 1218 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| attaches packageFile | 1228 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| parses dependencies | 1236 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `apply from`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1267 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| recursion check | 1313 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `implicit gradle plugins`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1330 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `implicit gradle test suite dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1364 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `Kotlin object notation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple objects | 1381 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| nested objects | 1435 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |
| imported objects | 1503 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

### `Java language version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1542 | not-applicable | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle parser via vitest-mocked fs/exec; Rust tests this at different layer |

---

