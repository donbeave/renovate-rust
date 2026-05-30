# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pep723/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep723/utils.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `parsePep723()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract dependencies | 6 | ported | `pep723.rs` | `pep723_extract_should_extract_dependencies` | — |
| should skip invalid dependencies | 42 | ported | `pep723.rs` | `pep723_extract_should_skip_invalid_dependencies` | — |
| should return null on missing dependencies | 71 | ported | `pep723.rs` | `pep723_extract_returns_none_on_missing_dependencies` | — |
| should return null on invalid TOML | 84 | ported | `pep723.rs` | `pep723_extract_returns_none_on_invalid_toml` | — |
| should return null if there is no PEP 723 metadata | 101 | ported | `pep723.rs` | `pep723_extract_returns_none_if_no_metadata_block` | — |

---

