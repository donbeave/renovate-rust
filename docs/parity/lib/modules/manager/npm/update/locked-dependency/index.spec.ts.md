# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/locked-dependency/index.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** pending

### `updateLockedDependency()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates filename | 45 | pending | — | — | — |
| validates versions | 54 | pending | — | — | — |
| returns null for unparseable files | 63 | pending | — | — | — |
| rejects lockFileVersion 2 | 72 | pending | — | — | — |
| returns null if no locked deps | 81 | pending | — | — | — |
| rejects null if no constraint found | 85 | pending | — | — | — |
| remediates in-range | 97 | pending | — | — | — |
| rejects in-range remediation if lockfile v2+ | 109 | pending | — | — | — |
| fails to remediate if parent dep cannot support | 120 | pending | — | — | — |
| remediates express | 140 | pending | — | — | — |
| remediates lock file v2 express | 150 | pending | — | — | — |
| returns already-updated if already remediated exactly | 161 | pending | — | — | — |
| returns already-updated if already v2 remediated exactly | 169 | pending | — | — | — |
| returns already-updated if already remediated higher | 178 | pending | — | — | — |
| returns already-updated if not found | 187 | pending | — | — | — |
| returns update-failed if other, lower version found | 196 | pending | — | — | — |
| remediates mime | 205 | pending | — | — | — |
| fails remediation if cannot update parent | 222 | pending | — | — | — |
| fails remediation if bundled | 231 | pending | — | — | — |
| rejects in-range remediation if pnpm | 241 | pending | — | — | — |

---

