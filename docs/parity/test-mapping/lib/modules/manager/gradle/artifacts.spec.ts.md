# `lib/modules/manager/gradle/artifacts.spec.ts`

[← `manager/gradle`](../../../../_by-module/manager/gradle.md) · [all modules](../../../../README.md)

**0/27 in-scope tests ported** (27 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 91 | returns false when allowedunsafeexecutions is empty (as it not enabled by default option) | pending | — |
| 102 | returns true when allowedunsafeexecutions includes `gradlewrapper` | pending | — |
| 113 | returns false when allowedunsafeexecutions does not include `gradlewrapper` | pending | — |
| 124 | logs when allowedunsafeexecutions does not include `gradlewrapper` | pending | — |
| 139 | aborts if no lockfile is found | pending | — |
| 158 | aborts if lock file exists but no gradle wrapper | pending | — |
| 177 | aborts if allowedunsafeexecutions does not include `gradlewrapper` | pending | — |
| 203 | uses custom jvm heap settings when toolsettings are configured | pending | — |
| 248 | updates lock file | pending | — |
| 289 | updates lock file in win32 | pending | — |
| 334 | prefers packagename over depname if provided | pending | — |
| 379 | aborts lock file maintenance if packagefilename is not build.gradle(.kts) in root project | pending | — |
| 394 | performs lock file maintenance | pending | — |
| 432 | performs lock file maintenance (docker) | pending | — |
| 496 | performs lock file maintenance (install) | pending | — |
| 535 | updates all included projects | pending | — |
| 579 | does not update lockfile if content is unchanged | pending | — |
| 593 | gradlew failed | pending | — |
| 622 | rethrows temporary error | pending | — |
| 641 | fallback to default java version if gradle version not extractable | pending | — |
| 685 | updates verification metadata file | pending | — |
| 732 | aborts verification metadata updates if allowedunsafeexecutions does not include `gradlewrapper` | pending | — |
| 766 | updates existing checksums also if verify-checksums is disabled | pending | — |
| 821 | updates verification metadata and lock file | pending | — |
| 895 | uses sha256 as default if only weak hash algorithms are found | pending | — |
| 940 | uses pgp hashtype if verify-signatures is enabled | pending | — |
| 984 | does not write verification metadata, when no checksums exist and neither checksum nor signature verification is enabled | pending | — |

