# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/host-rules.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** pending

### `util/host-rules › add()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if both domainName and hostName | 18 | pending | — | — | — |
| throws if both domainName and baseUrl | 28 | pending | — | — | — |
| throws if both hostName and baseUrl | 38 | pending | — | — | — |
| supports baseUrl-only | 48 | pending | — | — | — |
| does not match subpart of hostname | 72 | pending | — | — | — |
| massages host url | 84 | pending | — | — | — |

### `util/host-rules › find()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns and returns empty for bad search | 111 | pending | — | — | — |
| needs exact host matches | 115 | pending | — | — | — |
| matches on empty rules | 135 | pending | — | — | — |
| matches on hostType | 144 | pending | — | — | — |
| matches on domainName | 154 | pending | — | — | — |
| matches on specific path | 172 | pending | — | — | — |
| matches for several hostTypes when no hostType rule is configured | 199 | pending | — | — | — |
| matches if hostType is configured and host rule is filtered with datasource | 218 | pending | — | — | — |
| matches on hostName | 237 | pending | — | — | — |
| matches on matchHost with protocol | 247 | pending | — | — | — |
| matches on matchHost without protocol | 262 | pending | — | — | — |
| matches on matchHost with dot prefix | 272 | pending | — | — | — |
| matches on matchHost with port | 282 | pending | — | — | — |
| matches on hostType and endpoint | 292 | pending | — | — | — |
| matches on endpoint subresource | 304 | pending | — | — | — |
| matches shortest matchHost first | 318 | pending | — | — | — |
| matches readOnly requests | 334 | pending | — | — | — |

### `util/host-rules › hosts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns hosts | 355 | pending | — | — | — |

### `util/host-rules › findAll()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns and returns empty for bad search | 393 | pending | — | — | — |
| needs exact host matches | 397 | pending | — | — | — |

### `util/host-rules › getAll()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns all host rules | 418 | pending | — | — | — |

### `util/host-rules › hostType()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return hostType | 437 | pending | — | — | — |
| returns null | 459 | pending | — | — | — |

---

