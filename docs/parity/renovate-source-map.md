# Renovate Source Map

Maps Renovate TypeScript source files (`renovate/lib/`) to their Rust
counterparts in this workspace.  Use this file to track port coverage and
find where a given TypeScript module is implemented in Rust.

## Mapping philosophy

- One TypeScript file may map to **multiple** Rust files (a large TS module
  often becomes several focused Rust modules).
- Multiple TypeScript files may collapse into **one** Rust file when the
  abstractions merge naturally in Rust.
- Internal module structure (adapters, factories, type declarations) is
  **not** ported one-to-one; only behavior is tracked.
- Status reflects whether the observable behavior is matched, not whether
  all TypeScript lines have a Rust equivalent.

**Status values:**

| Status | Meaning |
|--------|---------|
| `full` | All observable behavior ported and tested |
| `partial` | Core behavior ported; some edge-cases or options missing |
| `stub` | File recognized; minimal or placeholder implementation |
| `not-started` | No Rust implementation yet |
| `out-of-scope` | Feature is hosted-only, infra, or explicitly deferred |

---

## CLI entry point

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/renovate.ts` | `crates/renovate-cli/src/main.rs` | partial | Core pipeline ported; PR creation / lockfile update deferred |
| `lib/workers/global/config/parse/cli.ts` | `crates/renovate-cli/src/cli.rs`, `crates/renovate-cli/src/config_builder.rs` | partial | All major flags; some advanced flags pending |
| `lib/workers/global/config/parse/env.ts` | `crates/renovate-cli/src/config_builder.rs` | partial | `RENOVATE_TOKEN`, `RENOVATE_PLATFORM`, `LOG_LEVEL` handled |
| `lib/config/migrate.ts` | `crates/renovate-cli/src/migrate.rs` | partial | Legacy flag migration for common flags |
| `lib/config/defaults.ts` | `crates/renovate-core/src/config.rs`, `crates/renovate-core/src/repo_config.rs` | partial | Key defaults ported; some rarely-used defaults pending |
| `lib/config/global.ts` | `crates/renovate-core/src/config.rs` | partial | `GlobalConfig` struct with major fields |
| `lib/config/index.ts` | `crates/renovate-cli/src/config_builder.rs` | partial | merge pipeline: defaults → file → CLI |
| `lib/config/app-strings.ts` | `crates/renovate-core/src/repo_config.rs` | full | `CONFIG_FILE_CANDIDATES` list |
| `lib/config/options/index.ts` | `crates/renovate-core/src/repo_config.rs`, `crates/renovate-core/src/config.rs` | partial | All packageRule matchers, most global options; `major`/`minor`/`patch` config blocks added |

---

## Repository config discovery

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/workers/repository/init/merge.ts` | `crates/renovate-core/src/repo_config.rs` | partial | `discover()` + onboarding detection |
| `lib/config/parse.ts` | `crates/renovate-core/src/repo_config.rs` | partial | JSON/JSON5 parsing; JSON5 via `json5` crate |
| `lib/config/validation.ts` | — | not-started | Schema validation pending |
| `lib/config/inherit.ts` | — | not-started | Organizational config inheritance pending |
| `lib/config/presets/index.ts` | `crates/renovate-core/src/repo_config.rs` | partial | Built-in presets only (`config:recommended`, `:ignoreModulesAndTests`, `:semanticCommits`) |
| `lib/config/presets/internal/config.preset.ts` | `crates/renovate-core/src/repo_config.rs` | partial | `config:recommended`, `config:base` inline expansion |
| `lib/config/presets/internal/default.preset.ts` | `crates/renovate-core/src/repo_config.rs` | partial | `:ignoreModulesAndTests`, `:semanticCommits`, `:semanticCommitsDisabled` |

---

