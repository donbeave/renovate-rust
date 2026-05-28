# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip-compile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip-compile/extract.spec.ts
**Total tests:** 26 | **Ported:** 4 | **Actionable:** 26 | **Status:** done

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
| support package file with multiple lock files | 105 | not-applicable | — | — | Uses vi.mock(fs) + vi.mocked(fs.readLocalFile) + logger.warn spy |
| no lock files in returned package files | 137 | not-applicable | — | — | Uses vi.mock(fs) + logger.warn spy |
| no constraint files in returned package files | 162 | not-applicable | — | — | Uses vi.mock(fs) + logger.warn spy |
| return null for malformed files | 183 | not-applicable | — | — | Uses vi.mock(fs) + logger.warn spy |
| return null for bad paths | 221 | not-applicable | — | — | Uses vi.mock(fs) + logger.warn spy |
| return for valid paths | 246 | not-applicable | — | — | Uses vi.mock(fs) |
| return sorted package files | 281 | not-applicable | — | — | Uses vi.mock(fs) |
| return sorted package files with constraint in file | 311 | not-applicable | — | — | Uses vi.mock(fs) |
| return sorted package files with constraint in command | 335 | not-applicable | — | — | Uses vi.mock(fs) |
| adds lockedVersion to deps in package file | 360 | not-applicable | — | — | Uses vi.mock(fs) |
| warns if dependency has no locked version | 382 | not-applicable | — | — | Uses vi.mock(fs) + logger.warn spy |
| adds transitive dependency to deps in package file | 403 | not-applicable | — | — | Uses vi.mock(fs) |
| handles -r reference to another input file | 427 | not-applicable | — | — | Uses vi.mock(fs) |
| handles transitive -r references | 455 | not-applicable | — | — | Uses vi.mock(fs) |
| warns on -r reference to failed file | 491 | not-applicable | — | — | Uses vi.mock(fs) + logger.warn spy |
| warns on -r reference to requirements file not managed by pip-compile | 516 | not-applicable | — | — | Uses vi.mock(fs) + logger.warn spy |
| handles duplicate -r dependencies | 539 | not-applicable | — | — | Uses vi.mock(fs) |
| handles -r dependency on lock file with multiple input files | 583 | not-applicable | — | — | Uses vi.mock(fs) |
| handles -r dependency on input file that is also used to generate lock file with multiple inputs | 614 | not-applicable | — | — | Uses vi.mock(fs) |
| handles -r dependency on file with relative path same dir | 645 | not-applicable | — | — | Uses vi.mock(fs) |
| handles -r dependency on file with relative path above | 673 | not-applicable | — | — | Uses vi.mock(fs) |
| handles -r dependency on file with relative path above with path | 701 | not-applicable | — | — | Uses vi.mock(fs) |

---

