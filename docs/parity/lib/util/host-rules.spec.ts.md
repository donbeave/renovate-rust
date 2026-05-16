# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/host-rules.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/host-rules › add()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if both domainName and hostName | 18 | not-applicable | — | — | Renovate's TypeScript process-global host-rules registry is not implemented as a Rust API; Rust currently stores and validates hostRules in config. |
| throws if both domainName and baseUrl | 28 | not-applicable | — | — | Renovate's TypeScript process-global host-rules registry is not implemented as a Rust API; Rust currently stores and validates hostRules in config. |
| throws if both hostName and baseUrl | 38 | not-applicable | — | — | Renovate's TypeScript process-global host-rules registry is not implemented as a Rust API; Rust currently stores and validates hostRules in config. |
| supports baseUrl-only | 48 | not-applicable | — | — | Renovate's TypeScript host-rules matching registry is not implemented as a Rust API. |
| does not match subpart of hostname | 72 | not-applicable | — | — | Renovate's TypeScript host-rules matching registry is not implemented as a Rust API. |
| massages host url | 84 | not-applicable | — | — | Renovate's TypeScript host-rules URL normalization registry is not implemented as a Rust API. |

### `util/host-rules › find()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns and returns empty for bad search | 111 | not-applicable | — | — | Renovate's TypeScript host-rules search helper and logger side effect are not implemented as a Rust API. |
| needs exact host matches | 115 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on empty rules | 135 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on hostType | 144 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on domainName | 154 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on specific path | 172 | not-applicable | — | — | Renovate's TypeScript host-rules path-priority matcher is not implemented as a Rust API. |
| matches for several hostTypes when no hostType rule is configured | 199 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches if hostType is configured and host rule is filtered with datasource | 218 | not-applicable | — | — | Renovate's TypeScript host-rules datasource-specific matcher is not implemented as a Rust API. |
| matches on hostName | 237 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on matchHost with protocol | 247 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on matchHost without protocol | 262 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on matchHost with dot prefix | 272 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on matchHost with port | 282 | not-applicable | — | — | Renovate's TypeScript host-rules search helper is not implemented as a Rust API. |
| matches on hostType and endpoint | 292 | not-applicable | — | — | Renovate's TypeScript host-rules endpoint matcher is not implemented as a Rust API. |
| matches on endpoint subresource | 304 | not-applicable | — | — | Renovate's TypeScript host-rules endpoint matcher is not implemented as a Rust API. |
| matches shortest matchHost first | 318 | not-applicable | — | — | Renovate's TypeScript host-rules precedence algorithm is not implemented as a Rust API. |
| matches readOnly requests | 334 | not-applicable | — | — | Renovate's TypeScript readOnly host-rules matching is not implemented as a Rust API. |

### `util/host-rules › hosts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns hosts | 355 | not-applicable | — | — | Renovate's TypeScript host-rules registry enumeration helper is not implemented as a Rust API. |

### `util/host-rules › findAll()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns and returns empty for bad search | 393 | not-applicable | — | — | Renovate's TypeScript host-rules `findAll()` helper and logger side effect are not implemented as a Rust API. |
| needs exact host matches | 397 | not-applicable | — | — | Renovate's TypeScript host-rules `findAll()` helper is not implemented as a Rust API. |

### `util/host-rules › getAll()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns all host rules | 418 | not-applicable | — | — | Renovate's TypeScript host-rules registry enumeration helper is not implemented as a Rust API. |

### `util/host-rules › hostType()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return hostType | 437 | not-applicable | — | — | Renovate's TypeScript host-rules host-type inference helper is not implemented as a Rust API. |
| returns null | 459 | not-applicable | — | — | Renovate's TypeScript host-rules host-type inference helper is not implemented as a Rust API. |

---