## packageRules evaluation

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/util/package-rules/index.ts` | `crates/renovate-core/src/repo_config.rs` | partial | `matches_context()` + `collect_rule_effects()`; `is_update_blocked_ctx()` + `is_version_restricted_ctx()` for full-context checks |
| `lib/util/package-rules/dep-names.ts` | `crates/renovate-core/src/package_rule.rs` | full | `matchDepNames` exact/regex/glob/negation |
| `lib/util/package-rules/package-names.ts` | `crates/renovate-core/src/package_rule.rs` | full | `matchPackageNames` exact/regex/glob/negation; deprecated fields merged |
| `lib/util/package-rules/files.ts` | `crates/renovate-core/src/package_rule.rs` | full | `matchFileNames` |
| `lib/util/package-rules/base-branches.ts` | `crates/renovate-core/src/package_rule.rs` | full | `matchBaseBranches` |
| `lib/util/package-rules/base.ts` | — | out-of-scope | Abstract base class — internal TypeScript abstraction |
| `lib/util/package-rules/matchers.ts` | — | out-of-scope | Matcher registry — internal TypeScript abstraction |
| `lib/util/package-rules/types.ts` | — | out-of-scope | Type definitions only |
| `lib/util/package-rules/merge-confidence.ts` | — | not-started | `matchConfidence` (MergeConfidence, hosted only) |
| `lib/util/package-rules/datasources.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchDatasources` |
| `lib/util/package-rules/managers.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchManagers` |
| `lib/util/package-rules/update-types.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchUpdateTypes` |
| `lib/util/package-rules/categories.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchCategories` |
| `lib/util/package-rules/sourceurls.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchSourceUrls` |
| `lib/util/package-rules/registryurls.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchRegistryUrls` |
| `lib/util/package-rules/repositories.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchRepositories` |
| `lib/util/package-rules/base-branch.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchBaseBranches` |
| `lib/util/package-rules/current-value.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchCurrentValue` |
| `lib/util/package-rules/new-value.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchNewValue` |
| `lib/util/package-rules/current-version.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchCurrentVersion` |
| `lib/util/package-rules/current-age.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchCurrentAge` |
| `lib/util/package-rules/file-names.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchFileNames` |
| `lib/util/package-rules/dep-types.ts` | `crates/renovate-core/src/repo_config.rs` | full | `matchDepTypes` |
| `lib/util/package-rules/confidence.ts` | — | not-started | `matchConfidence` (MergeConfidence, hosted only) |
| `lib/util/package-rules/jsonata.ts` | — | not-started | `matchJsonata` (complex expression engine) |
| `lib/util/string-match.ts` | `crates/renovate-core/src/string_match.rs` | full | `match_regex_or_glob`, `match_regex_or_glob_list` with negation |

---

## Schedule and release age

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/workers/repository/updates/schedule.ts` | `crates/renovate-core/src/schedule.rs` | partial | POSIX cron + later.js text DSL; timezone-aware via `chrono-tz`; missing: full later.js coverage |
| `lib/util/pretty-time.ts` | `crates/renovate-core/src/schedule.rs` | partial | `parse_age_duration`, `satisfies_date_range`, `is_within_release_age` |
| `lib/config/options/index.ts` (`minimumReleaseAge`) | `crates/renovate-core/src/schedule.rs`, `crates/renovate-cli/src/main.rs` | partial | Implemented for npm/pypi; crates.io pending |

---

## Platform clients

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/modules/platform/github/index.ts` | `crates/renovate-core/src/platform/github.rs` | partial | Auth, file listing, raw file fetch |
| `lib/modules/platform/gitlab/index.ts` | `crates/renovate-core/src/platform/gitlab.rs` | partial | Auth, file listing, raw file fetch |
| `lib/modules/platform/local/index.ts` | `crates/renovate-core/src/platform/local.rs` | partial | `git ls-files`, filesystem reads; no PR creation |
| `lib/modules/platform/azure/index.ts` | — | not-started | |
| `lib/modules/platform/bitbucket/index.ts` | — | not-started | |
| `lib/modules/platform/gitea/index.ts` | — | not-started | |
| `lib/modules/platform/forgejo/index.ts` | — | not-started | |

---

## Datasources

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/modules/datasource/npm/index.ts` | `crates/renovate-core/src/datasources/npm.rs` | partial | Packument fetch, version sort, timestamps; dist-tags partial |
| `lib/modules/datasource/pypi/index.ts` | `crates/renovate-core/src/datasources/pypi.rs` | partial | JSON API, yanked filtering, timestamps |
| `lib/modules/datasource/docker/index.ts` | `crates/renovate-core/src/datasources/docker_hub.rs` | partial | Docker Hub tags; no digest pinning |
| `lib/modules/datasource/github-releases/index.ts` | `crates/renovate-core/src/datasources/github_releases.rs` | partial | Latest release, version list; `release_timestamp` via `published_at` |
| `lib/modules/datasource/github-tags/index.ts` | `crates/renovate-core/src/datasources/github_tags.rs` | partial | Tag list, semantic sorting |
| `lib/modules/datasource/maven/index.ts` | `crates/renovate-core/src/datasources/maven.rs` | partial | Maven Central metadata |
| `lib/modules/datasource/gradle-version/index.ts` | `crates/renovate-core/src/datasources/gradle.rs` | partial | Gradle versions JSON |
| `lib/modules/datasource/nuget/index.ts` | `crates/renovate-core/src/datasources/nuget.rs` | partial | NuGet v3 index |
| `lib/modules/datasource/terraform-module/index.ts` | `crates/renovate-core/src/datasources/terraform.rs` | partial | Terraform Registry |
| `lib/modules/datasource/helm/index.ts` | `crates/renovate-core/src/datasources/helm.rs` | partial | Helm chart index.yaml |
| `lib/modules/datasource/go/index.ts` | `crates/renovate-core/src/datasources/gomod.rs` | partial | `go list -m` style fetch |
| `lib/modules/datasource/rubygems/index.ts` | `crates/renovate-core/src/datasources/rubygems.rs` | partial | RubyGems API; `release_timestamp` via `created_at` field |
| `lib/modules/datasource/packagist/index.ts` | `crates/renovate-core/src/datasources/packagist.rs` | partial | Packagist v2 API |
| `lib/modules/datasource/pub/index.ts` | `crates/renovate-core/src/datasources/pub_dev.rs` | partial | pub.dev API |
| `lib/modules/datasource/crate/index.ts` | `crates/renovate-core/src/datasources/crates_io.rs` | partial | Sparse index + REST API timestamps (`/api/v1/crates/{name}/versions`) |
| `lib/modules/datasource/crate/schema.ts` | `crates/renovate-core/src/datasources/crates_io.rs` | partial | `ReleaseTimestamp` / `created_at` via REST API |
| `lib/modules/datasource/bitrise-step/index.ts` | `crates/renovate-core/src/datasources/bitrise.rs` | partial | Bitrise steplib |
| `lib/modules/datasource/conda/index.ts` | `crates/renovate-core/src/datasources/conda.rs` | partial | Anaconda API |
| `lib/modules/datasource/jsr/index.ts` | `crates/renovate-core/src/datasources/jsr.rs` | partial | JSR registry |
| `lib/modules/datasource/endoflife-date/index.ts` | `crates/renovate-core/src/datasources/endoflife_date.rs` | partial | EOL date API |

