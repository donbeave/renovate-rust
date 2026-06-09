# `lib/util/fingerprint.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**2/10 in-scope tests ported** (8 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns empty string | ported | [`crates/renovate-core/src/util.rs:7941`](../../../../../crates/renovate-core/src/util.rs#L7941) |
| 32 | maintains deterministic order | ported | [`crates/renovate-core/src/util.rs:7947`](../../../../../crates/renovate-core/src/util.rs#L7947) |
| 39 | _(it.each / template — verify manually)_ | ? | — |
| 58 | returns empty string for root function/symbol | pending | — |
| 63 | drops undefined/function/symbol object values like json.stringify | pending | — |
| 74 | replaces undefined/function/symbol with null in arrays | pending | — |
| 79 | drops object keys whose tojson resolves to undefined | pending | — |
| 84 | renders array items whose tojson resolves to undefined as null | pending | — |
| 89 | handles circular references | pending | — |
| 98 | handles many entries without stack overflow | pending | — |

