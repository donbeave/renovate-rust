# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/reconfigure/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/reconfigure/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 8 | **Status:** pending-applicable-applicable-applicable

### `workers/repository/reconfigure/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no effect when running with platform=local  | 66 | pending | — | — | Worker orchestration / platform integration |
| no effect on repo with no reconfigure branch  | 75 | pending | — | — | Worker orchestration / platform integration |
| skips if reconfigure branch unchanged  | 85 | pending | — | — | Worker orchestration / platform integration |
| skips if error while finding reconfigure config  | 103 | pending | — | — | Worker orchestration / platform integration |
| skips if reconfigure config is invalid  | 115 | pending | — | — | Worker orchestration / platform integration |
| validates reconfigure branch and skips extraction if no reconfigure pr  | 124 | pending | — | — | Worker orchestration / platform integration |
| extracts deps and adds comment when branch and reconfigure pr both exist  | 133 | pending | — | — | Worker orchestration / platform integration |
| skips pr comment if error during deps extraction  | 144 | pending | — | — | Worker orchestration / platform integration |

---

