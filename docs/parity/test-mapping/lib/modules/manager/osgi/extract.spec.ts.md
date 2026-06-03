# `lib/modules/manager/osgi/extract.spec.ts`

[← `manager/osgi`](../../../../_by-module/manager/osgi.md) · [all modules](../../../../README.md)

**14/14 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 143 | returns null for empty file | ported | `crates/renovate-core/src/extractors/osgi.rs:268` |
| 147 | returns null for invalid file | ported | `crates/renovate-core/src/extractors/osgi.rs:241` |
| 151 | returns null for unsupported version of feature model definition | ported | `crates/renovate-core/src/extractors/osgi.rs:231` |
| 157 | returns null for an invalid version of feature model definition | ported | `crates/renovate-core/src/extractors/osgi.rs:281` |
| 163 | returns null for a null string passed in as a feature model definition | ported | `crates/renovate-core/src/extractors/osgi.rs:262` |
| 167 | returns null for a valid file with no artifact definitions | ported | `crates/renovate-core/src/extractors/osgi.rs:274` |
| 171 | extracts the bundles from a file with object bundles definitions | ported | `crates/renovate-core/src/extractors/osgi.rs:184` |
| 193 | extracts the bundles from a file with string bundles defintions | ported | `crates/renovate-core/src/extractors/osgi.rs:165` |
| 215 | extracts the bundles from a file with comments | ported | `crates/renovate-core/src/extractors/osgi.rs:247` |
| 228 | extracts the artifacts from an extension section | ported | `crates/renovate-core/src/extractors/osgi.rs:291` |
| 241 | extracts the artifacts a file with a double slash | ported | `crates/renovate-core/src/extractors/osgi.rs:306` |
| 263 | extracts the artifacts from the framework artifact section | ported | `crates/renovate-core/src/extractors/osgi.rs:332` |
| 276 | skips depedencies with with malformed definitions | ported | `crates/renovate-core/src/extractors/osgi.rs:351` |
| 297 | skips artifacts with variables in version | ported | `crates/renovate-core/src/extractors/osgi.rs:217` |

