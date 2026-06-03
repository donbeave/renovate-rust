# `lib/modules/datasource/rust-version/parse.spec.ts`

[← `datasource/rust-version`](../../../../_by-module/datasource/rust-version.md) · [all modules](../../../../README.md)

**13/13 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | parses nightly url | ported | `crates/renovate-core/src/datasources/rust_version.rs:184` |
| 15 | parses versioned release url | ported | `crates/renovate-core/src/datasources/rust_version.rs:198` |
| 25 | parses beta versioned url | ported | `crates/renovate-core/src/datasources/rust_version.rs:212` |
| 35 | parses stable channel url | ported | `crates/renovate-core/src/datasources/rust_version.rs:227` |
| 45 | parses beta channel url | ported | `crates/renovate-core/src/datasources/rust_version.rs:241` |
| 55 | parses url with https protocol | ported | `crates/renovate-core/src/datasources/rust_version.rs:255` |
| 65 | parses url with http protocol | ported | `crates/renovate-core/src/datasources/rust_version.rs:270` |
| 75 | returns null for url without date | ported | `crates/renovate-core/src/datasources/rust_version.rs:285` |
| 82 | returns null for url without channel-rust pattern | ported | `crates/renovate-core/src/datasources/rust_version.rs:292` |
| 89 | returns null for empty string | ported | `crates/renovate-core/src/datasources/rust_version.rs:299` |
| 94 | returns null for malformed date | ported | `crates/renovate-core/src/datasources/rust_version.rs:305` |
| 104 | parses url with different domain | ported | `crates/renovate-core/src/datasources/rust_version.rs:322` |
| 114 | parses url with complex version | ported | `crates/renovate-core/src/datasources/rust_version.rs:335` |

