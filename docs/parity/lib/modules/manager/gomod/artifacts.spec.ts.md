# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gomod/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/artifacts.spec.ts
**Total tests:** 56 | **Ported:** 0 | **Actionable:** 56 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no go.sum found | 93 | pending | — | — | — |
| returns null if unchanged | 106 | pending | — | — | — |
| returns updated go.sum | 144 | pending | — | — | — |
| runs go mod vendor with gomodVendor | 191 | pending | — | — | — |
| runs go work vendor with gomodVendor and go.work | 243 | pending | — | — | — |
| supports vendor directory update | 299 | pending | — | — | — |
| skips vendor directory update with gomodSkipVendor | 389 | pending | — | — | — |
| supports vendor directory update with go.work | 440 | pending | — | — | — |
| supports vendor directory in the parent directory | 543 | pending | — | — | — |
| supports go generate when configured | 646 | pending | — | — | — |
| only allows go generate usage when permitted globally | 734 | pending | — | — | — |
| supports docker mode without credentials | 789 | pending | — | — | — |
| supports install mode without credentials | 852 | pending | — | — | — |
| supports global mode | 896 | pending | — | — | — |
| supports docker mode with credentials | 933 | pending | — | — | — |
| supports docker mode with 2 credentials | 1038 | pending | — | — | — |
| supports docker mode with single credential | 1110 | pending | — | — | — |
| supports docker mode with multiple credentials for different paths | 1169 | pending | — | — | — |
| supports docker mode and ignores non http credentials | 1242 | pending | — | — | — |
| supports docker mode with many credentials | 1306 | pending | — | — | — |
| supports docker mode and ignores non git credentials | 1392 | pending | — | — | — |
| supports docker mode with goModTidy | 1455 | pending | — | — | — |
| supports docker mode with gomodTidy1.17 | 1519 | pending | — | — | — |
| supports docker mode with gomodTidyE and gomodTidy1.17 | 1583 | pending | — | — | — |
| supports docker mode with gomodTidyE | 1647 | pending | — | — | — |
| catches errors | 1711 | pending | — | — | — |
| updates import paths with gomodUpdateImportPaths | 1738 | pending | — | — | — |
| updates correct import paths with gomodUpdateImportPaths and multiple dependencies | 1794 | pending | — | — | — |
| skips updating import paths with gomodUpdateImportPaths on v0 to v1 | 1855 | pending | — | — | — |
| skips updating import paths when invalid major version | 1901 | pending | — | — | — |
| skips updating import paths when incompatible version | 1947 | pending | — | — | — |
| skips gomodTidy without gomodUpdateImportPaths on major update | 1997 | pending | — | — | — |
| does not execute go mod tidy when none of gomodTidy and gomodUpdateImportPaths are set | 2035 | pending | — | — | — |
| updates import paths with specific tool version from constraint | 2072 | pending | — | — | — |
| updates import paths with latest tool version on invalid version constraint | 2132 | pending | — | — | — |
| updates import paths for gopkg.in dependencies including v0 to v1 | 2192 | pending | — | — | — |
| gomod file and config do not contain GoConstraints | 2251 | pending | — | — | — |
| go.mod file contains go version | 2310 | pending | — | — | — |
| go.mod file contains go toolchain version | 2383 | pending | — | — | — |
| go.mod file contains full go version without toolchain | 2424 | pending | — | — | — |
| returns artifact notices | 2465 | pending | — | — | — |
| config contains go version | 2506 | pending | — | — | — |
| handles goGetDirs configuration correctly | 2581 | pending | — | — | — |
| returns updated go.sum when goGetDirs is specified | 2612 | pending | — | — | — |
| errors when goGetDirs is specified with all invalid paths | 2653 | pending | — | — | — |
| throws temporary error | 2680 | pending | — | — | — |
| uses -modfile flag for non-default go.mod filename | 2697 | pending | — | — | — |
| uses -modfile flag with go mod tidy for non-default go.mod filename | 2732 | pending | — | — | — |
| uses -modfile flag with go mod vendor for non-default go.mod filename | 2778 | pending | — | — | — |

### `deriveGoToolchainConstraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config constraint when set | 2836 | pending | — | — | — |
| config constraint takes precedence over go.mod content | 2842 | pending | — | — | — |
| returns toolchain version when toolchain directive is present | 2851 | pending | — | — | — |
| returns full go version when only full go directive is present (no toolchain) | 2857 | pending | — | — | — |
| returns range constraint for major.minor go directive | 2861 | pending | — | — | — |
| returns undefined when no go version in content and no config constraint | 2865 | pending | — | — | — |
| ignores constraints.golang and falls back to go.mod content | 2872 | pending | — | — | — |

---

