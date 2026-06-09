# `lib/modules/datasource/cpan/schema.spec.ts`

[← `datasource/cpan`](../../../../_by-module/datasource/cpan.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | filters out entries with empty module arrays | ported | [`crates/renovate-core/src/datasources/cpan.rs:225`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L225) |
| 26 | filters out entries where module has no version | ported | [`crates/renovate-core/src/datasources/cpan.rs:239`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L239) |
| 47 | includes valid entries | ported | [`crates/renovate-core/src/datasources/cpan.rs:255`](../../../../../../../crates/renovate-core/src/datasources/cpan.rs#L255) |

