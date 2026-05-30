# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/puppet/puppetfile-parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/puppet/puppetfile-parser.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `parsePuppetfile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Puppetfile_github_tag | 9 | not-applicable | — | — | Tests internal Puppetfile object model (getModulesOfForge/getForges); Rust extract() produces same PuppetDep output, covered by existing 16 puppet.rs tests |
| Puppetfile_github_tag_single_line | 31 | not-applicable | — | — | Same internal object model; single-line git/tag parsing covered by puppet.rs |
| Puppetfile with an invalid module creates PuppetfileModule with skipReason "invalid-config" | 58 | not-applicable | — | — | Invalid-config skip reason tested in puppet.rs extract tests |
| get default forge with null or undefined returns the same | 74 | not-applicable | — | — | Tests TS null/undefined handling of forge lookup; Rust uses Option<String> |
| Puppetfile_multiple_forges | 88 | not-applicable | — | — | Multiple forge parsing covered by puppet.rs extract tests |
| Puppetfile_no_forge | 133 | not-applicable | — | — | Default forge behavior covered by puppet.rs extract tests |
| Puppetfile_single_forge | 161 | not-applicable | — | — | Single forge parsing covered by puppet.rs extract tests |
| Puppetfile_with_comments | 192 | not-applicable | — | — | Comment handling covered by puppet.rs extract tests |

---
