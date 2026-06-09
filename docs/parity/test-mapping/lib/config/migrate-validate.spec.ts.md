# `lib/config/migrate-validate.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**4/4 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | handles empty | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4975`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4975) |
| 22 | handles migration | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4984`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4984) |
| 32 | handles invalid | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6713`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6713) |
| 40 | isonboarded | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6720`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6720) |
| 50 | logs errors | opt-out | asserts TypeScript logger spy behavior and injected migrateConfig throw path via runtime mocking; no direct Rust equivalent without changing production instrumentation |

