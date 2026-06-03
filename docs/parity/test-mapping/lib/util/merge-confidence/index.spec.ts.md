# `lib/util/merge-confidence/index.spec.ts`

[← `util/merge-confidence`](../../../_by-module/util/merge-confidence.md) · [all modules](../../../README.md)

**7/28 ported** (21 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 22 | returns false if null | ported | `crates/renovate-core/src/merge_confidence.rs:45` |
| 26 | returns false if low | ported | `crates/renovate-core/src/merge_confidence.rs:51` |
| 30 | returns false if nonsense | ported | `crates/renovate-core/src/merge_confidence.rs:57` |
| 34 | returns true if valid value (high) | ported | `crates/renovate-core/src/merge_confidence.rs:63` |
| 40 | returns false if less | ported | `crates/renovate-core/src/merge_confidence.rs:69` |
| 44 | returns true if equal | ported | `crates/renovate-core/src/merge_confidence.rs:75` |
| 48 | returns true if more | ported | `crates/renovate-core/src/merge_confidence.rs:81` |
| 71 | returns neutral if undefined updatetype | pending | — |
| 83 | returns neutral if irrelevant updatetype | pending | — |
| 95 | returns high if pinning | pending | — |
| 107 | returns undefined if no token | pending | — |
| 122 | returns undefined if datasource is unsupported | pending | — |
| 134 | returns valid confidence level | pending | — |
| 157 | escapes a package name containing a forward slash | pending | — |
| 181 | escapes a partial maven coordinate of groupid:artifactid from the packagename | pending | — |
| 207 | returns neutral on invalid merge confidence response from api | pending | — |
| 230 | returns neutral on non 403/5xx error from api | pending | — |
| 258 | throws on 403-forbidden response from api | pending | — |
| 286 | throws on server error responses | pending | — |
| 314 | returns high if pinning digest | pending | — |
| 332 | using default base url and supported datasources if either is set | pending | — |
| 356 | warns and then resolves if base url is invalid | pending | — |
| 377 | uses custom supported datasources and a base url containing a path | pending | — |
| 401 | resolves if no token | pending | — |
| 411 | resolves when token is valid | pending | — |
| 424 | throws on 403-forbidden from mc api | pending | — |
| 437 | throws on 5xx host errors from mc api | pending | — |
| 450 | throws on econnreset | pending | — |

