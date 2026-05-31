# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/internal/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** pending-applicable-applicable

### `config/presets/internal/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails for undefined internal preset | 19 | pending | — | — | Internal preset resolution system |
| ${groupName}:${presetName} validates | 31 | pending | — | — | Internal preset resolution system |
| internal presets should not contain handlebars | 48 | pending | — | — | Internal preset resolution system |
| returns undefined for unknown preset | 58 | pending | — | — | Internal preset resolution system |

### `config/presets/internal/index › isInternal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for a local> preset | 63 | pending | — | — | Internal preset resolution system |
| returns false for a github> preset | 67 | pending | — | — | Internal preset resolution system |
| returns false for an un-migrated preset | 71 | pending | — | — | Internal preset resolution system |
| returns false for an empty string | 75 | pending | — | — | Internal preset resolution system |
| returns true for `config:recommended` | 79 | pending | — | — | Internal preset resolution system |
| returns true for a parameterised preset | 83 | pending | — | — | Internal preset resolution system |
| returns true for a parameterised remote preset | 87 | pending | — | — | Internal preset resolution system |

---

