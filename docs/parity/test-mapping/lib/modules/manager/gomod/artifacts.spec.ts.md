# `lib/modules/manager/gomod/artifacts.spec.ts`

[← `manager/gomod`](../../../../_by-module/manager/gomod.md) · [all modules](../../../../README.md)

**26/56 in-scope tests ported** (30 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 94 | returns if no go.sum found | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:428`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L428) |
| 107 | returns null if unchanged | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:438`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L438) |
| 145 | returns updated go.sum | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:454`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L454) |
| 192 | runs go mod vendor with gomodvendor | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:633`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L633) |
| 244 | runs go work vendor with gomodvendor and go.work | pending | — |
| 300 | supports vendor directory update | pending | — |
| 390 | skips vendor directory update with gomodskipvendor | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:665`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L665) |
| 441 | supports vendor directory update with go.work | pending | — |
| 544 | supports vendor directory in the parent directory | pending | — |
| 647 | supports go generate when configured | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:700`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L700) |
| 735 | only allows go generate usage when permitted globally | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:742`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L742) |
| 790 | supports docker mode without credentials | pending | — |
| 853 | supports install mode without credentials | pending | — |
| 897 | supports global mode | pending | — |
| 934 | supports docker mode with credentials | pending | — |
| 1039 | supports docker mode with 2 credentials | pending | — |
| 1111 | supports docker mode with single credential | pending | — |
| 1170 | supports docker mode with multiple credentials for different paths | pending | — |
| 1243 | supports docker mode and ignores non http credentials | pending | — |
| 1307 | supports docker mode with many credentials | pending | — |
| 1393 | supports docker mode and ignores non git credentials | pending | — |
| 1456 | supports docker mode with gomodtidy | pending | — |
| 1520 | supports docker mode with gomodtidy1.17 | pending | — |
| 1584 | supports docker mode with gomodtidye and gomodtidy1.17 | pending | — |
| 1648 | supports docker mode with gomodtidye | pending | — |
| 1712 | catches errors | pending | — |
| 1739 | updates import paths with gomodupdateimportpaths | pending | — |
| 1795 | updates correct import paths with gomodupdateimportpaths and multiple dependencies | pending | — |
| 1856 | skips updating import paths with gomodupdateimportpaths on v0 to v1 | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:1068`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L1068) |
| 1902 | skips updating import paths when invalid major version | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:1023`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L1023) |
| 1948 | skips updating import paths when incompatible version | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:978`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L978) |
| 1998 | skips gomodtidy without gomodupdateimportpaths on major update | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:846`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L846) |
| 2036 | does not execute go mod tidy when none of gomodtidy and gomodupdateimportpaths are set | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:882`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L882) |
| 2073 | updates import paths with specific tool version from constraint | pending | — |
| 2133 | updates import paths with latest tool version on invalid version constraint | pending | — |
| 2193 | updates import paths for gopkg.in dependencies including v0 to v1 | pending | — |
| 2252 | gomod file and config do not contain goconstraints | pending | — |
| 2311 | go.mod file contains go version | pending | — |
| 2384 | go.mod file contains go toolchain version | pending | — |
| 2425 | go.mod file contains full go version without toolchain | pending | — |
| 2466 | returns artifact notices | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:1113`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L1113) |
| 2507 | config contains go version | pending | — |
| 2582 | handles gogetdirs configuration correctly | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:776`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L776) |
| 2613 | returns updated go.sum when gogetdirs is specified | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:805`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L805) |
| 2654 | errors when gogetdirs is specified with all invalid paths | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:829`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L829) |
| 2681 | throws temporary error | pending | — |
| 2698 | uses -modfile flag for non-default go.mod filename | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:570`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L570) |
| 2733 | uses -modfile flag with go mod tidy for non-default go.mod filename | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:602`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L602) |
| 2779 | uses -modfile flag with go mod vendor for non-default go.mod filename | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:1145`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L1145) |
| 2837 | returns config constraint when set | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:913`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L913) |
| 2843 | config constraint takes precedence over go.mod content | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:922`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L922) |
| 2852 | returns toolchain version when toolchain directive is present | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:931`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L931) |
| 2858 | returns full go version when only full go directive is present (no toolchain) | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:940`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L940) |
| 2862 | returns range constraint for major.minor go directive | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:949`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L949) |
| 2866 | returns undefined when no go version in content and no config constraint | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:958`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L958) |
| 2873 | ignores constraints.golang and falls back to go.mod content | ported | [`crates/renovate-core/src/extractors/gomod_artifact_runner.rs:967`](../../../../../../../crates/renovate-core/src/extractors/gomod_artifact_runner.rs#L967) |

