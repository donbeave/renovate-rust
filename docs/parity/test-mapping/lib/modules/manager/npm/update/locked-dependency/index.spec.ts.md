# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5540`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5540) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5555`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5555) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5570`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5570) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5578`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5578) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5586`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5586) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5637`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5637) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5652`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5652) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5600`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5600) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5666`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5666) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5681`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5681) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5695`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5695) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5609`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5609) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5709`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5709) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5718`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5718) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5618`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5618) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5726`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5726) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5735`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5735) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5743`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5743) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5752`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5752) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5767`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5767) |

