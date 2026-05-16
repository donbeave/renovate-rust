# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/forgejo/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/forgejo/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 19 | not-applicable | — | — | Forgejo remote preset fetching via JavaScript HTTP contents API is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| returns JSON5 | 36 | not-applicable | — | — | Forgejo remote preset fetching via JavaScript HTTP contents API is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws external host error | 53 | not-applicable | — | — | Forgejo remote preset fetching and host-rule error handling are not implemented in Rust. |

### `config/presets/forgejo/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 73 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if invalid content | 84 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if fails to parse | 95 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return default.json | 108 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file | 120 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query subpreset | 134 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return custom.json | 151 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query custom paths | 165 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should throws not-found | 180 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |

### `config/presets/forgejo/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 197 | not-applicable | — | — | Forgejo remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint | 209 | not-applicable | — | — | Forgejo remote preset endpoint/tag resolution is not implemented in Rust. |
| uses default endpoint with a tag | 228 | not-applicable | — | — | Forgejo remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint with a tag | 246 | not-applicable | — | — | Forgejo remote preset endpoint/tag resolution is not implemented in Rust. |

---

