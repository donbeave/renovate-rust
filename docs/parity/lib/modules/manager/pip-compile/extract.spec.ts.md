# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip-compile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip-compile/extract.spec.ts
**Total tests:** 26 | **Ported:** 4 | **Actionable:** 26 | **Status:** partial

The pip-compile `extractPackageFile()` adapter is ported. The remaining rows
are not applicable because all `extractAllPackageFiles()` tests use vi.mock(fs)
+ vi.mocked(fs.readLocalFile) async mocks and logger.warn spies — infrastructure
not portable to Rust.

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns object for requirements.in | 40 | ported | `pip_compile.rs` | `returns_object_for_requirements_in` | — |
| returns object for setup.py | 50 | ported | `pip_compile.rs` | `returns_object_for_setup_py` | — |
| returns object for pyproject.toml | 60 | ported | `pip_compile.rs` | `returns_object_for_pyproject_toml` | — |
| handles different file extensions (it.each) | 93 | ported | `pip_compile.rs` | `returns_null_on_not_supported_package_files` | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| support package file with multiple lock files | 105 | pending | — | — | —|
| no lock files in returned package files | 137 | pending | — | — | —|
| no constraint files in returned package files | 162 | pending | — | — | —|
| return null for malformed files | 183 | pending | — | — | —|
| return null for bad paths | 221 | pending | — | — | —|
| return for valid paths | 246 | pending | — | — | —|
| return sorted package files | 281 | pending | — | — | —|
| return sorted package files with constraint in file | 311 | pending | — | — | —|
| return sorted package files with constraint in command | 335 | pending | — | — | —|
| adds lockedVersion to deps in package file | 360 | pending | — | — | —|
| warns if dependency has no locked version | 382 | pending | — | — | —|
| adds transitive dependency to deps in package file | 403 | pending | — | — | —|
| handles -r reference to another input file | 427 | pending | — | — | —|
| handles transitive -r references | 455 | pending | — | — | —|
| warns on -r reference to failed file | 491 | pending | — | — | —|
| warns on -r reference to requirements file not managed by pip-compile | 516 | pending | — | — | —|
| handles duplicate -r dependencies | 539 | pending | — | — | —|
| handles -r dependency on lock file with multiple input files | 583 | pending | — | — | —|
| handles -r dependency on input file that is also used to generate lock file with multiple inputs | 614 | pending | — | — | —|
| handles -r dependency on file with relative path same dir | 645 | pending | — | — | —|
| handles -r dependency on file with relative path above | 673 | pending | — | — | —|
| handles -r dependency on file with relative path above with path | 701 | pending | — | — | —|

---

