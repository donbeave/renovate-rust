# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/url.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/url.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/url`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $baseUrl + $x => $result | 18 | not-applicable | — | — | Renovate's TypeScript shared URL helper module is not implemented as a Rust API; Rust uses local `url` crate parsing and module-specific URL handling. |
| replaceUrlPath("$baseUrl", "$x") => $result | 57 | not-applicable | — | — | Renovate's TypeScript shared URL path replacement helper is not implemented as a Rust API. |
| getQueryString | 97 | not-applicable | — | — | Renovate's TypeScript shared query-string serializer helper is not implemented as a Rust API. |
| validates http-based URLs | 101 | not-applicable | — | — | Renovate's TypeScript shared HTTP URL predicate is not implemented as a Rust API. |
| parses URL | 112 | not-applicable | — | — | Renovate's TypeScript shared nullable URL parser wrapper is not implemented as a Rust API. |
| trimTrailingSlash | 123 | not-applicable | — | — | Renovate's TypeScript shared URL/string slash helper is not implemented as a Rust API. |
| trimSlashes | 130 | not-applicable | — | — | Renovate's TypeScript shared URL/string slash helper is not implemented as a Rust API. |
| ensureTrailingSlash | 141 | not-applicable | — | — | Renovate's TypeScript shared URL/string slash helper is not implemented as a Rust API. |
| ensures path prefix | 146 | not-applicable | — | — | Renovate's TypeScript shared URL path-prefix helper is not implemented as a Rust API. |
| joinUrlParts | 164 | not-applicable | — | — | Renovate's TypeScript shared URL join helper is not implemented as a Rust API. |
| createURLFromHostOrURL | 180 | not-applicable | — | — | Renovate's TypeScript shared host-or-URL constructor helper is not implemented as a Rust API. |
| parseLinkHeader | 189 | not-applicable | — | — | Renovate's TypeScript shared HTTP Link header parser is not implemented as a Rust API. |
| massageHostUrl | 221 | not-applicable | — | — | Renovate's TypeScript shared host URL normalization helper is not implemented as a Rust API. |

---

