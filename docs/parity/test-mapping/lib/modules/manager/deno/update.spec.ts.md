# `lib/modules/manager/deno/update.spec.ts`

[← `manager/deno`](../../../../_by-module/manager/deno.md) · [all modules](../../../../README.md)

**27/38 ported** (11 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | updates dependency in imports | ported | `crates/renovate-core/src/extractors/deno.rs:541` |
| 39 | throws when multiple imports require more than one replacement | ported | `crates/renovate-core/src/extractors/deno.rs:560` |
| 64 | updates dependency in scopes | ported | `crates/renovate-core/src/extractors/deno.rs:576` |
| 98 | returns null when scopes element not found | ported | `crates/renovate-core/src/extractors/deno.rs:591` |
| 127 | updates dependency in tasks | ported | `crates/renovate-core/src/extractors/deno.rs:600` |
| 158 | updates dependency in tasks.command | ported | `crates/renovate-core/src/extractors/deno.rs:754` |
| 191 | returns null when tasks element not found | ported | `crates/renovate-core/src/extractors/deno.rs:613` |
| 221 | returns null when tasks.command element not found | ported | `crates/renovate-core/src/extractors/deno.rs:767` |
| 251 | updates dependency in compileroptions.types | ported | `crates/renovate-core/src/extractors/deno.rs:622` |
| 281 | returns null when compileroptions.types is empty array | ported | `crates/renovate-core/src/extractors/deno.rs:635` |
| 308 | returns null when compileroptions.types element not found | ported | `crates/renovate-core/src/extractors/deno.rs:776` |
| 335 | updates dependency in compileroptions.jsximportsource | ported | `crates/renovate-core/src/extractors/deno.rs:644` |
| 367 | returns null when compileroptions.jsximportsource does not exist | ported | `crates/renovate-core/src/extractors/deno.rs:791` |
| 394 | returns null when compileroptions.jsximportsourcetypes does not exist | ported | `crates/renovate-core/src/extractors/deno.rs:806` |
| 421 | updates dependency in compileroptions.jsximportsourcetypes | ported | `crates/renovate-core/src/extractors/deno.rs:821` |
| 453 | updates dependency in lint plugins | ported | `crates/renovate-core/src/extractors/deno.rs:665` |
| 481 | returns null when lint.plugins element not found | ported | `crates/renovate-core/src/extractors/deno.rs:842` |
| 508 | returns null when lint.plugins is empty array | ported | `crates/renovate-core/src/extractors/deno.rs:851` |
| 535 | handles dependency without version | ported | `crates/renovate-core/src/extractors/deno.rs:860` |
| 563 | returns null if packagefile is not defined | ported | `crates/renovate-core/src/extractors/deno.rs:678` |
| 575 | returns null for not supported datasource | ported | `crates/renovate-core/src/extractors/deno.rs:746` |
| 602 | currentvalue is not defined when deno datasource | ported | `crates/renovate-core/src/extractors/deno.rs:874` |
| 629 | returns null for missing required values | ported | `crates/renovate-core/src/extractors/deno.rs:889` |
| 648 | handles complex json with nested structures | ported | `crates/renovate-core/src/extractors/deno.rs:902` |
| 689 | handles the case where the desired version is already supported | ported | `crates/renovate-core/src/extractors/deno.rs:694` |
| 712 | returns null if empty file | ported | `crates/renovate-core/src/extractors/deno.rs:718` |
| 731 | handles error during update gracefully | ported | `crates/renovate-core/src/extractors/deno.rs:732` |
| 750 | depname is not defined | pending | — |
| 773 | unsupported packagefile | pending | — |
| 792 | replaces only exact matches | pending | — |
| 818 | returns null when deptype is not handled | pending | — |
| 841 | returns null when compileroptions.types does not exist | pending | — |
| 864 | returns null when lint.plugins does not exist | pending | — |
| 889 | updates dependency in imports | ported | `crates/renovate-core/src/extractors/deno.rs:541` |
| 916 | handles error during update gracefully | ported | `crates/renovate-core/src/extractors/deno.rs:732` |
| 938 | returns null for not supported datasource | ported | `crates/renovate-core/src/extractors/deno.rs:746` |
| 968 | depname is not defined | pending | — |
| 997 | replaces a dependency value | pending | — |

