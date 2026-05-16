# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/init/apis.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/apis.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/init/apis › initApis`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 24 | not-applicable | — | — | Rust worker layer does not implement Renovate repository initApis/platform bootstrap flow |
| throws for disabled | 34 | not-applicable | — | — | Rust worker layer does not implement optimizeForDisabled repository bootstrap checks |
| throws for forked | 49 | not-applicable | — | — | Rust worker layer does not implement fork-processing repository bootstrap checks |
| does not throw for includeForks=true | 66 | not-applicable | — | — | Rust worker layer does not implement fork-processing repository bootstrap checks |
| does not throw for forkProcessing=enabled | 79 | not-applicable | — | — | Rust worker layer does not implement fork-processing repository bootstrap checks |
| ignores platform.getJsonFile() failures | 92 | not-applicable | — | — | Rust worker layer does not implement platform getJsonFile probing during repository bootstrap |
| throws for fork with platform.getJsonFile() failures | 109 | not-applicable | — | — | Rust worker layer does not implement platform getJsonFile probing during repository bootstrap |
| uses the onboardingConfigFileName if set | 124 | not-applicable | — | — | Rust worker layer does not implement onboarding config file probing during repository bootstrap |
| falls back to "renovate.json" if onboardingConfigFileName is not set | 151 | not-applicable | — | — | Rust worker layer does not implement onboarding config file probing during repository bootstrap |
| falls back to "renovate.json" if onboardingConfigFileName is not valid | 172 | not-applicable | — | — | Rust worker layer does not implement onboarding config file probing during repository bootstrap |
| checks for re-enablement and continues | 191 | not-applicable | — | — | Rust worker layer does not implement disabled-repository re-enablement probing |
| checks for re-enablement and skips | 211 | not-applicable | — | — | Rust worker layer does not implement disabled-repository re-enablement probing |

---

