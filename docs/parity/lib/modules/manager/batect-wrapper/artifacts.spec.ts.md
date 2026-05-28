# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/batect-wrapper/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/batect-wrapper/artifacts.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** partial

### `modules/manager/batect-wrapper/artifacts › updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated files if the wrapper script is in the root directory | 54 | not-applicable | — | — | Requires httpMock.scope for wrapper script download |
| returns updated files if the wrapper script is in a subdirectory | 76 | not-applicable | — | — | Requires httpMock.scope for wrapper script download |
| returns an error if the updated wrapper script cannot be downloaded | 98 | not-applicable | — | — | Requires httpMock.scope for wrapper script download |

---

