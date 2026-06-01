# Renovate-Rust Implementation & Test Coverage Audit

**Date:** 2026-05-31
**Upstream reference:** `../renovate` (main branch)
**Rust codebase:** `renovate-rust/`

---

## Executive Summary

The overall **test coverage** (measured by `// Ported:` comments matching upstream
`it()` descriptions) is **49.6%** (5,787 / 11,670 distinct upstream tests).
This is the number that shows up in `parity_coverage.py report`.

**However, implementation coverage is significantly higher:**

| Metric | Count | % of 292 modules |
|--------|-------|-----------------|
| `full` implementation | 209 | 71.6% |
| `partial` implementation | 47 | 16.1% |
| `stub` (mostly empty) | 4 | 1.4% |
| `none` (missing entirely) | 32 | 11.0% |

**~72% of modules have substantial Rust code.** The 50% test-coverage figure is
low because many implemented modules have few or no `// Ported:` test
annotations — not because the code is missing.

---

## Why the gap between implementation and test coverage?

The coverage tool counts **only** `// Ported:` comments. It does not inspect
Rust source files for actual logic. This means:

- A module with 10,000 lines of working Rust code and zero `// Ported:` comments
  shows **0% coverage**.
- A module with 50 lines of stubs and 25 `// Ported:` comments shows **100%
  coverage**.

Both metrics are useful, but they measure different things:

| What you care about | Look at |
|---------------------|---------|
| "Is the code written?" | `Impl` column in `modules.md` (or `scripts/impl_audit.py`) |
| "Are the tests ported?" | `Coverage` column in `modules.md` (from `scripts/parity_coverage.py`) |

---

## Category breakdown

### Managers — 117 modules

| Status | Count | Code lines (approx) |
|--------|-------|---------------------|
| `full` | 79 | ~84,000 |
| `partial` | 19 | — |
| `stub` | 3 | — |
| `none` | 16 | — |

**Key gaps (missing):**
- `manager/swift` (59 upstream tests)
- `manager/custom` (54 tests)
- `manager/helmv3` (52 tests)
- `manager/pipenv` (37 tests)
- `manager/pip_requirements` (36 tests)
- `manager/bun` (34 tests)
- `manager/pub` (21 tests)

**Key gaps (implemented but low test coverage):**
- `manager/gradle` — 3,334 code lines, 29% test coverage
- `manager/pep621` — 1,308 code lines, 40% test coverage
- `manager/composer` — 1,210 code lines, 25% test coverage
- `manager/deno` — 975 code lines, 19% test coverage

### Datasources — 82 modules

| Status | Count | Code lines (approx) |
|--------|-------|---------------------|
| `full` | 61 | ~31,600 |
| `partial` | 14 | — |
| `none` | 7 | — |

**Key gaps (missing):**
- `datasource/_common` (125 upstream tests)
- `datasource/terraform-module` (23 tests)
- `datasource/terraform-provider` (18 tests)
- `datasource/pod` (19 tests)

**Key gaps (implemented but low test coverage):**
- `datasource/go` — 878 code lines, 9% test coverage
- `datasource/docker` — 766 code lines, 3% test coverage
- `datasource/pypi` — 441 code lines, 5% test coverage

### Platforms — 13 modules

| Status | Count | Code lines (approx) |
|--------|-------|---------------------|
| `full` | 11 | ~11,600 |
| `partial` | 1 | — |
| `none` | 1 | — |

**Key gaps (implemented but low test coverage):**
- `platform/gitlab` — 772 code lines, 9% test coverage
- `platform/bitbucket` — 719 code lines, 0% test coverage
- `platform/azure` — 718 code lines, 21% test coverage
- `platform/bitbucket-server` — 642 code lines, 4% test coverage
- `platform/gitea` — 596 code lines, 0% test coverage
- `platform/forgejo` — 588 code lines, 5% test coverage
- `platform/gerrit` — 524 code lines, 0% test coverage

**Missing:** `platform/codecommit` (58 upstream tests).

### Workers — 2 modules

| Status | Count | Code lines (approx) |
|--------|-------|---------------------|
| `full` | 2 | ~10,100 |

Both `worker/repository` (9,797 code lines) and `worker/global` (338 code lines)
are implemented, but `worker/repository` has only **15% test coverage**. This is
the single largest source of "missing" test coverage in the entire project
(1,425 of the ~5,900 unported tests live here).

### Versioning — 54 modules

| Status | Count | Code lines (approx) |
|--------|-------|---------------------|
| `full` | 42 | ~34,100 |
| `partial` | 10 | — |
| `none` | 2 | — |

This category is the most mature: **93% test coverage** overall. The two missing
modules (`versioning/_common`, `versioning/regex`) both have 0 Rust files but
coverage reports 83–86% because tests were ported from a different location.

### Config — 6 modules

| Status | Count | Code lines (approx) |
|--------|-------|---------------------|
| `full` | 6 | ~12,100 |

All config modules have substantial code, but:
- `config/options` — 2,901 code lines, **0% test coverage**
- `config/presets` — 1,331 code lines, **19% test coverage**
- `config/decrypt` — 542 code lines, **0% test coverage**

### Util — 13 modules

