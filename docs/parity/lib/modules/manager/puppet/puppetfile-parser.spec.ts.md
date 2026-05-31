# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/puppet/puppetfile-parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/puppet/puppetfile-parser.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `parsePuppetfile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Puppetfile_github_tag | 9 | done |  | extract tests | Rust extract() produces same PuppetDep output, covered by existing puppet.rs tests |
| Puppetfile_github_tag_single_line | 31 | done |  | extract tests | Single-line git/tag parsing covered by puppet.rs |
| Puppetfile with an invalid module creates PuppetfileModule with skipReason "invalid-config" | 58 | done |  | extract tests | Invalid-config skip reason tested in puppet.rs extract tests |
| get default forge with null or undefined returns the same | 74 | not-applicable | — | — | TypeScript type-system test; null/undefined handling has no Rust equivalent (uses Option<T>) |
| Puppetfile_multiple_forges | 88 | done |  | extract tests | Multiple forge parsing covered by puppet.rs extract tests |
| Puppetfile_no_forge | 133 | done |  | extract tests | Default forge behavior covered by puppet.rs extract tests |
| Puppetfile_single_forge | 161 | done |  | extract tests | Single forge parsing covered by puppet.rs extract tests |
| Puppetfile_with_comments | 192 | done |  | extract tests | Comment handling covered by puppet.rs extract tests |

---
