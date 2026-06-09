# `lib/modules/manager/npm/range.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | returns same if not auto | ported | [`crates/renovate-core/src/extractors/npm.rs:4715`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4715) |
| 10 | widens peerdependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4721`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4721) |
| 18 | widens complex ranges | ported | [`crates/renovate-core/src/extractors/npm.rs:4728`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4728) |
| 27 | widens complex bump | ported | [`crates/renovate-core/src/extractors/npm.rs:4735`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4735) |
| 36 | defaults to update-lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:4742`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4742) |

