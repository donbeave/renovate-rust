# Renovate Test Detail

[Back to test map](../renovate-test-map.md)

## `lib/proxy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/proxy.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `proxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| respects HTTP_PROXY | 15 | ported | `proxy.rs` | `respects_http_proxy` | — |
| copies upper case HTTP_PROXY to http_proxy | 21 | ported | `proxy.rs` | `copies_upper_http_proxy_to_lower` | — |
| respects HTTPS_PROXY | 33 | ported | `proxy.rs` | `respects_https_proxy` | — |
| copies upper case HTTPS_PROXY to https_proxy | 39 | ported | `proxy.rs` | `copies_upper_https_proxy_to_lower` | — |
| does nothing | 51 | ported | `proxy.rs` | `does_nothing_with_only_no_proxy` | — |

---

