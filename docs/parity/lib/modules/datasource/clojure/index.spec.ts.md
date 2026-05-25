# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/clojure/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/clojure/index.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** done

### `modules/datasource/clojure/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases from custom repository | 93 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `returns_releases_from_custom_repository` | custom registry → is_private=true, homepage from POM |
| collects releases from all registry urls | 101 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `collects_releases_from_all_registry_urls` | merge 8+1=9 versions; dedup |
| falls back to next registry url | 129 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `falls_back_to_next_registry_url` | 404 registry skipped; second succeeds |
| ignores unsupported protocols | 160 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `ignores_unsupported_protocols` | ftp:// skipped; http:// succeeds |
| skips registry with invalid metadata structure | 173 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `skips_registry_with_invalid_metadata_structure` | no <versions> element → skip |
| skips registry with invalid XML | 192 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `skips_registry_with_invalid_xml` | "###" → skip |
| handles optional slash at the end of registry url | 208 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `handles_optional_slash_at_end_of_registry_url` | with and without trailing slash → same releases |
| returns null for invalid registryUrls | 218 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `returns_null_for_invalid_registry_urls` | ${project.baseUri} → None |
| supports scm.url values prefixed with "scm:" | 227 | ported | `crates/renovate-core/src/datasources/clojure.rs` | `supports_scm_url_values_prefixed_with_scm` | scm: stripped + /tree/${…} removed → sourceUrl |

---
