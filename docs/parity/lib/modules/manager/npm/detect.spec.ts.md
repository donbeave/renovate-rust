# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/npm/detect.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/detect.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** done

### `.detectGlobalConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects .npmrc in home directory | 8 | ported | `extractors/npm.rs` | `detect_global_config_reads_npmrc` | — |
| handles no .npmrc | 24 | ported | `extractors/npm.rs` | `detect_global_config_no_npmrc` | — |

---

