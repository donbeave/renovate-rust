# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/jenkins/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/jenkins/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty list for an empty text file | 15 | ported | `jenkins.rs` | `txt_empty_file_returns_empty` | — |
| returns empty list for an empty yaml file | 21 | ported | `jenkins.rs` | `yml_empty_returns_empty` | — |
| returns empty list for an invalid yaml file | 27 | ported | `jenkins.rs` | `yml_invalid_yaml_returns_empty` | — |
| extracts multiple image lines in text format | 33 | ported | `jenkins.rs` | `txt_plugins_fixture_six_deps` | — |
| extracts multiple image lines in yaml format | 40 | ported | `jenkins.rs` | `yml_plugins_fixture_eight_deps` | — |

---

