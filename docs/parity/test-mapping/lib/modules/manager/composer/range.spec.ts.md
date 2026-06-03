# `lib/modules/manager/composer/range.spec.ts`

[← `manager/composer`](../../../../_by-module/manager/composer.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | returns same if not auto | ported | [`crates/renovate-core/src/extractors/composer.rs:1129`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1129) |
| 10 | replaces require-dev | ported | [`crates/renovate-core/src/extractors/composer.rs:1135`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1135) |
| 18 | replaces project require | ported | [`crates/renovate-core/src/extractors/composer.rs:1144`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1144) |
| 27 | widens complex ranges | ported | [`crates/renovate-core/src/extractors/composer.rs:1153`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1153) |
| 36 | widens complex bump | ported | [`crates/renovate-core/src/extractors/composer.rs:1162`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1162) |
| 45 | defaults to update-lockfile | ported | [`crates/renovate-core/src/extractors/composer.rs:1171`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1171) |
| 50 | defaults to widen for typo3 extensions | ported | [`crates/renovate-core/src/extractors/composer.rs:1180`](../../../../../../../crates/renovate-core/src/extractors/composer.rs#L1180) |

