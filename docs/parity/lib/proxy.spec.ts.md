# Renovate Test Detail

[Back to test map](../renovate-test-map.md)

## `lib/proxy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/proxy.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `proxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| respects HTTP_PROXY | 15 | not-applicable | — | — | Renovate's TypeScript proxy bootstrap that mirrors environment variables for global HTTP clients is not implemented as a Rust API. |
| copies upper case HTTP_PROXY to http_proxy | 21 | not-applicable | — | — | Renovate's TypeScript proxy environment variable mirroring is not implemented as a Rust API. |
| respects HTTPS_PROXY | 33 | not-applicable | — | — | Renovate's TypeScript proxy bootstrap that mirrors environment variables for global HTTP clients is not implemented as a Rust API. |
| copies upper case HTTPS_PROXY to https_proxy | 39 | not-applicable | — | — | Renovate's TypeScript proxy environment variable mirroring is not implemented as a Rust API. |
| does nothing | 51 | not-applicable | — | — | Renovate's TypeScript proxy bootstrap ignores NO_PROXY-only configuration; no equivalent Rust proxy bootstrap exists. |

---

