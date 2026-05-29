# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/common.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/go/common › getSourceUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ($datasource, $packageName) => $expected | 5 | ported | `crates/renovate-core/src/datasources/gomod.rs` | `get_source_url_maps_datasource_to_url` | 6 parametrized cases: bitbucket/forgejo/gitea/github/gitlab → URL; git-tags → None |

---
