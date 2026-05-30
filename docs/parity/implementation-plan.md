# Implementation Plan

Prioritized plan to close the 496 not-started + 303 partial gaps in the source map.
Ordered by impact: highest user-facing value first, infrastructure before dependents.

---

## Phase 1: Close Partial Gaps (303 files)

These already have Rust scaffolding — finishing them is the fastest path to more `full` rows.

### P1.1 Manager Partial → Full (222 files)

**Strategy:** Most partials are `index.ts` entries where only `fileMatch` is ported but not `defaultConfig`, `supportedDatasources`, `categories`. Second largest group: utility/helper files inlined but incomplete.

| Priority | Manager | Partial files | What's missing |
|---|---|---|---|
| HIGH | npm | 27 | post-update (npm/yarn/pnpm lockfiles), update/locked-dependency paths, range strategy |
| HIGH | gradle | 12 | parser completeness (apply-from, assignments, handlers, objects, plugins) |
| HIGH | terraform | 10 | HCL parser (currently regex-based), resource extractors, base utilities |
| HIGH | github-actions | 1+8 | community action index, parse completeness |
| MEDIUM | pep621 | 7 | abstract/hatch/pdm/uv processors, utils |
| MEDIUM | terragrunt | 5 | common, modules, providers, util completeness |
| MEDIUM | go (datasource) | 5 | GOPROXY multi-proxy protocol, direct VCS fallback |
| MEDIUM | docker (datasource) | 5 | ECR, GCR, digest lookup, common utilities |
| MEDIUM | ansible-galaxy | 5 | collections-metadata, roles, util completeness |
| MEDIUM | config | 5 | validation standalone, presets dispatch |
| MEDIUM | deno | 4 | compat layer, post-processing |
| MEDIUM | pip-compile | 4 | multi-file delegation, common, utils |
| MEDIUM | nuget | 4 | config-formatter, package-tree, util |
| MEDIUM | gitlab (platform) | 4 | merge-request, pr-cache, utils |
| MEDIUM | scm-manager | 6 | All files partial — mapper, helper, schema, utils |
| MEDIUM | conan | 3 | common, range completeness |
| MEDIUM | bun | 3 | extract (shares npm), utils |
| MEDIUM | puppet | 3 | common, puppetfile-parser completeness |
| MEDIUM | cpanfile | 3 | language, parser completeness |
| MEDIUM | tflint-plugin | 3 | plugins, util |
| MEDIUM | mise | 3 | lockfile, utils |
| MEDIUM | helmv3 | 3 | OCI, utils |
| MEDIUM | deb (datasource) | 3 | checksum, packages, release, url, utils |
| MEDIUM | custom managers | 3 | regex match_all, strategies |
| LOW | All other managers | ~100 | Various index.ts metadata, small utility functions |

### P1.2 Datasource Partial → Full (2 files)

| File | What's missing |
|---|---|
| `datasource/datasource.ts` | Base class pattern (Rust uses functions) |
| `datasource/postprocess-release.ts` | Per-datasource override wiring |

### P1.3 Versioning Partial → Full (1 file)

| File | What's missing |
|---|---|
| `versioning/common.ts` | GenericVersioningApi inherited functions |

### P1.4 Config Partial → Full (5 files)

| File | What's missing |
|---|---|
| `config/decrypt.ts` | bcpgp/openpgp sub-delegation |
| `config/validation.ts` | Standalone validate() |
| `config/options/index.ts` | Full option definitions |
| `config/presets/*/index.ts` | Preset resolution dispatch |
| `config/validation-helpers/` | Validation helper functions |

### P1.5 Util Partial → Full (27 files)

| Priority | Area | Files | What's missing |
|---|---|---|---|
| HIGH | util/git | ~5 | behind-base-branch, conflicts, modified, update-date caches |
| HIGH | util/http | ~5 | per-host clients, auth, cache |
| HIGH | util/cache | ~4 | package/backend, repository/impl |
| MEDIUM | util/package-rules | ~3 | all matchers (base, current, datasources, etc.) |
| MEDIUM | util/github | ~4 | graphql fetcher, cache strategies |
| MEDIUM | util/fs | ~3 | file system utilities |
| MEDIUM | util/exec | ~3 | child process execution |

### P1.6 Workers Partial → Full (8 files)

