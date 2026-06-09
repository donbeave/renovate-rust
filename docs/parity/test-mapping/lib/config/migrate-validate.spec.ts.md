# `lib/config/migrate-validate.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**4/4 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | handles empty | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4994`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4994) |
| 22 | handles migration | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5003`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5003) |
| 32 | handles invalid | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6732`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6732) |
| 40 | isonboarded | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6739`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6739) |
| 50 | logs errors | opt-out | asserts TypeScript logger spy behavior and injected migrateConfig throw path via runtime mocking; no direct Rust equivalent without changing production instrumentation |

