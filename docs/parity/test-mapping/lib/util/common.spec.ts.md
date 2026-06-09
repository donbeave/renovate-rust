# `lib/util/common.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**22/22 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | _(it.each / template — verify manually)_ | ? | — |
| 67 | uses host rules | ported | [`crates/renovate-core/src/util.rs:10513`](../../../../../crates/renovate-core/src/util.rs#L10513) |
| 119 | returns null | ported | [`crates/renovate-core/src/util.rs:10593`](../../../../../crates/renovate-core/src/util.rs#L10593) |
| 123 | returns parsed json | ported | [`crates/renovate-core/src/util.rs:10600`](../../../../../crates/renovate-core/src/util.rs#L10600) |
| 131 | supports jsonc | ported | [`crates/renovate-core/src/util.rs:10609`](../../../../../crates/renovate-core/src/util.rs#L10609) |
| 149 | throws error for invalid json | ported | [`crates/renovate-core/src/util.rs:10636`](../../../../../crates/renovate-core/src/util.rs#L10636) |
| 153 | catches and warns if content parsing failed with jsonc.parse but not with json5.parse | ported | [`crates/renovate-core/src/util.rs:10643`](../../../../../crates/renovate-core/src/util.rs#L10643) |
| 167 | does not warn if filename ends with .jsonc | ported | [`crates/renovate-core/src/util.rs:10652`](../../../../../crates/renovate-core/src/util.rs#L10652) |
| 172 | does not warn if filename ends with .json5 | ported | [`crates/renovate-core/src/util.rs:10660`](../../../../../crates/renovate-core/src/util.rs#L10660) |
| 179 | returns parsed jsonc | ported | [`crates/renovate-core/src/util.rs:10621`](../../../../../crates/renovate-core/src/util.rs#L10621) |
| 187 | throws error for invalid jsonc | ported | [`crates/renovate-core/src/util.rs:10668`](../../../../../crates/renovate-core/src/util.rs#L10668) |
| 198 | returns undefined if not set | ported | [`crates/renovate-core/src/util.rs:11585`](../../../../../crates/renovate-core/src/util.rs#L11585) |
| 202 | returns inherited value if only inherited value is set | ported | [`crates/renovate-core/src/util.rs:11592`](../../../../../crates/renovate-core/src/util.rs#L11592) |
| 209 | returns global value if only global value is set | ported | [`crates/renovate-core/src/util.rs:11615`](../../../../../crates/renovate-core/src/util.rs#L11615) |
| 216 | returns inherited value - when both global + inherited are set | ported | [`crates/renovate-core/src/util.rs:11622`](../../../../../crates/renovate-core/src/util.rs#L11622) |
| 227 | handles null inherited values | ported | [`crates/renovate-core/src/util.rs:11599`](../../../../../crates/renovate-core/src/util.rs#L11599) |
| 238 | handles undefined inherited values | ported | [`crates/renovate-core/src/util.rs:11607`](../../../../../crates/renovate-core/src/util.rs#L11607) |
| 249 | returns inherited value when inherited < global | ported | [`crates/renovate-core/src/util.rs:11629`](../../../../../crates/renovate-core/src/util.rs#L11629) |
| 259 | returns global value when inherited > global value | ported | [`crates/renovate-core/src/util.rs:11636`](../../../../../crates/renovate-core/src/util.rs#L11636) |
| 269 | returns inherited value when inherited == global | ported | [`crates/renovate-core/src/util.rs:11643`](../../../../../crates/renovate-core/src/util.rs#L11643) |
| 279 | returns inherited value when global value is not set | ported | [`crates/renovate-core/src/util.rs:11650`](../../../../../crates/renovate-core/src/util.rs#L11650) |
| 289 | returns global value when inherited value is not set | ported | [`crates/renovate-core/src/util.rs:11657`](../../../../../crates/renovate-core/src/util.rs#L11657) |

