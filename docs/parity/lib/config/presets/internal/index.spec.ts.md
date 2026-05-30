# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/internal/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable-applicable

### `config/presets/internal/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails for undefined internal preset | 19 | not-applicable | — | — | Internal preset resolution system |
| ${groupName}:${presetName} validates | 31 | not-applicable | — | — | Internal preset resolution system |
| internal presets should not contain handlebars | 48 | not-applicable | — | — | Internal preset resolution system |
| returns undefined for unknown preset | 58 | not-applicable | — | — | Internal preset resolution system |

### `config/presets/internal/index › isInternal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for a local> preset | 63 | not-applicable | — | — | Internal preset resolution system |
| returns false for a github> preset | 67 | not-applicable | — | — | Internal preset resolution system |
| returns false for an un-migrated preset | 71 | not-applicable | — | — | Internal preset resolution system |
| returns false for an empty string | 75 | not-applicable | — | — | Internal preset resolution system |
| returns true for `config:recommended` | 79 | not-applicable | — | — | Internal preset resolution system |
| returns true for a parameterised preset | 83 | not-applicable | — | — | Internal preset resolution system |
| returns true for a parameterised remote preset | 87 | not-applicable | — | — | Internal preset resolution system |

---

