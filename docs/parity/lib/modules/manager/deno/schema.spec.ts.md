# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/schema.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** pending

### `DenoLock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock file with specifiers | 21 | pending | — | — | — |
| parses lock file with specifiers that do not match regex | 43 | pending | — | — | — |
| parses lock file with redirects | 63 | pending | — | — | — |
| parses lock file with remote entries | 85 | pending | — | — | — |

### `Lock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock as string | 107 | pending | — | — | — |
| parses lock as object | 111 | pending | — | — | — |
| parses lock as boolean true | 117 | pending | — | — | — |
| parses lock as boolean false | 121 | pending | — | — | — |

### `Imports`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 127 | pending | — | — | — |
| parses imports | 131 | pending | — | — | — |

### `scopes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 150 | pending | — | — | — |
| parses scopes | 154 | pending | — | — | — |

### `Tasks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 175 | pending | — | — | — |
| parses tasks | 179 | pending | — | — | — |
| parses tasks.command | 196 | pending | — | — | — |

### `CompilerOptionsTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.types | 217 | pending | — | — | — |

### `CompilerOptionsJsxImportSource`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSource | 232 | pending | — | — | — |
| parses undefined compilerOptions.jsxImportSource | 245 | pending | — | — | — |

### `CompilerOptionsJsxImportSourceTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSourceTypes | 251 | pending | — | — | — |
| parses undefined compilerOptions.jsxImportSourceTypes | 266 | pending | — | — | — |

### `Lint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 272 | pending | — | — | — |
| parses lint.plugins | 276 | pending | — | — | — |

### `Workspace`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses workspace array | 295 | pending | — | — | — |
| parses workspace object | 302 | pending | — | — | — |

### `DenoDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses npm package names containing dots | 313 | pending | — | — | — |
| invalid npm package names | 329 | pending | — | — | — |
| invalid npm package versions | 342 | pending | — | — | — |
| invalid jsr package names | 355 | pending | — | — | — |
| invalid jsr package versions | 368 | pending | — | — | — |
| unsupported datasource | 381 | pending | — | — | — |
| deno.land URL without package name | 395 | pending | — | — | — |
| deno.land URL with version | 409 | pending | — | — | — |

### `DenoLock via DenoDependency transform path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty specifiers in lock file | 426 | pending | — | — | — |

### `UpdateDenoJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 446 | pending | — | — | — |

### `ImportMapExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses import map with imports and scopes | 464 | pending | — | — | — |
| parses import map with only imports | 500 | pending | — | — | — |
| parses empty import map | 523 | pending | — | — | — |

### `DenoExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses deno.json with all sections | 531 | pending | — | — | — |
| parses minimal deno.json | 565 | pending | — | — | — |
| parses deno.json with workspace | 575 | pending | — | — | — |
| parses deno.json with lock config | 589 | pending | — | — | — |
| parses deno.json with importMap | 600 | pending | — | — | — |

### `UpdateImportMapJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 613 | pending | — | — | — |

---

