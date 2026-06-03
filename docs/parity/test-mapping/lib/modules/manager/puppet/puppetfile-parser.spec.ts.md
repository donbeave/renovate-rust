# `lib/modules/manager/puppet/puppetfile-parser.spec.ts`

[← `manager/puppet`](../../../../_by-module/manager/puppet.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | puppetfile_github_tag | ported | [`crates/renovate-core/src/extractors/puppet.rs:551`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L551) |
| 31 | puppetfile_github_tag_single_line | ported | [`crates/renovate-core/src/extractors/puppet.rs:579`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L579) |
| 58 | puppetfile with an invalid module creates puppetfilemodule with skipreason "invalid-config" | ported | [`crates/renovate-core/src/extractors/puppet.rs:460`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L460) |
| 74 | get default forge with null or undefined returns the same | ported | [`crates/renovate-core/src/extractors/puppet.rs:339`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L339) |
| 88 | puppetfile_multiple_forges | ported | [`crates/renovate-core/src/extractors/puppet.rs:599`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L599) |
| 133 | puppetfile_no_forge | ported | [`crates/renovate-core/src/extractors/puppet.rs:640`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L640) |
| 161 | puppetfile_single_forge | ported | [`crates/renovate-core/src/extractors/puppet.rs:654`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L654) |
| 192 | puppetfile_with_comments | ported | [`crates/renovate-core/src/extractors/puppet.rs:674`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L674) |

