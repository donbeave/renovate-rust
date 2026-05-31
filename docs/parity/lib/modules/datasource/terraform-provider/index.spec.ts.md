# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-provider/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-provider/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `modules/datasource/terraform-provider/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when both default registries return $description | 68 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| processes real data | 87 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns null when a third-party registry returns $description | 136 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| processes real data from third party | 156 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| processes data with alternative backend | 184 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| processes real data from OpenTofu registry docs API | 219 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns an empty release list for OpenTofu registry without versions | 253 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| throws for empty result | 274 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns null for non hashicorp dependency and releases.hashicorp.com registryUrl | 290 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| works for hashicorp dependency and releases.hashicorp.com | 299 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| throws for hashicorp dependency and releases.hashicorp.com 500 | 312 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| rethrows external-host-error for hashicorp dependency and releases.hashicorp.com | 325 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| throws if service discovery error | 338 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| throws if a version is requested which is not available | 352 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| processes real data | 367 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| throws if the retrieval of a single build fails | 447 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| can fetch zip hashes | 487 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| does not hard fail when the ziphashes endpoint is not available | 511 | not-applicable | Mock framework internals — tests terraform-provider via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

---
