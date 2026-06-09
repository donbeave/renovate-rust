# Source Mapping — `cli`

[← all groups](README.md)

**Coverage:** 33/150 in-scope files mapped (full=10 partial=23 stub=0 pending=117 out-of-scope=0 opt-out=10) across 15 modules.

### `commander.d.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/commander.d.ts` | opt-out | — | Type declaration shim for Commander typings; no Rust runtime analogue is needed. |

### `config-validator.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/config-validator.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `data-files.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/data-files.generated.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `datasource-list.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/datasource-list.generated.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `expose.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/expose.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `global-config-option-defaults.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/global-config-option-defaults.generated.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `globals.d.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/globals.d.ts` | opt-out | — | TypeScript ambient declarations for global interfaces and module shims; no Rust runtime analogue. |

### `manager-default-configs.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/manager-default-configs.generated.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `manager-list.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/manager-list.generated.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `proxy.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/proxy.ts` | full | [`crates/renovate-core/src/proxy.rs`](../../../crates/renovate-core/src/proxy.rs) | — |

### `renovate.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/renovate.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `versioning-list.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/versioning-list.generated.ts` | partial | [`crates/renovate-core/src/lib.rs`](../../../crates/renovate-core/src/lib.rs) | — |

### `workers/_root`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/workers/types.ts` | opt-out | — | Type-only interface and type aliases used for TypeScript compile-time type safety; no Rust runtime equivalent. |