---

## Versioning schemes

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/modules/versioning/semver/index.ts` | `crates/renovate-core/src/versioning/semver_generic.rs` | partial | Parse, compare, classify; not full semver API |
| `lib/modules/versioning/npm/index.ts` | `crates/renovate-core/src/versioning/npm.rs` | partial | npm range resolution (`^`, `~`, `>=`, exact) |
| `lib/modules/versioning/pep440/index.ts` | `crates/renovate-core/src/versioning/pep440.rs` | partial | PEP 440 comparison, `==` specifier resolution |
| `lib/modules/versioning/cargo/index.ts` | `crates/renovate-core/src/versioning/cargo.rs` | partial | Cargo semver ranges |
| `lib/modules/versioning/ruby/index.ts` | `crates/renovate-core/src/versioning/ruby.rs` | partial | Gem version ranges |
| `lib/modules/versioning/docker/index.ts` | `crates/renovate-core/src/datasources/docker_hub.rs` | partial | Tag comparison + suffix filter inline in datasource; no separate module |
| `lib/modules/versioning/hashicorp/index.ts` | `crates/renovate-core/src/versioning/hashicorp.rs` | partial | `lower_bound`, `hashicorp_update_summary`; `getNewValue` (range update) not yet ported |
| `lib/modules/versioning/hashicorp/convertor.ts` | `crates/renovate-core/src/versioning/hashicorp.rs` | partial | Constraint parsing; `hashicorp2npm`/`npm2hashicorp` conversion not ported |
| `lib/modules/versioning/composer/index.ts` | — | not-started | Composer semver |

---

## Package manager extractors

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/modules/manager/cargo/extract.ts` | `crates/renovate-core/src/extractors/cargo.rs` | partial | `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`; workspace deps partial |
| `lib/modules/manager/npm/extract.ts` | `crates/renovate-core/src/extractors/npm.rs` | partial | `dependencies`, `devDependencies`, `peerDependencies`, `optionalDependencies`, `engines` |
| `lib/modules/manager/pip_requirements/extract.ts` | `crates/renovate-core/src/extractors/pip.rs` | partial | Requirements files; VCS deps skipped |
| `lib/modules/manager/pip-compile/extract.ts` | `crates/renovate-core/src/extractors/pip_compile.rs` | partial | `.in` source file parsing |
| `lib/modules/manager/github-actions/extract.ts` | `crates/renovate-core/src/extractors/github_actions.rs` | partial | `uses:` action references |
| `lib/modules/manager/dockerfile/extract.ts` | (docker pipeline in `main.rs`) | partial | `FROM` + `ARG`/`ENV` base image |
| `lib/modules/manager/maven/extract.ts` | `crates/renovate-core/src/extractors/maven.rs` | partial | `pom.xml` dependency coords |
| `lib/modules/manager/gradle/extract.ts` | `crates/renovate-core/src/extractors/gradle.rs` | partial | `build.gradle` dependencies |
| `lib/modules/manager/terraform/extract.ts` | `crates/renovate-core/src/extractors/terraform.rs` | partial | `source`, `version` fields |
| `lib/modules/manager/helm-values/extract.ts` | `crates/renovate-core/src/extractors/helm.rs` | partial | `Chart.yaml`, `requirements.yaml` |
| `lib/modules/manager/poetry/extract.ts` | `crates/renovate-core/src/extractors/poetry.rs` | partial | `pyproject.toml` poetry deps |
| `lib/modules/manager/bundler/extract.ts` | `crates/renovate-core/src/extractors/bundler.rs` | partial | `Gemfile` deps |
| `lib/modules/manager/composer/extract.ts` | `crates/renovate-core/src/extractors/composer.rs` | partial | `composer.json` deps |
| `lib/modules/manager/gomod/extract.ts` | `crates/renovate-core/src/extractors/gomod.rs` | partial | `go.mod` `require` stanzas |
| `lib/modules/manager/nuget/extract.ts` | `crates/renovate-core/src/extractors/nuget.rs` | partial | `.csproj`, `.props`, `.targets` |
| `lib/modules/manager/pub/extract.ts` | `crates/renovate-core/src/extractors/pubspec.rs` | partial | `pubspec.yaml` deps |
| `lib/modules/manager/pep621/extract.ts` | `crates/renovate-core/src/extractors/pep621.rs` | partial | `[project.dependencies]` |
| `lib/modules/manager/setup-cfg/extract.ts` | `crates/renovate-core/src/extractors/setup_cfg.rs` | partial | `install_requires`, `extras_require` |
| `lib/modules/manager/git-submodules/extract.ts` | `crates/renovate-core/src/extractors/git_submodules.rs` | partial | `.gitmodules` parsing |
| `lib/modules/manager/github-actions/extract.ts` | `crates/renovate-core/src/extractors/github_actions.rs` | partial | workflow `uses:` steps |
| `lib/modules/manager/homebrew/extract.ts` | `crates/renovate-core/src/extractors/homebrew.rs` | stub | URL + checksum pattern |
| `lib/modules/manager/gradle/extract.ts` | `crates/renovate-core/src/extractors/gradle.rs` | partial | TOML version catalog + plugins block |

