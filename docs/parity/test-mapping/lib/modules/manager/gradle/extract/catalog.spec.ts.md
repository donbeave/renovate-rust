# `lib/modules/manager/gradle/extract/catalog.spec.ts`

[← `manager/gradle`](../../../../../_by-module/manager/gradle.md) · [all modules](../../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | supports versions declared as single string | ported | `crates/renovate-core/src/extractors/gradle.rs:1822` |
| 134 | deletes commit message for plugins with version reference | ported | `crates/renovate-core/src/extractors/gradle.rs:1951` |
| 180 | ignores empty toml file | ported | `crates/renovate-core/src/extractors/gradle.rs:1931` |
| 185 | skips version entries with no resolvable literal value | ported | `crates/renovate-core/src/extractors/gradle.rs:1938` |
| 203 | changes the dependency version, not the comment version | ported | `crates/renovate-core/src/extractors/gradle.rs:1999` |
| 254 | supports templated toml | ported | `crates/renovate-core/src/extractors/gradle.rs:2051` |