### `workers/global`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/workers/global/autodiscover.ts` | partial | [`crates/renovate-core/src/workers/global/autodiscover.rs`](../../../crates/renovate-core/src/workers/global/autodiscover.rs) | local platform special case + proper regex/glob filter via shared match_regex_or_glob_list (the full platform.getRepos + AutodiscoverConfig building + pre-configured repo merge logic is orchestrated at CLI level in the current Rust architecture). |
| `lib/workers/global/config/parse/additional-config-file.ts` | partial | [`crates/renovate-core/src/config/file.rs`](../../../crates/renovate-core/src/config/file.rs) | parse-and-load support for `RENOVATE_ADDITIONAL_CONFIG_FILE` is implemented, including `processEnv` export and optional post-load deletion, but JS/yaml configs remain unsupported. |
| `lib/workers/global/config/parse/cli.ts` | full | [`crates/renovate-cli/src/migrate.rs`](../../../crates/renovate-cli/src/migrate.rs) | migrateArgs (exact ordered rewrites for legacy flags, JSON keys inside host-rules values, bare --dry-run/--require-config, --git-fs* drop), plus the getCliName equivalent in util and the clap program surface + getConfig collection/specials in cli.rs + config_builder. The migrate pipeline + special dryRun/requireConfig mappings after parse are covered. |
| `lib/workers/global/config/parse/codespaces.ts` | full | [`crates/renovate-cli/src/config_codespaces.rs`](../../../crates/renovate-cli/src/config_codespaces.rs) | in GitHub Codespaces, infer token from `GITHUB_TOKEN` and prompt for repository when none are configured. |
| `lib/workers/global/config/parse/coersions.ts` | full | [`crates/renovate-cli/src/config_env.rs`](../../../crates/renovate-cli/src/config_env.rs) | boolean (''/true->true, false->false or error), array (JSON5 array or csv split+trim+filter non-empty), object (JSON5 or {} or error), string (\n->actual newline), integer (parseInt). The parse_string_array / parse_*_json_* / parse_string_list etc in this file (and mappers in cli.rs + config_builder) implement the coersions used by env.ts getConfig and cli.ts. |
| `lib/workers/global/config/parse/env.ts` | full | [`crates/renovate-cli/src/config_env.rs`](../../../crates/renovate-cli/src/config_env.rs) | env prefix normalization, key renaming (legacy + migrated), experimental X_ var massaging to current names, RENOVATE_CONFIG merge, per-option coercion via env, special boolean mappings for dryRun/requireConfig/platformCommit, GITHUB_COM_TOKEN -> hostRule, deletion of unsupported legacy env names. Matches the prepareEnv + getConfig surface for self-hosted runs. |
| `lib/workers/global/config/parse/file.ts` | partial | [`crates/renovate-core/src/config/file.rs`](../../../crates/renovate-core/src/config/file.rs) | JSON/JSON5 parser and non-default file cleanup are implemented; CLI/global env integration and some migrate/validation flows are staged elsewhere. |
| `lib/workers/global/config/parse/host-rules-from-env.ts` | full | [`crates/renovate-core/src/config/host_rules_from_env.rs`](../../../crates/renovate-core/src/config/host_rules_from_env.rs) | parsing of RENOVATE_<HOST>[_PARTS]_[TOKEN|USERNAME|PASSWORD|https*] env vars into hostRules (with __ -> - , hosttype validation, https field restore), including precedence for GITHUB_COM_TOKEN. Matches hostRulesFromEnv surface. |
| `lib/workers/global/config/parse/index.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse/index.rs`](../../../crates/renovate-core/src/workers/global/config/parse/index.rs) | low-level `parse_config` / `parse_config_file` (now with JSON5 support for trailing commas/comments to match upstream config file flexibility and the usage inside the `parseConfigs` composition). The high-level `parseConfigs` (merging of defaults + file + additional + cli + env, globalExtends resolution, detectGlobalManagerConfig, detectHostRulesFromEnv, repository override warning, various massaging, private key loading, secrets/variables application, configFileNames, etc.) is implemented in the CLI layer (`config_builder.rs`, `main.rs`, and the sub-parsers) in the current Rust architecture. |
| `lib/workers/global/config/parse/types.ts` | opt-out | — | Type-only parse options type aliases used only by TypeScript configuration validation typing. |
| `lib/workers/global/config/parse/util.ts` | full | [`crates/renovate-core/src/config/migrate_validate.rs`](../../../crates/renovate-core/src/config/migrate_validate.rs) | migrateAndValidateConfig (migrateConfig + massageConfig + validateConfig('global'), logging of warnings/errors but return the massagedConfig). The core is implemented here as migrate_and_validate + validate_config_for_source (used by the global parse flow in index.ts / config_builder). getParsedContent file type dispatch is in the file parser layer. |
| `lib/workers/global/index.ts` | partial | [`crates/renovate-core/src/workers/global/index.rs`](../../../crates/renovate-core/src/workers/global/index.rs) | top-level global flow composition (calls to parseConfigs, autodiscoverRepositories, globalInitialize, isLimitReached, getRepositoryConfig, and spawning repository workers). In the current Rust architecture the full orchestration lives in the CLI main.rs + this stub + the sub modules (initialize, autodiscover, config/parse/index, limits) + repo_config + repository worker. The stub here wires the ported global pieces. |
| `lib/workers/global/initialize.ts` | partial | [`crates/renovate-core/src/workers/global/initialize.rs`](../../../crates/renovate-core/src/workers/global/initialize.rs) | git version check+error, directory (base/cache/containerbase) computation+ensure, host rules add (legacy too), commits limit, emoji config, third-party metadata env intent, global finalize stub; rate limits entry; (platform init, packageCache full init, merge-confidence, secret apply, and top-level global flow live in main.rs + config + platform for the broader workers/global/index.ts surface). |
| `lib/workers/global/limits.ts` | full | [`crates/renovate-core/src/limits.rs`](../../../crates/renovate-core/src/limits.rs) | simple 'Commits' limit (setMaxLimit/inc/isLimitReached + reset), plus the full concurrent/hourly/branch limit machinery (calcLimit, hasMultipleLimits, isLimitReached overloads for Branches/ConcurrentPRs/Hourly*, using per-upgrade limits with the null-inherit semantics for branchConcurrentLimit). All observable behavior for a self-hosted renovate run is covered. |

