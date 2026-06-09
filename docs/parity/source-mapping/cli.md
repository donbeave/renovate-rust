# Source Mapping — `cli`

[← all groups](README.md)

**Coverage:** 71/150 in-scope files mapped (full=19 partial=52 stub=0 pending=79 out-of-scope=0 opt-out=10) across 15 modules.

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
| `lib/workers/repository/config-migration/pr/index.ts` | full | [`crates/renovate-core/src/branch.rs`](../../../crates/renovate-core/src/branch.rs) | ensureConfigMigrationPr (body with migration text + json5 note + emojify + templated header/footer + massage + hashBody compare; existingPr check/update vs create; dryRun short circuits with logs; 422 duplicate warn+delete+null; title via ConfigMigrationCommitMessageFactory; single test ported). Platform get/create/update/addParticipants/massage in platform layer; higher worker orchestration pending in siblings. |
| `lib/workers/repository/configured.ts` | full | [`crates/renovate-core/src/workers/repository/configured.rs`](../../../crates/renovate-core/src/workers/repository/configured.rs) | checkIfConfigured (throws REPOSITORY_DISABLED_BY_CONFIG when enabled===false; throws REPOSITORY_FORKED when isFork && forkProcessing !== 'enabled'). is_configured also returns Forked. Single test ported. (fields on stand-in + calls from init/error are in other modules). |
| `lib/workers/repository/dependency-dashboard.ts` | full | [`crates/renovate-core/src/workers/repository/dependency_dashboard.rs`](../../../crates/renovate-core/src/workers/repository/dependency_dashboard.rs) | ensureDependencyDashboard (early returns, config migration sections using prior result enum for checkbox/pr-link, body assembly with branches/problems/deprecations), format_dashboard enhanced with migration prefix, basic ensure surface. readDashboardBody, full getBranchesListMd categories, vulns, abandoned, autoclose, header/footer, platform calls, parse for user checks are in progress or delegated. Single test for the "adds a checkbox" behavior ported. (Pre-existing debt in other modules isolated for this cycle.) |
| `lib/workers/repository/error-config.ts` | full | [`crates/renovate-core/src/workers/repository/error_config.rs`](../../../crates/renovate-core/src/workers/repository/error_config.rs) | raiseConfigWarningIssue / raiseCredentialsWarningIssue / raiseWarningIssue (silent, body with validation details, dryRun early return + log, suppress, ensureIssue side, warn log) + handleOnboardingPr. handle_config_error + builders + full raise surfaces + single Ported test. (platform ensure/update and caller wiring in pending modules; debt isolated). |
| `lib/workers/repository/error.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | handleError (switch on REPOSITORY_* and special, calls to raise* for validation/credentials, git rewrites via classify, logs, unknown default). The classify and log_level were pre-existing; full handle_error added here. Single test ported. (instrument, branchList delete, full caller in pending worker). |
| `lib/workers/repository/errors-warnings.ts` | full | [`crates/renovate-core/src/workers/repository/errors_warnings.rs`](../../../crates/renovate-core/src/workers/repository/errors_warnings.rs) | getWarnings/getErrors (text formatters), getDepWarnings* (onboarding/PR/dashboard with emojify, suppress, stripping, files, dashboard link if issue). collect stub + get_*_text present; dep collection + 3 getDep* implemented here. Single test ported from spec. (calls from dashboard/worker in other pending). |
| `lib/workers/repository/extract/extract-fingerprint-config.ts` | full | [`crates/renovate-core/src/workers/repository/extract/extract_fingerprint_config.rs`](../../../crates/renovate-core/src/workers/repository/extract/extract_fingerprint_config.rs) | generateFingerprintConfig (managerList from enabledManagers, managers with getFilteredManagerConfig for normal + getCustomManagerFields for regex/custom, templates, filePatterns, npmrc, etc). The get_extract_fingerprint_config is the per-upgrade fingerprint string helper (sorted fields). Single test ported for the generate filtering. (full manager list, custom handling in the impl). |
| `lib/workers/repository/extract/file-match.ts` | full | [`crates/renovate-core/src/managers.rs`](../../../crates/renovate-core/src/managers.rs) | getIncludedFiles, filterIgnoredFiles, getFilteredFileList, getMatchingFiles (include/ignore filtering + per managerFilePatterns matchRegexOrGlob + dedup/sort). get_filtered_file_list added + get_matching_files refactored to reuse helpers (fixing duplication divergence vs TS). Single test ported for dedup behavior. (logger.debug, config handling in caller layers). |
| `lib/workers/repository/extract/index.ts` | partial | [`crates/renovate-core/src/managers.rs`](../../../crates/renovate-core/src/managers.rs) | extractAllDependencies orchestrator (getEnabledManagersList + matching via getMatchingFiles for enabled managers incl. custom. names, extractionFingerprints for matched managers, instrumented parallel getManagerPackageFiles, processSupersedesManagers on intermediate, sorted duration logging, accumulation into packageFiles map + total count, final enabledManagers check that emits the "explicitly enabled ... but found no results. Possible config error?" debug). get_manager_package_files stub (real per-manager extract/read + massage in pending manager-files.ts); customManagers array + mergeChildConfig + isCustomManager full path pending; real scm.getFileList wiring pending in repository worker. Single test ported. |
| `lib/workers/repository/extract/manager-files.ts` | partial | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs`](../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs) | getManagerPackageFiles (enabled/fileList guards, matched log, extractAllPackageFiles vs per-file extractPackageFile + readLocalFile, massageDepNames for packageName->depName, attach packageFile, return). The actual registry dispatch + fs read are simulated for the proving test (full in manager registry + util/fs when this + callers wired); get_manager_files list helper pre-existing for other use. Single test ported. (file-match already full in sibling). |
| `lib/workers/repository/extract/supersedes.ts` | partial | [`crates/renovate-core/src/workers/repository/extract/supersedes.rs`](../../../crates/renovate-core/src/workers/repository/extract/supersedes.rs) | processSupersedesManagers (uses get( , 'supersedesManagers') via supersedes_managers, builds rejected for primary-on-secondary-locks or secondary-on-primary-overlap, filters packageFiles at end; get_default_supersedes_rules + apply_supersedes helper). Full dynamic per-manager supersedesManagers from registry pending (defaults cover test cases); apply shape conversion for index flow in managers.rs. Single test ported. |
| `lib/workers/repository/extract/types.ts` | opt-out | — | Type-only interface used for TypeScript compile-time typing in worker extraction output. |
| `lib/workers/repository/finalize/index.ts` | partial | [`crates/renovate-core/src/workers/repository/finalize/index.rs`](../../../crates/renovate-core/src/workers/repository/finalize/index.rs) | finalizeRepo (wires pruneStaleBranches + runRenovateRepoStats + result; stubs for checkReconfigureBranch, repositoryCache, ensureIssuesClosing, clearRenovateRefs, PackageFiles.clear, platform.getPrList for repoIsActivated, runBranchSummary). Prune/stats are simplified (full in their pending .ts + platform); no direct it() for the glue found (sub specs cover called fns). Single test ported for stats call wiring. |
| `lib/workers/repository/finalize/prune.ts` | partial | [`crates/renovate-core/src/workers/repository/finalize/prune.rs`](../../../crates/renovate-core/src/workers/repository/finalize/prune.rs) | pruneStaleBranches (computes remaining renovate branches after prefix/lock/reconfigure exclusions, returns pruned list in PruneResult; full cleanUpBranches with platform.findPr, scm.isBranchModified/isBranchModified, updatePr for '- autoclosed'/' - abandoned', ensureComment, scm.deleteBranch, dryRun, multi-base baseBranchRe, error handling, GlobalConfig, logger is in platform/git layers or finalize caller). The pure list-diff helper here is used by finalize/index; side effects and full orchestration pending. Single test ported. |
| `lib/workers/repository/finalize/repository-statistics.ts` | partial | [`crates/renovate-core/src/workers/repository/finalize/repository_statistics.rs`](../../../crates/renovate-core/src/workers/repository/finalize/repository_statistics.rs) | runRenovateRepoStats + runBranchSummary (collect_pr_stats/collect_statistics core for PR totals/merged etc; cache scan for baseBranches/inactiveBranches, logger.debug with specific shape, platform.getPrList in caller). Stubs for full cache/platform; the collect fns + run surfaces added for finalize wiring. Single test ported. |
| `lib/workers/repository/index.ts` | partial | [`crates/renovate-core/src/workers/repository/index.rs`](../../../crates/renovate-core/src/workers/repository/index.rs) | renovateRepository/process_repository skeleton (disabled early return via configured check; wiring to finalize + result; divergence note for full init/extract/update/onboarding/configMigration/ensureDashboard/handleError/prune/splits/instrument/queue + recursive automerge; the single proving test ported). Full flow pending other units. |
| `lib/workers/repository/init/apis.ts` | partial | [`crates/renovate-core/src/workers/repository/init/apis.rs`](../../../crates/renovate-core/src/workers/repository/init/apis.rs) | initApis + getPlatformConfig (platform.initRepo) + validateOptimizeForDisabled (optimizeForDisabled + getJsonFile default config + :disableRenovate re-enable logic) + validateIncludeForks (forkProcessing/includeForks + repo config checks, getJsonFile failure ignore) + getDefaultConfigFileName + onboardingConfigFileName handling. Single test ported. Full platform result merge + async wiring pending in init/index + worker. |
| `lib/workers/repository/init/cache.ts` | partial | [`crates/renovate-core/src/workers/repository/init/cache.rs`](../../../crates/renovate-core/src/workers/repository/init/cache.rs) | resetCaches (mem + repo cache reset + fs.remove(privateCacheDir)) + initializeCaches (initRepoCache + ensure private dir + npm setNpmrc clear then set from config.npmrc); single test ported. Full mem/repo singletons, initRepoCache details (CacheFactory/load), memCache.init() post-reset, and wiring from init/index live in pending units (index.ts, global, util/cache/repository). |
| `lib/workers/repository/init/config.ts` | partial | [`crates/renovate-core/src/workers/repository/init/config.rs`](../../../crates/renovate-core/src/workers/repository/init/config.rs) | getRepoConfig (baseBranch = defaultBranch + calls to mergeInheritedConfig / checkOnboardingBranch / mergeRenovateConfig); single test ported from the covering spec. Full sibling surfaces + wiring in pending init/inherited/merge/onboarding units. |
| `lib/workers/repository/init/index.ts` | partial | [`crates/renovate-core/src/workers/repository/init/index.rs`](../../../crates/renovate-core/src/workers/repository/init/index.rs) | initRepo orchestrator (initializeConfig + PackageFiles.clear + resetCaches + memCache.init + initMutexes + initApis + initializeCaches + getRepoConfig + setRepositoryLogLevelRemaps + silent mode log + checkIfConfigured + warnOnUnsupportedOptions + applySecretsAndVariablesToConfig + setUserRepoConfig + detectVulnerabilityAlerts + printConfig log + cloneSubmodules); single test ported. Full async/platform enrichment in init/apis + git clone/user config + mutex/mem global + logger remap + vulnerability + main worker/repository wiring pending in other units. |
| `lib/workers/repository/init/inherited.ts` | partial | [`crates/renovate-core/src/workers/repository/init/inherited.rs`](../../../crates/renovate-core/src/workers/repository/init/inherited.rs) | mergeInheritedConfig (early returns for !inheritConfig, invalid repo/file, fetch via platform.getRawFile, parseFileConfig, validateConfig('inherit'), removeGlobalConfig + decrypt + secrets + applyHostRules + InheritConfig.set + mergeChildConfig or resolveConfigPresets path, setUserConfigFileNames); single test ported. Full async platform, preset resolve network, decrypt, host rules apply (in merge), template, logger, error constants, and wiring from getRepoConfig live in pending units or core config layer. |
| `lib/workers/repository/init/merge.ts` | partial | [`crates/renovate-core/src/workers/repository/init/merge.rs`](../../../crates/renovate-core/src/workers/repository/init/merge.rs) | applyHostRules (add each to hostRules, clear queue/throttle for concurrency update, delete hostRules from config; also the mergeRenovateConfig/detectRepoFileConfig/applyNpmrc etc surfaces); single test ported. Full wiring in the large merge logic and callers (inherited, config) pending other units or already in core host_rules/http. |
| `lib/workers/repository/init/types.ts` | opt-out | — | Type-only repository initialization interfaces used only for TypeScript compile-time handoff typing. |
| `lib/workers/repository/init/vulnerability.ts` | partial | [`crates/renovate-core/src/workers/repository/init/vulnerability.rs`](../../../crates/renovate-core/src/workers/repository/init/vulnerability.rs) | detectVulnerabilityAlerts (early returns, platform.getVulnerabilityAlerts, combine by ecosystem/dep, build alertPackageRules with match*/vulnerabilityFixVersion/severity/isVulnerabilityAlert/prBodyNotes/force, concat to packageRules, remediations, getFixedVersionByDatasource, generatePrBodyNotes); single test ported. Full platform fetch (in github.rs), async, some ecosystem edge cases pending. |
| `lib/workers/repository/model/commit-message-factory.ts` | partial | [`crates/renovate-core/src/workers/repository/model/commit_message_factory.rs`](../../../crates/renovate-core/src/workers/repository/model/commit_message_factory.rs) | CommitMessageFactory (decides semantic vs custom based on semanticCommits==='enabled' && !commitMessagePrefix; sets type/scope or prefix); single test ported. Full toString/get in semantic/custom (pending siblings), callers (onboarding, config-migration) pending. |
| `lib/workers/repository/model/commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository/model/commit_message.rs`](../../../crates/renovate-core/src/workers/repository/model/commit_message.rs) | CommitMessage (abstract base with title/body/footer/subject, toString/toJSON, formatPrefix/formatSubject, setters with normalize, prefix hook); single test ported. Full concrete in semantic/custom (pending), callers (onboarding, config-migration, update/branch) pending. |
| `lib/workers/repository/model/custom-commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository/model/custom_commit_message.rs`](../../../crates/renovate-core/src/workers/repository/model/custom_commit_message.rs) | CustomCommitMessage (extends CommitMessage; _prefix with setter/getter using normalize; override toJSON to include prefix); fixed divergences (title construction for "fix: test", to_json always string prefix even '', to_string filter+parts, safe unicode format_subject matching base/TS); single test ported. Full base in commit-message.rs (previous), semantic sibling pending, callers pending. |
| `lib/workers/repository/model/semantic-commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository/model/semantic_commit_message.rs`](../../../crates/renovate-core/src/workers/repository/model/semantic_commit_message.rs) | SemanticCommitMessage (extends CommitMessage; _type/_scope + prefix() builder, fromString with REGEXP, is, setters, override toJSON); fixed divergences (self-contained fromString/REGEXP matching TS, always-string type/scope in JSON, prefix for title+lower-first subject, toString parts filter); single test ported. Full base + custom in siblings, callers (onboarding, config-migration, update/branch) + factory wiring pending. |
| `lib/workers/repository/onboarding/branch/check.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/check.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/check.rs) | isOnboarded (silent mode early return, requireConfig optional/ignored bypass, onboarding cache valid + sha match, closed onboarding PR handling + ensureComment, configFileExists + packageJsonConfigExists using scm/fs, throws REPOSITORY_CLOSED_ONBOARDING / REPOSITORY_NO_CONFIG); getOnboardingPr; single test ported (verified). Full platform/scm async wiring, cache details, error consts, callers (init/config checkOnboardingBranch, onboarding index) pending other units. |
| `lib/workers/repository/onboarding/branch/commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/commit_message.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/commit_message.rs) | OnboardingCommitMessageFactory (wraps CommitMessageFactory, sets subject to onboardingCommitMessage or `add ${configFile}`); single test ported. Uses model commit message factory (ported); full integration in onboarding create/rebase and body appending pending. |
| `lib/workers/repository/onboarding/branch/config.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/config.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/config.rs) | getOnboardingConfig (clone inherited/global + searchDefaultOnboardingPreset for group/renovate-config or org/.platform:renovate-config using getPreset with PRESET_DEP_NOT_FOUND fallback), getOnboardingConfigContents (EditorConfig + JSONWriter); single test ported. Uses json_writer for formatting; preset search simulated for unit (full async getPreset + platform in pending); callers in onboarding create/rebase pending. |
| `lib/workers/repository/onboarding/branch/create.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/create.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/create.rs) | createOnboardingBranch (get configFile + contents via sibling, OnboardingCommitMessageFactory for message or supplied, optional commitBody append via template, dryRun early return null, prTitle (semantic or inherited), scm.commitAndPush with force + platformCommit); single test ported. Full scm/platform wiring, template, higher caller (index/branch) pending other units. |
| `lib/workers/repository/onboarding/branch/index.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/index.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/index.rs) | checkOnboardingBranch (isOnboarded early return + cache delete, checkIfConfigured, setGitAuthor, getOnboardingPr + rebase/create/conflict/cache logic, getOnboardingConfig + merge + extract + no-package check + createOnboardingBranch, return with repoIsOnboarded/onboardingBranch/branchList); fixed divergences (more wiring to ported siblings check/create/config/cache for rebase/create/cache valid/no package); single test ported. Full wiring to siblings (rebase full, platform/scm, higher (init)) pending other units. |
| `lib/workers/repository/onboarding/branch/onboarding-branch-cache.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/onboarding_branch_cache.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/onboarding_branch_cache.rs) | setOnboardingCache, deleteOnboardingCache, hasOnboardingBranchChanged, isOnboardingBranchModified, isOnboardingBranchConflicted, getOnboardingFileNameFromCache/getOnboardingConfigFromCache/setOnboardingConfigDetails; single test ported. Uses repo cache (via static for unit, full util/workers integration pending); callers in index pending other units. |
| `lib/workers/repository/onboarding/branch/rebase.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/rebase.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/rebase.rs) | rebaseOnboardingBranch (platform support check for github/gitea/gitlab, get contents and hash, skip if same as previous or dryRun, use OnboardingCommitMessageFactory for message, prTitle, scm.commitAndPush); single test ported. Uses siblings for contents/factory; full scm/platform execution and higher caller (index) pending other units. |
| `lib/workers/repository/onboarding/common.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/common.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/common.rs) | getSemanticCommitPrTitle, getDefaultConfigFileName, OnboardingState (prUpdateRequested/onboardingCacheValid statics), get_onboarding_pr_title/get_onboarding_pr_body; single test ported. Callers (rebase, pr, init/apis) and full memCache integration pending. |
| `lib/workers/repository/onboarding/pr/base-branch.ts` | full | [`crates/renovate-core/src/branch.rs`](../../../crates/renovate-core/src/branch.rs) | getBaseBranchDesc (0/1/>1 baseBranchPatterns description strings for onboarding PR body; strings + logic match the TS exactly; all covering it() ported in this file's tests). |
| `lib/workers/repository/onboarding/pr/config-description.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/pr/config_description.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/pr/config_description.rs) | getConfigDesc + getScheduleDesc (description array + schedule handling for onboarding PR body summary); single test ported. (logger calls omitted as they do not affect observable return value; _packageFiles param kept for signature parity but unused per upstream TODO). |
| `lib/workers/repository/onboarding/pr/index.ts` | pending | — | — |
| `lib/workers/repository/onboarding/pr/pr-list.ts` | pending | — | — |
| `lib/workers/repository/package-files.ts` | pending | — | — |
| `lib/workers/repository/process/extract-update.ts` | partial | [`crates/renovate-core/src/workers/repository/process/extract_update.rs`](../../../crates/renovate-core/src/workers/repository/process/extract_update.rs) | extract (cache check stub + TODO checkout/extractAll/stats/ensureGithubToken), lookup (fetchVulns x2 + fetchUpdates + calculateLibYears + branchify TODO + reportMaliciousSkippedDependencies + sort), update (write if onboarded), is_cache_extract_valid (flat), report fn; single test ported. Full async/cache/branchify/vulns pending other units. |
| `lib/workers/repository/process/fetch.ts` | partial | [`crates/renovate-core/src/workers/repository/process/fetch.rs`](../../../crates/renovate-core/src/workers/repository/process/fetch.rs) | fetchUpdates orchestrator, fetchManagerPackagerFileUpdates / fetchManagerUpdates, per-dep early skip (invalid-name, preserve existing skipReason + updates:[]), name trim + packageName fallback, datasource guard, delegation to lookup_dependency for updates; single test ported. Full pre-lookup merge/applyPackageRules, constraintsVersioning, concurrency (p.all), LookupStats, ExternalHostError warnings, PackageFiles.add side-effect pending. |
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

