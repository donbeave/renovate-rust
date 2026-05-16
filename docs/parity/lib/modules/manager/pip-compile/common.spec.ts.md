# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip-compile/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip-compile/common.spec.ts
**Total tests:** 27 | **Ported:** 0 | **Actionable:** 27 | **Status:** pending

### `extractHeaderCommand()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| -v | 27 | pending | — | — | — |
| -v | 48 | pending | — | — | — |
| errors on malformed options with argument | 77 | pending | — | — | — |
| errors on unknown options | 89 | pending | — | — | — |
| always errors on not allowed options | 101 | pending | — | — | — |
| throws on duplicate options | 113 | pending | — | — | — |
| throws when no source files passed as arguments | 127 | pending | — | — | — |
| throws on malformed header | 136 | pending | — | — | — |
| throws on mutually exclusive options | 140 | pending | — | — | — |
| returned sourceFiles returns all source files | 151 | pending | — | — | — |
| returned sourceFiles must not contain options (pip-compile) | 169 | pending | — | — | — |
| returned sourceFiles must not contain options (uv) | 181 | pending | — | — | — |
| detects custom command | 193 | pending | — | — | — |
| infer exec directory (cwd) from output file path and header command | 202 | pending | — | — | — |

### `extractPythonVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts Python version from valid header | 217 | ported | `pip_compile.rs` | `extract_python_version_valid_header` | — |
| returns undefined if version cannot be extracted | 226 | ported | `pip_compile.rs` | `extract_python_version_empty_content` | — |

### `getRegistryCredVarsFromPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles both registryUrls and additionalRegistryUrls | 232 | pending | — | — | — |
| handles multiple additionalRegistryUrls | 259 | pending | — | — | — |
| handles hosts with only a username | 288 | pending | — | — | — |
| handles hosts with only a password | 306 | pending | — | — | — |
| handles invalid URLs | 324 | pending | — | — | — |
| handles multiple package files | 339 | pending | — | — | — |

### `matchManager()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches pip_setup setup.py | 370 | ported | `pip_compile.rs` | `match_manager_setup_py` | — |
| matches setup-cfg setup.cfg | 374 | ported | `pip_compile.rs` | `match_manager_setup_cfg` | — |
| matches pep621 pyproject.toml | 378 | ported | `pip_compile.rs` | `match_manager_pyproject_toml` | — |
| matches pip_requirements any .in file | 382 | ported | `pip_compile.rs` | `match_manager_in_file` | — |
| matches pip_requirements any .txt file | 387 | ported | `pip_compile.rs` | `match_manager_txt_file` | — |

---

