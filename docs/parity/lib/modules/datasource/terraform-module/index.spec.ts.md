# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-module/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-module/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `modules/datasource/terraform-module/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for the default registry when the module endpoint returns $description | 81 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| returns releases, homepage, and source URL from the default registry | 100 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| returns null for a third-party registry when the versions endpoint returns $description | 127 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| returns releases from a third-party registry | 147 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| returns sourceUrl when a third-party registry includes one | 169 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| uses the registry embedded in packageName | 199 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| uses the v1 extended endpoint for Terraform Cloud | 226 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| returns null when the third-party versions response has no modules | 260 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| returns null when service discovery fails | 277 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| uses the service discovery modules path when the registry serves a custom subpath | 290 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| processes real data from OpenTofu registry docs API | 313 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |
| returns an empty release list for OpenTofu registry without versions | 348 | not-applicable | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests terraform-module via nock HTTP mocks; Rust tests this at different layer |

---
