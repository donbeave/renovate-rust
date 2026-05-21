# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gomod/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/artifacts.spec.ts
**Total tests:** 56 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no go.sum found | 93 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if unchanged | 106 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated go.sum | 144 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| runs go mod vendor with gomodVendor | 191 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| runs go work vendor with gomodVendor and go.work | 243 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports vendor directory update | 299 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips vendor directory update with gomodSkipVendor | 389 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports vendor directory update with go.work | 440 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports vendor directory in the parent directory | 543 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports go generate when configured | 646 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| only allows go generate usage when permitted globally | 734 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode without credentials | 789 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports install mode without credentials | 852 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports global mode | 896 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with credentials | 933 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with 2 credentials | 1038 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with single credential | 1110 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with multiple credentials for different paths | 1169 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode and ignores non http credentials | 1242 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with many credentials | 1306 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode and ignores non git credentials | 1392 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with goModTidy | 1455 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with gomodTidy1.17 | 1519 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with gomodTidyE and gomodTidy1.17 | 1583 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with gomodTidyE | 1647 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches errors | 1711 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates import paths with gomodUpdateImportPaths | 1738 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates correct import paths with gomodUpdateImportPaths and multiple dependencies | 1794 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips updating import paths with gomodUpdateImportPaths on v0 to v1 | 1855 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips updating import paths when invalid major version | 1901 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips updating import paths when incompatible version | 1947 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips gomodTidy without gomodUpdateImportPaths on major update | 1997 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not execute go mod tidy when none of gomodTidy and gomodUpdateImportPaths are set | 2035 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates import paths with specific tool version from constraint | 2072 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates import paths with latest tool version on invalid version constraint | 2132 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates import paths for gopkg.in dependencies including v0 to v1 | 2192 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| gomod file and config do not contain GoConstraints | 2251 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| go.mod file contains go version | 2310 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| go.mod file contains go toolchain version | 2383 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| go.mod file contains full go version without toolchain | 2424 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns artifact notices | 2465 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| config contains go version | 2506 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles goGetDirs configuration correctly | 2581 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated go.sum when goGetDirs is specified | 2612 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| errors when goGetDirs is specified with all invalid paths | 2653 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| throws temporary error | 2680 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses -modfile flag for non-default go.mod filename | 2697 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses -modfile flag with go mod tidy for non-default go.mod filename | 2732 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses -modfile flag with go mod vendor for non-default go.mod filename | 2778 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `deriveGoToolchainConstraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config constraint when set | 2836 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| config constraint takes precedence over go.mod content | 2842 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns toolchain version when toolchain directive is present | 2851 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns full go version when only full go directive is present (no toolchain) | 2857 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns range constraint for major.minor go directive | 2861 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns undefined when no go version in content and no config constraint | 2865 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| ignores constraints.golang and falls back to go.mod content | 2872 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---

