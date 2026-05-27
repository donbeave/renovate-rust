# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/dockerfile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/dockerfile/extract.spec.ts
**Total tests:** 76 | **Ported:** 76 | **Actionable:** 76 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles no FROM | 14 | ported | `dockerfile.rs` | `no_from_returns_empty` | — |
| handles naked dep | 19 | ported | `dockerfile.rs` | `extracts_image_without_tag` | — |
| handles run --mount=from | 36 | ported | `dockerfile.rs` | `run_mount_from_extracts_external_images` | — |
| is case insensitive | 72 | ported | `dockerfile.rs` | `from_is_case_insensitive` | — |
| handles tag | 89 | ported | `dockerfile.rs` | `extracts_image_and_tag` | — |
| handles digest | 106 | ported | `dockerfile.rs` | `extracts_image_with_digest_only` | — |
| handles tag and digest | 129 | ported | `dockerfile.rs` | `extracts_image_with_digest` | — |
| handles from as | 152 | ported | `dockerfile.rs` | `as_alias_does_not_become_dep` | — |
| handles comments | 173 | ported | `dockerfile.rs` | `commented_from_ignored` | — |
| handles custom hosts | 194 | ported | `dockerfile.rs` | `extracts_image_with_custom_host` | — |
| handles custom hosts and suffix | 215 | ported | `dockerfile.rs` | `custom_host_with_suffix_in_tag` | — |
| handles custom hosts with port | 236 | ported | `dockerfile.rs` | `registry_port_not_confused_with_tag` | — |
| handles custom hosts with port without tag | 257 | ported | `dockerfile.rs` | `custom_host_with_port_no_tag` | — |
| handles quay hosts with port | 278 | ported | `dockerfile.rs` | `quay_host_with_port_no_tag` | — |
| handles namespaced images | 295 | ported | `dockerfile.rs` | `extracts_namespaced_image` | — |
| handles custom hosts with namespace | 312 | ported | `dockerfile.rs` | `extracts_scoped_image` | — |
| handles abnormal spacing | 333 | ported | `dockerfile.rs` | `abnormal_spacing_after_from` | — |
| extracts multiple FROM tags | 354 | ported | `dockerfile.rs` | `only_from_instructions_extracted` | — |
| extracts tags from Dockerfile which begins with a BOM marker | 386 | ported | `dockerfile.rs` | `bom_marker_stripped` | — |
| skips scratches | 407 | ported | `dockerfile.rs` | `scratch_is_skipped` | — |
| skips named multistage FROM tags | 412 | ported | `dockerfile.rs` | `stage_reference_is_skipped` | — |
| handles COPY --from | 433 | ported | `dockerfile.rs` | `copy_from_extracts_external_image` | — |
| handles COPY --from with digest | 454 | ported | `dockerfile.rs` | `copy_from_with_digest` | — |
| handles COPY --link --from | 481 | ported | `dockerfile.rs` | `copy_link_from_extracts_image` | — |
| skips named multistage COPY --from tags | 507 | ported | `dockerfile.rs` | `copy_from_stage_name_is_skipped` | — |
| skips index reference COPY --from tags | 528 | ported | `dockerfile.rs` | `copy_from_index_is_skipped` | — |
| detects ["stage"] and ["final"] deps of docker multi-stage build. | 549 | ported | `dockerfile.rs` | `multistage_build_with_copy_from_stage` | — |
| extracts images on adjacent lines | 598 | ported | `dockerfile.rs` | `renovate_fixture_1` | — |
| extracts images from all sorts of (maybe multiline) FROM and COPY --from statements | 628 | ported | `dockerfile.rs` | `renovate_fixture_2_multiline` | — |
| handles calico/node | 733 | ported | `dockerfile.rs` | `namespaced_image_without_tag` | — |
| handles ubuntu | 750 | ported | `dockerfile.rs` | `ubuntu_with_version_tag` | — |
| handles debian with codename | 768 | ported | `dockerfile.rs` | `debian_with_codename_tag` | — |
| handles debian with regular tag | 786 | ported | `dockerfile.rs` | `debian_with_version_tag` | — |
| handles debian with prefixes | 803 | ported | `dockerfile.rs` | `debian_with_platform_prefix` | — |
| handles debian with prefixes and registries | 821 | ported | `dockerfile.rs` | `debian_with_registry_prefix` | — |
| handles prefixes | 843 | ported | `dockerfile.rs` | `ubuntu_with_platform_prefix` | — |
| handles prefixes with registries | 861 | ported | `dockerfile.rs` | `registry_with_namespace_prefix` | — |
| handles implausible line continuation | 883 | ported | `dockerfile.rs` | `implausible_continuation_does_not_affect_from` | — |
| handles multi-line FROM with space after escape character | 904 | ported | `dockerfile.rs` | `multiline_from_with_space_after_escape` | — |
| handles FROM without ARG default value | 921 | ported | `dockerfile.rs` | `from_with_arg_variable_is_skipped` | — |
| handles FROM with empty ARG default value | 939 | ported | `dockerfile.rs` | `from_with_empty_arg_defaults_extracts_literal_image` | — |
| handles FROM with version in ARG value | 960 | ported | `dockerfile.rs` | `from_with_version_in_arg_value` | — |
| handles FROM with version in ARG default value | 981 | ported | `dockerfile.rs` | `from_with_version_in_arg_default_value` | — |
| handles FROM with digest in ARG default value | 1002 | ported | `dockerfile.rs` | `from_with_digest_in_arg_value` | — |
| handles FROM with overwritten ARG value | 1026 | ported | `dockerfile.rs` | `from_with_overwritten_arg_value` | — |
| handles FROM with multiple ARG values | 1058 | ported | `dockerfile.rs` | `from_with_multiple_arg_values` | — |
| skips scratch if provided in ARG value | 1079 | ported | `dockerfile.rs` | `scratch_from_arg_value_is_skipped` | — |
| extracts images from multi-line ARG statements | 1088 | ported | `dockerfile.rs` | `extracts_images_from_multiline_arg_statements` | — |
| ignores parser directives in wrong order | 1131 | ported | `dockerfile.rs` | `parser_directives_in_wrong_order_ignored` | — |
| handles an alternative escape character | 1152 | ported | `dockerfile.rs` | `alternative_escape_character` | — |
| handles FROM with version in ARG default value and quotes | 1227 | ported | `dockerfile.rs` | `from_with_quoted_arg_default_value` | — |
| handles version in ARG and digest in FROM with CRLF linefeed | 1249 | ported | `dockerfile.rs` | `from_with_arg_tag_and_digest_with_crlf` | — |
| handles updates of multiple ARG values | 1272 | ported | `dockerfile.rs` | `from_with_multiple_arg_components` | — |
| handles same argument multiple times | 1308 | ported | `dockerfile.rs` | `same_arg_used_multiple_times` | — |
| handles empty optional parameters | 1329 | ported | `dockerfile.rs` | `handles_empty_optional_parameters` | — |
| handles registry alias | 1352 | ported | `dockerfile.rs` | `handles_registry_alias` | — |
| replaces registry alias from start only | 1380 | ported | `dockerfile.rs` | `registry_alias_matches_start_only` | — |
| handles empty registry | 1407 | ported | `dockerfile.rs` | `namespaced_image_without_registry_extracted_normally` | — |
| handles # syntax statements | 1435 | ported | `dockerfile.rs` | `syntax_directive_extracted` | — |
| ignores # syntax statements after first line | 1469 | ported | `dockerfile.rs` | `syntax_directive_after_from_ignored` | — |

