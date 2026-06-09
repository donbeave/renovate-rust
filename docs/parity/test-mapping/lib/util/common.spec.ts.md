# `lib/util/common.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**22/22 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | _(it.each / template — verify manually)_ | ? | — |
| 67 | uses host rules | ported | [`crates/renovate-core/src/util.rs:10611`](../../../../../crates/renovate-core/src/util.rs#L10611) |
| 119 | returns null | ported | [`crates/renovate-core/src/util.rs:10691`](../../../../../crates/renovate-core/src/util.rs#L10691) |
| 123 | returns parsed json | ported | [`crates/renovate-core/src/util.rs:10698`](../../../../../crates/renovate-core/src/util.rs#L10698) |
| 131 | supports jsonc | ported | [`crates/renovate-core/src/util.rs:10707`](../../../../../crates/renovate-core/src/util.rs#L10707) |
| 149 | throws error for invalid json | ported | [`crates/renovate-core/src/util.rs:10734`](../../../../../crates/renovate-core/src/util.rs#L10734) |
| 153 | catches and warns if content parsing failed with jsonc.parse but not with json5.parse | ported | [`crates/renovate-core/src/util.rs:10741`](../../../../../crates/renovate-core/src/util.rs#L10741) |
| 167 | does not warn if filename ends with .jsonc | ported | [`crates/renovate-core/src/util.rs:10750`](../../../../../crates/renovate-core/src/util.rs#L10750) |
| 172 | does not warn if filename ends with .json5 | ported | [`crates/renovate-core/src/util.rs:10758`](../../../../../crates/renovate-core/src/util.rs#L10758) |
| 179 | returns parsed jsonc | ported | [`crates/renovate-core/src/util.rs:10719`](../../../../../crates/renovate-core/src/util.rs#L10719) |
| 187 | throws error for invalid jsonc | ported | [`crates/renovate-core/src/util.rs:10766`](../../../../../crates/renovate-core/src/util.rs#L10766) |
| 198 | returns undefined if not set | ported | [`crates/renovate-core/src/util.rs:11683`](../../../../../crates/renovate-core/src/util.rs#L11683) |
| 202 | returns inherited value if only inherited value is set | ported | [`crates/renovate-core/src/util.rs:11690`](../../../../../crates/renovate-core/src/util.rs#L11690) |
| 209 | returns global value if only global value is set | ported | [`crates/renovate-core/src/util.rs:11713`](../../../../../crates/renovate-core/src/util.rs#L11713) |
| 216 | returns inherited value - when both global + inherited are set | ported | [`crates/renovate-core/src/util.rs:11720`](../../../../../crates/renovate-core/src/util.rs#L11720) |
| 227 | handles null inherited values | ported | [`crates/renovate-core/src/util.rs:11697`](../../../../../crates/renovate-core/src/util.rs#L11697) |
| 238 | handles undefined inherited values | ported | [`crates/renovate-core/src/util.rs:11705`](../../../../../crates/renovate-core/src/util.rs#L11705) |
| 249 | returns inherited value when inherited < global | ported | [`crates/renovate-core/src/util.rs:11727`](../../../../../crates/renovate-core/src/util.rs#L11727) |
| 259 | returns global value when inherited > global value | ported | [`crates/renovate-core/src/util.rs:11734`](../../../../../crates/renovate-core/src/util.rs#L11734) |
| 269 | returns inherited value when inherited == global | ported | [`crates/renovate-core/src/util.rs:11741`](../../../../../crates/renovate-core/src/util.rs#L11741) |
| 279 | returns inherited value when global value is not set | ported | [`crates/renovate-core/src/util.rs:11748`](../../../../../crates/renovate-core/src/util.rs#L11748) |
| 289 | returns global value when inherited value is not set | ported | [`crates/renovate-core/src/util.rs:11755`](../../../../../crates/renovate-core/src/util.rs#L11755) |

