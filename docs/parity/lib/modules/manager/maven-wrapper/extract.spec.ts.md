# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/maven-wrapper/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven-wrapper/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts version for property file with distribution type "bin" in distributionUrl | 14 | ported | `maven_wrapper.rs` | `extracts_wrapper_and_maven_properties` | — |
| extracts version for property file with only a wrapper url | 37 | ported | `maven_wrapper.rs` | `extracts_only_wrapper_url` | — |
| extracts version for property file with only a wrapper version | 51 | ported | `maven_wrapper.rs` | `extracts_only_wrapper_version_key` | — |
| extracts wrapper information from wrapperUrl in precedence to wrapperVersion | 64 | ported | `maven_wrapper.rs` | `wrapper_url_takes_precedence_over_wrapper_version` | — |
| extracts maven warapper version from mvnw file | 80 | ported | `maven_wrapper.rs` | `extracts_version_from_mvnw_unix` | — |
| extracts maven warapper version from mvnw file - Windows | 93 | ported | `maven_wrapper.rs` | `extracts_version_from_mvnw_windows` | — |
| returns null for invalid wrapper version string in from mvnw file | 106 | ported | `maven_wrapper.rs` | `invalid_mvnw_prefix_returns_empty` | — |
| extracts version for property file with only a maven url | 111 | ported | `maven_wrapper.rs` | `extracts_maven_version` | — |
| should return null when there is no string matching the maven properties regex | 125 | ported | `maven_wrapper.rs` | `no_matching_key_returns_empty` | — |

---

