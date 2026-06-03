# `lib/modules/manager/pip-compile/extract.spec.ts`

[← `manager/pip-compile`](../../../../_by-module/manager/pip-compile.md) · [all modules](../../../../README.md)

**4/27 in-scope tests ported** (23 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 40 | returns object for requirements.in | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:171`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L171) |
| 50 | returns object for setup.py | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:183`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L183) |
| 60 | returns object for pyproject.toml | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:203`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L203) |
| 93 | _(it.each / template — verify manually)_ | ? | — |
| 105 | support package file with multiple lock files | pending | — |
| 137 | no lock files in returned package files | pending | — |
| 161 | no override files in returned package files | pending | — |
| 179 | no constraint files in returned package files | pending | — |
| 200 | return null for malformed files | pending | — |
| 238 | return null for bad paths | pending | — |
| 263 | return for valid paths | pending | — |
| 298 | return sorted package files | pending | — |
| 328 | return sorted package files with constraint in file | pending | — |
| 352 | return sorted package files with constraint in command | pending | — |
| 377 | adds lockedversion to deps in package file | pending | — |
| 399 | warns if dependency has no locked version | pending | — |
| 420 | adds transitive dependency to deps in package file | pending | — |
| 444 | handles -r reference to another input file | pending | — |
| 472 | handles transitive -r references | pending | — |
| 508 | warns on -r reference to failed file | pending | — |
| 533 | warns on -r reference to requirements file not managed by pip-compile | pending | — |
| 556 | handles duplicate -r dependencies | pending | — |
| 600 | handles -r dependency on lock file with multiple input files | pending | — |
| 631 | handles -r dependency on input file that is also used to generate lock file with multiple inputs | pending | — |
| 662 | handles -r dependency on file with relative path same dir | pending | — |
| 690 | handles -r dependency on file with relative path above | pending | — |
| 718 | handles -r dependency on file with relative path above with path | pending | — |

