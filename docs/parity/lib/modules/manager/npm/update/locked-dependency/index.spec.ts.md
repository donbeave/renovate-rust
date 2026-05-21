# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/locked-dependency/index.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateLockedDependency()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates filename | 45 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| validates versions | 54 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| returns null for unparseable files | 63 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| rejects lockFileVersion 2 | 72 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| returns null if no locked deps | 81 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| rejects null if no constraint found | 85 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| remediates in-range | 97 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| rejects in-range remediation if lockfile v2+ | 109 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| fails to remediate if parent dep cannot support | 120 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| remediates express | 140 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| remediates lock file v2 express | 150 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| returns already-updated if already remediated exactly | 161 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| returns already-updated if already v2 remediated exactly | 169 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| returns already-updated if already remediated higher | 178 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| returns already-updated if not found | 187 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| returns update-failed if other, lower version found | 196 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| remediates mime | 205 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| fails remediation if cannot update parent | 222 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| fails remediation if bundled | 231 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |
| rejects in-range remediation if pnpm | 241 | not-applicable | — | — | tests locked dependency update requiring full npm/yarn lockfile manipulation |

---

