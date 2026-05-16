# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/github/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/github/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 17 | not-applicable | — | — | GitHub remote preset fetching via JavaScript contents API is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws external host error | 34 | not-applicable | — | — | GitHub remote preset fetching and host-rule error handling are not implemented in Rust. |

### `config/presets/github/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 54 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if invalid content | 65 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if fails to parse | 76 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return default.json | 89 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file | 101 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file when .json extension provided | 115 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file when .json5 extension provided | 129 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query subpreset | 143 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return custom.json | 160 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query custom paths | 174 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should throws not-found | 189 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |

### `config/presets/github/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 206 | not-applicable | — | — | GitHub remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint | 218 | not-applicable | — | — | GitHub remote preset endpoint/tag resolution is not implemented in Rust. |
| uses default endpoint with a tag | 238 | not-applicable | — | — | GitHub remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint with a tag | 256 | not-applicable | — | — | GitHub remote preset endpoint/tag resolution is not implemented in Rust. |

---

