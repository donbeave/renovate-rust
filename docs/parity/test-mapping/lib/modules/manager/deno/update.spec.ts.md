# `lib/modules/manager/deno/update.spec.ts`

[← `manager/deno`](../../../../_by-module/manager/deno.md) · [all modules](../../../../README.md)

**27/38 ported** (11 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | updates dependency in imports | ported | [`crates/renovate-core/src/extractors/deno.rs:541`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L541) |
| 39 | throws when multiple imports require more than one replacement | ported | [`crates/renovate-core/src/extractors/deno.rs:560`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L560) |
| 64 | updates dependency in scopes | ported | [`crates/renovate-core/src/extractors/deno.rs:576`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L576) |
| 98 | returns null when scopes element not found | ported | [`crates/renovate-core/src/extractors/deno.rs:591`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L591) |
| 127 | updates dependency in tasks | ported | [`crates/renovate-core/src/extractors/deno.rs:600`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L600) |
| 158 | updates dependency in tasks.command | ported | [`crates/renovate-core/src/extractors/deno.rs:754`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L754) |
| 191 | returns null when tasks element not found | ported | [`crates/renovate-core/src/extractors/deno.rs:613`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L613) |
| 221 | returns null when tasks.command element not found | ported | [`crates/renovate-core/src/extractors/deno.rs:767`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L767) |
| 251 | updates dependency in compileroptions.types | ported | [`crates/renovate-core/src/extractors/deno.rs:622`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L622) |
| 281 | returns null when compileroptions.types is empty array | ported | [`crates/renovate-core/src/extractors/deno.rs:635`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L635) |
| 308 | returns null when compileroptions.types element not found | ported | [`crates/renovate-core/src/extractors/deno.rs:776`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L776) |
| 335 | updates dependency in compileroptions.jsximportsource | ported | [`crates/renovate-core/src/extractors/deno.rs:644`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L644) |
| 367 | returns null when compileroptions.jsximportsource does not exist | ported | [`crates/renovate-core/src/extractors/deno.rs:791`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L791) |
| 394 | returns null when compileroptions.jsximportsourcetypes does not exist | ported | [`crates/renovate-core/src/extractors/deno.rs:806`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L806) |
| 421 | updates dependency in compileroptions.jsximportsourcetypes | ported | [`crates/renovate-core/src/extractors/deno.rs:821`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L821) |
| 453 | updates dependency in lint plugins | ported | [`crates/renovate-core/src/extractors/deno.rs:665`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L665) |
| 481 | returns null when lint.plugins element not found | ported | [`crates/renovate-core/src/extractors/deno.rs:842`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L842) |
| 508 | returns null when lint.plugins is empty array | ported | [`crates/renovate-core/src/extractors/deno.rs:851`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L851) |
| 535 | handles dependency without version | ported | [`crates/renovate-core/src/extractors/deno.rs:860`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L860) |
| 563 | returns null if packagefile is not defined | ported | [`crates/renovate-core/src/extractors/deno.rs:678`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L678) |
| 575 | returns null for not supported datasource | ported | [`crates/renovate-core/src/extractors/deno.rs:746`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L746) |
| 602 | currentvalue is not defined when deno datasource | ported | [`crates/renovate-core/src/extractors/deno.rs:874`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L874) |
| 629 | returns null for missing required values | ported | [`crates/renovate-core/src/extractors/deno.rs:889`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L889) |
| 648 | handles complex json with nested structures | ported | [`crates/renovate-core/src/extractors/deno.rs:902`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L902) |
| 689 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/deno.rs:694`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L694) |
| 712 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/deno.rs:718`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L718) |
| 731 | handles error during update gracefully | ported | [`crates/renovate-core/src/extractors/deno.rs:732`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L732) |
| 750 | depname is not defined | pending | — |
| 773 | unsupported packagefile | pending | — |
| 792 | replaces only exact matches | pending | — |
| 818 | returns null when deptype is not handled | pending | — |
| 841 | returns null when compileroptions.types does not exist | pending | — |
| 864 | returns null when lint.plugins does not exist | pending | — |
| 889 | updates dependency in imports | ported | [`crates/renovate-core/src/extractors/deno.rs:541`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L541) |
| 916 | handles error during update gracefully | ported | [`crates/renovate-core/src/extractors/deno.rs:732`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L732) |
| 938 | returns null for not supported datasource | ported | [`crates/renovate-core/src/extractors/deno.rs:746`](../../../../../../../crates/renovate-core/src/extractors/deno.rs#L746) |
| 968 | depname is not defined | pending | — |
| 997 | replaces a dependency value | pending | — |

