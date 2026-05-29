# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/host-rules.spec.ts
**Total tests:** 29 | **Ported:** 29 | **Actionable:** 29 | **Status:** ported

### `util/host-rules › add()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if both domainName and hostName | 18 | ported | `util/host_rules.rs` | `add_throws_if_both_domain_name_and_host_name` | — |
| throws if both domainName and baseUrl | 28 | ported | `util/host_rules.rs` | `add_throws_if_both_domain_name_and_base_url` | — |
| throws if both hostName and baseUrl | 38 | ported | `util/host_rules.rs` | `add_throws_if_both_host_name_and_base_url` | — |
| supports baseUrl-only | 48 | ported | `util/host_rules.rs` | `add_supports_base_url_only` | — |
| does not match subpart of hostname | 72 | ported | `util/host_rules.rs` | `add_does_not_match_subpart_of_hostname` | — |
| massages host url | 84 | ported | `util/host_rules.rs` | `add_massages_host_url` | — |

### `util/host-rules › find()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns and returns empty for bad search | 111 | ported | `util/host_rules.rs` | `find_warns_and_returns_empty_for_bad_search` | — |
| needs exact host matches | 115 | ported | `util/host_rules.rs` | `find_needs_exact_host_matches` | — |
| matches on empty rules | 135 | ported | `util/host_rules.rs` | `find_matches_on_empty_rules` | — |
| matches on hostType | 144 | ported | `util/host_rules.rs` | `find_matches_on_host_type` | — |
| matches on domainName | 154 | ported | `util/host_rules.rs` | `find_matches_on_domain_name` | — |
| matches on specific path | 172 | ported | `util/host_rules.rs` | `find_matches_on_specific_path` | — |
| matches for several hostTypes when no hostType rule is configured | 199 | ported | `util/host_rules.rs` | `find_matches_for_several_host_types` | — |
| matches if hostType is configured and host rule is filtered with datasource | 218 | ported | `util/host_rules.rs` | `find_matches_if_host_type_filtered_with_datasource` | — |
| matches on hostName | 237 | ported | `util/host_rules.rs` | `find_matches_on_host_name` | — |
| matches on matchHost with protocol | 247 | ported | `util/host_rules.rs` | `find_matches_on_match_host_with_protocol` | — |
| matches on matchHost without protocol | 262 | ported | `util/host_rules.rs` | `find_matches_on_match_host_without_protocol` | — |
| matches on matchHost with dot prefix | 272 | ported | `util/host_rules.rs` | `find_matches_on_match_host_with_dot_prefix` | — |
| matches on matchHost with port | 282 | ported | `util/host_rules.rs` | `find_matches_on_match_host_with_port` | — |
| matches on hostType and endpoint | 292 | ported | `util/host_rules.rs` | `find_matches_on_host_type_and_endpoint` | — |
| matches on endpoint subresource | 304 | ported | `util/host_rules.rs` | `find_matches_on_endpoint_subresource` | — |
| matches shortest matchHost first | 318 | ported | `util/host_rules.rs` | `find_matches_shortest_match_host_first` | — |
| matches readOnly requests | 334 | ported | `util/host_rules.rs` | `find_matches_read_only_requests` | — |

### `util/host-rules › hosts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns hosts | 355 | ported | `util/host_rules.rs` | `hosts_returns_hosts` | — |

### `util/host-rules › findAll()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns and returns empty for bad search | 393 | ported | `util/host_rules.rs` | `find_all_returns_empty_for_unknown_host_type` | — |
| needs exact host matches | 397 | ported | `util/host_rules.rs` | `find_all_needs_exact_host_matches` | — |

### `util/host-rules › getAll()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns all host rules | 418 | ported | `util/host_rules.rs` | `get_all_returns_all_rules` | — |

### `util/host-rules › hostType()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return hostType | 437 | ported | `util/host_rules.rs` | `host_type_for_url_returns_host_type` | — |
| returns null | 459 | ported | `util/host_rules.rs` | `host_type_for_url_returns_none_for_no_match` | — |

---