| File | What's missing |
|---|---|
| `workers/global/index.ts` | autodiscover, cache init |
| `workers/global/initialize.ts` | cache init, rate-limit init |
| `workers/repository/index.ts` | full pipeline stages |
| `workers/repository/init/*.ts` | cache flush, config errors |
| `workers/repository/process/*.ts` | fetch/lookup pipeline |
| `workers/repository/update/branch/index.ts` | full branch lifecycle |
| `workers/repository/update/pr/index.ts` | PR update, labels, automerge |

---

## Phase 2: Close Not-Started Gaps (496 files)

### P2.1 Manager Artifacts — 40 files

**Strategy:** These require shell command execution infrastructure. Build a generic `ArtifactRunner` trait first, then implement per-manager.

| Priority | Manager | Notes |
|---|---|---|
| HIGH | npm/artifacts.ts | npm/yarn/pnpm lockfile update |
| HIGH | cargo/artifacts.ts | cargo update |
| HIGH | bundler/artifacts.ts | bundle install |
| HIGH | composer/artifacts.ts | composer install |
| HIGH | gomod/artifacts.ts | go mod tidy |
| HIGH | gradle/artifacts.ts | gradle dependencies |
| HIGH | pip_requirements/artifacts.ts | pip install |
| HIGH | poetry/artifacts.ts | poetry lock |
| MEDIUM | helmv3/artifacts.ts | helm dep update |
| MEDIUM | mix/artifacts.ts | mix deps.get |
| MEDIUM | pipenv/artifacts.ts | pipenv lock |
| MEDIUM | nuget/artifacts.ts | dotnet restore |
| MEDIUM | nix/artifacts.ts | nix flake update |
| MEDIUM | swift/artifacts.ts | swift package resolve |
| MEDIUM | deno/artifacts.ts | deno cache |
| MEDIUM | bun/artifacts.ts | bun install |
| MEDIUM | pep621/artifacts.ts | uv/pdm lock |
| MEDIUM | pixi/artifacts.ts | pixi install |
| LOW | bazel, bazel-module, bazelisk, cocoapods, conan, copier, devbox, flux, gleam, gradle-wrapper, helmfile, hermit, jsonnet-bundler, kustomize, maven-wrapper, mise, pub, terragrunt, vendir, git-submodules |

### P2.2 Manager Non-Artifact Not-Started — 18 files

