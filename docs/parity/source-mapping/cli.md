# Source Mapping — `cli`

[← all groups](README.md)

**Coverage:** 7/150 in-scope files mapped (full=2 partial=5 stub=0 pending=143 out-of-scope=0 opt-out=10) across 15 modules.

### `commander.d.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/commander.d.ts` | opt-out | — | Type declaration shim for Commander typings; no Rust runtime analogue is needed. |

### `config-validator.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/config-validator.ts` | pending | — | — |

### `data-files.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/data-files.generated.ts` | pending | — | — |

### `datasource-list.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/datasource-list.generated.ts` | pending | — | — |

### `expose.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/expose.ts` | pending | — | — |

### `global-config-option-defaults.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/global-config-option-defaults.generated.ts` | pending | — | — |

### `globals.d.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/globals.d.ts` | opt-out | — | TypeScript ambient declarations for global interfaces and module shims; no Rust runtime analogue. |

### `manager-default-configs.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/manager-default-configs.generated.ts` | pending | — | — |

### `manager-list.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/manager-list.generated.ts` | pending | — | — |

### `proxy.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/proxy.ts` | pending | — | — |

### `renovate.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/renovate.ts` | pending | — | — |

### `versioning-list.generated.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/versioning-list.generated.ts` | pending | — | — |

### `workers/_root`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/workers/types.ts` | opt-out | — | Type-only interface and type aliases used for TypeScript compile-time type safety; no Rust runtime equivalent. |

### `workers/global`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/workers/global/autodiscover.ts` | pending | — | — |
| `lib/workers/global/config/parse/additional-config-file.ts` | pending | — | — |
| `lib/workers/global/config/parse/cli.ts` | pending | — | — |
| `lib/workers/global/config/parse/codespaces.ts` | pending | — | — |
| `lib/workers/global/config/parse/coersions.ts` | full | [`crates/renovate-cli/src/config_env.rs`](../../../crates/renovate-cli/src/config_env.rs) | boolean (''/true->true, false->false or error), array (JSON5 array or csv split+trim+filter non-empty), object (JSON5 or {} or error), string (\n->actual newline), integer (parseInt). The parse_string_array / parse_*_json_* / parse_string_list etc in this file (and mappers in cli.rs + config_builder) implement the coersions used by env.ts getConfig and cli.ts. |
| `lib/workers/global/config/parse/env.ts` | full | [`crates/renovate-cli/src/config_env.rs`](../../../crates/renovate-cli/src/config_env.rs) | env prefix normalization, key renaming (legacy + migrated), experimental X_ var massaging to current names, RENOVATE_CONFIG merge, per-option coercion via env, special boolean mappings for dryRun/requireConfig/platformCommit, GITHUB_COM_TOKEN -> hostRule, deletion of unsupported legacy env names. Matches the prepareEnv + getConfig surface for self-hosted runs. |
| `lib/workers/global/config/parse/file.ts` | pending | — | — |
| `lib/workers/global/config/parse/host-rules-from-env.ts` | pending | — | — |
| `lib/workers/global/config/parse/index.ts` | pending | — | — |
| `lib/workers/global/config/parse/types.ts` | opt-out | — | Type-only parse options type aliases used only by TypeScript configuration validation typing. |
| `lib/workers/global/config/parse/util.ts` | pending | — | — |
| `lib/workers/global/index.ts` | pending | — | — |
| `lib/workers/global/initialize.ts` | pending | — | — |
| `lib/workers/global/limits.ts` | pending | — | — |

