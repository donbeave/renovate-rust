# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/init/apis.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/apis.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** done

### `workers/repository/init/apis › initApis`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 24 | not-applicable | — | — | Requires platform mock (vi.mock platform) + scm mock infrastructure |
| throws for disabled | 34 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| throws for forked | 49 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| does not throw for includeForks=true | 66 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| does not throw for forkProcessing=enabled | 79 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| ignores platform.getJsonFile() failures | 92 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| throws for fork with platform.getJsonFile() failures | 109 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| uses the onboardingConfigFileName if set | 124 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| falls back to "renovate.json" if onboardingConfigFileName is not set | 151 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| falls back to "renovate.json" if onboardingConfigFileName is not valid | 172 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| checks for re-enablement and continues | 191 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |
| checks for re-enablement and skips | 211 | not-applicable | — | — | Requires platform mock + scm mock infrastructure |

---
