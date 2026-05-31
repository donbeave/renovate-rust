# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gomod/integration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/integration.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 1 | **Status:** pending-applicable

### `when constraintsFiltering=strict`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only suggests updates within the minor version of the `go` directive | 21 | pending | — | — | integration test with complex HTTP mocking — tests end-to-end Go directive constraint filtering with httpMock; Rust architecture differs significantly (HTTP layer, extraction, versioning separation); individual components tested elsewhere |

---

