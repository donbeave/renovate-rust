# Comprehensive Implementation Plan

Complete plan to close ALL gaps between the Renovate TypeScript reference and the Rust implementation.
Covers source map (58 not-started + 222 partial rows), test map (174 not-done spec files), and
all infrastructure needed to reach full parity.

**Current stats:** 240 full / 222 partial / 58 not-started / 103 out-of-scope (source files)
**Build:** 6599 unit tests + 8 doctests passing

---

## Completed Work

| Phase | Item | Commit(s) | Tests |
|---|---|---|---|
| P0 | Git storage layer | `GitStorage` with clone/fetch/commit/push | 8 |
| P0 | util/exec infrastructure | `raw_exec`, Docker sidecar, containerbase, hermit, orchestrator | 36 |
| P0 | util/http infrastructure | Host rules, rate limits, throttle/queue per-host | 54 |
| P0 | npm post-extract | `detect_monorepos`, `get_locked_versions`, `resolve_npmrc`, `NpmPackageFile` | via existing npm suite |

---

## Phase A: Infrastructure Foundations (blocks everything else)

### A1. util/cache — Package + Repository Cache Backends
- `cache/package.ts` → package-level memoization cache
- `cache/repository/index.ts` → repository cache orchestration
- `cache/repository/impl/local.ts` → local file-based cache
- `cache/repository/http-cache.ts` → HTTP response cache (partially done in `http/mod.rs`)
- **Depends on:** nothing (foundational)
- **Blocks:** workers, datasources

### A2. util/fs — File System Utilities
- `fs/index.ts` → `readLocalFile`, `writeLocalFile`, `findLocalSiblingOrParent`, `getParentDir`, `getSiblingFileName`
- `fs/proxies.ts` → GitFS proxy operations
- `fs/util.ts` → `matchesAnyPattern`, file matching
- **Depends on:** nothing
- **Blocks:** post-update, artifacts, npmrc resolution

### A3. util/package-rules — Package Rule Matchers
- `package-rules/index.ts` → rule evaluation engine
- `package-rules/base.ts` → base matcher
- All matchers: `currentVersion`, `currentValue`, `datasource`, `depName`, `depType`, `encoding`, `extractVersion`, `language`, `manager`, `packageName`, `sourceUrl`, `updateType`, `versioning`
- **Depends on:** nothing
- **Blocks:** config validation, workers

### A4. util/template — Handlebars Templating
- `template/index.ts` → Handlebars template compilation and rendering
- Used by commit messages, PR titles, PR bodies
- **Depends on:** `handlebars` crate
- **Blocks:** workers (PR generation)

### A5. util/github — GitHub GraphQL + Cache
- `github/graphql/index.ts` → GraphQL query execution
- `github/graphql/datasource-fetcher.ts` → Batched release fetching
- `github/cache.ts` → GitHub issue/release cache
- **Depends on:** http infrastructure (done)
- **Blocks:** github-actions manager, github datasource completeness

### A6. util/git — Extended Git Operations
- `git/index.ts` → `isBehindBase`, `isBranchConflicted`, `isBranchModified`, `setBranchPrefix`
- `git/auth.ts` → SSH key handling for git operations
- `git/error.ts` → Git error types
- **Depends on:** git storage (done)
- **Blocks:** workers branch lifecycle

### A7. util/merge-confidence — Merge Confidence Data
- `merge-confidence/index.ts` → fetch and cache merge confidence data
- **Depends on:** http infrastructure (done)
- **Blocks:** workers update pipeline

---

## Phase B: Manager Artifacts (highest user-facing value, 40 files)

**Strategy:** Build a generic `ArtifactRunner` trait using the exec infrastructure, then implement per-manager. Each artifact runner calls the exec orchestrator with the right tool constraints and commands.

### B1. ArtifactRunner trait + npm artifacts (8 files)
- Create `ArtifactRunner` trait in `exec/artifact_runner.rs`
- `npm/artifacts.ts` → artifact update entry point
- `npm/post-update/index.ts` → lock file orchestration
- `npm/post-update/npm.ts` → `npm install --package-lock-only`
- `npm/post-update/yarn.ts` → `yarn install` with version-specific args
- `npm/post-update/pnpm.ts` → `pnpm install --lockfile-only`
- `npm/post-update/utils.ts` → `writeExistingFiles`, `writeUpdatedPackageFiles`
- `npm/post-update/node-version.ts` → node constraint resolution
- **Depends on:** A1 (cache), A2 (fs), exec (done)

