# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/extract.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 36 | ported | `bundler.rs` | `extract_returns_none_for_empty` | — |
| parses rails Gemfile | 40 | ported | `bundler.rs` | `extract_parses_rails_gemfile` | — |
| parses sourceGroups | 57 | ported | `bundler.rs` | `extract_parses_source_groups` | — |
| parse webpacker Gemfile | 63 | ported | `bundler.rs` | `extract_parses_webpacker_gemfile` | — |
| parse mastodon Gemfile | 75 | ported | `bundler.rs` | `extract_parses_mastodon_gemfile` | — |
| parse Ruby CI Gemfile | 91 | ported | `bundler.rs` | `extract_parses_rubyci_gemfile` | — |
| parse Gitlab Foss Gemfile | 104 | ported | `bundler.rs` | `extract_parses_gitlab_foss_gemfile` | — |
| parse source blocks in Gemfile | 116 | ported | `bundler.rs` | `extract_parses_source_block_gemfile` | — |
| parse source blocks with spaces in Gemfile | 122 | ported | `bundler.rs` | `extract_parses_source_block_with_new_lines` | — |
| parses source blocks with groups in Gemfile | 132 | ported | `bundler.rs` | `extract_parses_source_block_with_groups` | — |
| parses source variable in Gemfile | 146 | ported | `bundler.rs` | `extract_parses_source_variable` | — |
| parses inline source in Gemfile | 171 | ported | `bundler.rs` | `extract_parses_inline_source` | — |
| parses git refs in Gemfile | 223 | ported | `bundler.rs` | `extract_parses_git_refs` | — |
| parses multiple current values Gemfile | 259 | ported | `bundler.rs` | `extract_parses_multiple_current_values` | — |
| skips local gems in Gemfile | 284 | ported | `bundler.rs` | `extract_skips_local_gems` | — |

---

