# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/schema.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** not-applicable

### `DenoLock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock file with specifiers | 21 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses lock file with specifiers that do not match regex | 43 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses lock file with redirects | 63 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses lock file with remote entries | 85 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `Lock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lock as string | 107 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses lock as object | 111 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses lock as boolean true | 117 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses lock as boolean false | 121 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `Imports`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 127 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses imports | 131 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `scopes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 150 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses scopes | 154 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `Tasks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 175 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses tasks | 179 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses tasks.command | 196 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `CompilerOptionsTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.types | 217 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `CompilerOptionsJsxImportSource`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSource | 232 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses undefined compilerOptions.jsxImportSource | 245 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `CompilerOptionsJsxImportSourceTypes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses compilerOptions.jsxImportSourceTypes | 251 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses undefined compilerOptions.jsxImportSourceTypes | 266 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `Lint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses default values | 272 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses lint.plugins | 276 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `Workspace`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses workspace array | 295 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses workspace object | 302 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `DenoDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses npm package names containing dots | 313 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| invalid npm package names | 329 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| invalid npm package versions | 342 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| invalid jsr package names | 355 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| invalid jsr package versions | 368 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| unsupported datasource | 381 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| deno.land URL without package name | 395 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| deno.land URL with version | 409 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `DenoLock via DenoDependency transform path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty specifiers in lock file | 426 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `UpdateDenoJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 446 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `ImportMapExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses import map with imports and scopes | 464 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses import map with only imports | 500 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses empty import map | 523 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `DenoExtract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses deno.json with all sections | 531 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses minimal deno.json | 565 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses deno.json with workspace | 575 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses deno.json with lock config | 589 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |
| parses deno.json with importMap | 600 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

### `UpdateImportMapJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keep original field that is irrelevant for schema | 613 | not-applicable | — | — | TS-library-specific; tests Zod schema definitions (DenoLock, DenoDependency, etc.) for Deno config format; Rust uses serde |

---

