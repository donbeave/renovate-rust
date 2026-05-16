# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/platform/scm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/scm.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/platform/scm`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no platform chosen | 9 | not-applicable | — | — | Renovate's TypeScript global SCM facade is not implemented as a Rust API; Rust uses `AnyPlatformClient` and local git behavior separately. |
| unknown platform | 13 | not-applicable | — | — | Renovate's TypeScript global SCM facade and platform registry are not implemented as a Rust API. |
| use util/git module as default implementation for platform %s | 19 | not-applicable | — | — | Renovate's TypeScript default SCM-to-git delegation facade is not implemented as a Rust API. |

---

