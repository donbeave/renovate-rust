# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/goproxy-parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/goproxy-parser.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/go/goproxy-parser › parseGoproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses single url | 10 | not-applicable | — | — | Renovate's GOPROXY list parser and fallback separator model are not implemented in Rust; Rust Go module datasource accepts a single explicit proxy base URL. |
| parses multiple urls | 15 | not-applicable | — | — | Renovate's GOPROXY list parser and fallback separator model are not implemented in Rust; Rust Go module datasource accepts a single explicit proxy base URL. |
| ignores everything starting from "direct" and "off" keywords | 25 | not-applicable | — | — | Renovate's GOPROXY direct/off handling is not implemented in Rust; Rust Go module datasource accepts a single explicit proxy base URL. |
| caches results | 43 | not-applicable | — | — | Renovate's GOPROXY parser memory cache is not implemented in Rust. |

### `modules/datasource/go/goproxy-parser › parseNoproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| produces regex | 49 | not-applicable | — | — | Renovate's GONOPROXY glob-to-regex parser is not implemented in Rust; Rust Go module datasource does not switch between proxy/direct sources. |
| matches on real package prefixes | 68 | not-applicable | — | — | Renovate's GONOPROXY glob matcher is not implemented in Rust; Rust Go module datasource does not switch between proxy/direct sources. |
| matches on wildcards | 100 | not-applicable | — | — | Renovate's GONOPROXY wildcard matcher is not implemented in Rust; Rust Go module datasource does not switch between proxy/direct sources. |
| matches on character ranges | 126 | not-applicable | — | — | Renovate's GONOPROXY character-range matcher is not implemented in Rust; Rust Go module datasource does not switch between proxy/direct sources. |
| caches results | 131 | not-applicable | — | — | Renovate's GONOPROXY parser memory cache is not implemented in Rust. |

---

