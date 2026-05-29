# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/parser.spec.ts
**Total tests:** 45 | **Ported:** 0 | **Actionable:** 45 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles end of input | 30 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `variables › Groovy: single var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 37 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `variables › Groovy: single var assignments (non-match)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 62 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `variables › Groovy: multi var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple map | 74 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| nested map | 118 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| map with interpolated dependency strings | 190 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `variables › Kotlin: single var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 243 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `variables › Kotlin: single var assignments (non-match)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 255 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `variables › Kotlin: single extra var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 266 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `variables › Kotlin: multi var assignments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple map | 279 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| nested map | 300 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| map with interpolated dependency strings | 350 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › simple dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 384 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › interpolated dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 406 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › concatenated dependency strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 430 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › property accessors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $accessor | 451 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › kotlin() short notation dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 489 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › map notation dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 511 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › dependencySet dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple dependencySet | 541 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › dependencySet dependencies › dependencySet variants`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 590 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › dependencySubstitution constructs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 617 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `dependencies › plugins`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 630 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `registries › predefined registries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 665 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `registries › custom registries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 679 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| pluginManagement | 731 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `registries › content descriptors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| valid combinations | 781 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `registries › content descriptors › invalid or unsupported regEx patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $pattern | 909 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| exclusiveContent | 936 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `version catalog`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $str | 1018 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `heuristic dependency matching`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1056 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| handles 3 independent dependencies mismatched as groupId, artifactId, version | 1069 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `calculations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calculates offset | 1082 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| parses fixture from "gradle" manager | 1093 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `gradle.properties`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1205 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| handles multi-line file | 1218 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| attaches packageFile | 1228 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| parses dependencies | 1236 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `apply from`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1267 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| recursion check | 1313 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `implicit gradle plugins`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1330 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `implicit gradle test suite dependencies`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $def \| $input | 1364 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `Kotlin object notation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| simple objects | 1381 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| nested objects | 1435 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|
| imported objects | 1503 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

### `Java language version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1542 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Gradle parser with filesystem mock|

---

