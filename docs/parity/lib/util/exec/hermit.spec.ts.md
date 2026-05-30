# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/hermit.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/hermit.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `util/exec/hermit › isHermit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when binarySource is hermit | 16 | ported | `exec/hermit.rs` | `is_hermit_true`, `is_hermit_false` | Rust takes `BinarySource` as param instead of reading global config |

### `util/exec/hermit › findHermitCwd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ("$dir") === $expected (hermit: $hermitLocation) | 30 | ported | `exec/hermit.rs` | `find_hermit_cwd_in_same_dir`, `find_hermit_cwd_in_parent_dir`, `find_hermit_cwd_in_nested_dir` | 3 of 4 parameterized cases ported; Rust uses manual parent traversal instead of `find-up` |
| should throw error when hermit cwd is not found | 49 | ported | `exec/hermit.rs` | `find_hermit_cwd_nonexistent` | Rust returns `None` instead of throwing |

### `util/exec/hermit › getHermitEnvs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return hermit environment variables when hermit env returns successfully | 62 | ported | `exec/hermit.rs` | `parse_hermit_env_output_parses_valid_lines`, `parse_hermit_env_output_skips_comments_and_empty_lines`, `parse_hermit_env_output_empty` | Parsing logic tested; actual `hermit env` execution not tested |

---

