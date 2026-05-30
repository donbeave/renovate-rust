# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip_requirements/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip_requirements/extract.spec.ts
**Total tests:** 22 | **Ported:** 22 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 39 | ported | `pip.rs` | `invalid_line_returns_empty` | — |
| extracts dependencies | 43 | ported | `pip.rs` | `extracts_exact_pin` (+ extracts_unconstrained_package, requirements1_fixture, blank_lines_ignored) | — |
| extracts dependencies with --index-url short code | 50 | ported | `pip.rs` | `index_url_short_code_skipped_package_extracted` | — |
| extracts --requirement short code option | 68 | ported | `pip.rs` | `sub_requirement_is_skipped` | — |
| extracts --constraints short code option | 79 | ported | `pip.rs` | `constraints_file_is_skipped` | — |
| extracts multiple dependencies | 90 | ported | `pip.rs` | `handles_multiple_packages` (+ requirements2_fixture) | — |
| handles comments and commands | 96 | ported | `pip.rs` | `comment_only_lines_ignored` (+ blank_lines_ignored, index_url_directive_ignored) | — |
| handles extras and complex index url | 102 | ported | `pip.rs` | `extracts_range_constraint` (+ handles_extras_and_complex_index_url_registry) | — |
| handles extra index url | 111 | ported | `pip.rs` | `handles_extra_index_url` | — |
| handles extra index url and defaults without index to config | 123 | ported | `pip.rs` | `handles_extra_index_url_without_index_for_config_default` | — |
| handles extra index url and defaults without index to pypi | 132 | ported | `pip.rs` | `handles_extra_index_url_without_index_for_pypi_default` | — |
| handles extra spaces around pinned dependency equal signs | 141 | ported | `pip.rs` | `extra_spaces_around_equal_signs` | — |
| should not replace env vars in low trust mode | 155 | ported | `pip.rs` | `does_not_replace_env_vars_in_low_trust_mode` | — |
| should replace env vars in high trust mode | 166 | ported | `pip.rs` | `replaces_env_vars_in_high_trust_mode` | — |
| should handle hashes | 178 | ported | `pip.rs` | `hash_continuation_lines_handled` | — |
| should handle package with extras and no version specifiers | 184 | ported | `pip.rs` | `extracts_unconstrained_package` | — |
| should handle dependency and ignore env markers | 198 | ported | `pip.rs` | `extracts_range_constraint` | — |
| should handle git packages | 213 | ported | `pip.rs` | `git_source_is_skipped` | — |
| extracts a file with only --index-url flags | 258 | ported | `pip.rs` | `url_install_is_skipped` | — |
| extracts a file with only --extra-index-url flags | 266 | ported | `pip.rs` | `extra_index_url_only_file_returns_no_deps` | — |
| extracts a file with only -r flags | 276 | ported | `pip.rs` | `r_flag_only_file_has_no_actionable_deps` | — |
| extracts a file with only -c flags | 286 | ported | `pip.rs` | `c_flag_only_file_has_no_actionable_deps` | — |

---

