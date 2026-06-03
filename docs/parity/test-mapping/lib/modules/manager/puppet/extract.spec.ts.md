# `lib/modules/manager/puppet/extract.spec.ts`

[← `manager/puppet`](../../../../_by-module/manager/puppet.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns null for empty puppetfile | ported | [`crates/renovate-core/src/extractors/puppet.rs:430`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L430) |
| 14 | extracts multiple modules from puppetfile without a forge | ported | [`crates/renovate-core/src/extractors/puppet.rs:338`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L338) |
| 47 | extracts multiple modules from puppetfile with multiple forges/registries | ported | [`crates/renovate-core/src/extractors/puppet.rs:354`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L354) |
| 100 | extracts multiple git tag modules from puppetfile | ported | [`crates/renovate-core/src/extractors/puppet.rs:368`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L368) |
| 125 | use githubtagsdatasource only if host is exactly github.com | ported | [`crates/renovate-core/src/extractors/puppet.rs:436`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L436) |
| 146 | github url without https is skipped | ported | [`crates/renovate-core/src/extractors/puppet.rs:449`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L449) |
| 162 | git module without a tag should result in a skip reason | ported | [`crates/renovate-core/src/extractors/puppet.rs:387`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L387) |
| 181 | skip reason should be overwritten by parser | ported | [`crates/renovate-core/src/extractors/puppet.rs:461`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L461) |
| 200 | gittagsdatasource | ported | [`crates/renovate-core/src/extractors/puppet.rs:472`](../../../../../../../crates/renovate-core/src/extractors/puppet.rs#L472) |