### B2. Cargo artifacts
- `cargo/artifacts.ts` → `cargo update` with workspace awareness
- `cargo/range.ts` → range strategy
- `cargo/update.ts` → `bumpPackageVersion`
- `cargo/update-locked.ts` → update-locked status
- **Depends on:** ArtifactRunner trait (B1)

### B3. Bundler artifacts
- `bundler/artifacts.ts` → `bundle install` with Gemfile manipulation
- **Depends on:** B1

### B4. Composer artifacts
- `composer/artifacts.ts` → `composer install` with auth
- **Depends on:** B1

### B5. Go modules artifacts
- `gomod/artifacts.ts` → `go mod tidy`, `go get`, `go mod download`
- **Depends on:** B1

### B6. Gradle artifacts
- `gradle/artifacts.ts` → gradle dependency resolution with wrapper
- **Depends on:** B1

### B7. Python ecosystem artifacts
- `pip_requirements/artifacts.ts` → `pip install`
- `poetry/artifacts.ts` → `poetry lock`
- `pipenv/artifacts.ts` → `pipenv lock`
- `pep621/artifacts.ts` → `uv lock` / `pdm lock`
- **Depends on:** B1

### B8. Remaining HIGH artifacts
- `helmv3/artifacts.ts` → `helm dep update`
- `nuget/artifacts.ts` → `dotnet restore`
- **Depends on:** B1

### B9. MEDIUM artifacts (16 managers)
- `mix/artifacts.ts`, `nix/artifacts.ts`, `swift/artifacts.ts`, `deno/artifacts.ts`
- `bun/artifacts.ts`, `pixi/artifacts.ts`, `maven-wrapper/artifacts.ts`
- `gradle-wrapper/artifacts.ts`, `mise/artifacts.ts`
- **Depends on:** B1

### B10. LOW artifacts (20 managers)
- `bazel/artifacts.ts`, `bazel-module/artifacts.ts`, `bazelisk/artifacts.ts`
- `cocoapods/artifacts.ts`, `conan/artifacts.ts`, `copier/artifacts.ts`
- `devbox/artifacts.ts`, `flux/artifacts.ts`, `gleam/artifacts.ts`
- `helmfile/artifacts.ts`, `hermit/artifacts.ts`, `jsonnet-bundler/artifacts.ts`
- `kustomize/artifacts.ts`, `pub/artifacts.ts`, `terragrunt/artifacts.ts`
- `vendir/artifacts.ts`, `batect-wrapper/artifacts.ts`
- **Depends on:** B1

---

## Phase C: Manager Non-Artifact Gaps

### C1. npm remaining (3 files)
- `npm/update/package-version/index.ts` → package version bumping
- Already have: post-extract (locked versions, monorepo, npmrc), update-dependency, update-locked
- **Depends on:** nothing

### C2. Custom managers (5 files)
- `custom/api.ts` → custom manager registration API
- `custom/regex/index.ts` → regex-based extraction
- `custom/regex/strategies.ts` → regex replacement strategies
- `custom/jsonata/index.ts` → JSONata-based extraction
- `custom/jsonata/utils.ts` → JSONata utility functions
- **Depends on:** nothing

### C3. Other manager updates
- `homebrew/update.ts` → URL update in formula
- `git-submodules/update.ts` → update dependency
- `deno/post.ts` → post-processing
- `bun/utils.ts` → shared npm utilities
- `terraform/hcl/index.ts` → full HCL parser (replace regex-based)
- **Depends on:** nothing

---

## Phase D: Manager Partial → Full (222 files)

### D1. HIGH priority partials
- **npm (27 partial files):** post-update paths, range strategy, remaining utility completeness
- **gradle (12 partial files):** parser — apply-from, assignments, handlers, objects, plugins
- **terraform (10 partial files):** HCL parser, resource extractors, base utilities
- **github-actions (9 partial files):** community action index, parse completeness
- **pep621 (7 partial files):** abstract/hatch/pdm/uv processors
- **terragrunt (5 partial files):** common, modules, providers, util
- **go datasource (5 partial files):** GOPROXY, direct VCS fallback
- **docker datasource (5 partial files):** ECR, GCR, digest lookup
- **ansible-galaxy (5 partial files):** collections-metadata, roles, util
- **deno (4 partial files):** compat layer, post-processing
- **pip-compile (4 partial files):** multi-file delegation, common, utils
- **nuget (4 partial files):** config-formatter, package-tree, util
- **scm-manager (6 partial files):** mapper, helper, schema, utils
- **conan (3 partial files):** common, range
- **bun (3 partial files):** extract, utils
- **puppet (3 partial files):** common, puppetfile-parser
- **cpanfile (3 partial files):** language, parser
- **tflint-plugin (3 partial files):** plugins, util
- **mise (3 partial files):** lockfile, utils
- **helmv3 (3 partial files):** OCI, utils
- **deb datasource (3 partial files):** checksum, packages, release, url
- **custom managers (3 partial files):** regex match_all, strategies

