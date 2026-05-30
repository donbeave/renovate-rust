# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/url.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/url.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 0 | **Status:** done

### `util/url`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $baseUrl + $x => $result | 18 | ported | `util.rs` | `test_resolve_base_url` | — |
| replaceUrlPath("$baseUrl", "$x") => $result | 57 | ported | `util.rs` | `test_replace_url_path` | — |
| getQueryString | 97 | ported | `util.rs` | `test_get_query_string` | — |
| validates http-based URLs | 101 | ported | `util.rs` | `test_is_http_url` | — |
| parses URL | 112 | ported | `util.rs` | `test_parse_url` | — |
| trimTrailingSlash | 123 | ported | `util.rs` | `test_trim_trailing_slash` | — |
| trimSlashes | 130 | ported | `util.rs` | `test_trim_slashes` | — |
| ensureTrailingSlash | 141 | ported | `util.rs` | `test_ensure_trailing_slash` | — |
| ensures path prefix | 146 | ported | `util.rs` | `test_ensure_path_prefix` | — |
| joinUrlParts | 164 | ported | `util.rs` | `test_join_url_parts` | — |
| createURLFromHostOrURL | 180 | ported | `util.rs` | `test_create_url_from_host_or_url` | — |
| parseLinkHeader | 189 | ported | `util.rs` | `test_parse_link_header` | — |
| massageHostUrl | 221 | ported | `util.rs` | `test_massage_host_url` | — |

---

