# `lib/util/common.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**22/22 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 46 | _(it.each / template — verify manually)_ | ? | — |
| 67 | uses host rules | ported | [`crates/renovate-core/src/util.rs:10515`](../../../../../crates/renovate-core/src/util.rs#L10515) |
| 119 | returns null | ported | [`crates/renovate-core/src/util.rs:10595`](../../../../../crates/renovate-core/src/util.rs#L10595) |
| 123 | returns parsed json | ported | [`crates/renovate-core/src/util.rs:10602`](../../../../../crates/renovate-core/src/util.rs#L10602) |
| 131 | supports jsonc | ported | [`crates/renovate-core/src/util.rs:10611`](../../../../../crates/renovate-core/src/util.rs#L10611) |
| 149 | throws error for invalid json | ported | [`crates/renovate-core/src/util.rs:10638`](../../../../../crates/renovate-core/src/util.rs#L10638) |
| 153 | catches and warns if content parsing failed with jsonc.parse but not with json5.parse | ported | [`crates/renovate-core/src/util.rs:10645`](../../../../../crates/renovate-core/src/util.rs#L10645) |
| 167 | does not warn if filename ends with .jsonc | ported | [`crates/renovate-core/src/util.rs:10654`](../../../../../crates/renovate-core/src/util.rs#L10654) |
| 172 | does not warn if filename ends with .json5 | ported | [`crates/renovate-core/src/util.rs:10662`](../../../../../crates/renovate-core/src/util.rs#L10662) |
| 179 | returns parsed jsonc | ported | [`crates/renovate-core/src/util.rs:10623`](../../../../../crates/renovate-core/src/util.rs#L10623) |
| 187 | throws error for invalid jsonc | ported | [`crates/renovate-core/src/util.rs:10670`](../../../../../crates/renovate-core/src/util.rs#L10670) |
| 198 | returns undefined if not set | ported | [`crates/renovate-core/src/util.rs:11587`](../../../../../crates/renovate-core/src/util.rs#L11587) |
| 202 | returns inherited value if only inherited value is set | ported | [`crates/renovate-core/src/util.rs:11594`](../../../../../crates/renovate-core/src/util.rs#L11594) |
| 209 | returns global value if only global value is set | ported | [`crates/renovate-core/src/util.rs:11617`](../../../../../crates/renovate-core/src/util.rs#L11617) |
| 216 | returns inherited value - when both global + inherited are set | ported | [`crates/renovate-core/src/util.rs:11624`](../../../../../crates/renovate-core/src/util.rs#L11624) |
| 227 | handles null inherited values | ported | [`crates/renovate-core/src/util.rs:11601`](../../../../../crates/renovate-core/src/util.rs#L11601) |
| 238 | handles undefined inherited values | ported | [`crates/renovate-core/src/util.rs:11609`](../../../../../crates/renovate-core/src/util.rs#L11609) |
| 249 | returns inherited value when inherited < global | ported | [`crates/renovate-core/src/util.rs:11631`](../../../../../crates/renovate-core/src/util.rs#L11631) |
| 259 | returns global value when inherited > global value | ported | [`crates/renovate-core/src/util.rs:11638`](../../../../../crates/renovate-core/src/util.rs#L11638) |
| 269 | returns inherited value when inherited == global | ported | [`crates/renovate-core/src/util.rs:11645`](../../../../../crates/renovate-core/src/util.rs#L11645) |
| 279 | returns inherited value when global value is not set | ported | [`crates/renovate-core/src/util.rs:11652`](../../../../../crates/renovate-core/src/util.rs#L11652) |
| 289 | returns global value when inherited value is not set | ported | [`crates/renovate-core/src/util.rs:11659`](../../../../../crates/renovate-core/src/util.rs#L11659) |

