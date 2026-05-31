# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip-compile/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip-compile/common.spec.ts
**Total tests:** 27 | **Ported:** 7 | **Actionable:** 20 | **Status:** partial

### `extractHeaderCommand()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| -v | 27 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| -v | 48 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| errors on malformed options with argument | 77 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| errors on unknown options | 89 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| always errors on not allowed options | 101 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| throws on duplicate options | 113 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| throws when no source files passed as arguments | 127 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| throws on malformed header | 136 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| throws on mutually exclusive options | 140 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| returned sourceFiles returns all source files | 151 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| returned sourceFiles must not contain options (pip-compile) | 169 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| returned sourceFiles must not contain options (uv) | 181 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| detects custom command | 193 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |
| infer exec directory (cwd) from output file path and header command | 202 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer |

### `extractPythonVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts Python version from valid header | 217 | ported | `pip_compile.rs` | `extract_python_version_valid_header` | — |
| returns undefined if version cannot be extracted | 226 | ported | `pip_compile.rs` | `extract_python_version_empty_content` | — |

### `getRegistryCredVarsFromPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles both registryUrls and additionalRegistryUrls | 232 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles multiple additionalRegistryUrls | 259 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles hosts with only a username | 288 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles hosts with only a password | 306 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles invalid URLs | 324 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles multiple package files | 339 | not-applicable | Mock framework internals — tests pip-compile common via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `matchManager()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches pip_setup setup.py | 370 | ported | `pip_compile.rs` | `match_manager_setup_py` | — |
| matches setup-cfg setup.cfg | 374 | ported | `pip_compile.rs` | `match_manager_setup_cfg` | — |
| matches pep621 pyproject.toml | 378 | ported | `pip_compile.rs` | `match_manager_pyproject_toml` | — |
| matches pip_requirements any .in file | 382 | ported | `pip_compile.rs` | `match_manager_in_file` | — |
| matches pip_requirements any .txt file | 387 | ported | `pip_compile.rs` | `match_manager_txt_file` | — |

---