### `getDep()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects null | 1493 | ported | `dockerfile.rs` | `get_dep_rejects_null` | — |
| rejects empty or whitespace | 1497 | ported | `dockerfile.rs` | `get_dep_rejects_empty_or_whitespace` | — |
| handles default environment variable values | 1501 | ported | `dockerfile.rs` | `default_variable_value_extracted` | — |
| skips tag containing a variable | 1563 | ported | `dockerfile.rs` | `tag_with_variable_is_skipped` | — |
| skips depName containing a non default variable at start | 1574 | ported | `dockerfile.rs` | `arg_variable_is_skipped` | — |
| skips depName containing a non default variable with brackets at start | 1585 | ported | `dockerfile.rs` | `arg_braces_variable_is_skipped` | — |
| skips depName containing a non default variable | 1596 | ported | `dockerfile.rs` | `variable_in_image_path_is_skipped` | — |
| skips depName containing a non default variable with brackets | 1607 | ported | `dockerfile.rs` | `braced_variable_in_image_path_is_skipped` | — |
| supports registry aliases - $name | 1623 | ported | `dockerfile.rs` | `supports_get_dep_registry_alias_table` | — |

### `extractVariables()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles no variable | 1651 | ported | `dockerfile.rs` | `extract_variables_handles_no_variable` | — |
| handles simple variable | 1655 | ported | `dockerfile.rs` | `extract_variables_handles_simple_variable` | — |
| handles escaped variable | 1661 | ported | `dockerfile.rs` | `extract_variables_handles_escaped_variable` | — |
| handles complex variable | 1667 | ported | `dockerfile.rs` | `extract_variables_handles_complex_variable` | — |
| handles complex variable with static default value | 1673 | ported | `dockerfile.rs` | `extract_variables_handles_complex_variable_with_static_default` | — |
| handles complex variable with other variable as default value | 1679 | ported | `dockerfile.rs` | `extract_variables_handles_complex_variable_with_variable_default` | — |
| handles multiple variables | 1685 | ported | `dockerfile.rs` | `extract_variables_handles_multiple_variables` | — |

---