### D2. MEDIUM priority partials (~80 managers)
- Most are `index.ts` entries missing `defaultConfig`, `supportedDatasources`, `categories`
- Second group: utility/helper files inlined but incomplete
- Pattern: add missing metadata to `managers.rs`, flesh out inline helpers

### D3. LOW priority partials (~40 managers)
- Various index.ts metadata, small utility functions
- Many are single-line additions to `managers.rs`

---

## Phase E: Config System

### E1. Config migrations (62 files)
- `config/migrations/custom/*.ts` → individual migration files
- Pattern: each migration transforms a deprecated config key to its replacement
- **Strategy:** mechanical, generate from TS source patterns
- **Depends on:** config types (existing)

### E2. Config presets (~20 files)
- `config/presets/index.ts` → preset resolution engine
- `config/presets/github/index.ts` → GitHub-hosted presets
- `config/presets/gitlab/index.ts` → GitLab-hosted presets
- `config/presets/gitea/index.ts` → Gitea-hosted presets
- `config/presets/forgejo/index.ts` → Forgejo-hosted presets
- `config/presets/local/index.ts` → local file presets
- `config/presets/npm/index.ts` → npm-hosted presets
- `config/presets/internal/*.ts` → built-in presets
- **Depends on:** http (done), platform (Phase G)

### E3. Config options + validation
- `config/options/index.ts` → full option definitions (currently partial)
- `config/validation.ts` → standalone validate()
- `config/validation-helpers/` → validation helper functions
- `config/decrypt/bcpgp.ts` → PGP decryption (bcpgp)
- `config/decrypt/openpgp.ts` → PGP decryption (openpgp)
- `config/inherit.ts` → config inheritance
- `config/utils.ts` → config utilities
- `config/parse.ts` → config parsing
- **Depends on:** A3 (package-rules)

---

## Phase F: Datasource Completeness

### F1. Datasource not-started (7 items)
- `aws-eks-addon` → AWS SDK datasource
- `aws-machine-image` → AWS SDK datasource
- `aws-rds` → AWS SDK datasource
- `azure-tags` → Azure API
- `rpm` → RPM repository parsing
- `github-digest` → digest computation
- `github-release-attachments` → attachment handling
- **Depends on:** AWS/Azure SDK crates

### F2. Datasource partial → full (2 items)
- `datasource.ts` → base class pattern (use functions)
- `postprocess-release.ts` → per-datasource override wiring

### F3. Versioning partial → full (1 item)
- `common.ts` → GenericVersioningApi inherited functions

---

## Phase G: Platform Completeness

### G1. Platform not-started
- `bitbucket/index.ts` → full Bitbucket API client
- `bitbucket-server/index.ts` → full Bitbucket Server client
- `gitea/index.ts` → full Gitea API client
- `forgejo/index.ts` → full Forgejo API client (similar to Gitea)
- `azure/index.ts` → full Azure DevOps client
- `codecommit/index.ts` → AWS CodeCommit client
- `gerrit/index.ts` → Gerrit client
- **Depends on:** http (done)

### G2. Platform infrastructure
- `platform/comment.ts` → platform comment management
- `platform/scm.ts` → SCM abstraction layer
- `platform/default-scm.ts` → default SCM implementation
- **Depends on:** git (done), platform clients

### G3. Platform partials
- `gitlab/merge-request.ts`, `gitlab/pr-cache.ts`, `gitlab/utils.ts`
- `github/branch.ts`, `github/graphql.ts`, etc.
- **Depends on:** http (done)

---

## Phase H: Workers

### H1. Workers not-started (133 files)
- `workers/repository/update/branch/` → branch lifecycle (automerge, rebase, status checks)
- `workers/repository/process/lookup/` → version lookup pipeline (bucket, rollback, generate)
- `workers/repository/onboarding/` → onboarding branch/PR creation
- `workers/repository/update/pr/changelog/` → changelog/release notes rendering
- `workers/repository/config-migration/` → config migration PR pipeline
- `workers/repository/finalize/` → finalization, pruning
- `workers/repository/reconfigure/` → reconfiguration pipeline
- `workers/repository/model/` → commit message model
- `workers/global/` → autodiscover, additional config
- **Depends on:** ALL infrastructure (A1-A7), platform (G), config (E)

