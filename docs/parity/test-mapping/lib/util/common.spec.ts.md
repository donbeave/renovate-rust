# `lib/util/common.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**22/22 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | _(it.each / template — verify manually)_ | ? | — |
| 67 | uses host rules | ported | [`crates/renovate-core/src/util.rs:10529`](../../../../../crates/renovate-core/src/util.rs#L10529) |
| 119 | returns null | ported | [`crates/renovate-core/src/util.rs:10609`](../../../../../crates/renovate-core/src/util.rs#L10609) |
| 123 | returns parsed json | ported | [`crates/renovate-core/src/util.rs:10616`](../../../../../crates/renovate-core/src/util.rs#L10616) |
| 131 | supports jsonc | ported | [`crates/renovate-core/src/util.rs:10625`](../../../../../crates/renovate-core/src/util.rs#L10625) |
| 149 | throws error for invalid json | ported | [`crates/renovate-core/src/util.rs:10652`](../../../../../crates/renovate-core/src/util.rs#L10652) |
| 153 | catches and warns if content parsing failed with jsonc.parse but not with json5.parse | ported | [`crates/renovate-core/src/util.rs:10659`](../../../../../crates/renovate-core/src/util.rs#L10659) |
| 167 | does not warn if filename ends with .jsonc | ported | [`crates/renovate-core/src/util.rs:10668`](../../../../../crates/renovate-core/src/util.rs#L10668) |
| 172 | does not warn if filename ends with .json5 | ported | [`crates/renovate-core/src/util.rs:10676`](../../../../../crates/renovate-core/src/util.rs#L10676) |
| 179 | returns parsed jsonc | ported | [`crates/renovate-core/src/util.rs:10637`](../../../../../crates/renovate-core/src/util.rs#L10637) |
| 187 | throws error for invalid jsonc | ported | [`crates/renovate-core/src/util.rs:10684`](../../../../../crates/renovate-core/src/util.rs#L10684) |
| 198 | returns undefined if not set | ported | [`crates/renovate-core/src/util.rs:11601`](../../../../../crates/renovate-core/src/util.rs#L11601) |
| 202 | returns inherited value if only inherited value is set | ported | [`crates/renovate-core/src/util.rs:11608`](../../../../../crates/renovate-core/src/util.rs#L11608) |
| 209 | returns global value if only global value is set | ported | [`crates/renovate-core/src/util.rs:11631`](../../../../../crates/renovate-core/src/util.rs#L11631) |
| 216 | returns inherited value - when both global + inherited are set | ported | [`crates/renovate-core/src/util.rs:11638`](../../../../../crates/renovate-core/src/util.rs#L11638) |
| 227 | handles null inherited values | ported | [`crates/renovate-core/src/util.rs:11615`](../../../../../crates/renovate-core/src/util.rs#L11615) |
| 238 | handles undefined inherited values | ported | [`crates/renovate-core/src/util.rs:11623`](../../../../../crates/renovate-core/src/util.rs#L11623) |
| 249 | returns inherited value when inherited < global | ported | [`crates/renovate-core/src/util.rs:11645`](../../../../../crates/renovate-core/src/util.rs#L11645) |
| 259 | returns global value when inherited > global value | ported | [`crates/renovate-core/src/util.rs:11652`](../../../../../crates/renovate-core/src/util.rs#L11652) |
| 269 | returns inherited value when inherited == global | ported | [`crates/renovate-core/src/util.rs:11659`](../../../../../crates/renovate-core/src/util.rs#L11659) |
| 279 | returns inherited value when global value is not set | ported | [`crates/renovate-core/src/util.rs:11666`](../../../../../crates/renovate-core/src/util.rs#L11666) |
| 289 | returns global value when inherited value is not set | ported | [`crates/renovate-core/src/util.rs:11673`](../../../../../crates/renovate-core/src/util.rs#L11673) |

