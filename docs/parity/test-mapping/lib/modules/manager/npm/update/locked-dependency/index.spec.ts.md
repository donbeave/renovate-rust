# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 45 | validates filename | ported | [`crates/renovate-core/src/extractors/npm.rs:5543`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5543) |
| 54 | validates versions | ported | [`crates/renovate-core/src/extractors/npm.rs:5558`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5558) |
| 63 | returns null for unparseable files | ported | [`crates/renovate-core/src/extractors/npm.rs:5573`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5573) |
| 72 | rejects lockfileversion 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5581`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5581) |
| 81 | returns null if no locked deps | ported | [`crates/renovate-core/src/extractors/npm.rs:5589`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5589) |
| 85 | rejects null if no constraint found | ported | [`crates/renovate-core/src/extractors/npm.rs:5640`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5640) |
| 97 | remediates in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5655`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5655) |
| 109 | rejects in-range remediation if lockfile v2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5603`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5603) |
| 120 | fails to remediate if parent dep cannot support | ported | [`crates/renovate-core/src/extractors/npm.rs:5669`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5669) |
| 140 | remediates express | ported | [`crates/renovate-core/src/extractors/npm.rs:5684`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5684) |
| 150 | remediates lock file v2 express | ported | [`crates/renovate-core/src/extractors/npm.rs:5698`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5698) |
| 161 | returns already-updated if already remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5612`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5612) |
| 169 | returns already-updated if already v2 remediated exactly | ported | [`crates/renovate-core/src/extractors/npm.rs:5712`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5712) |
| 178 | returns already-updated if already remediated higher | ported | [`crates/renovate-core/src/extractors/npm.rs:5721`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5721) |
| 187 | returns already-updated if not found | ported | [`crates/renovate-core/src/extractors/npm.rs:5621`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5621) |
| 196 | returns update-failed if other, lower version found | ported | [`crates/renovate-core/src/extractors/npm.rs:5729`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5729) |
| 205 | remediates mime | ported | [`crates/renovate-core/src/extractors/npm.rs:5738`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5738) |
| 222 | fails remediation if cannot update parent | ported | [`crates/renovate-core/src/extractors/npm.rs:5746`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5746) |
| 231 | fails remediation if bundled | ported | [`crates/renovate-core/src/extractors/npm.rs:5755`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5755) |
| 241 | rejects in-range remediation if pnpm | ported | [`crates/renovate-core/src/extractors/npm.rs:5770`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5770) |

