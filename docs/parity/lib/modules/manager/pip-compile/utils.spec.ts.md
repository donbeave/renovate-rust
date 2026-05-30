# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip-compile/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip-compile/utils.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `inferCommandExecDir()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns object on correct options | 5 | ported | `pip_compile.rs` | `infer_exec_dir_same_subdir` + `infer_exec_dir_output_in_parent` | — |
| throw if --output-file basename differs from path | 23 | ported | `pip_compile.rs` | `infer_exec_dir_throws_on_basename_mismatch` | — |

---

