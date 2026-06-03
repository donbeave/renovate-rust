# `lib/util/common.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**18/22 ported** (4 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 46 | _(it.each / template — verify manually)_ | ? | — |
| 67 | uses host rules | ported | `crates/renovate-core/src/util.rs:8959` |
| 119 | returns null | ported | `crates/renovate-core/src/util.rs:9039` |
| 123 | returns parsed json | ported | `crates/renovate-core/src/util.rs:9046` |
| 131 | supports jsonc | ported | `crates/renovate-core/src/util.rs:9055` |
| 149 | throws error for invalid json | ported | `crates/renovate-core/src/util.rs:9067` |
| 153 | catches and warns if content parsing failed with jsonc.parse but not with json5.parse | ported | `crates/renovate-core/src/util.rs:9074` |
| 167 | does not warn if filename ends with .jsonc | ported | `crates/renovate-core/src/util.rs:9083` |
| 172 | does not warn if filename ends with .json5 | ported | `crates/renovate-core/src/util.rs:9091` |
| 179 | returns parsed jsonc | pending | — |
| 187 | throws error for invalid jsonc | pending | — |
| 198 | returns undefined if not set | ported | `crates/renovate-core/src/util.rs:9981` |
| 202 | returns inherited value if only inherited value is set | ported | `crates/renovate-core/src/util.rs:9988` |
| 209 | returns global value if only global value is set | ported | `crates/renovate-core/src/util.rs:9995` |
| 216 | returns inherited value - when both global + inherited are set | ported | `crates/renovate-core/src/util.rs:10002` |
| 227 | handles null inherited values | pending | — |
| 238 | handles undefined inherited values | pending | — |
| 249 | returns inherited value when inherited < global | ported | `crates/renovate-core/src/util.rs:10009` |
| 259 | returns global value when inherited > global value | ported | `crates/renovate-core/src/util.rs:10016` |
| 269 | returns inherited value when inherited == global | ported | `crates/renovate-core/src/util.rs:10023` |
| 279 | returns inherited value when global value is not set | ported | `crates/renovate-core/src/util.rs:10030` |
| 289 | returns global value when inherited value is not set | ported | `crates/renovate-core/src/util.rs:10037` |

