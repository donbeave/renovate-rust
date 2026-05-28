# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/aws-eks-addon/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/aws-eks-addon/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 7 | **Status:** not-applicable

### `modules/datasource/aws-eks-addon/index › getPkgReleases()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returned $des addons to be null | 92 | not-applicable | — | — | Uses aws-sdk-client-mock (EKSClient); AWS SDK mock infrastructure not portable to Rust |
| with addonName not supplied | 113 | not-applicable | — | — | Uses aws-sdk-client-mock (EKSClient); AWS SDK mock infrastructure not portable to Rust |
| with addonName only | 129 | not-applicable | — | — | Uses aws-sdk-client-mock (EKSClient); AWS SDK mock infrastructure not portable to Rust |
| with addon and profile | 160 | not-applicable | — | — | Uses aws-sdk-client-mock (EKSClient); AWS SDK mock infrastructure not portable to Rust |
| with addon and region | 169 | not-applicable | — | — | Uses aws-sdk-client-mock (EKSClient); AWS SDK mock infrastructure not portable to Rust |
| with addonName and default only config | 178 | not-applicable | — | — | Uses aws-sdk-client-mock (EKSClient); AWS SDK mock infrastructure not portable to Rust |
| with matched addon to return all versions of the addon | 204 | not-applicable | — | — | Uses aws-sdk-client-mock (EKSClient); AWS SDK mock infrastructure not portable to Rust |

---

