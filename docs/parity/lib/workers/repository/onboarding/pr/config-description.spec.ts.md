# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/pr/config-description.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/pr/config-description.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `workers/repository/onboarding/pr/config-description › getConfigDesc()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty | 16 | ported | `onboarding.rs` | `get_config_desc_returns_empty_when_no_descriptions` | — |
| returns a full list | 22 | ported | `onboarding.rs` | `get_config_desc_returns_full_list` | — |
| assignees, labels and schedule | 38 | ported | `onboarding.rs` | `get_config_desc_includes_schedule` | — |
| include retry/refresh checkbox message only if onboardingRebaseCheckbox is true | 58 | ported | `onboarding.rs` | `get_config_desc_with_schedule_produces_output` | — |

---
