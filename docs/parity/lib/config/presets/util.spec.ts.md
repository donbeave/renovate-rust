# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/presets/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/util.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 16 | not-applicable | — | — | Generic remote preset fetch/deep-preset helper behavior is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote preset references. |
| fails | 37 | not-applicable | — | — | Generic remote preset fetch error propagation is not implemented in Rust. |
| dep not found | 42 | not-applicable | — | — | Generic remote preset dependency-not-found retry/error mapping is not implemented in Rust. |
| preset not found | 54 | not-applicable | — | — | Generic remote preset nested-preset lookup and preset-not-found mapping are not implemented in Rust. |

---

