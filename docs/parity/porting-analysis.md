# Porting Analysis: 4,020 Pending Tests

**Date:** 2026-05-30  
**Scope:** All `pending` rows in parity detail files  
**Total pending:** 4,020 across 179 files  
**Total ported:** 5,084 across ~400 files  
**NA ratio:** 19.9% (2,321 / 11,665)

---

## 1. Executive Summary

After completing Phase 0 (attribution backfill), the remaining work is **porting pending upstream tests from Renovate's TypeScript spec files into Rust**. This is a fundamentally different challenge from attribution:

- **Attribution** = documenting what exists
- **Porting** = writing new Rust tests that replicate TS test behavior

The 4,020 pending tests fall into three distinct portability tiers:

| Tier | Count | Description | Effort |
|------|-------|-------------|--------|
| **A: Immediately Portable** | ~2,200 | Rust source exists; tests are pure/unit-testable | Low-Medium |
| **B: Needs Implementation** | ~1,200 | Rust source exists but missing the specific function/behavior | Medium-High |
| **C: Not Yet Implementable** | ~620 | No corresponding Rust source or requires unbuilt infrastructure | Very High |

---

## 2. Pending Tests by Area

### 2.1 Platform (705 pending) — 92.6% have Rust source

| File | Pending | Rust Source | Notes |
|------|---------|-------------|-------|
| `github/index.spec.ts` | 206 | `platform/github.rs` (1,056 lines) | Core `PlatformClient` exists but tests focus on `initPlatform`, `getRepos`, `initRepo` — orchestration functions not yet in Rust |
| `gitlab/index.spec.ts` | 163 | `platform/gitlab.rs` (739 lines) | Same pattern — client methods exist, orchestration missing |
| `gitea/index.spec.ts` | 134 | `platform/gitea.rs` (646 lines) | Same pattern |
| `azure/index.spec.ts` | 79 | `platform/azure.rs` (716 lines) | Same pattern |
| `scm-manager/index.spec.ts` | 35 | None | No Rust source |
| `gitea-helper.spec.ts` | 39 | `platform/gitea.rs` | Helper functions exist but not tested |

**Key insight:** The `PlatformClient` trait defines 7 core methods (`get_current_user`, `get_raw_file`, `get_file_list`, `create_pr`, `update_pr`, `get_branch_status`, `write_file`). All platform implementations have these. But the TS tests cover **20+ additional behaviors**: `initPlatform` (auth, email, app tokens, GHE versioning), `getRepos` (filtering, pagination), `initRepo` (forking, clone URLs), `getPrList`, `getBranchForceRebase`, `addAssignees`, `ensureComment`, etc.

**Portability:** Tests for the 7 core methods are immediately portable using `wiremock` + `tokio::test`. Tests for orchestration methods require those methods to exist first.

---

### 2.2 Workers / Repository (1,411 pending) — 71% have Rust source

| File | Pending | Rust Source | Notes |
|------|---------|-------------|-------|
| `process/lookup/index.spec.ts` | 169 | `repo_config.rs` (16,499 lines) | Lookup logic partially in `repo_config.rs` but the full `lookup` worker isn't a standalone module |
| `update/branch/index.spec.ts` | 101 | `branch.rs` (2,559 lines) | `branch.rs` has branch naming, PR titles, commit messages — but not `processBranch` orchestration |
| `update/branch/auto-replace.spec.ts` | 70 | `branch.rs` | Auto-replace logic not yet implemented |
| `dependency-dashboard.spec.ts` | 63 | None | No dependency dashboard module |
| `update/branch/get-updated.spec.ts` | 53 | `branch.rs` | Get-updated logic not separated |
| `update/pr/index.spec.ts` | 53 | `platform/pr_body.rs` (414 lines) | PR body generation exists but PR orchestration doesn't |
| `updates/generate.spec.ts` | 55 | None | No generate module |
| `process/vulnerabilities.spec.ts` | 41 | None | No vulnerability processing worker |

**Key insight:** The `workers/repository` tree in TypeScript represents the **main execution pipeline** of Renovate:

```
init → merge config → extract → lookup → generate updates → create branches → create PRs
```

