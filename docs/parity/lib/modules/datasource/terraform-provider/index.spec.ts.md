# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-provider/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-provider/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `modules/datasource/terraform-provider/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when both default registries return $description | 68 | pending | — | — | —|
| processes real data | 87 | pending | — | — | —|
| returns null when a third-party registry returns $description | 136 | pending | — | — | —|
| processes real data from third party | 156 | pending | — | — | —|
| processes data with alternative backend | 184 | pending | — | — | —|
| processes real data from OpenTofu registry docs API | 219 | pending | — | — | —|
| returns an empty release list for OpenTofu registry without versions | 253 | pending | — | — | —|
| throws for empty result | 274 | pending | — | — | —|
| returns null for non hashicorp dependency and releases.hashicorp.com registryUrl | 290 | pending | — | — | —|
| works for hashicorp dependency and releases.hashicorp.com | 299 | pending | — | — | —|
| throws for hashicorp dependency and releases.hashicorp.com 500 | 312 | pending | — | — | —|
| rethrows external-host-error for hashicorp dependency and releases.hashicorp.com | 325 | pending | — | — | —|
| throws if service discovery error | 338 | pending | — | — | —|
| throws if a version is requested which is not available | 352 | pending | — | — | —|
| processes real data | 367 | pending | — | — | —|
| throws if the retrieval of a single build fails | 447 | pending | — | — | —|
| can fetch zip hashes | 487 | pending | — | — | —|
| does not hard fail when the ziphashes endpoint is not available | 511 | pending | — | — | —|

---
