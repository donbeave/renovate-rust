# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/reconfigure/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/reconfigure/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 8 | **Status:** done

### `workers/repository/reconfigure/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no effect when running with platform=local | 66 | not-applicable | — | — | Requires vi.mock platform/git/config mock infrastructure |
| no effect on repo with no reconfigure branch | 75 | not-applicable | — | — | Requires vi.mock platform/git/config mock infrastructure |
| skips if reconfigure branch unchanged | 85 | not-applicable | — | — | Requires vi.mock platform/git/config mock infrastructure |
| skips if error while finding reconfigure config | 103 | not-applicable | — | — | Requires vi.mock platform/git/config mock infrastructure |
| skips if reconfigure config is invalid | 115 | not-applicable | — | — | Requires vi.mock platform/git/config mock infrastructure |
| validates reconfigure branch and skips extraction if no reconfigure pr | 124 | not-applicable | — | — | Requires vi.mock platform/git/config mock infrastructure |
| extracts deps and adds comment when branch and reconfigure pr both exist | 133 | not-applicable | — | — | Requires vi.mock platform/git/config mock infrastructure |
| skips pr comment if error during deps extraction | 144 | not-applicable | — | — | Requires vi.mock platform/git/config mock infrastructure |

---

