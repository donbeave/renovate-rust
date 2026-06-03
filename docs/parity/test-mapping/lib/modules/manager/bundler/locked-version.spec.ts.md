# `lib/modules/manager/bundler/locked-version.spec.ts`

[← `manager/bundler`](../../../../_by-module/manager/bundler.md) · [all modules](../../../../README.md)

**12/12 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | parse rails gem lock file | ported | [`crates/renovate-core/src/extractors/bundler.rs:1036`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1036) |
| 19 | parse webpacker gem lock file | ported | [`crates/renovate-core/src/extractors/bundler.rs:1044`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1044) |
| 25 | parse mastodon gem lock file | ported | [`crates/renovate-core/src/extractors/bundler.rs:1051`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1051) |
| 31 | parse ruby ci gem lock file | ported | [`crates/renovate-core/src/extractors/bundler.rs:1058`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1058) |
| 37 | parse gitlab foss gem lock file | ported | [`crates/renovate-core/src/extractors/bundler.rs:1065`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1065) |
| 43 | returns empty map for empty string | ported | [`crates/renovate-core/src/extractors/bundler.rs:1072`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1072) |
| 48 | returns empty map when errors occur | ported | [`crates/renovate-core/src/extractors/bundler.rs:1078`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1078) |
| 54 | strips platform suffixes from dependencies | ported | [`crates/renovate-core/src/extractors/bundler.rs:1084`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1084) |
| 84 | extracts simple versions from parentheses | ported | [`crates/renovate-core/src/extractors/bundler.rs:1094`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1094) |
| 98 | extracts complex version formats from parentheses | ported | [`crates/renovate-core/src/extractors/bundler.rs:1103`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1103) |
| 114 | correctly extracts gem names when versions contain special characters | ported | [`crates/renovate-core/src/extractors/bundler.rs:1119`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1119) |
| 130 | handles gems with platform-specific versions | ported | [`crates/renovate-core/src/extractors/bundler.rs:1132`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L1132) |