### `workers/repository`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/workers/repository/cache.ts` | pending | — | — |
| `lib/workers/repository/changelog/index.ts` | pending | — | — |
| `lib/workers/repository/changelog/types.ts` | opt-out | — | Type-only changelog option/result type aliases with no runtime behavior in Rust implementation. |
| `lib/workers/repository/common.ts` | pending | — | — |
| `lib/workers/repository/config-migration/branch/commit-message.ts` | pending | — | — |
| `lib/workers/repository/config-migration/branch/create.ts` | pending | — | — |
| `lib/workers/repository/config-migration/branch/index.ts` | pending | — | — |
| `lib/workers/repository/config-migration/branch/migrated-data.ts` | pending | — | — |
| `lib/workers/repository/config-migration/branch/rebase.ts` | pending | — | — |
| `lib/workers/repository/config-migration/common.ts` | pending | — | — |
| `lib/workers/repository/config-migration/index.ts` | pending | — | — |
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
| `lib/workers/repository/process/lookup/bucket.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/bucket.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/bucket.rs) | getBucket (uses separate* flags from config + versioning getMajor/getMinor to decide bucket for update grouping); single test ported (covering it from lookup/index.spec that exercises separateMajorMinor etc for bucketing). Full callers in lookup/index or update/branchify pending other units. |
| `lib/workers/repository/process/lookup/current.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/current.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/current.rs) | getCurrentVersion (currentValue + lockedVersion + rangeStrategy pin/bump + latestVersion filter + useVersions via matches + getSatisfying/minSatisfying + isVersion/isSingleVersion fallbacks + = strip for single); single test ported (covering it from lookup/index.spec that exercises separate* no, the tilde pin range current resolution). Full callers (in lookup/index.ts etc) and integration pending other units. |
| `lib/workers/repository/process/lookup/filter-checks.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/filter_checks.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/filter_checks.rs) | filterInternalChecks (early none pop; strict/flexible with age + confidence + postprocess + updateType application + pending collection + fallback to latest pending); single test ported (covering "returns latest release if internalChecksFilter=none" from filter-checks.spec). Full async paths, merge confidence integration, datasource interception, and callers in lookup/index pending other units. |
| `lib/workers/repository/process/lookup/filter.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs) | filterVersions (greater-than-current + ignoreDeprecated + maxMajorIncrement + allowedVersions templating/regex/semver/pep440 fallbacks + followTag + respectLatest + ignoreUnstable + stable filtering with allowUnstableMajorUpgrades); single test ported (covering "filters versions with major increment greater than maxMajorIncrement" from filter.spec). Full callers (lookup/index etc) and some allowedVersions edge paths pending other units. |
| `lib/workers/repository/process/lookup/generate.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/generate.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/generate.rs) | generateUpdate (newValue via getNewValue + metadata copy from release + newMajor/Minor/Patch + updateType + isBreaking (with versioning.isBreaking or major default) + mergeConfidence when configured + isRange + isLockfileUpdate for update-lockfile + isBump for bump strategy); single test ported (covering "supports lock file updates mixed with regular updates" from index.spec that exercises generate paths). Full async confidence, getUpdateType wiring, and callers in lookup/index pending (update-type.ts also pending). |
| `lib/workers/repository/process/lookup/index.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/index.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/index.rs) | lookupUpdates (orchestrates invalid checks, rollback, currentVersion (getCurrentVersion), filterVersions, filterInternalChecks, generateUpdate per release, pin special, abandonment; full datasource/fetch, timestamps, getRollbackUpdate, confidence, and all edge paths pending other units/subs). |
| `lib/workers/repository/process/lookup/rollback.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/rollback.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/rollback.rs) | getRollbackUpdate (isLessThanRange filter on versions, stable prefer if current stable, sort + pop highest qualifying, getNewValue with 'replace', rollback update shape + prBodyNotes); single test ported (covering "returns rollback for ranged version" from index.spec). Full callers (in lookup/index) and integration with datasource releases pending other units. |
| `lib/workers/repository/process/lookup/timestamps.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/timestamps.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/timestamps.rs) | calculateMostRecentTimestamp (highest version by isGreaterThan/isVersion, skip if deprecated or invalid or if any lower has newer ts, set mostRecentTimestamp on the highest's ts if it is overall max); single test ported (covering "returns the timestamp of the latest version" from timestamps.spec). Full callers (in lookup/index, abandonment, libyear etc) pending other units. |
| `lib/workers/repository/process/lookup/types.ts` | opt-out | — | Type-only lookup/result interfaces used only for TypeScript compile-time types. |
| `lib/workers/repository/process/lookup/update-type.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/update_type.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/update_type.rs) | getUpdateType (major if not same major or getMajor(new)>getMajor(current); minor if getMinor(new)>getMinor(current); else patch; separate* flags in config present but not used in core logic here); single test ported (covering "supports minor and major upgrades for tilde ranges" from index.spec that exercises update type decisions). Full callers (in generate, filter-checks, lookup/index) and integration pending other units. |
| `lib/workers/repository/process/lookup/utils.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/utils.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/utils.rs) | addReplacementUpdateIfValid + isReplacementRulesConfigured + determineNewReplacementName/Value + getNewVersion (template/compile stubs, getNewValue with isReplacement); single test ported (covering "handles replacements - name only without pinDigests enabled" from index.spec). Full callers (in lookup/index) and template/getRangeStrategy wiring pending other units. |
| `lib/workers/repository/process/sort.ts` | pending | — | — |
| `lib/workers/repository/process/types.ts` | opt-out | — | Type-only vulnerability/process interfaces used only for TypeScript compile-time typing. |
| `lib/workers/repository/process/vulnerabilities.ts` | partial | [`crates/renovate-core/src/workers/repository/process/vulnerabilities.rs`](../../../crates/renovate-core/src/workers/repository/process/vulnerabilities.rs) | appendVulnerabilityPackageRules, fetchVulnerabilities, vulnerabilityToPackageRules, isPackageVulnerable, getFixedVersion, skipMalicious, sortByFixedVersion, generatePrBodyNotes, extractSeverityDetails (OSV stubbed, no full osv-offline/CVSS deps yet); single test ported (covering "fetches vulnerabilities" from extract-update.spec.ts line 122). Full OSV integration, async fetch, CVSS, etc. pending other units. |
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
| `lib/workers/repository/update/branch/get-updated.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/get_updated.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/get_updated.rs) | getUpdatedPackageFiles (loop upgrades for content via autoReplace/updateDependency/updateLocked, assemble FileAdditions, updateArtifacts + check pending versions for minReleaseAge, return PackageFilesResult); single test ported (covering "handles empty" — lib/workers/repository/update/branch/get-updated.spec.ts line 119). Full manager extract/updateLocked, artifact update, rebase recursion, lockFileMaintenance, remediation, git-submodules special, bumpVersion, pending version artifactErrors, and cross wiring pending other units. |
| `lib/workers/repository/update/branch/handle-existing.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/handle_existing.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/handle_existing.rs) | handleClosedPr (closed PR: compile ignore* comment per updateType, ensureComment unless prIgnoreNotification suppressed, delete branch; dryRun logs), handleModifiedPr (edited PR: ensure 'Edited/Blocked Notification' or remove if dd check/rebaseRequested, unless prEditedNotification suppressed; dryRun logs); single test ported (covering "skips branch if edited PR found" — lib/workers/repository/update/branch/index.spec.ts line 451). Full platform/scm/GlobalConfig/template/userStrings/dependencyDashboardChecks wiring, other callers, closed non-dry paths pending other units. |
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

