# `lib/util/http/retry-after.spec.ts`

[← `util/http`](../../../_by-module/util/http.md) · [all modules](../../../README.md)

**10/13 ported** (3 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 27 | works | ported | `crates/renovate-core/src/http.rs:728` |
| 34 | throws | ported | `crates/renovate-core/src/http.rs:803` |
| 44 | retries | ported | `crates/renovate-core/src/http.rs:729` |
| 59 | gives up after max retries | ported | `crates/renovate-core/src/http.rs:759` |
| 76 | gives up when delay exceeds maxretryafter | ported | `crates/renovate-core/src/http.rs:781` |
| 89 | returns null for non-requesterror | pending | — |
| 93 | returns null for requesterror without response | pending | — |
| 97 | returns null for status other than 429 | pending | — |
| 103 | returns null missing "retry-after" header | ported | `crates/renovate-core/src/http.rs:1627` |
| 109 | returns null for non-integer "retry-after" header | ported | `crates/renovate-core/src/http.rs:1594` |
| 122 | returns delay in seconds from date | ported | `crates/renovate-core/src/http.rs:1602` |
| 136 | returns delay in seconds from number | ported | `crates/renovate-core/src/http.rs:1613` |
| 149 | returns null for invalid header value | ported | `crates/renovate-core/src/http.rs:1620` |