In Rust, this pipeline is **decomposed differently**:
- Config handling → `repo_config.rs`
- Branch naming/PR body → `branch.rs`, `platform/pr_body.rs`
- Schedule → `schedule.rs`
- Extractors → `extractors/*.rs`
- Datasources → `datasources/*.rs`

The **orchestration layer** (the "glue" that runs the pipeline) is mostly in `renovate-cli`, not `renovate-core`. This means many worker tests test CLI-level behavior that doesn't have a direct Rust equivalent yet.

**Portability:** Low for orchestration tests. Medium for utility tests within workers (e.g., `schedule.spec.ts` is already done, `branch-name.spec.ts` is done).

---

### 2.3 Manager Extractors (1,127 pending) — 74.5% have Rust source

| File | Pending | Rust Source | Notes |
|------|---------|-------------|-------|
| `gomod/artifacts.spec.ts` | 56 | `extractors/gomod.rs` (2,668 lines, 102 tests) | Extraction is ported; **artifacts** (lockfile updates) are not |
| `gradle/parser.spec.ts` | 45 | `extractors/gradle.rs` (3,206 lines, 60 tests) | Extraction ported; parser as separate module not implemented |
| `deno/schema.spec.ts` | 43 | `extractors/deno.rs` (890 lines) | Schema validation not separated |
| `npm/post-update/npm.spec.ts` | 35 | `extractors/npm.rs` (7,012 lines, 203 tests) | Post-update logic exists but not fully tested |
| `pip-compile/artifacts.spec.ts` | 34 | None | No pip-compile extractor |
| `composer/utils.spec.ts` | 33 | `extractors/composer.rs` (1,244 lines) | Utils not separated |
| `maven/index.spec.ts` | 33 | `extractors/maven.rs` (2,400+ lines) | Index behavior partially implemented |
| `bundler/artifacts.spec.ts` | 31 | `extractors/bundler.rs` | Artifacts not implemented |
| `npm/post-update/pnpm.spec.ts` | 31 | `extractors/npm.rs` | Post-update logic exists |
| `terraform/artifacts.spec.ts` | 30 | `extractors/terraform_hcl.rs` | Artifacts not implemented |

**Key insight:** The pattern is clear:
- **`extract.spec.ts`** → Usually ported (basic dependency extraction)
- **`artifacts.spec.ts`** → Usually NOT ported (lockfile update generation requires external tool execution)
- **`parser.spec.ts`** / **`schema.spec.ts`** / **`utils.spec.ts`** → Sometimes ported, sometimes not (depends on whether Rust implements those as separate modules)
- **`index.spec.ts`** → Mixed (includes detection, default config, etc.)

**Portability:** Medium. Many extractors have substantial Rust implementations but lack tests for advanced features.

---

### 2.4 Datasource (272 pending) — 100% have Rust source

| File | Pending | Rust Source | Notes |
|------|---------|-------------|-------|
| `maven/index.spec.ts` | 46 | `datasources/maven.rs` (711 lines) | Maven datasource exists but lacks test coverage |
| `nuget/index.spec.ts` | 35 | `datasources/nuget.rs` (331 lines) | NuGet datasource exists |
| `pypi/index.spec.ts` | 37 | `datasources/pypi.rs` (408 lines) | PyPI datasource exists |
| `hex/index.spec.ts` | 30 | `datasources/hex.rs` | Hex datasource exists |
| `npm/get.spec.ts` | 23 | `datasources/npm.rs` (517 lines) | NPM `get` module not separated |
| `npm/index.spec.ts` | 18 | `datasources/npm.rs` | NPM datasource exists |
| `npm/npmrc.spec.ts` | 15 | None | No `.npmrc` parsing module |
| `docker/index.spec.ts` | 22 | `datasources/docker_hub.rs` | Docker hub datasource exists |
| `rubygems/index.spec.ts` | 17 | `datasources/rubygems.rs` | Rubygems datasource exists |

**Key insight:** ALL datasource pending tests have corresponding Rust source. Datasources are the **most portable category** because:
1. Each datasource is a self-contained HTTP client
2. `wiremock` + `tokio::test` infrastructure is mature (used in 50+ datasource test suites)
3. The pattern is well-established: mock HTTP server → call function → assert result

**Portability:** High. These are the best ROI targets for bulk porting.

---

