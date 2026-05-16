# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/common.spec.ts
**Total tests:** 22 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/common › detectPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ("$url") === $hostType | 46 | not-applicable | — | — | Renovate's URL-based platform detector helper is not implemented as a Rust API; Rust platform selection is config-driven. |
| uses host rules | 67 | not-applicable | — | — | Renovate's host-rules-backed platform detector helper is not implemented as a Rust API. |

### `util/common › parseJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 119 | not-applicable | — | — | Renovate's nullable TypeScript `parseJson` helper is not implemented as a Rust API; Rust config parsing uses typed file loaders. |
| returns parsed json | 123 | not-applicable | — | — | Renovate's TypeScript `parseJson` helper is not implemented as a Rust API; Rust config parsing coverage is tracked under `config/parse.spec.ts`. |
| supports jsonc | 131 | not-applicable | — | — | Renovate's JSONC-first helper behavior is not implemented as a Rust utility API; Rust repo config parsing has separate JSON5-based coverage. |
| throws error for invalid json | 149 | not-applicable | — | — | Renovate's TypeScript `parseJson` helper is not implemented as a Rust API; Rust config parse errors are tracked under `config/parse.spec.ts`. |
| catches and warns if content parsing failed with JSONC.parse but not with JSON5.parse | 153 | not-applicable | — | — | Renovate's JSONC-to-JSON5 warning fallback is a TypeScript helper side effect with no Rust utility API equivalent. |
| does not warn if filename ends with .jsonc | 167 | not-applicable | — | — | Renovate's TypeScript logger side effect around JSONC parsing has no Rust utility API equivalent. |
| does not warn if filename ends with .json5 | 172 | not-applicable | — | — | Renovate's TypeScript logger side effect around JSON5 parsing has no Rust utility API equivalent. |

### `util/common › parseJsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns parsed jsonc | 179 | not-applicable | — | — | Renovate's TypeScript `parseJson`/JSONC helper is not implemented as a Rust API; Rust config parsing uses typed file loaders. |
| throws error for invalid jsonc | 187 | not-applicable | — | — | Renovate's TypeScript `parseJson`/JSONC helper is not implemented as a Rust API; Rust config parse errors are tracked under `config/parse.spec.ts`. |

### `util/common › getInheritedOrGlobal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined if not set | 198 | not-applicable | — | — | Renovate's process-global `GlobalConfig`/`InheritConfig` lookup helper is not implemented as a Rust API. |
| returns inherited value if only inherited value is set | 202 | not-applicable | — | — | Renovate's process-global inherited config lookup helper is not implemented as a Rust API. |
| returns global value if only global value is set | 209 | not-applicable | — | — | Renovate's process-global config lookup helper is not implemented as a Rust API. |
| returns inherited value - when both global + inherited are set | 216 | not-applicable | — | — | Renovate's process-global config precedence helper is not implemented as a Rust API. |
| handles null inherited values | 227 | not-applicable | — | — | Renovate's TypeScript null inherited-config coverage is not representable in the typed Rust config API. |
| handles undefined inherited values | 238 | not-applicable | — | — | Renovate's TypeScript undefined inherited-config coverage is not representable in the typed Rust config API. |

### `util/common › getInheritedOrGlobal › when requesting onboardingAutoCloseAge, do not allow inherit config to override global config`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns inherited value when inherited < global | 249 | not-applicable | — | — | Renovate's process-global `onboardingAutoCloseAge` inheritance helper is not implemented as a Rust API. |
| returns global value when inherited > global value | 259 | not-applicable | — | — | Renovate's process-global `onboardingAutoCloseAge` inheritance helper is not implemented as a Rust API. |
| returns inherited value when inherited == global | 269 | not-applicable | — | — | Renovate's process-global `onboardingAutoCloseAge` inheritance helper is not implemented as a Rust API. |
| returns inherited value when global value is not set | 279 | not-applicable | — | — | Renovate's process-global `onboardingAutoCloseAge` inheritance helper is not implemented as a Rust API. |
| returns global value when inherited value is not set | 289 | not-applicable | — | — | Renovate's process-global `onboardingAutoCloseAge` inheritance helper is not implemented as a Rust API. |

---

