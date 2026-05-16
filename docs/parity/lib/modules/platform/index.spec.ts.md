# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/platform/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/platform/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates | 17 | not-applicable | — | — | Renovate's TypeScript dynamic platform module registry validation is not implemented as a Rust API. |
| throws if no platform | 40 | not-applicable | — | — | Renovate's TypeScript singleton platform placeholder is not implemented as a Rust API. |
| throws if wrong platform | 46 | not-applicable | — | — | Renovate's TypeScript platform initialization registry is not implemented as a Rust API; Rust uses a smaller `AnyPlatformClient::create` surface. |
| initializes | 55 | not-applicable | — | — | Renovate's TypeScript platform initialization, host-rule generation, and Bitbucket API flow are not implemented as a Rust API. |
| merges config hostRules with platform hostRules | 82 | not-applicable | — | — | Renovate's TypeScript platform host-rule merge behavior is not implemented as a Rust API. |
| merges config hostRules with platform hostRules | 128 | not-applicable | — | — | Renovate's TypeScript GitHub package host-rule merge behavior is not implemented as a Rust API. |
| merges platform hostRules with additionalHostRules | 196 | not-applicable | — | — | Renovate's TypeScript platform additional host-rule generation is not implemented as a Rust API. |

### `modules/platform/index › getPlatformList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has the same values as PLATFORM_HOST_TYPES | 252 | not-applicable | — | — | Renovate's TypeScript exported platform list helper is not implemented as a Rust API. |

---

