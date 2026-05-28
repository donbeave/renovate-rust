# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/local/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/local/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `config/presets/local/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for unsupported platform | 34 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to azure | 59 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to bitbucket | 77 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to gerrit | 95 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to custom bitbucket-server | 113 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to gitea | 131 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to forgejo | 149 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to custom gitea | 167 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to custom forgejo | 186 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to github | 205 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to custom github | 223 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to github with a tag | 243 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to custom github with a tag | 262 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to gitlab | 283 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to custom gitlab | 302 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to gitlab with a tag | 322 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| forwards to custom gitlab with a tag | 340 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |
| throws for platform that does not support local presets | 361 | not-applicable | — | — | Uses vi.mock(platform presets) + GlobalConfig; platform/module mock infrastructure not portable to Rust |

---

