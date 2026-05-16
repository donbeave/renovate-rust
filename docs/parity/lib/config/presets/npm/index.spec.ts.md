# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/npm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/npm/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/npm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no package | 10 | not-applicable | — | — | npm-hosted preset package fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/npm preset references. |
| should throw if no renovate-config | 17 | not-applicable | — | — | npm-hosted preset package fetching and renovate-config package parsing are not implemented in Rust. |
| should throw if preset name not found | 48 | not-applicable | — | — | npm-hosted preset package fetching and preset-name lookup are not implemented in Rust. |
| should return preset | 83 | not-applicable | — | — | npm-hosted preset package fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/npm preset references. |

---

