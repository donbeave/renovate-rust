# `lib/util/common.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**22/22 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | _(it.each / template — verify manually)_ | ? | — |
| 67 | uses host rules | ported | [`crates/renovate-core/src/util.rs:10516`](../../../../../crates/renovate-core/src/util.rs#L10516) |
| 119 | returns null | ported | [`crates/renovate-core/src/util.rs:10596`](../../../../../crates/renovate-core/src/util.rs#L10596) |
| 123 | returns parsed json | ported | [`crates/renovate-core/src/util.rs:10603`](../../../../../crates/renovate-core/src/util.rs#L10603) |
| 131 | supports jsonc | ported | [`crates/renovate-core/src/util.rs:10612`](../../../../../crates/renovate-core/src/util.rs#L10612) |
| 149 | throws error for invalid json | ported | [`crates/renovate-core/src/util.rs:10639`](../../../../../crates/renovate-core/src/util.rs#L10639) |
| 153 | catches and warns if content parsing failed with jsonc.parse but not with json5.parse | ported | [`crates/renovate-core/src/util.rs:10646`](../../../../../crates/renovate-core/src/util.rs#L10646) |
| 167 | does not warn if filename ends with .jsonc | ported | [`crates/renovate-core/src/util.rs:10655`](../../../../../crates/renovate-core/src/util.rs#L10655) |
| 172 | does not warn if filename ends with .json5 | ported | [`crates/renovate-core/src/util.rs:10663`](../../../../../crates/renovate-core/src/util.rs#L10663) |
| 179 | returns parsed jsonc | ported | [`crates/renovate-core/src/util.rs:10624`](../../../../../crates/renovate-core/src/util.rs#L10624) |
| 187 | throws error for invalid jsonc | ported | [`crates/renovate-core/src/util.rs:10671`](../../../../../crates/renovate-core/src/util.rs#L10671) |
| 198 | returns undefined if not set | ported | [`crates/renovate-core/src/util.rs:11588`](../../../../../crates/renovate-core/src/util.rs#L11588) |
| 202 | returns inherited value if only inherited value is set | ported | [`crates/renovate-core/src/util.rs:11595`](../../../../../crates/renovate-core/src/util.rs#L11595) |
| 209 | returns global value if only global value is set | ported | [`crates/renovate-core/src/util.rs:11618`](../../../../../crates/renovate-core/src/util.rs#L11618) |
| 216 | returns inherited value - when both global + inherited are set | ported | [`crates/renovate-core/src/util.rs:11625`](../../../../../crates/renovate-core/src/util.rs#L11625) |
| 227 | handles null inherited values | ported | [`crates/renovate-core/src/util.rs:11602`](../../../../../crates/renovate-core/src/util.rs#L11602) |
| 238 | handles undefined inherited values | ported | [`crates/renovate-core/src/util.rs:11610`](../../../../../crates/renovate-core/src/util.rs#L11610) |
| 249 | returns inherited value when inherited < global | ported | [`crates/renovate-core/src/util.rs:11632`](../../../../../crates/renovate-core/src/util.rs#L11632) |
| 259 | returns global value when inherited > global value | ported | [`crates/renovate-core/src/util.rs:11639`](../../../../../crates/renovate-core/src/util.rs#L11639) |
| 269 | returns inherited value when inherited == global | ported | [`crates/renovate-core/src/util.rs:11646`](../../../../../crates/renovate-core/src/util.rs#L11646) |
| 279 | returns inherited value when global value is not set | ported | [`crates/renovate-core/src/util.rs:11653`](../../../../../crates/renovate-core/src/util.rs#L11653) |
| 289 | returns global value when inherited value is not set | ported | [`crates/renovate-core/src/util.rs:11660`](../../../../../crates/renovate-core/src/util.rs#L11660) |

