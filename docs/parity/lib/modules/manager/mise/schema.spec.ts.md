# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mise/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mise/schema.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `modules/manager/mise/schema › MiseFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| defaults tools to empty object when [tools] is absent | 6 | ported | `mise.rs` | `mise_file_no_tools_section_produces_no_deps` | — |
| defaults tools to empty object for empty TOML | 13 | ported | `mise.rs` | `mise_file_empty_toml_produces_no_deps` | — |
| parses [tools] when present | 17 | ported | `mise.rs` | `mise_file_with_tools_section_parses_correctly` | — |
