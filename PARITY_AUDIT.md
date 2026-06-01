# Renovate-Rust Parity Audit — M1 through M5

**Date:** 2026-05-31
**Coverage:** 5821 / 11670 upstream tests (49.9%)
**Build gates:** ✅ All pass
**Test suite:** ✅ 9401 passed, 0 failed

---

## Milestone Summary

| Milestone | Theme | Status | Formal Blockers |
|---|---|---|---|
| **M0** | Bootstrap, project skeleton | ✅ Complete | — |
| **M1** | `manager/cargo` — extraction + `Cargo.lock` artifacts | ✅ Complete | — |
| **M2** | `manager/npm` — extraction + lockfile artifacts | ✅ Complete | — |
| **M3** | `platform/github` — read + dry-write surface | ✅ Complete | — |
| **M4** | `util/exec` + artifact pipeline verification | ✅ Complete | — |
| **M5** | Top-10 modules ≥80% coverage, all `Impl=full` | ⚠️ Complete with known gaps | `manager/terraform` ArtifactRunner not registered; CLI auto-replace naive |

---

## Critical Production Gaps (🔴 Must Fix) — ALL RESOLVED

| # | Gap | Status | Notes |
|---|---|---|---|
| 1 | Template replacement leaks literal `"TODO"` | ✅ **FIXED** | `apply_template` properly substitutes `{{packageName}}`, `{{depName}}`, `{{currentValue}}`, `{{currentVersion}}`. |
| 2 | All artifact runners bypass `util/exec` orchestrator | ✅ **FIXED** | `ArtifactConfig::exec_config` + `exec_artifact_command()` route all 6 runners through `exec::orchestrator::exec`. |
| 3 | GitHub `write_file` is stubbed (`NotSupported`) | ✅ **FIXED** | Implemented via GitHub Contents API (`GET` for SHA, `PUT` to create/update). |
| 4 | GitHub platform has no fork handling | ✅ **FIXED** | `init_repo` now accepts `fork_token`/`fork_creation`/`fork_org`, implements `find_fork`/`create_fork`. |
| 5 | npm artifact runner is severely simplified | ✅ **FIXED** | `NpmArtifactRunner` now parses `packageManager`, builds tool constraints from `engines`/`volta`, handles `.npmrc`, uses proper install builders. |

---

## Medium Gaps (🟡 Should Fix)

### 6. Terraform artifact runner not registered in CLI
**Impact:** `.terraform.lock.hcl` lockfiles are not regenerated during terraform updates. The `update_terraform_artifacts` function exists in `extractors/terraform.rs` but has no `ArtifactRunner` wrapper and is not in the CLI registry.
**Fix:** Wrap `update_terraform_artifacts` in `TerraformArtifactRunner`, register in CLI `ArtifactRegistry`.

### 7. CLI auto-replace is naive (no verification)
**Impact:** The Rust `auto_replace` function (`workers/repository/update/branch/auto_replace.rs`) does a simple string find+replace without re-extraction verification. Managers without explicit `updateDependency` (terraform, terragrunt, bicep, ansible, etc.) rely on this for manifest updates. Ambiguous replacements (multiple occurrences) silently return the original content.
**Fix:** Add re-extraction verification to the CLI's auto-replace fallback, matching upstream `doAutoReplace` behavior.

### 8. No PR caching on GitHub platform
**Impact:** `getPrList` fetches fresh from API every time. Excessive API usage on repos with many PRs.
**Fix:** Add `pr_cache` module with TTL-based caching (mirrors upstream `pr.ts`).

### 9. Cargo artifact runner missing git auth env vars
**Impact:** Private git dependencies in `Cargo.toml` may fail during `cargo update`.
**Fix:** Inject `getGitEnvironmentVariables(["cargo"])` before running cargo.

### 10. Terraform extractor is regex-based, not HCL-parsed
**Impact:** Does not handle string interpolation, multi-line values, or heredocs. Coverage is good for common patterns but edge cases fail silently.
**Fix:** Integrate `hcl-rs` parser for robust HCL parsing (currently used elsewhere in the crate but not in the terraform extractor).

### 11. `worker/repository` legacy module inflates denominator
**Impact:** The legacy worker module shows 252/1675 tests. Production path is CLI `process_repo` + `branch.rs`, but the old module still inflates the denominator.
**Fix:** Either port the legacy module or remove it from coverage counting and document that workers are implemented via CLI orchestration.

### 12. `platform/gitlab` `init_repo` is hardcoded
**Impact:** Returns fixed defaults (`default_branch: "main"`, `is_fork: false`). Real GitLab usage will have incorrect metadata.
**Fix:** Implement REST API call to fetch project metadata.

---

## What's Actually Complete (🟢 Solid)