### 2.5 Config / Presets (158 pending) — 100% have Rust source

All pending tests map to `repo_config.rs` (16,499 lines). The config migration and preset resolution logic is implemented but many edge cases aren't tested.

**Portability:** High. Mostly pure functions on config objects.

---

### 2.6 Util Modules (222 pending) — Mixed

| File | Pending | Rust Source | Notes |
|------|---------|-------------|-------|
| `git/index.spec.ts` | 108 | `git.rs` (114 lines) | Git operations mostly in CLI; Rust `git.rs` is minimal |
| `exec/index.spec.ts` | 40 | `exec.rs` (13 lines!) | Exec module is a stub |
| `http/index.spec.ts` | 24 | `http.rs` (1,500+ lines) | HTTP client has substantial implementation |
| `merge-confidence/index.spec.ts` | 21 | None | No merge-confidence module |
| `github/index.spec.ts` | 18 | None | No GitHub utility module (some in `platform/github.rs`) |
| `cache/index.spec.ts` | 13 | `cache/` (multiple files) | Cache module exists |

**Portability:** Medium for `http`, low for `git`/`exec` (stubs), very low for missing modules.

---

## 3. Infrastructure Assessment

### What's Available

| Tool | Status | Used By | Applicability |
|------|--------|---------|---------------|
| `wiremock` | ✅ Available | 50+ datasource test suites | HTTP mocking for datasources, platforms |
| `tokio::test` | ✅ Available | 594 tests | Async test infrastructure |
| `mockall` | ❌ Not in deps | — | Could be added for trait mocking |
| `insta` (snapshot) | ❌ Not in deps | — | Could be added for large output comparison |
| `pretty_assertions` | ❌ Not in deps | — | Could be added for better diffs |

### Test Patterns That Work

1. **Datasource test** (proven, 50+ files):
   ```rust
   #[tokio::test]
   async fn fetch_latest_returns_version() {
       let server = MockServer::start().await;
       Mock::given(method("GET"))
           .and(path("/api/v1/package"))
           .respond_with(ResponseTemplate::new(200).set_body_json(json!({...})))
           .mount(&server).await;
       let result = fetch_latest("pkg", &http, &server.uri()).await;
       assert_eq!(result, Some("1.2.3".to_owned()));
   }
   ```

2. **Extractor test** (proven, 100+ files):
   ```rust
   #[test]
   fn extracts_dependencies_from_fixture() {
       let content = include_str!("testdata/package.json");
       let deps = extract_package_json(content).unwrap();
       assert_eq!(deps[0].dep_name, "lodash");
   }
   ```

3. **Config/utility test** (proven, 500+ files):
   ```rust
   #[test]
   fn migrates_deprecated_config_key() {
       let input = r#"{"oldKey": true}"#;
       let config = RepoConfig::parse(input);
       assert!(config.new_key);
   }
   ```

### What's Missing

1. **Platform orchestration tests** need a way to mock `PlatformClient` trait implementations. Currently only integration-style tests with real HTTP mocking would work.
2. **Worker/orchestration tests** need the worker modules to exist as testable units.
3. **Artifact generation tests** need the external tool execution pipeline (running `go mod tidy`, `npm install`, etc.) to be abstracted behind a trait.

---

## 4. Recommended Porting Strategy

### Tier A: Datasources + Config + Utilities (~2,200 tests)

**Why first:** High success rate, established patterns, immediate value.

**Approach:**
1. Pick a datasource detail file (e.g., `maven/index.spec.ts.md` with 46 pending)
2. Read the TS spec to understand test inputs/outputs
3. Add `#[tokio::test]` + `wiremock` tests to the corresponding Rust datasource file
4. Update the detail file `pending` → `ported`
5. Add `// Ported:` comment to the Rust test

**Estimated rate:** 15-25 tests/hour for simple datasources, 5-10/hour for complex ones.

**Top targets:**
- `datasource/maven/index.spec.ts` (46 pending) — high value, common datasource
- `datasource/pypi/index.spec.ts` (37 pending) — Python ecosystem
- `datasource/nuget/index.spec.ts` (35 pending) — .NET ecosystem
- `datasource/hex/index.spec.ts` (30 pending) — Elixir ecosystem
- `config/presets/index.spec.ts` (69 pending) — pure config logic, no HTTP needed

