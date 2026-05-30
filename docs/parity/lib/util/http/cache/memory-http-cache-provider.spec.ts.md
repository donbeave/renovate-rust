# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/http/cache/memory-http-cache-provider.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/cache/memory-http-cache-provider.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `util/http/cache/memory-http-cache-provider`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reuses data with etag | 17 | not-applicable | — | — | Tests in-memory HTTP cache provider with httpMock (nock) for real HTTP round-trip caching; Rust HTTP client uses different caching architecture |
| does not allow cached responses to be mutated | 40 | not-applicable | — | — | Tests cache immutability via HTTP mock round-trip; Rust HTTP client uses different caching architecture |

---
