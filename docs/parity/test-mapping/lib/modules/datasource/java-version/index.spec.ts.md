# `lib/modules/datasource/java-version/index.spec.ts`

[← `datasource/java-version`](../../../../_by-module/datasource/java-version.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | throws for error | ported | `crates/renovate-core/src/datasources/java_version.rs:207` |
| 29 | returns null for 404 | ported | `crates/renovate-core/src/datasources/java_version.rs:224` |
| 39 | returns null for empty result | ported | `crates/renovate-core/src/datasources/java_version.rs:238` |
| 49 | returns null for empty 200 ok | ported | `crates/renovate-core/src/datasources/java_version.rs:252` |
| 62 | throws for 5xx | ported | `crates/renovate-core/src/datasources/java_version.rs:268` |
| 72 | processes real data | ported | `crates/renovate-core/src/datasources/java_version.rs:282` |
| 85 | processes real data (jre) | ported | `crates/renovate-core/src/datasources/java_version.rs:303` |
| 98 | processes real data (jre,windows,x64) | ported | `crates/renovate-core/src/datasources/java_version.rs:323` |
| 110 | pages | ported | `crates/renovate-core/src/datasources/java_version.rs:332` |
| 128 | processes real data (jre,system) | ported | `crates/renovate-core/src/datasources/java_version.rs:362` |