### `workers/repository`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/workers/repository/cache.ts` | partial | [`crates/renovate-core/src/workers/repository/cache.rs`](../../../crates/renovate-core/src/workers/repository/cache.rs) | set_cache + BranchCacheEntry/BranchUpgradeCacheEntry projection (generate_branch_upgrade_cache_entry + population of shas, is_* flags, commit_fingerprint, upgrades list etc from BranchConfig which carries precomputed values in Rust flow) implemented. The async generateBranchCache (scm.getBranchCommit + platform.getBranchPr + getCachedPristine/Modified/Behind/Conflict + commit date + prCache) + side-effect `getCache().branches = ...` + full RepoCacheData integration (load/save) live in other repository/* + util/cache/repository modules in the current architecture. |
| `lib/workers/repository/changelog/index.ts` | partial | [`crates/renovate-core/src/workers/repository/changelog/index.rs`](../../../crates/renovate-core/src/workers/repository/changelog/index.rs) | embedChangelog + embedChangelogs (stage filter on fetchChangeLogs, pre-provided changelogContent synthetic path, delegation to get / skip if logJSON already set) implemented using EmbeddableUpgrade + the ChangeLog* types. The actual getChangeLogJSON (release notes fetch), wiring into BranchUpgrade during branchify/update/pr, and full PR body rendering live in other (pending) repository/update/pr/changelog and branch modules. |
| `lib/workers/repository/changelog/types.ts` | opt-out | — | Type-only changelog option/result type aliases with no runtime behavior in Rust implementation. |
| `lib/workers/repository/common.ts` | partial | [`crates/renovate-core/src/workers/repository/common.rs`](../../../crates/renovate-core/src/workers/repository/common.rs) | extractRepoProblems + formatProblemLevel added (format also present in branch.rs for other use; extract is stub pending full logger problems integration in problem-stream). The cache types (BaseBranchCache, PackageFile) were already here as "related". |
| `lib/workers/repository/config-migration/branch/commit-message.ts` | full | [`crates/renovate-core/src/branch.rs`](../../../crates/renovate-core/src/branch.rs) | getCommitMessage / getPrTitle (ConfigMigrationCommitMessageFactory) using tweaked scope + custom commitMessage template support when provided (empty falls back to default topic-based). The fns are the direct surface for creating the migration branch/PR commit message. |
| `lib/workers/repository/config-migration/branch/create.ts` | partial | [`crates/renovate-core/src/branch.rs`](../../../crates/renovate-core/src/branch.rs) | createConfigMigrationBranch uses the ConfigMigrationCommitMessageFactory (getCommitMessage/getPrTitle with custom support) to get the message and prTitle for the migration branch; dryRun early return, checkout, MigratedData prettier, file changes (config + optional package.json renovate field cleanup), and scm.commitAndPush (force, platformCommit) are in the (pending) worker/index orchestration. |
| `lib/workers/repository/config-migration/branch/index.ts` | partial | [`crates/renovate-core/src/branch.rs`](../../../crates/renovate-core/src/branch.rs) | checkConfigMigrationBranch orchestrator (checkbox state, PR/branch existence via platform, closed PR handling, create vs rebase decision, return migrationBranch); uses the ConfigMigrationCommitMessageFactory and helpers from here (full worker orchestration noted as pending in siblings). |
| `lib/workers/repository/config-migration/branch/migrated-data.ts` | partial | [`crates/renovate-core/src/json_writer.rs`](../../../crates/renovate-core/src/json_writer.rs) | MigratedData, MigratedDataFactory (getAsync singleton, reset, applyPrettierFormatting using detect/migrate/weave/stringify + prettier if config/editorconfig/package.json), Indent; the build of migrated config data for create/index (full platform/scm/migrate integration pending in worker). |
| `lib/workers/repository/config-migration/branch/rebase.ts` | partial | [`crates/renovate-core/src/json_writer.rs`](../../../crates/renovate-core/src/json_writer.rs) | jsonStripWhitespaces (the strip for migration rebase); uses the json write/strip surface here (full rebase logic in pending worker). |
| `lib/workers/repository/config-migration/common.ts` | full | [`crates/renovate-core/src/branch.rs`](../../../crates/renovate-core/src/branch.rs) | getMigrationBranchName (the template for the migrate-config branch name used by create, rebase, pr, index etc.). |
| `lib/workers/repository/config-migration/index.ts` | partial | [`crates/renovate-core/src/config/migration.rs`](../../../crates/renovate-core/src/config/migration.rs) | configMigration orchestrator (the top-level glue: silent mode, get migrated data via factory, check branch, push to branchList, ensure PR, return result for dashboard; full check/ensure/PR creation in pending worker submodules, using the service here for the core migrate/is_migrated). |
| `lib/workers/repository/config-migration/pr/index.ts` | pending | — | — |
| `lib/workers/repository/configured.ts` | pending | — | — |
| `lib/workers/repository/dependency-dashboard.ts` | pending | — | — |
| `lib/workers/repository/error-config.ts` | pending | — | — |
| `lib/workers/repository/error.ts` | pending | — | — |
| `lib/workers/repository/errors-warnings.ts` | pending | — | — |
| `lib/workers/repository/extract/extract-fingerprint-config.ts` | pending | — | — |
| `lib/workers/repository/extract/file-match.ts` | pending | — | — |
| `lib/workers/repository/extract/index.ts` | pending | — | — |
| `lib/workers/repository/extract/manager-files.ts` | pending | — | — |
| `lib/workers/repository/extract/supersedes.ts` | pending | — | — |
| `lib/workers/repository/extract/types.ts` | opt-out | — | Type-only interface used for TypeScript compile-time typing in worker extraction output. |
| `lib/workers/repository/finalize/index.ts` | pending | — | — |
| `lib/workers/repository/finalize/prune.ts` | pending | — | — |
| `lib/workers/repository/finalize/repository-statistics.ts` | pending | — | — |
| `lib/workers/repository/index.ts` | pending | — | — |
| `lib/workers/repository/init/apis.ts` | pending | — | — |
| `lib/workers/repository/init/cache.ts` | pending | — | — |
| `lib/workers/repository/init/config.ts` | pending | — | — |
| `lib/workers/repository/init/index.ts` | pending | — | — |
| `lib/workers/repository/init/inherited.ts` | pending | — | — |
| `lib/workers/repository/init/merge.ts` | pending | — | — |
| `lib/workers/repository/init/types.ts` | opt-out | — | Type-only repository initialization interfaces used only for TypeScript compile-time handoff typing. |
| `lib/workers/repository/init/vulnerability.ts` | pending | — | — |
| `lib/workers/repository/model/commit-message-factory.ts` | pending | — | — |
| `lib/workers/repository/model/commit-message.ts` | pending | — | — |
| `lib/workers/repository/model/custom-commit-message.ts` | pending | — | — |
| `lib/workers/repository/model/semantic-commit-message.ts` | pending | — | — |
| `lib/workers/repository/onboarding/branch/check.ts` | pending | — | — |
| `lib/workers/repository/onboarding/branch/commit-message.ts` | pending | — | — |
| `lib/workers/repository/onboarding/branch/config.ts` | pending | — | — |
| `lib/workers/repository/onboarding/branch/create.ts` | pending | — | — |
| `lib/workers/repository/onboarding/branch/index.ts` | pending | — | — |
| `lib/workers/repository/onboarding/branch/onboarding-branch-cache.ts` | pending | — | — |
| `lib/workers/repository/onboarding/branch/rebase.ts` | pending | — | — |
| `lib/workers/repository/onboarding/common.ts` | pending | — | — |
| `lib/workers/repository/onboarding/pr/base-branch.ts` | pending | — | — |
| `lib/workers/repository/onboarding/pr/config-description.ts` | pending | — | — |
| `lib/workers/repository/onboarding/pr/index.ts` | pending | — | — |
| `lib/workers/repository/onboarding/pr/pr-list.ts` | pending | — | — |
| `lib/workers/repository/package-files.ts` | pending | — | — |
| `lib/workers/repository/process/extract-update.ts` | pending | — | — |
| `lib/workers/repository/process/fetch.ts` | pending | — | — |
| `lib/workers/repository/process/fingerprint-fields.ts` | pending | — | — |
| `lib/workers/repository/process/index.ts` | pending | — | — |
| `lib/workers/repository/process/libyear.ts` | pending | — | — |
| `lib/workers/repository/process/limits.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/abandonment.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/bucket.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/current.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/filter-checks.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/filter.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/generate.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/index.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/rollback.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/timestamps.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/types.ts` | opt-out | — | Type-only lookup/result interfaces used only for TypeScript compile-time types. |
| `lib/workers/repository/process/lookup/update-type.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/utils.ts` | pending | — | — |
| `lib/workers/repository/process/sort.ts` | pending | — | — |
| `lib/workers/repository/process/types.ts` | opt-out | — | Type-only vulnerability/process interfaces used only for TypeScript compile-time typing. |
| `lib/workers/repository/process/vulnerabilities.ts` | pending | — | — |
| `lib/workers/repository/process/write.ts` | pending | — | — |
| `lib/workers/repository/reconfigure/comment.ts` | pending | — | — |
| `lib/workers/repository/reconfigure/index.ts` | pending | — | — |
| `lib/workers/repository/reconfigure/reconfigure-cache.ts` | pending | — | — |
| `lib/workers/repository/reconfigure/utils.ts` | pending | — | — |
| `lib/workers/repository/reconfigure/validate.ts` | pending | — | — |
| `lib/workers/repository/result.ts` | pending | — | — |
| `lib/workers/repository/update/branch/artifacts.ts` | pending | — | — |
| `lib/workers/repository/update/branch/auto-replace.ts` | pending | — | — |
| `lib/workers/repository/update/branch/automerge.ts` | pending | — | — |
| `lib/workers/repository/update/branch/bump-versions.ts` | pending | — | — |
| `lib/workers/repository/update/branch/check-existing.ts` | pending | — | — |
| `lib/workers/repository/update/branch/commit.ts` | pending | — | — |
| `lib/workers/repository/update/branch/execute-post-upgrade-commands.ts` | pending | — | — |
| `lib/workers/repository/update/branch/get-updated.ts` | pending | — | — |
| `lib/workers/repository/update/branch/handle-existing.ts` | pending | — | — |
| `lib/workers/repository/update/branch/index.ts` | pending | — | — |
| `lib/workers/repository/update/branch/reuse.ts` | pending | — | — |
| `lib/workers/repository/update/branch/schedule.ts` | pending | — | — |
| `lib/workers/repository/update/branch/status-checks.ts` | pending | — | — |
| `lib/workers/repository/update/pr/automerge.ts` | pending | — | — |
| `lib/workers/repository/update/pr/body/changelogs.ts` | pending | — | — |
| `lib/workers/repository/update/pr/body/config-description.ts` | pending | — | — |
| `lib/workers/repository/update/pr/body/controls.ts` | pending | — | — |
| `lib/workers/repository/update/pr/body/footer.ts` | pending | — | — |
| `lib/workers/repository/update/pr/body/header.ts` | pending | — | — |
| `lib/workers/repository/update/pr/body/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/body/notes.ts` | pending | — | — |
| `lib/workers/repository/update/pr/body/updates-table.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/api.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/bitbucket-server/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/bitbucket-server/source.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/bitbucket/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/bitbucket/source.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/common.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/forgejo/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/forgejo/source.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/gitea/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/gitea/source.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/github/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/github/source.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/gitlab/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/gitlab/source.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/hbs-template.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/release-notes.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/releases.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/source.ts` | pending | — | — |
| `lib/workers/repository/update/pr/changelog/types.ts` | opt-out | — | Type-only changelog result types used only for TypeScript compile-time data contracts. |
| `lib/workers/repository/update/pr/code-owners.ts` | pending | — | — |
| `lib/workers/repository/update/pr/index.ts` | pending | — | — |
| `lib/workers/repository/update/pr/labels.ts` | pending | — | — |
| `lib/workers/repository/update/pr/participants.ts` | pending | — | — |
| `lib/workers/repository/update/pr/pr-cache.ts` | pending | — | — |
| `lib/workers/repository/update/pr/pr-fingerprint.ts` | pending | — | — |
| `lib/workers/repository/update/pr/pr-reuse.ts` | pending | — | — |
| `lib/workers/repository/updates/branch-name.ts` | pending | — | — |
| `lib/workers/repository/updates/branchify.ts` | pending | — | — |
| `lib/workers/repository/updates/flatten.ts` | pending | — | — |
| `lib/workers/repository/updates/generate.ts` | pending | — | — |

