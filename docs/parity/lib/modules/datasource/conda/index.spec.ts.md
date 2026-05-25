# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/conda/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/conda/index.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** done

### `modules/datasource/conda/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for error | 14 | ported | `crates/renovate-core/src/datasources/conda.rs` | `throws_for_network_error` | network error → Err |
| returns null for 404 | 24 | ported | `crates/renovate-core/src/datasources/conda.rs` | `returns_null_for_404` | 404 → None |
| returns null for empty result | 34 | ported | `crates/renovate-core/src/datasources/conda.rs` | `returns_null_for_empty_versions` | empty versions → None |
| throws for 5xx | 47 | ported | `crates/renovate-core/src/datasources/conda.rs` | `throws_for_5xx` | 502 → Err |
| processes real data | 57 | ported | `crates/renovate-core/src/datasources/conda.rs` | `processes_real_data` | pytest fixture → 94 releases |
| returns null without registryUrl | 70 | ported | `crates/renovate-core/src/datasources/conda.rs` | `returns_null_without_registry_url` | empty registryUrl → None |
| supports multiple custom datasource urls | 79 | ported | `crates/renovate-core/src/datasources/conda.rs` | `supports_multiple_custom_datasource_urls` | hunt strategy; rapids 404 → conda-forge 200 |
| supports channel from prefix.dev with null response | 118 | ported | `crates/renovate-core/src/datasources/conda.rs` | `prefix_dev_null_response` | variants: null → empty → None |
| supports channel from prefix.dev with multiple page responses | 135 | ported | `crates/renovate-core/src/datasources/conda.rs` | `prefix_dev_multiple_pages` | de-duplication; url extraction; timestamp priority |

---

