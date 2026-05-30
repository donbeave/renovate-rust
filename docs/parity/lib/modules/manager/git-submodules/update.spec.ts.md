# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/git-submodules/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/git-submodules/update.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** pending

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null on error | 49 | pending | — | — | git submodule update error behavior is in scope |
| returns content on update | 60 | pending | — | — | git submodule update behavior is in scope |
| returns content on update and uses git environment variables | 72 | pending | — | — | git submodule host-rule auth behavior is in scope |
| update gitmodule branch value if value changed | 107 | pending | — | — | `.gitmodules` branch update behavior is in scope |
| do not update gitmodule branch value if value not changed | 136 | pending | — | — | `.gitmodules` branch no-op behavior is in scope |
| returns content on update and uses git environment variables for git-tags/git-refs | 154 | pending | — | — | git-tags/git-refs host-rule auth behavior is in scope |

---
