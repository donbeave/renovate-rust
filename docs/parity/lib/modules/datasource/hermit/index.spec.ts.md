# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hermit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hermit/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

### `modules/datasource/hermit/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return result from hermit list | 14 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_return_result_from_hermit_list` | versions + channels merged; source_url from Repository |
| should fail on no result found | 79 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_fail_on_no_result_found` | empty index array → None |
| should fail on network error | 106 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_fail_on_network_error` | 404 on index asset → Err |
| should get null result on non github url given | 133 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_get_null_on_non_github_url` | gitlab.com → None |
| should get null result on missing repo or owner | 142 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_get_null_on_missing_repo_or_owner` | github.com/test → None |
| should get null for extra path provided in registry url | 157 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_get_null_for_extra_path` | github.com/test/repo/extra-path → None |
| should get null result on empty registryUrl | 166 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_get_null_on_empty_registry_url` | None registryUrl → None |
| should fail on missing index.json asset | 174 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_fail_on_missing_index_json` | no index.json in assets → None |
| should get null on invalid index.json asset | 195 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_get_null_on_invalid_index_json` | "invalid content" response → None |
| should get null on invalid registry url | 221 | ported | `crates/renovate-core/src/datasources/hermit.rs` | `should_get_null_on_invalid_registry_url` | "invalid url" → None |

---
