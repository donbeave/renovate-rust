# `lib/util/common.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**22/22 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | _(it.each / template — verify manually)_ | ? | — |
| 67 | uses host rules | ported | [`crates/renovate-core/src/util.rs:10519`](../../../../../crates/renovate-core/src/util.rs#L10519) |
| 119 | returns null | ported | [`crates/renovate-core/src/util.rs:10599`](../../../../../crates/renovate-core/src/util.rs#L10599) |
| 123 | returns parsed json | ported | [`crates/renovate-core/src/util.rs:10606`](../../../../../crates/renovate-core/src/util.rs#L10606) |
| 131 | supports jsonc | ported | [`crates/renovate-core/src/util.rs:10615`](../../../../../crates/renovate-core/src/util.rs#L10615) |
| 149 | throws error for invalid json | ported | [`crates/renovate-core/src/util.rs:10642`](../../../../../crates/renovate-core/src/util.rs#L10642) |
| 153 | catches and warns if content parsing failed with jsonc.parse but not with json5.parse | ported | [`crates/renovate-core/src/util.rs:10649`](../../../../../crates/renovate-core/src/util.rs#L10649) |
| 167 | does not warn if filename ends with .jsonc | ported | [`crates/renovate-core/src/util.rs:10658`](../../../../../crates/renovate-core/src/util.rs#L10658) |
| 172 | does not warn if filename ends with .json5 | ported | [`crates/renovate-core/src/util.rs:10666`](../../../../../crates/renovate-core/src/util.rs#L10666) |
| 179 | returns parsed jsonc | ported | [`crates/renovate-core/src/util.rs:10627`](../../../../../crates/renovate-core/src/util.rs#L10627) |
| 187 | throws error for invalid jsonc | ported | [`crates/renovate-core/src/util.rs:10674`](../../../../../crates/renovate-core/src/util.rs#L10674) |
| 198 | returns undefined if not set | ported | [`crates/renovate-core/src/util.rs:11591`](../../../../../crates/renovate-core/src/util.rs#L11591) |
| 202 | returns inherited value if only inherited value is set | ported | [`crates/renovate-core/src/util.rs:11598`](../../../../../crates/renovate-core/src/util.rs#L11598) |
| 209 | returns global value if only global value is set | ported | [`crates/renovate-core/src/util.rs:11621`](../../../../../crates/renovate-core/src/util.rs#L11621) |
| 216 | returns inherited value - when both global + inherited are set | ported | [`crates/renovate-core/src/util.rs:11628`](../../../../../crates/renovate-core/src/util.rs#L11628) |
| 227 | handles null inherited values | ported | [`crates/renovate-core/src/util.rs:11605`](../../../../../crates/renovate-core/src/util.rs#L11605) |
| 238 | handles undefined inherited values | ported | [`crates/renovate-core/src/util.rs:11613`](../../../../../crates/renovate-core/src/util.rs#L11613) |
| 249 | returns inherited value when inherited < global | ported | [`crates/renovate-core/src/util.rs:11635`](../../../../../crates/renovate-core/src/util.rs#L11635) |
| 259 | returns global value when inherited > global value | ported | [`crates/renovate-core/src/util.rs:11642`](../../../../../crates/renovate-core/src/util.rs#L11642) |
| 269 | returns inherited value when inherited == global | ported | [`crates/renovate-core/src/util.rs:11649`](../../../../../crates/renovate-core/src/util.rs#L11649) |
| 279 | returns inherited value when global value is not set | ported | [`crates/renovate-core/src/util.rs:11656`](../../../../../crates/renovate-core/src/util.rs#L11656) |
| 289 | returns global value when inherited value is not set | ported | [`crates/renovate-core/src/util.rs:11663`](../../../../../crates/renovate-core/src/util.rs#L11663) |

