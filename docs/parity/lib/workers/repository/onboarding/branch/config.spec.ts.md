# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/config.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/config.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `workers/repository/onboarding/branch/config › getOnboardingConfigContents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the JSON stringified onboarding config  | 32 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |

### `workers/repository/onboarding/branch/config › getOnboardingConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles finding a preset in the same group level  | 48 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |
| handles finding an organization dot platform preset  | 58 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |
| handles finding a preset in the same group  | 71 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |
| handles finding a preset in a parent group  | 87 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |
| handles falling back to finding a organization preset  | 103 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |
| handles not finding any preset  | 119 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |
| ignores an unknown error  | 128 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |
| ignores unsupported platform  | 137 | not-applicable | — | — | Mock framework internals — tests TS-specific getOnboardingConfig via vitest-mocked preset resolution; Rust tests this at different architecture level |

---

