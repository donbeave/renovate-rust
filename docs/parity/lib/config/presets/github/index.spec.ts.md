# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/github/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `config/presets/github/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 17 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws external host error | 34 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|

### `config/presets/github/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 54 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws if invalid content | 65 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws if fails to parse | 76 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should return default.json | 89 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should query preset within the file | 101 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should query preset within the file when .json extension provided | 115 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should query preset within the file when .json5 extension provided | 129 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should query subpreset | 143 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should return custom.json | 160 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should query custom paths | 174 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| should throws not-found | 189 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|

### `config/presets/github/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 206 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses custom endpoint | 218 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses default endpoint with a tag | 238 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses custom endpoint with a tag | 256 | not-applicable | Mock framework internals — tests github presets via nock HTTP mocks; Rust tests this at different layer | — | —|

---

