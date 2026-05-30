# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/schema.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** pending

### `DenoLock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock file with specifiers | 21 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses lock file with specifiers that do not match regex | 43 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses lock file with redirects | 63 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses lock file with remote entries | 85 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `Lock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock as string | 107 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses lock as object | 111 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses lock as boolean true | 117 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses lock as boolean false | 121 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `Imports`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 127 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses imports | 131 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `scopes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 150 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses scopes | 154 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `Tasks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 175 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses tasks | 179 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses tasks.command | 196 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `CompilerOptionsTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.types | 217 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `CompilerOptionsJsxImportSource`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSource | 232 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses undefined compilerOptions.jsxImportSource | 245 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `CompilerOptionsJsxImportSourceTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSourceTypes | 251 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses undefined compilerOptions.jsxImportSourceTypes | 266 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `Lint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 272 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses lint.plugins | 276 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `Workspace`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses workspace array | 295 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses workspace object | 302 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `DenoDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses npm package names containing dots | 313 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| invalid npm package names | 329 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| invalid npm package versions | 342 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| invalid jsr package names | 355 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| invalid jsr package versions | 368 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| unsupported datasource | 381 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| deno.land URL without package name | 395 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| deno.land URL with version | 409 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `DenoLock via DenoDependency transform path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty specifiers in lock file | 426 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `UpdateDenoJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 446 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `ImportMapExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses import map with imports and scopes | 464 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses import map with only imports | 500 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses empty import map | 523 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `DenoExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses deno.json with all sections | 531 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses minimal deno.json | 565 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses deno.json with workspace | 575 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses deno.json with lock config | 589 | pending | — | — | Deno schema parsing and extraction behavior is in scope |
| parses deno.json with importMap | 600 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

### `UpdateImportMapJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 613 | pending | — | — | Deno schema parsing and extraction behavior is in scope |

---

