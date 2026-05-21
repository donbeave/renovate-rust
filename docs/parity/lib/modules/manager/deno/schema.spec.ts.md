# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/schema.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `DenoLock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock file with specifiers | 21 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses lock file with specifiers that do not match regex | 43 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses lock file with redirects | 63 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses lock file with remote entries | 85 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `Lock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock as string | 107 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses lock as object | 111 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses lock as boolean true | 117 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses lock as boolean false | 121 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `Imports`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 127 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses imports | 131 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `scopes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 150 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses scopes | 154 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `Tasks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 175 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses tasks | 179 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses tasks.command | 196 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `CompilerOptionsTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.types | 217 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `CompilerOptionsJsxImportSource`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSource | 232 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses undefined compilerOptions.jsxImportSource | 245 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `CompilerOptionsJsxImportSourceTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSourceTypes | 251 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses undefined compilerOptions.jsxImportSourceTypes | 266 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `Lint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 272 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses lint.plugins | 276 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `Workspace`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses workspace array | 295 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses workspace object | 302 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `DenoDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses npm package names containing dots | 313 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| invalid npm package names | 329 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| invalid npm package versions | 342 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| invalid jsr package names | 355 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| invalid jsr package versions | 368 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| unsupported datasource | 381 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| deno.land URL without package name | 395 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| deno.land URL with version | 409 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `DenoLock via DenoDependency transform path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty specifiers in lock file | 426 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `UpdateDenoJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 446 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `ImportMapExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses import map with imports and scopes | 464 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses import map with only imports | 500 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses empty import map | 523 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `DenoExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses deno.json with all sections | 531 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses minimal deno.json | 565 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses deno.json with workspace | 575 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses deno.json with lock config | 589 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |
| parses deno.json with importMap | 600 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

### `UpdateImportMapJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 613 | not-applicable | — | — | tests Deno import map JSON schema validation with Zod; Rust uses serde-based schema |

---

