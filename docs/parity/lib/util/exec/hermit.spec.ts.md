# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/hermit.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/hermit.spec.ts
**Total tests:** 6 | **Ported:** 2 | **Actionable:** 4 | **Status:** partial

### `util/exec/hermit › isHermit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when binarySource is hermit | 16 | ported | `exec/hermit.rs` | `is_hermit_true`, `is_hermit_false` | Rust takes `BinarySource` as param instead of reading global config |

### `util/exec/hermit › findHermitCwd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ("$dir") === $expected (hermit: $hermitLocation) | 30 | pending | — | — | TS uses `find-up` library; Rust uses manual parent traversal — different impl |
| should throw error when hermit cwd is not found | 49 | pending | — | — | Rust returns `None` from `find_hermit_cwd`; error thrown by `get_hermit_envs` instead |

### `util/exec/hermit › getHermitEnvs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return hermit environment variables when hermit env returns successfully | 62 | pending | — | — | Requires running `hermit` binary in test environment |

---

