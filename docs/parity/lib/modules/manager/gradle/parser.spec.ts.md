# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/parser.spec.ts
**Total tests:** 45 | **Ported:** 0 | **Actionable:** 45 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles end of input | 30 | pending | — | — | — |

### `variables › Groovy: single var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 37 | pending | — | — | — |

### `variables › Groovy: single var assignments (non-match)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 62 | pending | — | — | — |

### `variables › Groovy: multi var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple map | 74 | pending | — | — | — |
| nested map | 118 | pending | — | — | — |
| map with interpolated dependency strings | 190 | pending | — | — | — |

### `variables › Kotlin: single var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 243 | pending | — | — | — |

### `variables › Kotlin: single var assignments (non-match)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 255 | pending | — | — | — |

### `variables › Kotlin: single extra var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 266 | pending | — | — | — |

### `variables › Kotlin: multi var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple map | 279 | pending | — | — | — |
| nested map | 300 | pending | — | — | — |
| map with interpolated dependency strings | 350 | pending | — | — | — |

### `dependencies › simple dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 384 | pending | — | — | — |

### `dependencies › interpolated dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 406 | pending | — | — | — |

### `dependencies › concatenated dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 430 | pending | — | — | — |

### `dependencies › property accessors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $accessor | 451 | pending | — | — | — |

### `dependencies › kotlin() short notation dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 489 | pending | — | — | — |

### `dependencies › map notation dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 511 | pending | — | — | — |

### `dependencies › dependencySet dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple dependencySet | 541 | pending | — | — | — |

### `dependencies › dependencySet dependencies › dependencySet variants`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 590 | pending | — | — | — |

### `dependencies › dependencySubstitution constructs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 617 | pending | — | — | — |

### `dependencies › plugins`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 630 | pending | — | — | — |

### `registries › predefined registries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 665 | pending | — | — | — |

### `registries › custom registries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 679 | pending | — | — | — |
| pluginManagement | 731 | pending | — | — | — |

### `registries › content descriptors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| valid combinations | 781 | pending | — | — | — |

### `registries › content descriptors › invalid or unsupported regEx patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $pattern | 909 | not-applicable | — | — | Asserts expect(logger.logger.debug).toHaveBeenCalledWith — logger spy |
| exclusiveContent | 936 | pending | — | — | — |

### `version catalog`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 1018 | pending | — | — | — |

### `heuristic dependency matching`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1056 | pending | — | — | — |
| handles 3 independent dependencies mismatched as groupId, artifactId, version | 1069 | pending | — | — | — |

### `calculations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calculates offset | 1082 | pending | — | — | — |
| parses fixture from "gradle" manager | 1093 | pending | — | — | — |

### `gradle.properties`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1205 | pending | — | — | — |
| handles multi-line file | 1218 | pending | — | — | — |
| attaches packageFile | 1228 | pending | — | — | — |
| parses dependencies | 1236 | pending | — | — | — |

### `apply from`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1267 | pending | — | — | — |
| recursion check | 1313 | not-applicable | — | — | Asserts expect(logger.logger.debug).toHaveBeenCalledWith — logger spy |

### `implicit gradle plugins`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1330 | pending | — | — | — |

### `implicit gradle test suite dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1364 | pending | — | — | — |

### `Kotlin object notation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple objects | 1381 | pending | — | — | — |
| nested objects | 1435 | pending | — | — | — |
| imported objects | 1503 | pending | — | — | — |

### `Java language version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1542 | pending | — | — | — |

---

