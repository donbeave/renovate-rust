# `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**20/20 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 45 | validates filename | ported | `crates/renovate-core/src/extractors/npm.rs:5496` |
| 54 | validates versions | ported | `crates/renovate-core/src/extractors/npm.rs:5511` |
| 63 | returns null for unparseable files | ported | `crates/renovate-core/src/extractors/npm.rs:5526` |
| 72 | rejects lockfileversion 2 | ported | `crates/renovate-core/src/extractors/npm.rs:5534` |
| 81 | returns null if no locked deps | ported | `crates/renovate-core/src/extractors/npm.rs:5542` |
| 85 | rejects null if no constraint found | ported | `crates/renovate-core/src/extractors/npm.rs:5593` |
| 97 | remediates in-range | ported | `crates/renovate-core/src/extractors/npm.rs:5608` |
| 109 | rejects in-range remediation if lockfile v2+ | ported | `crates/renovate-core/src/extractors/npm.rs:5556` |
| 120 | fails to remediate if parent dep cannot support | ported | `crates/renovate-core/src/extractors/npm.rs:5622` |
| 140 | remediates express | ported | `crates/renovate-core/src/extractors/npm.rs:5637` |
| 150 | remediates lock file v2 express | ported | `crates/renovate-core/src/extractors/npm.rs:5651` |
| 161 | returns already-updated if already remediated exactly | ported | `crates/renovate-core/src/extractors/npm.rs:5565` |
| 169 | returns already-updated if already v2 remediated exactly | ported | `crates/renovate-core/src/extractors/npm.rs:5665` |
| 178 | returns already-updated if already remediated higher | ported | `crates/renovate-core/src/extractors/npm.rs:5674` |
| 187 | returns already-updated if not found | ported | `crates/renovate-core/src/extractors/npm.rs:5574` |
| 196 | returns update-failed if other, lower version found | ported | `crates/renovate-core/src/extractors/npm.rs:5682` |
| 205 | remediates mime | ported | `crates/renovate-core/src/extractors/npm.rs:5691` |
| 222 | fails remediation if cannot update parent | ported | `crates/renovate-core/src/extractors/npm.rs:5699` |
| 231 | fails remediation if bundled | ported | `crates/renovate-core/src/extractors/npm.rs:5708` |
| 241 | rejects in-range remediation if pnpm | ported | `crates/renovate-core/src/extractors/npm.rs:5723` |

