# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5541`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5541) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5556`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5556) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5571`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5571) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5579`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5579) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5587`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5587) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5638`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5638) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5653`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5653) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5601`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5601) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5667`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5667) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5682`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5682) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5696`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5696) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5610`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5610) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5710`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5710) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5719`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5719) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5619`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5619) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5727`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5727) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5736`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5736) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5744`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5744) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5753`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5753) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5768`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5768) |

