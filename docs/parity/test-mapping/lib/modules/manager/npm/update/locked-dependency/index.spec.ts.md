# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5538`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5538) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5553`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5553) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5568`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5568) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5576`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5576) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5584`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5584) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5635`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5635) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5650`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5650) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5598`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5598) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5664`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5664) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5679`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5679) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5693`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5693) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5607`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5607) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5707`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5707) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5716`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5716) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5616`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5616) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5724`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5724) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5733`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5733) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5741`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5741) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5750`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5750) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5765`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5765) |

