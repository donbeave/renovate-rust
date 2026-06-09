# `lib/util/host-rules.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**27/29 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | throws if both domainname and hostname | ported | [`crates/renovate-core/src/util/host_rules.rs:427`](../../../../../crates/renovate-core/src/util/host_rules.rs#L427) |
| 28 | throws if both domainname and baseurl | ported | [`crates/renovate-core/src/util/host_rules.rs:445`](../../../../../crates/renovate-core/src/util/host_rules.rs#L445) |
| 38 | throws if both hostname and baseurl | ported | [`crates/renovate-core/src/util/host_rules.rs:463`](../../../../../crates/renovate-core/src/util/host_rules.rs#L463) |
| 48 | supports baseurl-only | ported | [`crates/renovate-core/src/util/host_rules.rs:481`](../../../../../crates/renovate-core/src/util/host_rules.rs#L481) |
| 72 | does not match subpart of hostname | ported | [`crates/renovate-core/src/util/host_rules.rs:521`](../../../../../crates/renovate-core/src/util/host_rules.rs#L521) |
| 84 | massages host url | ported | [`crates/renovate-core/src/util/host_rules.rs:549`](../../../../../crates/renovate-core/src/util/host_rules.rs#L549) |
| 111 | warns and returns empty for bad search | ported | [`crates/renovate-core/src/util/host_rules.rs:583`](../../../../../crates/renovate-core/src/util/host_rules.rs#L583) |
| 115 | needs exact host matches | ported | [`crates/renovate-core/src/util/host_rules.rs:591`](../../../../../crates/renovate-core/src/util/host_rules.rs#L591) |
| 135 | matches on empty rules | ported | [`crates/renovate-core/src/util/host_rules.rs:641`](../../../../../crates/renovate-core/src/util/host_rules.rs#L641) |
| 144 | matches on hosttype | ported | [`crates/renovate-core/src/util/host_rules.rs:658`](../../../../../crates/renovate-core/src/util/host_rules.rs#L658) |
| 154 | matches on domainname | ported | [`crates/renovate-core/src/util/host_rules.rs:676`](../../../../../crates/renovate-core/src/util/host_rules.rs#L676) |
| 172 | matches on specific path | ported | [`crates/renovate-core/src/util/host_rules.rs:723`](../../../../../crates/renovate-core/src/util/host_rules.rs#L723) |
| 199 | matches for several hosttypes when no hosttype rule is configured | ported | [`crates/renovate-core/src/util/host_rules.rs:757`](../../../../../crates/renovate-core/src/util/host_rules.rs#L757) |
| 218 | matches if hosttype is configured and host rule is filtered with datasource | ported | [`crates/renovate-core/src/util/host_rules.rs:776`](../../../../../crates/renovate-core/src/util/host_rules.rs#L776) |
| 237 | matches on hostname | ported | [`crates/renovate-core/src/util/host_rules.rs:803`](../../../../../crates/renovate-core/src/util/host_rules.rs#L803) |
| 247 | matches on matchhost with protocol | ported | [`crates/renovate-core/src/util/host_rules.rs:830`](../../../../../crates/renovate-core/src/util/host_rules.rs#L830) |
| 262 | matches on matchhost without protocol | ported | [`crates/renovate-core/src/util/host_rules.rs:879`](../../../../../crates/renovate-core/src/util/host_rules.rs#L879) |
| 272 | matches on matchhost with dot prefix | ported | [`crates/renovate-core/src/util/host_rules.rs:920`](../../../../../crates/renovate-core/src/util/host_rules.rs#L920) |
| 282 | matches on matchhost with port | ported | [`crates/renovate-core/src/util/host_rules.rs:960`](../../../../../crates/renovate-core/src/util/host_rules.rs#L960) |
| 292 | matches on hosttype and endpoint | ported | [`crates/renovate-core/src/util/host_rules.rs:990`](../../../../../crates/renovate-core/src/util/host_rules.rs#L990) |
| 304 | matches on endpoint subresource | ported | [`crates/renovate-core/src/util/host_rules.rs:1009`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1009) |
| 318 | matches shortest matchhost first | ported | [`crates/renovate-core/src/util/host_rules.rs:1028`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1028) |
| 334 | matches readonly requests | ported | [`crates/renovate-core/src/util/host_rules.rs:1052`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1052) |
| 355 | returns hosts | ported | [`crates/renovate-core/src/util/host_rules.rs:1080`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1080) |
| 393 | warns and returns empty for bad search | ported | [`crates/renovate-core/src/util/host_rules.rs:583`](../../../../../crates/renovate-core/src/util/host_rules.rs#L583) |
| 397 | needs exact host matches | ported | [`crates/renovate-core/src/util/host_rules.rs:591`](../../../../../crates/renovate-core/src/util/host_rules.rs#L591) |
| 418 | returns all host rules | ported | [`crates/renovate-core/src/util/host_rules.rs:1175`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1175) |
| 437 | return hosttype | ported | [`crates/renovate-core/src/util/host_rules.rs:1203`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1203) |
| 459 | returns null | ported | [`crates/renovate-core/src/util/host_rules.rs:1234`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1234) |

