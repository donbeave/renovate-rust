# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmfile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmfile/extract.spec.ts
**Total tests:** 20 | **Ported:** 19 | **Actionable:** 20 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skip null YAML document | 18 | ported | `helmfile.rs` | `null_yaml_document_returns_empty` | — |
| returns null if no releases | 31 | ported | `helmfile.rs` | `no_releases_section_returns_empty` | — |
| do not crash on invalid helmfile.yaml | 46 | ported | `helmfile.rs` | `invalid_yaml_does_not_crash` | — |
| skip if repository details are not specified | 63 | ported | `helmfile.rs` | `unknown_repo_alias_has_skip_reason` | — |
| skip templetized release with invalid characters | 84 | ported | `helmfile.rs` | `invalid_chart_name_chars_skipped` | — |
| skip local charts | 118 | ported | `helmfile.rs` | `local_path_chart_gets_skip_reason` | — |
| skip chart with unknown repository | 139 | ported | `helmfile.rs` | `chart_with_no_matching_repo_skipped` | — |
| skip chart with special character in the name | 160 | ported | `helmfile.rs` | `chart_with_special_chars_skipped` | — |
| skip chart that does not have specified version | 184 | ported | `helmfile.rs` | `release_without_version_has_invalid_version_skip` | — |
| parses multidoc yaml | 204 | ported | `helmfile.rs` | `parses_multidoc_yaml` | — |
| parses a chart with a go templating | 242 | ported | `helmfile.rs` | `go_template_chart_skipped_real_chart_kept` | — |
| parses a chart with empty strings for template values | 280 | ported | `helmfile.rs` | `template_version_gets_invalid_version_skip` | — |
| parses a chart with an oci repository and non-oci one | 316 | ported | `helmfile.rs` | `oci_backed_repo_uses_docker_datasource` | — |
| allows OCI chart names containing forward slashes | 366 | ported | `helmfile.rs` | `oci_nested_path_chart_uses_docker_datasource` | — |
| parses a chart with an oci repository with --- | 392 | ported | `helmfile.rs` | `oci_repo_with_yaml_document_separator` | — |
| parses and replaces templating strings | 423 | ported | `helmfile.rs` | `go_template_fixture_resolves_fallbacks_and_registry_aliases` | — |
| detects kustomize and respects relative paths | 477 | ported | `helmfile.rs` | `local_chart_marks_need_kustomize_and_keeps_relative_dep` | — |
| makes sure url joiner works correctly | 513 | ported | `helmfile.rs` | `oci_url_with_port_in_chart_ref` | — |
| skips helm-git repos | 539 | ported | `helmfile.rs` | `helm_git_repo_releases_get_unknown_registry` | — |

| parses templates key alongside releases | 576 | pending | — | — | — |
---

