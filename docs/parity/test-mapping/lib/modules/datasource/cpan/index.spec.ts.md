# `lib/modules/datasource/cpan/index.spec.ts`

[← `datasource/cpan`](../../../../_by-module/datasource/cpan.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/cpan.rs:277`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L277) |
| 27 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/cpan.rs:297`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L297) |
| 37 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/cpan.rs:314`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L314) |
| 47 | returns null for unknown error | ported | [`crates/renovate-core/src/datasources/cpan.rs:329`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L329) |
| 57 | processes real data | ported | [`crates/renovate-core/src/datasources/cpan.rs:346`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L346) |

