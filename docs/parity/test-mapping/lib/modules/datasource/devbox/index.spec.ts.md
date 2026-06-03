# `lib/modules/datasource/devbox/index.spec.ts`

[← `datasource/devbox`](../../../../_by-module/datasource/devbox.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 29 | throws for error | ported | `crates/renovate-core/src/datasources/devbox.rs:143` |
| 43 | returns null for 404 | ported | `crates/renovate-core/src/datasources/devbox.rs:159` |
| 53 | returns null for empty result | ported | `crates/renovate-core/src/datasources/devbox.rs:177` |
| 63 | returns null for empty 200 ok | ported | `crates/renovate-core/src/datasources/devbox.rs:195` |
| 76 | throws for 5xx | ported | `crates/renovate-core/src/datasources/devbox.rs:213` |
| 86 | processes real data | ported | `crates/renovate-core/src/datasources/devbox.rs:229` |
| 118 | processes empty data | ported | `crates/renovate-core/src/datasources/devbox.rs:278` |
| 133 | returns null when no body is returned | ported | `crates/renovate-core/src/datasources/devbox.rs:304` |
| 145 | falls back to a default homepage_url | ported | `crates/renovate-core/src/datasources/devbox.rs:322` |

