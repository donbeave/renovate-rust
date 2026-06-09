# `lib/util/host-rules.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**27/29 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | throws if both domainname and hostname | ported | [`crates/renovate-core/src/util/host_rules.rs:426`](../../../../../crates/renovate-core/src/util/host_rules.rs#L426) |
| 28 | throws if both domainname and baseurl | ported | [`crates/renovate-core/src/util/host_rules.rs:444`](../../../../../crates/renovate-core/src/util/host_rules.rs#L444) |
| 38 | throws if both hostname and baseurl | ported | [`crates/renovate-core/src/util/host_rules.rs:462`](../../../../../crates/renovate-core/src/util/host_rules.rs#L462) |
| 48 | supports baseurl-only | ported | [`crates/renovate-core/src/util/host_rules.rs:480`](../../../../../crates/renovate-core/src/util/host_rules.rs#L480) |
| 72 | does not match subpart of hostname | ported | [`crates/renovate-core/src/util/host_rules.rs:520`](../../../../../crates/renovate-core/src/util/host_rules.rs#L520) |
| 84 | massages host url | ported | [`crates/renovate-core/src/util/host_rules.rs:548`](../../../../../crates/renovate-core/src/util/host_rules.rs#L548) |
| 111 | warns and returns empty for bad search | ported | [`crates/renovate-core/src/util/host_rules.rs:582`](../../../../../crates/renovate-core/src/util/host_rules.rs#L582) |
| 115 | needs exact host matches | ported | [`crates/renovate-core/src/util/host_rules.rs:590`](../../../../../crates/renovate-core/src/util/host_rules.rs#L590) |
| 135 | matches on empty rules | ported | [`crates/renovate-core/src/util/host_rules.rs:640`](../../../../../crates/renovate-core/src/util/host_rules.rs#L640) |
| 144 | matches on hosttype | ported | [`crates/renovate-core/src/util/host_rules.rs:657`](../../../../../crates/renovate-core/src/util/host_rules.rs#L657) |
| 154 | matches on domainname | ported | [`crates/renovate-core/src/util/host_rules.rs:675`](../../../../../crates/renovate-core/src/util/host_rules.rs#L675) |
| 172 | matches on specific path | ported | [`crates/renovate-core/src/util/host_rules.rs:722`](../../../../../crates/renovate-core/src/util/host_rules.rs#L722) |
| 199 | matches for several hosttypes when no hosttype rule is configured | ported | [`crates/renovate-core/src/util/host_rules.rs:756`](../../../../../crates/renovate-core/src/util/host_rules.rs#L756) |
| 218 | matches if hosttype is configured and host rule is filtered with datasource | ported | [`crates/renovate-core/src/util/host_rules.rs:775`](../../../../../crates/renovate-core/src/util/host_rules.rs#L775) |
| 237 | matches on hostname | ported | [`crates/renovate-core/src/util/host_rules.rs:802`](../../../../../crates/renovate-core/src/util/host_rules.rs#L802) |
| 247 | matches on matchhost with protocol | ported | [`crates/renovate-core/src/util/host_rules.rs:829`](../../../../../crates/renovate-core/src/util/host_rules.rs#L829) |
| 262 | matches on matchhost without protocol | ported | [`crates/renovate-core/src/util/host_rules.rs:878`](../../../../../crates/renovate-core/src/util/host_rules.rs#L878) |
| 272 | matches on matchhost with dot prefix | ported | [`crates/renovate-core/src/util/host_rules.rs:919`](../../../../../crates/renovate-core/src/util/host_rules.rs#L919) |
| 282 | matches on matchhost with port | ported | [`crates/renovate-core/src/util/host_rules.rs:959`](../../../../../crates/renovate-core/src/util/host_rules.rs#L959) |
| 292 | matches on hosttype and endpoint | ported | [`crates/renovate-core/src/util/host_rules.rs:989`](../../../../../crates/renovate-core/src/util/host_rules.rs#L989) |
| 304 | matches on endpoint subresource | ported | [`crates/renovate-core/src/util/host_rules.rs:1008`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1008) |
| 318 | matches shortest matchhost first | ported | [`crates/renovate-core/src/util/host_rules.rs:1027`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1027) |
| 334 | matches readonly requests | ported | [`crates/renovate-core/src/util/host_rules.rs:1051`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1051) |
| 355 | returns hosts | ported | [`crates/renovate-core/src/util/host_rules.rs:1079`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1079) |
| 393 | warns and returns empty for bad search | ported | [`crates/renovate-core/src/util/host_rules.rs:582`](../../../../../crates/renovate-core/src/util/host_rules.rs#L582) |
| 397 | needs exact host matches | ported | [`crates/renovate-core/src/util/host_rules.rs:590`](../../../../../crates/renovate-core/src/util/host_rules.rs#L590) |
| 418 | returns all host rules | ported | [`crates/renovate-core/src/util/host_rules.rs:1174`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1174) |
| 437 | return hosttype | ported | [`crates/renovate-core/src/util/host_rules.rs:1202`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1202) |
| 459 | returns null | ported | [`crates/renovate-core/src/util/host_rules.rs:1233`](../../../../../crates/renovate-core/src/util/host_rules.rs#L1233) |

