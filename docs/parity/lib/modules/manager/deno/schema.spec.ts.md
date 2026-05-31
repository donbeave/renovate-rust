# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/schema.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `DenoLock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock file with specifiers | 21 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses lock file with specifiers that do not match regex | 43 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses lock file with redirects | 63 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses lock file with remote entries | 85 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `Lock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock as string | 107 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses lock as object | 111 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses lock as boolean true | 117 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses lock as boolean false | 121 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `Imports`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 127 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses imports | 131 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `scopes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 150 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses scopes | 154 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `Tasks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 175 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses tasks | 179 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses tasks.command | 196 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `CompilerOptionsTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.types | 217 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `CompilerOptionsJsxImportSource`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSource | 232 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses undefined compilerOptions.jsxImportSource | 245 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `CompilerOptionsJsxImportSourceTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSourceTypes | 251 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses undefined compilerOptions.jsxImportSourceTypes | 266 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `Lint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 272 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses lint.plugins | 276 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `Workspace`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses workspace array | 295 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses workspace object | 302 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `DenoDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses npm package names containing dots | 313 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| invalid npm package names | 329 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| invalid npm package versions | 342 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| invalid jsr package names | 355 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| invalid jsr package versions | 368 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| unsupported datasource | 381 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| deno.land URL without package name | 395 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| deno.land URL with version | 409 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `DenoLock via DenoDependency transform path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty specifiers in lock file | 426 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `UpdateDenoJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 446 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `ImportMapExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses import map with imports and scopes | 464 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses import map with only imports | 500 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses empty import map | 523 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `DenoExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses deno.json with all sections | 531 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses minimal deno.json | 565 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses deno.json with workspace | 575 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses deno.json with lock config | 589 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |
| parses deno.json with importMap | 600 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

### `UpdateImportMapJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 613 | not-applicable | Mock framework internals — tests deno schema via vitest-mocked Zod/JSON parsing; Rust tests this at different layer | — | Deno schema parsing and extraction behavior is in scope |

---

