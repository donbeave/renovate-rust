# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5536`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5536) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5551`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5551) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5566`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5566) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5574`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5574) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5582`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5582) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5633`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5633) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5648`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5648) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5596`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5596) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5662`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5662) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5677`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5677) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5691`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5691) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5605`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5605) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5705`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5705) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5714`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5714) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5614`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5614) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5722`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5722) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5731`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5731) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5739`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5739) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5748`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5748) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5763`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5763) |

