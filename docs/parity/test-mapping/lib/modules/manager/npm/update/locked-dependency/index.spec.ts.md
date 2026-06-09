# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5545`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5545) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5560`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5560) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5575`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5575) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5583`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5583) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5591`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5591) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5642`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5642) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5657`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5657) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5605`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5605) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5671`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5671) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5686`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5686) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5700`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5700) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5614`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5614) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5714`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5714) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5723`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5723) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5623`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5623) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5731`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5731) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5740`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5740) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5748`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5748) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5757`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5757) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5772`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5772) |