### H2. Workers partial → full (8 files)
- `workers/global/index.ts` → autodiscover, cache init
- `workers/global/initialize.ts` → cache init, rate-limit init
- `workers/repository/index.ts` → full pipeline stages
- `workers/repository/init/*.ts` → cache flush, config errors
- `workers/repository/process/*.ts` → fetch/lookup pipeline
- `workers/repository/update/branch/index.ts` → full branch lifecycle
- `workers/repository/update/pr/index.ts` → PR update, labels, automerge
- **Depends on:** H1

---

## Phase I: Test Coverage Parity (174 spec files)

Port remaining `.spec.ts` files to Rust tests. Ordered by area:

### I1. Manager artifact tests (~40 files)
- One spec file per manager artifacts implementation
- **Depends on:** Phase B complete

### I2. Datasource tests (~20 files)
- `docker/index.spec.ts`, `docker/common.spec.ts`
- `npm/get.spec.ts`, `npm/index.spec.ts`, `npm/npmrc.spec.ts`
- `go/index.spec.ts`, `go/base.spec.ts`, `go/releases-direct.spec.ts`, `go/releases-goproxy.spec.ts`
- `maven/index.spec.ts`, `maven/util.spec.ts`
- `pypi/index.spec.ts`, `nuget/index.spec.ts`
- etc.
- **Depends on:** Phase F

### I3. Platform tests (~10 files)
- `github/`, `gitlab/`, `bitbucket/`, `bitbucket-server/`, `gitea/`, `forgejo/`
- **Depends on:** Phase G

### I4. Config tests (~6 files)
- `presets/index.spec.ts`, `presets/*/index.spec.ts`
- **Depends on:** Phase E

### I5. Util tests (~15 files)
- `http/got.spec.ts`, `http/host-rules.spec.ts`, `http/index.spec.ts`
- `http/forgejo.spec.ts`, `http/gerrit.spec.ts`, `http/gitea.spec.ts`
- `http/gitlab.spec.ts`, `http/github.spec.ts`, `http/jira.spec.ts`
- `exec/index.spec.ts`, `exec/common.spec.ts`, `exec/docker/`
- `github/graphql/`, `git/index.spec.ts`, `template/index.spec.ts`
- **Depends on:** Phase A

### I6. Workers tests (~15 files)
- All workers spec files
- **Depends on:** Phase H

### I7. Remaining manager tests (~70 files)
- Non-artifact manager specs (extract, update, range, etc.)
- **Depends on:** Phases C, D

---

## Execution Priority Order

Work through in this order, committing each coherent slice:

1. **A1-A7** Infrastructure foundations (cache, fs, package-rules, template, github, git, merge-confidence)
2. **B1** ArtifactRunner trait + npm artifacts (unblocks all other artifacts)
3. **B2-B10** All remaining manager artifacts
4. **C1-C3** Manager non-artifact gaps
5. **D1** HIGH priority manager partials
6. **E1-E3** Config system (migrations, presets, options, validation)
7. **F1-F3** Datasource completeness
8. **G1-G3** Platform completeness
9. **H1-H2** Workers
10. **D2-D3** MEDIUM/LOW manager partials
11. **I1-I7** Test coverage parity (ongoing alongside above)

---

## Progress Tracking

| Phase | Target | Completed | Status |
|---|---|---|---|
| A1. Cache backends | 4 files | 0 | pending |
| A2. File system utils | 3 files | 0 | pending |
| A3. Package rules | 13 files | 0 | pending |
| A4. Template engine | 1 file | 0 | pending |
| A5. GitHub GraphQL | 3 files | 0 | pending |
| A6. Extended git ops | 5 files | 0 | pending |
| A7. Merge confidence | 1 file | 0 | pending |
| B1. ArtifactRunner + npm | 8 files | 0 | pending |
| B2-B10. Other artifacts | 32 files | 0 | pending |
| C1-C3. Manager non-artifact | 13 files | 0 | pending |
| D1. HIGH partials | ~80 files | 0 | pending |
| D2-D3. MEDIUM/LOW partials | ~140 files | 0 | pending |
| E1-E3. Config system | ~100 files | 0 | pending |
| F1-F3. Datasources | 10 files | 0 | pending |
| G1-G3. Platforms | ~45 files | 0 | pending |
| H1-H2. Workers | ~140 files | 0 | pending |
| I1-I7. Test parity | 174 specs | 0 | pending |
| **Total remaining** | **~670 items** | **~30 done** | — |

---

## Key Constraints

- Each slice must compile and pass tests before committing
- Use `Co-authored-by: opencode-agent[bot] <opencode-agent[bot]@users.noreply.github.com>` trailer
- Push after every commit
- Follow existing code conventions (no comments unless asked)
- Port tests with `// Ported:` attribution comments
- Source map rows update as work progresses (partial→full, not-started→partial/full)
