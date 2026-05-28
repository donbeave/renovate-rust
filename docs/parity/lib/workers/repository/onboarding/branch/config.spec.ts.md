# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/config.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/config.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** done

### `workers/repository/onboarding/branch/config › getOnboardingConfigContents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the JSON stringified onboarding config | 32 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |

### `workers/repository/onboarding/branch/config › getOnboardingConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles finding a preset in the same group level | 48 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |
| handles finding an organization dot platform preset | 58 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |
| handles finding a preset in the same group | 71 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |
| handles finding a preset in a parent group | 87 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |
| handles falling back to finding a organization preset | 103 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |
| handles not finding any preset | 119 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |
| ignores an unknown error | 128 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |
| ignores unsupported platform | 137 | not-applicable | — | — | Requires vi.mock platform/git/scm mock infrastructure |

---

