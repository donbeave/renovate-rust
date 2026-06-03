# `lib/util/host-rules.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**27/29 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 18 | throws if both domainname and hostname | ported | `crates/renovate-core/src/util/host_rules.rs:426` |
| 28 | throws if both domainname and baseurl | ported | `crates/renovate-core/src/util/host_rules.rs:444` |
| 38 | throws if both hostname and baseurl | ported | `crates/renovate-core/src/util/host_rules.rs:462` |
| 48 | supports baseurl-only | ported | `crates/renovate-core/src/util/host_rules.rs:480` |
| 72 | does not match subpart of hostname | ported | `crates/renovate-core/src/util/host_rules.rs:520` |
| 84 | massages host url | ported | `crates/renovate-core/src/util/host_rules.rs:548` |
| 111 | warns and returns empty for bad search | ported | `crates/renovate-core/src/util/host_rules.rs:582` |
| 115 | needs exact host matches | ported | `crates/renovate-core/src/util/host_rules.rs:590` |
| 135 | matches on empty rules | ported | `crates/renovate-core/src/util/host_rules.rs:640` |
| 144 | matches on hosttype | ported | `crates/renovate-core/src/util/host_rules.rs:657` |
| 154 | matches on domainname | ported | `crates/renovate-core/src/util/host_rules.rs:675` |
| 172 | matches on specific path | ported | `crates/renovate-core/src/util/host_rules.rs:722` |
| 199 | matches for several hosttypes when no hosttype rule is configured | ported | `crates/renovate-core/src/util/host_rules.rs:756` |
| 218 | matches if hosttype is configured and host rule is filtered with datasource | ported | `crates/renovate-core/src/util/host_rules.rs:775` |
| 237 | matches on hostname | ported | `crates/renovate-core/src/util/host_rules.rs:802` |
| 247 | matches on matchhost with protocol | ported | `crates/renovate-core/src/util/host_rules.rs:829` |
| 262 | matches on matchhost without protocol | ported | `crates/renovate-core/src/util/host_rules.rs:878` |
| 272 | matches on matchhost with dot prefix | ported | `crates/renovate-core/src/util/host_rules.rs:919` |
| 282 | matches on matchhost with port | ported | `crates/renovate-core/src/util/host_rules.rs:959` |
| 292 | matches on hosttype and endpoint | ported | `crates/renovate-core/src/util/host_rules.rs:989` |
| 304 | matches on endpoint subresource | ported | `crates/renovate-core/src/util/host_rules.rs:1008` |
| 318 | matches shortest matchhost first | ported | `crates/renovate-core/src/util/host_rules.rs:1027` |
| 334 | matches readonly requests | ported | `crates/renovate-core/src/util/host_rules.rs:1051` |
| 355 | returns hosts | ported | `crates/renovate-core/src/util/host_rules.rs:1079` |
| 393 | warns and returns empty for bad search | ported | `crates/renovate-core/src/util/host_rules.rs:582` |
| 397 | needs exact host matches | ported | `crates/renovate-core/src/util/host_rules.rs:590` |
| 418 | returns all host rules | ported | `crates/renovate-core/src/util/host_rules.rs:1174` |
| 437 | return hosttype | ported | `crates/renovate-core/src/util/host_rules.rs:1202` |
| 459 | returns null | ported | `crates/renovate-core/src/util/host_rules.rs:1233` |

