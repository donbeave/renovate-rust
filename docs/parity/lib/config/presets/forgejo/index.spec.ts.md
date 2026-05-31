# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/forgejo/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `config/presets/forgejo/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 19 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns JSON5 | 36 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws external host error | 53 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|

### `config/presets/forgejo/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 73 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws if invalid content | 84 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws if fails to parse | 95 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should return default.json | 108 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should query preset within the file | 120 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should query subpreset | 134 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should return custom.json | 151 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should query custom paths | 165 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should throws not-found | 180 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|

### `config/presets/forgejo/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 197 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses custom endpoint | 209 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses default endpoint with a tag | 228 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses custom endpoint with a tag | 246 | not-applicable | Mock framework internals — tests forgejo presets via nock HTTP mocks; Rust tests this at different layer | — | —|

---

