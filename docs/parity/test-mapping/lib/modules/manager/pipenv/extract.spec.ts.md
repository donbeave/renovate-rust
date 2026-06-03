# `lib/modules/manager/pipenv/extract.spec.ts`

[← `manager/pipenv`](../../../../_by-module/manager/pipenv.md) · [all modules](../../../../README.md)

**16/16 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 37 | returns null for empty | ported | [`crates/renovate-core/src/extractors/pipfile.rs:374`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L374) |
| 41 | returns null for invalid toml file | ported | [`crates/renovate-core/src/extractors/pipfile.rs:368`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L368) |
| 45 | extracts dependencies | ported | [`crates/renovate-core/src/extractors/pipfile.rs:290`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L290) |
| 136 | marks packages with "extras" as skipreason === unspecified-version | ported | [`crates/renovate-core/src/extractors/pipfile.rs:380`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L380) |
| 142 | extracts multiple dependencies | ported | [`crates/renovate-core/src/extractors/pipfile.rs:668`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L668) |
| 192 | ignores git dependencies | ported | [`crates/renovate-core/src/extractors/pipfile.rs:325`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L325) |
| 202 | ignores invalid package names | ported | [`crates/renovate-core/src/extractors/pipfile.rs:409`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L409) |
| 213 | ignores relative path dependencies | ported | [`crates/renovate-core/src/extractors/pipfile.rs:333`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L333) |
| 223 | ignores invalid versions | ported | [`crates/renovate-core/src/extractors/pipfile.rs:317`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L317) |
| 234 | extracts all sources | ported | [`crates/renovate-core/src/extractors/pipfile.rs:450`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L450) |
| 247 | extracts example pipfile | ported | [`crates/renovate-core/src/extractors/pipfile.rs:468`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L468) |
| 313 | supports custom index | ported | [`crates/renovate-core/src/extractors/pipfile.rs:574`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L574) |
| 338 | gets python constraint from python_version | ported | [`crates/renovate-core/src/extractors/pipfile.rs:610`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L610) |
| 350 | gets python constraint from python_full_version | ported | [`crates/renovate-core/src/extractors/pipfile.rs:626`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L626) |
| 362 | gets pipenv constraint from packages | ported | [`crates/renovate-core/src/extractors/pipfile.rs:642`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L642) |
| 372 | gets pipenv constraint from dev-packages | ported | [`crates/renovate-core/src/extractors/pipfile.rs:655`](../../../../../../../crates/renovate-core/src/extractors/pipfile.rs#L655) |