---

## Branch and PR logic

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/workers/repository/updates/flatten.ts` | `crates/renovate-core/src/branch.rs` | partial | `sanitize_dep_name`, `branch_topic`, `branch_name` |
| `lib/workers/repository/updates/branch-name.ts` | `crates/renovate-core/src/branch.rs` | partial | Branch name generation; group slug; `hashedBranchLength` hashing; `major_group_slug()` prefix for `separateMajorMinor`/`separateMultipleMajor` |
| `lib/util/hash.ts` | `crates/renovate-core/src/branch.rs` | full | SHA-512 hex hashing via `sha2` crate |
| `lib/config/options/index.ts` (`commitMessageAction`, `commitMessagePrefix`) | `crates/renovate-core/src/branch.rs`, `crates/renovate-core/src/repo_config.rs` | partial | PR title generation with semantic commits |

---

## Utilities

| TypeScript file | Rust file(s) | Status | Notes |
|-----------------|-------------|--------|-------|
| `lib/util/pretty-time.ts` | `crates/renovate-core/src/schedule.rs` | partial | `parse_age_duration`, `satisfies_date_range` |
| `lib/util/http/index.ts` | `crates/renovate-core/src/http.rs` | partial | Retry logic, user-agent, timeouts |
| `lib/logger/index.ts` | `crates/renovate-cli/src/logging.rs` | partial | JSON log mode, level from env |
| `lib/config/options/env.ts` | `crates/renovate-cli/src/config_builder.rs` | partial | Key env vars handled |

---

## Out of scope (hosted / infra only)

| TypeScript file | Notes |
|-----------------|-------|
| `lib/workers/repository/updates/prWorker.ts` | PR creation, branch push — requires git operations |
| `lib/workers/repository/process/lookup/index.ts` | Lookup worker orchestration — partially in `main.rs` |
| `lib/instrumentation/` | Telemetry, OpenTelemetry integration |
| `lib/config/decrypt/` | Config decryption (PGP) |
| `lib/workers/global/autodiscover.ts` | GitHub/GitLab auto-discovery |
| `lib/workers/global/initialize.ts` | Global init (caching, auth setup) |

---

## Maintenance notes

- Update this file whenever a TypeScript source file is newly ported or its
  port status changes.
- Status should reflect **observable behavior coverage**, not line count.
- When a Rust file covers multiple TypeScript files, list all of them as
  separate rows pointing to the same Rust file.
- When a TypeScript file is not yet started, still list it here with
  `not-started` so the coverage gap is visible.
- This table is the canonical reference for "did we port X?" — keep it current.
