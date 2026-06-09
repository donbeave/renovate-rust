# `lib/config/migrations/base/abstract-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**0/0 in-scope tests ported** (0 pending, 2 opt-out) · status: opt-out

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should not allow to use method rewrite | opt-out | tests enforcement in AbstractMigration base class that subclasses must not directly invoke internal rewrite/delete helpers (TypeScript inheritance and runtime guard mechanics with no direct equivalent in Rust Migration trait + map-based design) |
| 23 | should not allow to use method delete | opt-out | tests enforcement in AbstractMigration base class that subclasses must not directly invoke internal rewrite/delete helpers (TypeScript inheritance and runtime guard mechanics with no direct equivalent in Rust Migration trait + map-based design) |

