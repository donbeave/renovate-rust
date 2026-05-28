# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/options/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/options/index.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** not-applicable

### `config/options/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| test manager should have no defaultConfig | 9 | not-applicable | — | — | Requires vi.doMock() to inject fake manager map |
| supportedManagers should have valid names | 18 | not-applicable | — | — | Validates full options registry against manager list; TypeScript registry metadata test |
| supportedPlatforms should have valid names | 32 | not-applicable | — | — | Validates full options registry against platform list; TypeScript registry metadata test |
| should not contain duplicate option names | 46 | not-applicable | — | — | Validates full TypeScript options registry for duplicates |

### `config/options/index › every option with allowedValues and a default must have the default in allowedValues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| `${option.name}: \`${option.default}\` is in ${JSON.stringify(option.allowedValues)}` | 57 | not-applicable | — | — | Validates full TypeScript options registry allowedValues/default consistency |

### `config/options/index › every option with a siblingProperties has a \`property\` that matches a known option`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| `${option.name}'s reference to ${prop.property} is valid` | 77 | not-applicable | — | — | Validates full TypeScript options registry siblingProperties |
| `${option.name}'s value for ${prop.property} is valid, according to allowedValues` | 84 | not-applicable | — | — | Validates full TypeScript options registry siblingProperties |

| ${option.name}: \ | 76 | not-applicable | — | — | Validates full TypeScript options registry |
| ${option.name} should be of type string or array of strings | 93 | not-applicable | — | — | Validates full TypeScript options registry supportsTemplating |
---

