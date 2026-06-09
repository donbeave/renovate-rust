# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5550`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5550) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5565`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5565) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5580`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5580) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5588`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5588) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5596`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5596) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5647`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5647) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5662`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5662) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5610`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5610) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5676`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5676) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5691`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5691) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5705`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5705) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5619`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5619) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5719`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5719) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5728`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5728) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5628`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5628) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5736`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5736) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5745`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5745) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5753`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5753) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5762`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5762) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5777`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5777) |

