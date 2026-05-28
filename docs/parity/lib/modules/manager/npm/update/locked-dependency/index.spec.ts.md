# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/locked-dependency/index.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** done

### `updateLockedDependency()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates filename | 45 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| validates versions | 54 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| returns null for unparseable files | 63 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| rejects lockFileVersion 2 | 72 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| returns null if no locked deps | 81 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| rejects null if no constraint found | 85 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| remediates in-range | 97 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| rejects in-range remediation if lockfile v2+ | 109 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| fails to remediate if parent dep cannot support | 120 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| remediates express | 140 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| remediates lock file v2 express | 150 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| returns already-updated if already remediated exactly | 161 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| returns already-updated if already v2 remediated exactly | 169 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| returns already-updated if already remediated higher | 178 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| returns already-updated if not found | 187 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| returns update-failed if other, lower version found | 196 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| remediates mime | 205 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| fails remediation if cannot update parent | 222 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| fails remediation if bundled | 231 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |
| rejects in-range remediation if pnpm | 241 | not-applicable | — | — | Requires httpMock for npm registry fixture responses |

---
