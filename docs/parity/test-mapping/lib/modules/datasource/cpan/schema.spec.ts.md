# `lib/modules/datasource/cpan/schema.spec.ts`

[← `datasource/cpan`](../../../../_by-module/datasource/cpan.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | filters out entries with empty module arrays | ported | `crates/renovate-core/src/datasources/cpan.rs:224` |
| 26 | filters out entries where module has no version | ported | `crates/renovate-core/src/datasources/cpan.rs:238` |
| 47 | includes valid entries | ported | `crates/renovate-core/src/datasources/cpan.rs:254` |

