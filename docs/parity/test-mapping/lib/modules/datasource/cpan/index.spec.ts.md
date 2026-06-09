# `lib/modules/datasource/cpan/index.spec.ts`

[← `datasource/cpan`](../../../../_by-module/datasource/cpan.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/cpan.rs:278`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L278) |
| 27 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/cpan.rs:298`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L298) |
| 37 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/cpan.rs:315`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L315) |
| 47 | returns null for unknown error | ported | [`crates/renovate-core/src/datasources/cpan.rs:330`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L330) |
| 57 | processes real data | ported | [`crates/renovate-core/src/datasources/cpan.rs:347`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L347) |

