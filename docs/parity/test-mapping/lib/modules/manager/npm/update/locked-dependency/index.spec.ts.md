# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5544`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5544) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5559`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5559) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5574`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5574) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5582`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5582) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5590`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5590) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5641`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5641) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5656`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5656) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5604`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5604) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5670`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5670) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5685`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5685) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5699`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5699) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5613`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5613) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5713`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5713) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5722`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5722) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5622`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5622) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5730`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5730) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5739`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5739) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5747`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5747) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5756`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5756) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5771`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5771) |