| Priority | File | What's needed |
|---|---|---|
| HIGH | npm/post-update/*.ts (6 files) | Lock file regeneration: npm, yarn, pnpm |
| HIGH | npm/update/package-version/index.ts | Package version bumping |
| HIGH | custom/regex/index.ts | Regex custom manager extraction |
| HIGH | custom/jsonata/index.ts | JSONata custom manager |
| MEDIUM | homebrew/update.ts | URL update in formula |
| MEDIUM | cargo/range.ts | Range strategy |
| MEDIUM | cargo/update.ts | bumpPackageVersion |
| MEDIUM | cargo/update-locked.ts | Update-locked status |
| MEDIUM | git-submodules/update.ts | Update dependency |
| MEDIUM | custom/regex/strategies.ts | Regex strategies |
| LOW | bun/utils.ts, deno/post.ts, terraform/hcl/index.ts |

### P2.3 Datasource Not-Started — 2 files

| Priority | Datasource | What's needed |
|---|---|---|
| MEDIUM | aws-eks-addon | AWS SDK datasource |
| MEDIUM | aws-machine-image | AWS SDK datasource |
| LOW | aws-rds | AWS SDK datasource |
| LOW | azure-tags | Azure API |
| LOW | rpm | RPM repository parsing |
| LOW | github-digest | Digest computation |
| LOW | github-release-attachments | Attachment handling |

### P2.4 Platform Not-Started — 40 files

| Priority | Platform | What's needed |
|---|---|---|
| HIGH | bitbucket/index.ts | Full Bitbucket client |
| HIGH | bitbucket-server/index.ts | Full Bitbucket Server client |
| MEDIUM | gitea/index.ts | Full Gitea client |
| MEDIUM | forgejo/index.ts | Full Forgejo client |
| MEDIUM | azure/index.ts | Full Azure DevOps client |
| LOW | codecommit, gerrit | Niche platforms |
| LOW | platform/comment.ts, scm.ts, default-scm.ts | Platform infrastructure |

### P2.5 Config Not-Started — 102 files

| Priority | Area | Count | What's needed |
|---|---|---|---|
| HIGH | config/migrations/custom/*.ts | 62 | Individual migration files |
| HIGH | config/presets/* | ~20 | Preset resolution (gitlab, http, npm, etc.) |
| MEDIUM | config/options/ | ~8 | Full option definitions |
| MEDIUM | config/decrypt/bcpgp.ts, openpgp.ts | 2 | PGP decryption |
| LOW | config/inherit.ts, utils.ts, parse.ts, schema.ts | 4 | Config infrastructure |

### P2.6 Util Not-Started — 126 files

| Priority | Area | Count | What's needed |
|---|---|---|---|
| HIGH | util/exec/ | ~8 | Shell execution infrastructure (needed for artifacts) |
| HIGH | util/git/ | ~12 | Git operations (clone, commit, push, merge) |
| HIGH | util/http/ | ~20 | HTTP client per-host, auth, caching |
| HIGH | util/cache/ | ~10 | Package/repository cache backends |
| MEDIUM | util/github/ | ~10 | GitHub GraphQL, cache strategies |
| MEDIUM | util/package-rules/ | ~10 | Package rule matchers |
| MEDIUM | util/template/ | ~5 | Handlebars templating |
| MEDIUM | util/fs/ | ~8 | File system utilities |
| MEDIUM | util/merge-confidence/ | ~5 | Merge confidence data |
| LOW | util/*.ts (standalone) | ~38 | Individual utility functions |

### P2.7 Workers Not-Started — 133 files

| Priority | Area | Count | What's needed |
|---|---|---|---|
| HIGH | workers/repository/update/branch/ | 10 | Branch lifecycle (automerge, rebase, status checks, commit) |
| HIGH | workers/repository/process/lookup/ | 7 | Version lookup pipeline (bucket, rollback, generate) |
| HIGH | workers/repository/onboarding/ | 10 | Onboarding branch/PR creation |
| MEDIUM | workers/repository/update/pr/changelog/ | 15 | Changelog/release notes rendering |
| MEDIUM | workers/repository/config-migration/ | 6 | Config migration PR pipeline |
| MEDIUM | workers/repository/finalize/ | 3 | Finalization, pruning |
| MEDIUM | workers/repository/reconfigure/ | 5 | Reconfiguration pipeline |
| MEDIUM | workers/repository/ (top-level) | 8 | Dependency dashboard, error handling, package files |
| LOW | workers/repository/model/ | 2 | Commit message model |
| LOW | workers/global/ | 2 | Autodiscover, additional config |

---

## Execution Order

The plan is executed top-to-bottom within each phase. Phases overlap:

1. **Week 1-2:** P1.1 npm partial → full (highest impact single manager)
2. **Week 1-2:** P1.1 gradle, terraform partial → full
3. **Week 2-3:** P2.6 util/exec + util/git + util/http (infrastructure for artifacts)
4. **Week 3-4:** P2.1 Manager artifacts (using exec infrastructure)
5. **Week 2-4:** P1.5 util partial → full (in parallel with artifacts)
6. **Week 3-5:** P2.5 config migrations (mechanical, can parallelize)
7. **Week 4-6:** P2.7 workers not-started (depends on platform + util)
8. **Week 4-6:** P2.4 platform not-started (Bitbucket, Gitea, etc.)
9. **Week 5-7:** P1.6 workers partial → full
10. **Week 6+:** P2.2, P2.3 remaining items

---

## Progress Tracking

| Phase | Target | Completed | Status |
|---|---|---|---|
| P1.1 Manager partial → full | 222 | 0 | not started |
| P1.2 DS partial → full | 2 | 0 | not started |
| P1.3 Versioning partial → full | 1 | 0 | not started |
| P1.4 Config partial → full | 5 | 0 | not started |
| P1.5 Util partial → full | 27 | 0 | not started |
| P1.6 Workers partial → full | 8 | 0 | not started |
| P2.1 Manager artifacts | 40 | 0 | not started |
| P2.2 Manager non-artifact | 18 | 0 | not started |
| P2.3 Datasource not-started | 7 | 0 | not started |
| P2.4 Platform not-started | 40 | 0 | not started |
| P2.5 Config not-started | 102 | 0 | not started |
| P2.6 Util not-started | 126 | 0 | not started |
| P2.7 Workers not-started | 133 | 0 | not started |
| **Total** | **731** | **0** | — |