| Status | Count | Code lines (approx) |
|--------|-------|---------------------|
| `full` | 6 | ~3,600 |
| `partial` | 3 | — |
| `stub` | 1 | — |
| `none` | 3 | — |

**Missing:** `util/_root` (403 upstream tests), `util/github` (45 tests),
`util/schema-utils` (47 tests).

**Low coverage:** `util/http` (1,478 code lines, 17%), `util/git` (541 code lines,
29%), `util/fs` (520 code lines, 48%).

---

## The two kinds of work remaining

### 1. Port tests for already-implemented modules

**Impact: high on coverage number, low risk.**

41 implemented modules have <50% test coverage. The code already exists; the
work is to:

1. Read the upstream `.spec.ts` files.
2. Write equivalent Rust tests.
3. Add `// Ported: "description" — spec-file.spec.ts line N` comments so the
coverage tool counts them.

**Top targets by "coverage uplift per hour":**

| Module | Current coverage | Code exists? | Upstream tests | Why it's easy |
|--------|-----------------|--------------|----------------|---------------|
| `worker/repository` | 15% | 9,797 lines | 1,675 | Huge codebase, many integration tests already written in Rust but not annotated |
| `platform/gitea` | 0% | 596 lines | 183 | Platform tests are often repetitive (getRepos, initRepo, etc.) |
| `platform/bitbucket` | 0% | 719 lines | 109 | Same pattern as gitea |
| `platform/gerrit` | 0% | 524 lines | 158 | Same pattern |
| `config/decrypt` | 0% | 542 lines | 16 | Small, focused module |
| `config/options` | 0% | 2,901 lines | 16 | Small spec, large implementation |
| `datasource/docker` | 3% | 766 lines | 174 | Three-file split (hub, ecr, google) |
| `datasource/go` | 9% | 878 lines | 181 | Multi-file split |

### 2. Implement genuinely missing modules

**Impact: medium on coverage number, medium-to-high risk (new code).**

32 modules have **no Rust implementation at all.** Some are small wrappers
(`constants`, `data`, `logger`), others are substantial managers/datasources.

**Top targets by user impact:**

| Module | Upstream tests | Category | Notes |
|--------|---------------|----------|-------|
| `manager/swift` | 59 | manager | iOS ecosystem |
| `manager/custom` | 54 | manager | Allows user-defined regex managers |
| `manager/helmv3` | 52 | manager | Helm charts (v3 is the current standard) |
| `manager/pipenv` | 37 | manager | Python lockfile |
| `manager/pip_requirements` | 36 | manager | Python requirements.txt |
| `platform/codecommit` | 58 | platform | AWS CodeCommit |
| `datasource/_common` | 125 | datasource | Shared datasource utilities |
| `util/_root` | 403 | util | Root-level utility functions |
| `util/github` | 45 | util | GitHub URL/graphql helpers |
| `util/schema-utils` | 47 | util | JSON schema validation utilities |

---

## Recommendations

### Short term (next 2–4 weeks): boost test coverage for implemented modules

Focus on modules that are **already implemented** but have **<20% test coverage**.
This is the fastest way to move the overall number from 50% → 60% → 70%.

1. **Batch-annotate existing Rust tests.** Many tests already exist in the Rust
   codebase but don't have `// Ported:` comments. A single pass with
   `scripts/parity_coverage.py gaps <module>` + `grep` can find them.
2. **Target platform modules.** `platform/gitea`, `platform/bitbucket`,
   `platform/gerrit`, `platform/forgejo`, and `platform/gitlab` are all
   implemented (500–800 code lines each) but have 0–9% coverage. Platform tests
   are highly patterned — porting one makes the next much faster.
3. **Target `worker/repository`.** At 15% coverage with 1,675 upstream tests,
   this module alone accounts for **~24% of all unported tests.** Even bringing
   it to 40% would add ~400 to the total ported count.

### Medium term (next 1–2 months): fill implementation gaps

Focus on the **missing modules with the highest user impact:**

1. `manager/helmv3` — Helm is widely used; v3 is the current standard.
2. `manager/pip_requirements` + `manager/pipenv` — Python is a top-3 language.
3. `manager/custom` — enables user-defined managers, a common self-hosted need.
4. `datasource/_common` — shared utilities that many other datasources depend on.
5. `util/_root`, `util/github`, `util/schema-utils` — infrastructure that other
   modules need.

### Long term: chase the long tail

Modules like `manager/swift`, `platform/codecommit`, `manager/bun`, etc. can be
worked opportunistically or when a specific user requests them.

---

## How to run the audits

```bash
# Test coverage (auto-generated Coverage column)
python3 scripts/parity_coverage.py report
python3 scripts/parity_coverage.py gaps <module>

# Implementation audit (auto-generated Impl column — now integrated into modules.md)
python3 scripts/impl_audit.py report
python3 scripts/impl_audit.py detail <module>

# Combined: regenerate the full ledger
python3 scripts/parity_coverage.py ledger
```

---

## Appendix: modules that changed status in this audit

Previously, 178 modules were marked `?` (unassessed). After this audit:

- **209** → `full` (substantial Rust implementation exists)
- **51** → `partial` (some Rust code, but clearly incomplete)
- **32** → `none` (no Rust files found)

The `modules.md` ledger has been updated to reflect these findings.
