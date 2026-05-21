# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmv3/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmv3/extract.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips invalid registry urls | 16 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| parses simple Chart.yaml correctly | 40 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| extract correctly oci references | 67 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| resolves aliased registry urls | 100 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| doesn't fail if Chart.yaml is invalid | 131 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| skips local dependencies | 142 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| returns null if no dependencies key | 167 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| returns null if dependencies are an empty list | 183 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| returns null if dependencies key is invalid | 199 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| returns null if Chart.yaml is empty | 215 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| returns null if Chart.yaml uses an unsupported apiVersion | 222 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |
| returns null if name and version are missing for all dependencies | 235 | not-applicable | — | — | tests Helm chart extraction; Rust helm extractor uses own parsing approach |

---

