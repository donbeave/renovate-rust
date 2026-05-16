# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/internal/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/internal/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails for undefined internal preset | 19 | not-applicable | — | — | TypeScript preset resolver error-path test; Rust does not expose the same internal preset fetcher and instead expands known built-ins while retaining unresolved preset references. |
| ${groupName}:${presetName} validates | 31 | not-applicable | — | — | TypeScript static validation sweep over the generated internal preset object map; Rust resolves supported presets through behavior-oriented match arms rather than exposing and validating the TypeScript preset map. |
| internal presets should not contain handlebars | 48 | not-applicable | — | — | TypeScript static preset-map invariant; Rust does not expose the generated internal preset map or handlebars-bearing preset names. |
| returns undefined for unknown preset | 58 | not-applicable | — | — | TypeScript internal.getPreset helper behavior; Rust does not expose that helper and treats unknown presets as unresolved config references. |

### `config/presets/internal/index › isInternal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for a local> preset | 63 | not-applicable | — | — | TypeScript isInternal routing helper; Rust does not expose internal-vs-remote preset classification as a public behavior. |
| returns false for a github> preset | 67 | not-applicable | — | — | TypeScript isInternal routing helper; Rust does not expose internal-vs-remote preset classification as a public behavior. |
| returns false for an un-migrated preset | 71 | not-applicable | — | — | TypeScript isInternal routing helper; Rust normalizes supported legacy presets through config parsing rather than exposing this classifier. |
| returns false for an empty string | 75 | not-applicable | — | — | TypeScript isInternal routing helper; Rust does not expose internal-vs-remote preset classification as a public behavior. |
| returns true for `config:recommended` | 79 | not-applicable | — | — | TypeScript isInternal routing helper; Rust covers config:recommended through built-in expansion tests rather than exposing this classifier. |
| returns true for a parameterised preset | 83 | not-applicable | — | — | TypeScript isInternal routing helper; Rust covers parameterized presets through config parsing tests rather than exposing this classifier. |
| returns true for a parameterised remote preset | 87 | not-applicable | — | — | TypeScript isInternal routing helper; Rust does not expose internal-vs-remote preset classification as a public behavior. |

---

