# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmv3/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmv3/extract.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** pending

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips invalid registry urls | 16 | pending | — | — | — |
| parses simple Chart.yaml correctly | 40 | pending | — | — | — |
| extract correctly oci references | 67 | pending | — | — | — |
| resolves aliased registry urls | 100 | pending | — | — | — |
| doesn't fail if Chart.yaml is invalid | 131 | pending | — | — | — |
| skips local dependencies | 142 | pending | — | — | — |
| returns null if no dependencies key | 167 | pending | — | — | — |
| returns null if dependencies are an empty list | 183 | pending | — | — | — |
| returns null if dependencies key is invalid | 199 | pending | — | — | — |
| returns null if Chart.yaml is empty | 215 | pending | — | — | — |
| returns null if Chart.yaml uses an unsupported apiVersion | 222 | pending | — | — | — |
| returns null if name and version are missing for all dependencies | 235 | pending | — | — | — |

---