### Extractors (very strong)
| Module | Status | Notes |
|---|---|---|
| `manager/cargo` | 🟢 Production-ready | Handles workspaces, git deps, registries, locked versions |
| `manager/npm` | 🟢 Production-ready | Largest extractor; covers npm/yarn/pnpm, workspaces, .npmrc, yarnrc |
| `manager/github-actions` | 🟢 Production-ready | Container/services extraction, ratchet, comment pins |
| `manager/gitlabci` | 🟢 Production-ready | Dependency-proxy stripping, registry aliases |
| `manager/dockerfile` | 🟢 Production-ready | Multi-stage, parser directives, arg resolution |
| `manager/gomod` | 🟢 Production-ready | `require`/`replace`/`exclude`, `toolchain`, `tool`, pseudo-versions |
| `manager/maven` | 🟢 Production-ready | Cross-file parent resolution, property resolution, XML parsing |
| `manager/terraform` | 🟡 Extractor + lockfile ready | `write_lock_updates`, `create_hashes`, `update_terraform_artifacts` all implemented. Needs ArtifactRunner wrapper + CLI registration. |
| `manager/bundler` | 🟢 Production-ready | Gemfile + lockfile, source blocks, platforms, git sources |
| `manager/pip_requirements` | 🟢 Production-ready | Index URLs, hashes, git sources, sub-requirements |

### Platforms
| Module | Status | Notes |
|---|---|---|
| `platform/github` | 🟢 Complete | Read, write, PR ops, fork handling all done |
| `platform/local` | 🟢 No-op stubs (intentional) | Self-hosted runs |
| `platform/gitlab` | 🔴 Hardcoded stubs | `init_repo` returns fixed values |
| Other platforms | 🔴 Not implemented | bitbucket, gitea, gerrit, codecommit, etc. |

### Versioning
| Module | Status | Notes |
|---|---|---|
| `versioning/cargo` | 🟢 Complete | |
| `versioning/npm` | 🟢 Complete | |
| `versioning/maven` | 🟢 Complete | |
| `versioning/hashicorp` | 🟢 Complete | |
| `versioning/semver` | 🟢 Complete | |
| Others | 🟢 Most complete | 93% overall coverage |

### Datasources
| Module | Status | Notes |
|---|---|---|
| `datasource/npm` | 🟢 Complete | Registry auth, scoped packages |
| `datasource/maven` | 🟢 Complete | Multi-registry, parent POMs, S3 |
| `datasource/github-releases` | 🟢 Complete | |
| `datasource/github-tags` | 🟢 Complete | |
| `datasource/pypi` | 🟢 Complete | |
| `datasource/terraform-provider` | 🟢 Complete | Registry v2, OpenTofu, HashiCorp backend |
| `datasource/crate` | 🟢 Complete | |
| `datasource/rubygems` | 🟢 Complete | |
| Others | 🟡 Partial / 🔴 Missing | Many at 0% coverage |

---

## Long-Tail Modules at 0% (Not in Critical Path)

These drag the overall coverage to ~50% but are not required for the core workflow:

- **Platforms:** `bitbucket` (0/109), `gitea` (0/183), `gerrit` (0/158), `codecommit` (0/58)
- **Datasources:** `aws-machine-image`, `aws-rds`, `conan`, `github-digest`, `github-release-attachments`, etc.
- **Managers:** `bun` (3/34), `composer` (22/89), `cocoapods` (2/13), `swift` (28/59)
- **Workers:** `worker/repository` legacy module (252/1675)
- **Util:** `template` (0/54), `http` (43/251)

---

## Recommended Priority Order

### Phase A: M5 Completion
1. ✅ All critical gaps fixed
2. Wrap `update_terraform_artifacts` as `ArtifactRunner` + register in CLI
3. Wire `auto_replace` into CLI `_ =>` branch for managers without explicit `updateDependency`

### Phase B: Important Features
4. Add re-extraction verification to CLI auto-replace
5. Add GitHub PR caching
6. Add cargo git auth env vars
7. Improve Terraform HCL parsing

### Phase C: Coverage & Long Tail
8. Fix `// Ported:` comment mismatches to increase reported coverage
9. Decide fate of `worker/repository` legacy module
10. Implement missing platforms (gitea is highest-value self-hosted option)
11. Implement missing managers based on user demand

---

## Metrics

| Metric | Value |
|---|---|
| Total upstream `it()`s | 11,670 |
| Ported tests | 5,821 (49.9%) |
| Duplicate ported comments | 684 |
| Malformed ported comments | 0 |
| Modules with `Impl=full` | ~70 |
| Modules with `Impl=partial` | ~25 |
| Modules with `Impl=none` | ~40 |
| Build gates | ✅ All pass |
| Test suite | ✅ 9401/9401 pass |
