# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/local/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/local/index.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/local/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for unsupported platform | 34 | not-applicable | — | — | Platform-dispatched local preset fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| throws for missing platform | 47 | not-applicable | — | — | Platform-dispatched local preset fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to azure | 59 | not-applicable | — | — | Azure local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to bitbucket | 77 | not-applicable | — | — | Bitbucket local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to gerrit | 95 | not-applicable | — | — | Gerrit local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom bitbucket-server | 113 | not-applicable | — | — | Bitbucket Server local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to gitea | 131 | not-applicable | — | — | Gitea local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to forgejo | 149 | not-applicable | — | — | Forgejo local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom gitea | 167 | not-applicable | — | — | Custom-endpoint Gitea local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom forgejo | 186 | not-applicable | — | — | Custom-endpoint Forgejo local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to github | 205 | not-applicable | — | — | GitHub local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom github | 223 | not-applicable | — | — | Custom-endpoint GitHub local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to github with a tag | 243 | not-applicable | — | — | Tagged GitHub local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom github with a tag | 262 | not-applicable | — | — | Tagged custom-endpoint GitHub local preset file fetching is not implemented in Rust. |
| forwards to gitlab | 283 | not-applicable | — | — | GitLab local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom gitlab | 302 | not-applicable | — | — | Custom-endpoint GitLab local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to gitlab with a tag | 322 | not-applicable | — | — | Tagged GitLab local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom gitlab with a tag | 340 | not-applicable | — | — | Tagged custom-endpoint GitLab local preset file fetching is not implemented in Rust. |
| throws for platform that does not support local presets | 361 | not-applicable | — | — | Platform-dispatched local preset fetching and unsupported-platform errors are not implemented in Rust. |

---

