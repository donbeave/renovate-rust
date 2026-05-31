# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/internal/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `config/presets/internal/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails for undefined internal preset | 19 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| ${groupName}:${presetName} validates | 31 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| internal presets should not contain handlebars | 48 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| returns undefined for unknown preset | 58 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |

### `config/presets/internal/index › isInternal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for a local> preset | 63 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| returns false for a github> preset | 67 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| returns false for an un-migrated preset | 71 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| returns false for an empty string | 75 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| returns true for `config:recommended` | 79 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| returns true for a parameterised preset | 83 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |
| returns true for a parameterised remote preset | 87 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal preset resolution system; Rust uses different config system |

---

