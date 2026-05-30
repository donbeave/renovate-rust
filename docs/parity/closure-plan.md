# Test Parity Closure Plan

## Current State (as of latest iteration)

| Metric | Count |
|--------|-------|
| Total Rust `#[test]` attributes | 6,504 |
| `// Ported:` comments | 5,084 |
| `// Rust-specific:` comments | 1,420 |
| **Unattributed tests** | **0** |
| Pending spec tests | 4,020 across 179 files |
| NA tests | 2,321 (19.9%) |

## Phase Breakdown

### Phase 0: Attribution Backfill ✅ COMPLETE

All 6,504 Rust tests now have attribution:
- `// Ported:` — links test to upstream Renovate TS spec
- `// Rust-specific:` — documents Rust-native behavior test

**Files processed:** 138+ files including all major modules:
- `repo_config.rs` (605 tests)
- `branch.rs` (125 tests)
- `schedule.rs` (123 tests)
- `config_env.rs`, `string_match.rs`, `migrate.rs`, `cli.rs`, `output.rs`
- All extractor, datasource, platform, and utility modules

### Phase 3: Pending Test Porting (TARGET: minimize pending)

**Criteria for porting priority:**
1. Spec files where Rust implementation exists and is substantial
2. Tests that are unit-testable (don't require unimplemented integrations)
3. Files with highest pending / lowest NA ratio

**High-value targets to investigate:**
- `lib/util/http/index.spec.ts` — 4 actionable pending (HTTP client features exist)
- `lib/modules/datasource/npm/index.spec.ts` — 18 actionable pending (npm datasource has tests)
- Various manager extractors with partial coverage

### Phase 2: Coverage Mapping (continuous)

After each batch of ports, re-run `verify_parity.py` and update counts.

### Terminal State Definition

Parity is "closed" when:
1. All Rust tests have `// Ported:` or `// Rust-specific:` attribution ✅
2. `verify_parity.py` reports 0 genuinely missing ✅
3. All spec files marked `done` have cross-checked source coverage
4. NA ratio stays under 25%

## Execution Notes

- **Commit frequency:** After every coherent unit of work
- **Quality gates:** `cargo test -p renovate-core --lib` must pass before each push
- **Tooling:** Use `awk`/`python` scripts for bulk attribution where safe
