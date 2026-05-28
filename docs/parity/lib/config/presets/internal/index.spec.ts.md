# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/internal/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** partial

### `config/presets/internal/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails for undefined internal preset | 19 | not-applicable | — | — | Requires vi.mock(npm) + vi.spyOn + full preset resolution pipeline |
| ${groupName}:${presetName} validates | 31 | not-applicable | — | — | Requires vi.mock + full preset registry validation via resolveConfigPresets |
| internal presets should not contain handlebars | 48 | not-applicable | — | — | Preset resolution requires full preset registry data |
| returns undefined for unknown preset | 58 | pending | — | — | — |

### `config/presets/internal/index › isInternal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for a local> preset | 63 | pending | — | — | — |
| returns false for a github> preset | 67 | pending | — | — | — |
| returns false for an un-migrated preset | 71 | pending | — | — | — |
| returns false for an empty string | 75 | pending | — | — | — |
| returns true for `config:recommended` | 79 | pending | — | — | — |
| returns true for a parameterised preset | 83 | pending | — | — | — |
| returns true for a parameterised remote preset | 87 | pending | — | — | — |

---

