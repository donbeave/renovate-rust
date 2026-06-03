# `lib/modules/manager/gradle/extract/catalog.spec.ts`

[← `manager/gradle`](../../../../../_by-module/manager/gradle.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | supports versions declared as single string | ported | [`crates/renovate-core/src/extractors/gradle.rs:1822`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1822) |
| 134 | deletes commit message for plugins with version reference | ported | [`crates/renovate-core/src/extractors/gradle.rs:1951`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1951) |
| 180 | ignores empty toml file | ported | [`crates/renovate-core/src/extractors/gradle.rs:1931`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1931) |
| 185 | skips version entries with no resolvable literal value | ported | [`crates/renovate-core/src/extractors/gradle.rs:1938`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1938) |
| 203 | changes the dependency version, not the comment version | ported | [`crates/renovate-core/src/extractors/gradle.rs:1999`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1999) |
| 254 | supports templated toml | ported | [`crates/renovate-core/src/extractors/gradle.rs:2051`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2051) |

