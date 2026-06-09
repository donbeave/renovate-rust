# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5542`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5542) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5557`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5557) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5572`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5572) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5580`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5580) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5588`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5588) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5639`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5639) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5654`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5654) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5602`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5602) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5668`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5668) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5683`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5683) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5697`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5697) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5611`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5611) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5711`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5711) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5720`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5720) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5620`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5620) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5728`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5728) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5737`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5737) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5745`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5745) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5754`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5754) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5769`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5769) |

