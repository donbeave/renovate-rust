# `lib/workers/repository/errors-warnings.spec.ts`

[← `worker/repository`](../../../_by-module/worker/repository.md) · [all modules](../../../README.md)

**14/16 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 20 | returns warning text | ported | `crates/renovate-core/src/util.rs:11143` |
| 41 | getwarning returns empty string | ported | `crates/renovate-core/src/util.rs:11153` |
| 49 | returns 2 pr warnings text dependencydashboard true | ported | `crates/renovate-core/src/util.rs:11174` |
| 97 | returns 2 pr warnings text dependencydashboard true with issue link | ported | `crates/renovate-core/src/util.rs:11198` |
| 120 | returns 2 pr warnings text dependencydashboard false | ported | `crates/renovate-core/src/util.rs:11209` |
| 168 | pr warning returns empty string | ported | `crates/renovate-core/src/util.rs:11220` |
| 175 | suppress notifications contains dependencylookupwarnings flag then return empty string | ported | `crates/renovate-core/src/util.rs:11226` |
| 186 | returns dependency dashboard warning text | ported | `crates/renovate-core/src/util.rs:11232` |
| 236 | dependency dashboard warning returns empty string | ported | `crates/renovate-core/src/util.rs:11256` |
| 243 | suppress notifications contains dependencylookupwarnings flag then return empty string | ported | `crates/renovate-core/src/util.rs:11226` |
| 260 | returns error text | ported | `crates/renovate-core/src/util.rs:11159` |
| 281 | geterror returns empty string | ported | `crates/renovate-core/src/util.rs:11168` |
| 289 | returns onboarding warning text | ported | `crates/renovate-core/src/util.rs:11268` |
| 345 | handle empty package files | ported | `crates/renovate-core/src/util.rs:11293` |
| 356 | suppress notifications contains dependencylookupwarnings flag then return empty string | ported | `crates/renovate-core/src/util.rs:11226` |
| 365 | handles undefined | ported | `crates/renovate-core/src/util.rs:11305` |

