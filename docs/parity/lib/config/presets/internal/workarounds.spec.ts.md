# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/internal/workarounds.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/workarounds.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

### `config/presets/internal/workarounds › bitnamiDockerImageVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 13 | ported | `repo_config.rs` | `workaround_bitnami_docker_image_versioning_matches_upstream_cases` | — |
| matchCurrentValue("$input") == "$expected" | 28 | ported | `repo_config.rs` | `workaround_bitnami_docker_image_match_current_value_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › clamavDockerImageVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 49 | ported | `repo_config.rs` | `workaround_clamav_docker_image_versioning_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › libericaJdkDockerVersioning › Liberica JDK Lite`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 80 | ported | `repo_config.rs` | `workaround_liberica_jdk_lite_versioning_matches_upstream_cases` | — |
| matchCurrentValue("$input") == "$expected" | 95 | ported | `repo_config.rs` | `workaround_liberica_jdk_lite_match_current_value_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › libericaJdkDockerVersioning › Liberica JDK`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 118 | ported | `repo_config.rs` | `workaround_liberica_jdk_versioning_matches_upstream_cases` | — |
| matchCurrentValue("$input") == "$expected" | 133 | ported | `repo_config.rs` | `workaround_liberica_jdk_match_current_value_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › libericaJdkDockerVersioning › Liberica JRE`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 156 | ported | `repo_config.rs` | `workaround_liberica_jre_versioning_matches_upstream_cases` | — |
| matchCurrentValue("$input") == "$expected" | 171 | ported | `repo_config.rs` | `workaround_liberica_jre_match_current_value_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › javaLTSVersions › bellsoft/liberica-runtime-container`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| allowedVersisons("$input") == "$expected" | 196 | ported | `repo_config.rs` | `workaround_java_lts_liberica_runtime_allowed_versions_match_upstream_cases` | — |

---

