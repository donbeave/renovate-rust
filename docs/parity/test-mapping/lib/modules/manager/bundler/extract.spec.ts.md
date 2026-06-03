# `lib/modules/manager/bundler/extract.spec.ts`

[← `manager/bundler`](../../../../_by-module/manager/bundler.md) · [all modules](../../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 36 | returns null for empty | ported | [`crates/renovate-core/src/extractors/bundler.rs:1146`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1146) |
| 40 | parses rails gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1152`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1152) |
| 57 | parses sourcegroups | ported | [`crates/renovate-core/src/extractors/bundler.rs:1169`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1169) |
| 63 | parse webpacker gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1176`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1176) |
| 75 | parse mastodon gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1187`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1187) |
| 91 | parse ruby ci gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1194`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1194) |
| 104 | parse gitlab foss gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1205`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1205) |
| 116 | parse source blocks in gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1216`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1216) |
| 122 | parse source blocks with spaces in gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1225`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1225) |
| 132 | parses source blocks with groups in gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1232`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1232) |
| 146 | parses source variable in gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1268`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1268) |
| 171 | parses inline source in gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1279`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1279) |
| 223 | parses git refs in gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1307`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1307) |
| 259 | parses multiple current values gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1354`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1354) |
| 284 | skips local gems in gemfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:1511`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1511) |

