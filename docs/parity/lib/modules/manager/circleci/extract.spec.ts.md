# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/circleci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/circleci/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `circleci.rs` | `empty_content_returns_no_deps` | — |
| handles registry alias | 16 | ported | `circleci.rs` | `handles_registry_alias` | — |
| extracts multiple image and resolves yaml anchors | 48 | ported | `circleci.rs` | `fixture_config_resolves_yaml_anchor_images` | — |
| extracts orbs too | 93 | ported | `circleci.rs` | `extracts_orbs` | — |
| extracts image without leading dash | 200 | ported | `circleci.rs` | `anchor_image_without_leading_dash_is_resolved` | — |
| extracts and exclude android images | 226 | ported | `circleci.rs` | `machine_image_not_extracted` | — |
| extracts orbs without jobs | 237 | ported | `circleci.rs` | `extracts_orbs_without_jobs` | — |
| extracts executors | 251 | ported | `circleci.rs` | `executor_docker_image_extracted` | — |
| extracts orb definitions | 273 | ported | `circleci.rs` | `extracts_orb_definitions` | — |

---

