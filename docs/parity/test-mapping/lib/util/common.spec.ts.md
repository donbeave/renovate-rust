# `lib/util/common.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**22/22 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | _(it.each / template — verify manually)_ | ? | — |
| 67 | uses host rules | ported | [`crates/renovate-core/src/util.rs:10514`](../../../../../crates/renovate-core/src/util.rs#L10514) |
| 119 | returns null | ported | [`crates/renovate-core/src/util.rs:10594`](../../../../../crates/renovate-core/src/util.rs#L10594) |
| 123 | returns parsed json | ported | [`crates/renovate-core/src/util.rs:10601`](../../../../../crates/renovate-core/src/util.rs#L10601) |
| 131 | supports jsonc | ported | [`crates/renovate-core/src/util.rs:10610`](../../../../../crates/renovate-core/src/util.rs#L10610) |
| 149 | throws error for invalid json | ported | [`crates/renovate-core/src/util.rs:10637`](../../../../../crates/renovate-core/src/util.rs#L10637) |
| 153 | catches and warns if content parsing failed with jsonc.parse but not with json5.parse | ported | [`crates/renovate-core/src/util.rs:10644`](../../../../../crates/renovate-core/src/util.rs#L10644) |
| 167 | does not warn if filename ends with .jsonc | ported | [`crates/renovate-core/src/util.rs:10653`](../../../../../crates/renovate-core/src/util.rs#L10653) |
| 172 | does not warn if filename ends with .json5 | ported | [`crates/renovate-core/src/util.rs:10661`](../../../../../crates/renovate-core/src/util.rs#L10661) |
| 179 | returns parsed jsonc | ported | [`crates/renovate-core/src/util.rs:10622`](../../../../../crates/renovate-core/src/util.rs#L10622) |
| 187 | throws error for invalid jsonc | ported | [`crates/renovate-core/src/util.rs:10669`](../../../../../crates/renovate-core/src/util.rs#L10669) |
| 198 | returns undefined if not set | ported | [`crates/renovate-core/src/util.rs:11586`](../../../../../crates/renovate-core/src/util.rs#L11586) |
| 202 | returns inherited value if only inherited value is set | ported | [`crates/renovate-core/src/util.rs:11593`](../../../../../crates/renovate-core/src/util.rs#L11593) |
| 209 | returns global value if only global value is set | ported | [`crates/renovate-core/src/util.rs:11616`](../../../../../crates/renovate-core/src/util.rs#L11616) |
| 216 | returns inherited value - when both global + inherited are set | ported | [`crates/renovate-core/src/util.rs:11623`](../../../../../crates/renovate-core/src/util.rs#L11623) |
| 227 | handles null inherited values | ported | [`crates/renovate-core/src/util.rs:11600`](../../../../../crates/renovate-core/src/util.rs#L11600) |
| 238 | handles undefined inherited values | ported | [`crates/renovate-core/src/util.rs:11608`](../../../../../crates/renovate-core/src/util.rs#L11608) |
| 249 | returns inherited value when inherited < global | ported | [`crates/renovate-core/src/util.rs:11630`](../../../../../crates/renovate-core/src/util.rs#L11630) |
| 259 | returns global value when inherited > global value | ported | [`crates/renovate-core/src/util.rs:11637`](../../../../../crates/renovate-core/src/util.rs#L11637) |
| 269 | returns inherited value when inherited == global | ported | [`crates/renovate-core/src/util.rs:11644`](../../../../../crates/renovate-core/src/util.rs#L11644) |
| 279 | returns inherited value when global value is not set | ported | [`crates/renovate-core/src/util.rs:11651`](../../../../../crates/renovate-core/src/util.rs#L11651) |
| 289 | returns global value when inherited value is not set | ported | [`crates/renovate-core/src/util.rs:11658`](../../../../../crates/renovate-core/src/util.rs#L11658) |

