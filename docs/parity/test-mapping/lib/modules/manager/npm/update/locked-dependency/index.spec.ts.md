# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5546`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5546) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5561`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5561) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5576`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5576) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5584`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5584) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5592`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5592) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5643`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5643) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5658`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5658) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5606`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5606) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5672`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5672) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5687`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5687) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5701`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5701) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5615`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5615) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5715`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5715) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5724`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5724) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5624`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5624) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5732`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5732) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5741`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5741) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5749`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5749) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5758`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5758) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5773`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5773) |

