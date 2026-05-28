# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/reconfigure/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/reconfigure/utils.spec.ts
**Total tests:** 5 | **Ported:** 1 | **Actionable:** 5 | **Status:** done

### `workers/repository/reconfigure/utils › getReconfigureConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no config file found | 17 | not-applicable | — | — | All tests use vi.mock(fs) and vi.mock(merge) to simulate filesystem and config-detection infrastructure; tests exercise platform file I/O and module wiring, not pure logic |
| handles error while reading reconfigure config file | 26 | not-applicable | — | — | All tests use vi.mock(fs) and vi.mock(merge) to simulate filesystem and config-detection infrastructure; tests exercise platform file I/O and module wiring, not pure logic |
| handles invalid reconfigure config | 36 | not-applicable | — | — | All tests use vi.mock(fs) and vi.mock(merge) to simulate filesystem and config-detection infrastructure; tests exercise platform file I/O and module wiring, not pure logic |
| return config | 51 | not-applicable | — | — | All tests use vi.mock(fs) and vi.mock(merge) to simulate filesystem and config-detection infrastructure; tests exercise platform file I/O and module wiring, not pure logic |

### `workers/repository/reconfigure/utils › getReconfigureBranchName()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns reconfigure branch name | 64 | ported | `util.rs` | `test_get_reconfigure_branch_name` | — |

---

