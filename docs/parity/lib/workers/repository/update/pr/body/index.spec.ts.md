# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/body/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/body/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 8 | **Status:** pending

### `workers/repository/update/pr/body/index › getPrBody`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty template | 53 | pending | — | — | Requires Handlebars PR body template compilation |
| massages upgrades | 73 | pending | — | — | Requires upgrade massage logic and template compilation |
| templates changelogUrl | 177 | pending | — | — | Requires changelog URL template variable |
| uses dependencyUrl as primary link | 225 | pending | — | — | Requires dependency URL linking in table |
| compiles template | 257 | pending | — | — | Requires Handlebars template engine |
| supports custom rebasing message | 281 | pending | — | — | Requires rebase message template support |
| updates PR due to body change without pr data | 305 | pending | — | — | Requires PR update detection logic |
| pr body warning | 330 | pending | — | — | Requires PR body warning template support |

---
