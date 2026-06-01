# Renovate-Rust Test Parity Audit

Auto-generated during M5 post-verification sweep.

## Executive Summary

- **Milestone acceptance:** M0–M5 acceptance criteria are met.
- **Test coverage:** 49.7% of upstream tests ported (5799 / 11670).
- **Orphans/malformed:** 0.
- **Implementation gaps found:** Yes — several stubs exist that block further test porting.

---

## Test Porting Results by Module

### ✅ Completed (no gaps or gaps are duplicates)

| Module | Before | After | Notes |
|--------|--------|-------|-------|
| `manager/maven` | 98% | 98% | 1 "missing" test is a duplicate description. |
| `manager/dockerfile` | 100% | 100% | — |
| `versioning/semver` | 100% | 100% | — |
| `platform/local` | 100% | 100% | — |

### ✅ Improved

| Module | Before | After | Ported |
|--------|--------|-------|--------|
| `manager/terraform` | 84% | 94% | 9 tests (hash.spec.ts + lockfile/index.spec.ts) |
| `datasource/npm` | 81% | 86% | 5 tests (get.spec.ts auth) |

### ⏳ Remaining gaps

| Module | Missing | Blocker |
|--------|---------|---------|
| `manager/npm` | 87 | Many require artifact/post-update infra; extract-only gaps remain |
| `platform/github` | 48 | Many require HTTP mock infra or unimplemented platform functions |
| `manager/github-actions` | 17 | All in integration.spec.ts; require `lookup_updates` worker infra |
| `datasource/maven` | 12 | Cache spec requires `packageCache` HTTP response caching (unimplemented) |

---

## Implementation Stubs Blocking Test Ports

### Critical

| Stub | Location | Impact |
|------|----------|--------|
| `TerraformProviderHash::create_hashes()` | `extractors/terraform.rs` | Returns `"h1:stubhash-{repo}-{ver}"`. Invalid lockfile output. Blocks 1 hash test. |
| `npm_post_update/artifact_runner.rs` | `extractors/npm/` | ~10% stub. Hardcodes generic install. Blocks 33+ post-update tests + 13 artifact tests. |
| `worker/repository::process_repository` | `workers/repository/` | 43-line stub vs upstream ~500 lines. Blocks all integration tests. |
| `worker/repository::process_branch` | `workers/repository/` | 90-line stub vs upstream ~1122 lines. Blocks branch lifecycle tests. |
| `worker/repository::ensure_pr` | `workers/repository/` | ~100-line stub vs upstream ~632 lines. Blocks PR lifecycle tests. |
| `platform/github::init_repo` | `platforms/github.rs` | Missing entirely. Blocks all default-run platform tests. |

### Medium

| Stub | Location | Impact |
|------|----------|--------|
| `platform/github::get_branch_status` | `platforms/github.rs` | Missing check-runs API. |
| `platform/github::massage_markdown` | `platforms/github.rs` | Missing smart-truncate, alert blocks, href rewrite. |
| `platform/github` fork mode | `platforms/github.rs` | Missing `create_fork`, `find_fork`, `list_forks`. |
| `datasource/maven` cache layer | `datasources/maven.rs` | Only has 404 metadata cache. No `packageCache` for HTTP responses. Blocks 5 cache tests. |
| `datasource/npm` cache layer | `datasources/npm.rs` | No cache implementation in `get_npm_releases`. Blocks 3 cache tests. |

---

## Duplicate Description Findings

The coverage script deduplicates by `it('...')` description. Several apparent "gaps" are actually duplicate descriptions already covered by a single `// Ported:` comment:

- `manager/maven/extract.spec.ts`: `"returns null for invalid XML"` appears at lines 22 and 471.
- `manager/terraform/modules.spec.ts`: `"should split project and tag from source"` appears in 3 describe blocks; `"should parse alpha-numeric characters..."` appears in 4 describe blocks.
- `datasource/npm/get.spec.ts`: `it.each(configs)('%p', ...)` tests share the description `'%p'`.

These are **not coverage gaps** — one `// Ported:` comment legitimately covers all instances of the same description.

---

## Next Actions (Test Parity Side)

1. **manager/npm** — Port extract-only gaps (yarnrc, pnpm, locked-versions) that don't require artifact infra.
2. **platform/github** — Port tests for already-implemented functions (findPR, getBranchPr, etc.).
3. **datasource/maven** — Blocked until `packageCache` HTTP caching is implemented.
4. **manager/github-actions** — Blocked until `lookup_updates` worker infra is implemented.

## Next Actions (Implementation Side)

1. **manager/terraform** — Replace `create_hashes` stub with real zip-download SHA-256.
2. **manager/npm** — Implement artifact runner using existing `npm.rs`/`yarn.rs`/`pnpm.rs` helpers.
3. **platform/github** — Implement `init_repo`, check-runs, `massage_markdown`.
4. **worker/repository** — Implement `process_repository`, `process_branch`, `ensure_pr`.
5. **datasource/maven** — Implement `packageCache` HTTP response caching layer.
