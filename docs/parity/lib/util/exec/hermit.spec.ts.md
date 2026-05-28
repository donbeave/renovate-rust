# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/hermit.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/hermit.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 4 | **Status:** not-applicable

### `util/exec/hermit › isHermit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when binarySource is hermit | 16 | not-applicable | — | — | Uses vi.mock find-up + mockExecAll; module mock + exec mock infrastructure not portable |

### `util/exec/hermit › findHermitCwd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ("$dir") === $expected (hermit: $hermitLocation) | 30 | not-applicable | — | — | Uses vi.mock find-up + mockExecAll; module mock + exec mock infrastructure not portable |
| should throw error when hermit cwd is not found | 49 | not-applicable | — | — | Uses vi.mock find-up + mockExecAll; module mock + exec mock infrastructure not portable |

### `util/exec/hermit › getHermitEnvs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return hermit environment variables when hermit env returns successfully | 62 | not-applicable | — | — | Uses vi.mock find-up + mockExecAll; module mock + exec mock infrastructure not portable |

---

