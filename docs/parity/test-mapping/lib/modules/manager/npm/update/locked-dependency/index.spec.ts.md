# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5539`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5539) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5554`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5554) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5569`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5569) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5577`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5577) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5585`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5585) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5636`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5636) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5651`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5651) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5599`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5599) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5665`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5665) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5680`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5680) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5694`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5694) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5608`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5608) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5708`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5708) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5717`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5717) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5617`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5617) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5725`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5725) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5734`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5734) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5742`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5742) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5751`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5751) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5766`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5766) |

