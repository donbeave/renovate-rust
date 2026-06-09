# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5523`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5523) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5538`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5538) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5553`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5553) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5561`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5561) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5569`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5569) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5620`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5620) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5635`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5635) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5583`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5583) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5649`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5649) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5664`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5664) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5678`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5678) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5592`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5592) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5692`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5692) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5701`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5701) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5601`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5601) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5709`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5709) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5718`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5718) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5726`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5726) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5735`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5735) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5750`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5750) |

