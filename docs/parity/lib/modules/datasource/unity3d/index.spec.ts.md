# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/unity3d/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/unity3d/index.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `modules/datasource/unity3d/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns lts if requested %s | 55 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `returns_lts_if_requested` | lts stream + legacy stream → LTS releases, is_stable=true |
| returns tech if requested | 86 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `returns_tech_if_requested` | TECH stream → is_stable=false |
| returns alpha if requested | 107 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `returns_alpha_if_requested` | ALPHA stream → is_stable=false |
| returns beta if requested %s | 127 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `returns_beta_if_requested` | BETA stream + legacy → is_stable=false |
| returns lts releases by default | 147 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `returns_lts_releases_by_default` | default → LTS only |
| returns hash if requested | 184 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `returns_hash_if_requested` | with_hash=true → version includes shortRevision |
| returns no hash if not requested | 203 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `returns_no_hash_if_not_requested` | with_hash=false → no revision in version |
| returns only lts by default | 222 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `returns_only_lts_by_default` | default → no beta/alpha versions |
| uses pagination | 241 | ported | `crates/renovate-core/src/datasources/unity3d.rs` | `uses_pagination` | 30 total across 2 pages |

---