### Tier B: Manager Extractors (~1,200 tests)

**Why second:** Medium effort, requires understanding both TS and Rust extractor architecture.

**Approach:**
1. Focus on `index.spec.ts` and `extract.spec.ts` (not `artifacts.spec.ts`)
2. For extractors with substantial Rust code but few tests, read the TS spec fixtures
3. Add fixture files to `extractors/testdata/` if needed
4. Port test cases one by one

**Estimated rate:** 5-15 tests/hour.

**Top targets:**
- `manager/gradle/parser.spec.ts` (45 pending) — but parser may not be a separate module
- `manager/deno/schema.spec.ts` (43 pending) — schema validation
- `manager/npm/post-update/index.spec.ts` (33 pending) — post-update logic exists
- `manager/composer/utils.spec.ts` (33 pending) — utility functions

### Tier C: Platform Core Methods + HTTP Utils (~400 tests)

**Why third:** Requires understanding trait mocking or building test wrappers.

**Approach:**
1. Start with `PlatformClient` method tests for GitHub (the most complete implementation)
2. Use `wiremock` to mock the GitHub API
3. Test each trait method in isolation

**Estimated rate:** 5-10 tests/hour.

**Top targets:**
- `platform/github/index.spec.ts` — but focus ONLY on tests for the 7 `PlatformClient` methods
- `util/http/index.spec.ts` (24 pending) — HTTP client utilities

### Tier D: Workers + Orchestration (~620 tests)

**Why last:** Requires architectural decisions about how to structure worker modules in Rust.

**Approach:**
1. First implement the missing worker modules in Rust (or mark as `not-applicable`)
2. Then port tests
3. OR: mark many of these as `not-applicable` if the Rust architecture intentionally doesn't have the same module boundaries

**Critical question:** Does `renovate-rust` intend to reimplement the full worker pipeline, or is it a "config + extract + datasource" library with the pipeline living in `renovate-cli`?

If the pipeline is CLI-only, then many worker tests should be marked `not-applicable` with reason "Pipeline behavior lives in CLI, not core library."

---

## 5. Automation Potential

### What CAN be automated

1. **Finding untested Rust functions:** Script that lists `pub fn` in a Rust file with no corresponding `#[test]`
2. **Fixture translation:** Many TS tests use inline JSON fixtures — can be mechanically translated
3. **Detail file updates:** After porting a batch, script updates `pending` → `ported`
4. **Coverage reports:** Track pending count over time

### What CANNOT be automated

1. **Understanding test intent:** TS tests often use mocking libraries (jest mock, nock) that don't map 1:1 to Rust
2. **Deciding NA:** Whether a test is applicable requires architectural judgment
3. **Writing assertions:** TypeScript's loose typing vs Rust's strict types means assertions often need restructuring

---

## 6. Risk Factors

| Risk | Impact | Mitigation |
|------|--------|------------|
| Porting tests for code that doesn't exist yet | Wasted effort | Only port tests for functions that exist in Rust |
| TS tests test runtime behavior (jest mocks, timers) | Hard to replicate | Mark as `not-applicable` or implement differently |
| Architectural divergence (Rust modules ≠ TS modules) | Tests don't fit | Document divergence in detail files |
| NA ratio exceeding 25% | Violates closure criteria | Be conservative about marking NA; prefer porting |
| Test maintenance burden | Ongoing cost | Prioritize tests that catch real regressions |

---

## 7. Next Steps (Immediate Actions)

1. **Decide on worker/orchestration scope:** Are worker tests in scope for `renovate-core`, or should many be NA?
2. **Pick 3-5 high-ROI datasource targets:** Start with `maven`, `pypi`, `nuget`, `hex`
3. **Verify the porting pattern:** Port 5-10 tests from one datasource to validate the approach
4. **Set up tracking:** Update `closure-plan.md` with weekly targets
5. **Consider adding `mockall`:** For platform trait mocking, `mockall` would significantly improve testability

---

## Appendix: Data Sources

- `verify_parity.py` — verifies ported rows match actual Rust tests
- `docs/parity/**/*.md` — detail files with per-test status
- `docs/parity/renovate-test-map.md` — root tracker
- `crates/**/*.rs` — Rust source code
