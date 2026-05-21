# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/parser.spec.ts
**Total tests:** 45 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles end of input | 30 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `variables › Groovy: single var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 37 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `variables › Groovy: single var assignments (non-match)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 62 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `variables › Groovy: multi var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple map | 74 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| nested map | 118 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| map with interpolated dependency strings | 190 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `variables › Kotlin: single var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 243 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `variables › Kotlin: single var assignments (non-match)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 255 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `variables › Kotlin: single extra var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 266 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `variables › Kotlin: multi var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple map | 279 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| nested map | 300 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| map with interpolated dependency strings | 350 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › simple dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 384 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › interpolated dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 406 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › concatenated dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 430 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › property accessors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $accessor | 451 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › kotlin() short notation dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 489 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › map notation dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 511 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › dependencySet dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple dependencySet | 541 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › dependencySet dependencies › dependencySet variants`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 590 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › dependencySubstitution constructs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 617 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `dependencies › plugins`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 630 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `registries › predefined registries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 665 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `registries › custom registries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 679 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| pluginManagement | 731 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `registries › content descriptors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| valid combinations | 781 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `registries › content descriptors › invalid or unsupported regEx patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $pattern | 909 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| exclusiveContent | 936 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `version catalog`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 1018 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `heuristic dependency matching`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1056 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| handles 3 independent dependencies mismatched as groupId, artifactId, version | 1069 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `calculations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calculates offset | 1082 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| parses fixture from "gradle" manager | 1093 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `gradle.properties`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1205 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| handles multi-line file | 1218 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| attaches packageFile | 1228 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| parses dependencies | 1236 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `apply from`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1267 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| recursion check | 1313 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `implicit gradle plugins`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1330 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `implicit gradle test suite dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1364 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `Kotlin object notation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple objects | 1381 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| nested objects | 1435 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |
| imported objects | 1503 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

### `Java language version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1542 | not-applicable | — | — | tests TypeScript-specific Gradle DSL parser (parseGradle/parseProps); Rust uses own parsing implementation |

---

