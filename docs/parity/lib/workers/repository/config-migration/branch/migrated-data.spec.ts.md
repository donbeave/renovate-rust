# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/branch/migrated-data.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/branch/migrated-data.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.getAsync`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Calls getAsync a first when migration not needed  | 54 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| Calls getAsync a first time to initialize the factory  | 62 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| Calls getAsync a second time to get the saved data from before  | 69 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.getAsync › MigratedData class`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets the filename from the class instance  | 77 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| gets the content from the class instance  | 82 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| Resets the factory and gets a new value  | 88 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| Resets the factory and gets a new value with default indentation  | 95 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| Migrate a JSON5 config file  | 110 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| Falls back to JSON.stringify when weave fails  | 120 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| Uses JSON.stringify when raw is null  | 138 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |
| Returns nothing due to detectRepoFileConfig throwing  | 150 | not-applicable | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture | — | TS-library-specific schema internals — tests TypeScript MigratedDataFactory/MigratedData internal class; Rust uses different config migration architecture |

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.applyPrettierFormatting`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not format when no prettier config is present  | 184 | not-applicable | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent | — | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent |
| does not format when failing to fetch package.json file  | 193 | not-applicable | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent | — | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent |
| does not format when there is an invalid package.json file  | 202 | not-applicable | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent | — | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent |
| formats when prettier config file is found  | 211 | not-applicable | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent | — | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent |
| formats without prettier if in .renovaterc  | 220 | not-applicable | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent | — | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent |
| formats when finds prettier config inside the package.json file  | 231 | not-applicable | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent | — | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent |
| formats with default 2 spaces  | 243 | not-applicable | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent | — | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent |
| formats with printWith=Infinity  | 259 | not-applicable | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent | — | TS-library-specific schema internals — prettier is a TypeScript-specific formatting tool with no Rust equivalent |

---
