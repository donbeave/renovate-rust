# `lib/modules/manager/pip_requirements/extract.spec.ts`

[← `manager/pip_requirements`](../../../../_by-module/manager/pip_requirements.md) · [all modules](../../../../README.md)

**22/22 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 39 | returns null for empty | ported | [`crates/renovate-core/src/extractors/pip.rs:572`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L572) |
| 43 | extracts dependencies | ported | [`crates/renovate-core/src/extractors/pip.rs:416`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L416) |
| 50 | extracts dependencies with --index-url short code | ported | [`crates/renovate-core/src/extractors/pip.rs:579`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L579) |
| 68 | extracts --requirement short code option | ported | [`crates/renovate-core/src/extractors/pip.rs:497`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L497) |
| 79 | extracts --constraints short code option | ported | [`crates/renovate-core/src/extractors/pip.rs:505`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L505) |
| 90 | extracts multiple dependencies | ported | [`crates/renovate-core/src/extractors/pip.rs:473`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L473) |
| 96 | handles comments and commands | ported | [`crates/renovate-core/src/extractors/pip.rs:443`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L443) |
| 102 | handles extras and complex index url | ported | [`crates/renovate-core/src/extractors/pip.rs:466`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L466) |
| 111 | handles extra index url | ported | [`crates/renovate-core/src/extractors/pip.rs:604`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L604) |
| 123 | handles extra index url and defaults without index to config | ported | [`crates/renovate-core/src/extractors/pip.rs:627`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L627) |
| 132 | handles extra index url and defaults without index to pypi | ported | [`crates/renovate-core/src/extractors/pip.rs:646`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L646) |
| 141 | handles extra spaces around pinned dependency equal signs | ported | [`crates/renovate-core/src/extractors/pip.rs:705`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L705) |
| 155 | should not replace env vars in low trust mode | ported | [`crates/renovate-core/src/extractors/pip.rs:665`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L665) |
| 166 | should replace env vars in high trust mode | ported | [`crates/renovate-core/src/extractors/pip.rs:685`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L685) |
| 178 | should handle hashes | ported | [`crates/renovate-core/src/extractors/pip.rs:725`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L725) |
| 184 | should handle package with extras and no version specifiers | ported | [`crates/renovate-core/src/extractors/pip.rs:458`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L458) |
| 198 | should handle dependency and ignore env markers | ported | [`crates/renovate-core/src/extractors/pip.rs:450`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L450) |
| 213 | should handle git packages | ported | [`crates/renovate-core/src/extractors/pip.rs:483`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L483) |
| 258 | extracts a file with only --index-url flags | ported | [`crates/renovate-core/src/extractors/npm.rs:4101`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4101) |
| 266 | extracts a file with only --extra-index-url flags | ported | [`crates/renovate-core/src/extractors/pip.rs:536`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L536) |
| 276 | extracts a file with only -r flags | ported | [`crates/renovate-core/src/extractors/pip.rs:550`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L550) |
| 286 | extracts a file with only -c flags | ported | [`crates/renovate-core/src/extractors/pip.rs:561`](../../../../../../../crates/renovate-core/src/extractors/pip.rs#L561) |

