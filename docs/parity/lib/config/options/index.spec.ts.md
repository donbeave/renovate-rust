# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/options/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/options/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/options/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| test manager should have no defaultConfig | 9 | not-applicable | — | — | TypeScript option metadata registry; Rust does not generate options from manager `defaultConfig` metadata. |
| supportedManagers should have valid names | 18 | not-applicable | — | — | TypeScript option metadata registry; Rust uses typed manager modules rather than dynamic option metadata. |
| supportedPlatforms should have valid names | 32 | not-applicable | — | — | TypeScript option metadata registry; Rust platform values are static enums/constants rather than dynamic option metadata. |
| should not contain duplicate option names | 46 | not-applicable | — | — | TypeScript option metadata registry; Rust options are typed fields and clap definitions. |

### `config/options/index › every option with allowedValues and a default must have the default in allowedValues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| `${option.name}: \`${option.default}\` is in ${JSON.stringify(option.allowedValues)}` | 57 | not-applicable | — | — | TypeScript option metadata registry; Rust does not expose allowedValues/default metadata tables. |

### `config/options/index › every option with a siblingProperties has a \`property\` that matches a known option`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| `${option.name}'s reference to ${prop.property} is valid` | 77 | not-applicable | — | — | TypeScript option metadata registry; Rust does not expose requiredIf sibling property metadata tables. |
| `${option.name}'s value for ${prop.property} is valid, according to allowedValues` | 84 | not-applicable | — | — | TypeScript option metadata registry; Rust does not expose requiredIf sibling property metadata tables. |

---

