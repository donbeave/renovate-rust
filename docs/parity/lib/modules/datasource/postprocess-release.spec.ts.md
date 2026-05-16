# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/postprocess-release.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/postprocess-release.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/postprocess-release`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns original release for empty datasource field | 27 | not-applicable | — | — | Renovate's dynamic datasource `postprocessRelease` hook dispatch is not implemented in Rust. |
| returns original release for missing datasource | 36 | not-applicable | — | — | Renovate's dynamic datasource `postprocessRelease` hook dispatch is not implemented in Rust. |
| returns original release for datasource with missing `postprocessRelease` method | 48 | not-applicable | — | — | Renovate's dynamic datasource `postprocessRelease` hook dispatch is not implemented in Rust. |
| returns original release for datasource with missing `packageName` field | 60 | not-applicable | — | — | Renovate's dynamic datasource `postprocessRelease` hook dispatch is not implemented in Rust. |
| updates release via `postprocessRelease` method | 81 | not-applicable | — | — | Renovate's dynamic datasource `postprocessRelease` hook dispatch is not implemented in Rust. |
| rejects release via `postprocessRelease` method | 110 | not-applicable | — | — | Renovate's dynamic datasource `postprocessRelease` hook dispatch is not implemented in Rust. |
| falls back when error was thrown | 131 | not-applicable | — | — | Renovate's dynamic datasource `postprocessRelease` hook dispatch is not implemented in Rust. |

---

