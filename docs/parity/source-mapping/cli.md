# Source Mapping — `cli`

[← all groups](README.md)

**Coverage:** 158/158 in-scope files mapped (full=0 partial=158 stub=0 pending=0 out-of-scope=0 opt-out=2) across 15 modules.

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
| `lib/proxy.ts` | partial | [`crates/renovate-core/src/proxy.rs`](../../../crates/renovate-core/src/proxy.rs) | — |

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
| `lib/workers/types.ts` | partial | [`crates/renovate-core/src/workers/types.rs`](../../../crates/renovate-core/src/workers/types.rs) | — |

### `workers/global`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/workers/global/autodiscover.ts` | partial | [`crates/renovate-core/src/workers/global/autodiscover.rs`](../../../crates/renovate-core/src/workers/global/autodiscover.rs) | — |
| `lib/workers/global/config/parse/additional-config-file.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/config/parse/cli.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/config/parse/codespaces.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/config/parse/coersions.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/config/parse/env.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/config/parse/file.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/config/parse/host-rules-from-env.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/config/parse/index.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse/index.rs`](../../../crates/renovate-core/src/workers/global/config/parse/index.rs) | — |
| `lib/workers/global/config/parse/types.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/config/parse/util.ts` | partial | [`crates/renovate-core/src/workers/global/config/parse.rs`](../../../crates/renovate-core/src/workers/global/config/parse.rs) | — |
| `lib/workers/global/index.ts` | partial | [`crates/renovate-core/src/workers/global/index.rs`](../../../crates/renovate-core/src/workers/global/index.rs) | — |
| `lib/workers/global/initialize.ts` | partial | [`crates/renovate-core/src/workers/global/initialize.rs`](../../../crates/renovate-core/src/workers/global/initialize.rs) | — |
| `lib/workers/global/limits.ts` | partial | [`crates/renovate-core/src/workers/global.rs`](../../../crates/renovate-core/src/workers/global.rs) | — |

### `workers/repository`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/workers/repository/cache.ts` | partial | [`crates/renovate-core/src/workers/repository/cache.rs`](../../../crates/renovate-core/src/workers/repository/cache.rs) | — |
| `lib/workers/repository/changelog/index.ts` | partial | [`crates/renovate-core/src/workers/repository/changelog/index.rs`](../../../crates/renovate-core/src/workers/repository/changelog/index.rs) | — |
| `lib/workers/repository/changelog/types.ts` | partial | [`crates/renovate-core/src/workers/repository/changelog.rs`](../../../crates/renovate-core/src/workers/repository/changelog.rs) | — |
| `lib/workers/repository/common.ts` | partial | [`crates/renovate-core/src/workers/repository/common.rs`](../../../crates/renovate-core/src/workers/repository/common.rs) | — |
| `lib/workers/repository/config-migration/branch/commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/config-migration/branch/create.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/config-migration/branch/index.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/config-migration/branch/migrated-data.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/config-migration/branch/rebase.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/config-migration/common.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/config-migration/index.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/config-migration/pr/index.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/configured.ts` | partial | [`crates/renovate-core/src/workers/repository/configured.rs`](../../../crates/renovate-core/src/workers/repository/configured.rs) | — |
| `lib/workers/repository/dependency-dashboard.ts` | partial | [`crates/renovate-core/src/workers/repository/dependency_dashboard.rs`](../../../crates/renovate-core/src/workers/repository/dependency_dashboard.rs) | — |
| `lib/workers/repository/error-config.ts` | partial | [`crates/renovate-core/src/workers/repository/error_config.rs`](../../../crates/renovate-core/src/workers/repository/error_config.rs) | — |
| `lib/workers/repository/error.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/errors-warnings.ts` | partial | [`crates/renovate-core/src/workers/repository/errors_warnings.rs`](../../../crates/renovate-core/src/workers/repository/errors_warnings.rs) | — |
| `lib/workers/repository/extract/extract-fingerprint-config.ts` | partial | [`crates/renovate-core/src/workers/repository/extract/extract_fingerprint_config.rs`](../../../crates/renovate-core/src/workers/repository/extract/extract_fingerprint_config.rs) | — |
| `lib/workers/repository/extract/file-match.ts` | partial | [`crates/renovate-core/src/workers/repository/extract.rs`](../../../crates/renovate-core/src/workers/repository/extract.rs) | — |
| `lib/workers/repository/extract/index.ts` | partial | [`crates/renovate-core/src/workers/repository/extract.rs`](../../../crates/renovate-core/src/workers/repository/extract.rs) | — |
| `lib/workers/repository/extract/manager-files.ts` | partial | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs`](../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs) | — |
| `lib/workers/repository/extract/supersedes.ts` | partial | [`crates/renovate-core/src/workers/repository/extract/supersedes.rs`](../../../crates/renovate-core/src/workers/repository/extract/supersedes.rs) | — |
| `lib/workers/repository/extract/types.ts` | partial | [`crates/renovate-core/src/workers/repository/extract/types.rs`](../../../crates/renovate-core/src/workers/repository/extract/types.rs) | — |
| `lib/workers/repository/finalize/index.ts` | partial | [`crates/renovate-core/src/workers/repository/finalize/index.rs`](../../../crates/renovate-core/src/workers/repository/finalize/index.rs) | — |
| `lib/workers/repository/finalize/prune.ts` | partial | [`crates/renovate-core/src/workers/repository/finalize/prune.rs`](../../../crates/renovate-core/src/workers/repository/finalize/prune.rs) | — |
| `lib/workers/repository/finalize/repository-statistics.ts` | partial | [`crates/renovate-core/src/workers/repository/finalize/repository_statistics.rs`](../../../crates/renovate-core/src/workers/repository/finalize/repository_statistics.rs) | — |
| `lib/workers/repository/index.ts` | partial | [`crates/renovate-core/src/workers/repository/index.rs`](../../../crates/renovate-core/src/workers/repository/index.rs) | — |
| `lib/workers/repository/init/apis.ts` | partial | [`crates/renovate-core/src/workers/repository/init/apis.rs`](../../../crates/renovate-core/src/workers/repository/init/apis.rs) | — |
| `lib/workers/repository/init/cache.ts` | partial | [`crates/renovate-core/src/workers/repository/init/cache.rs`](../../../crates/renovate-core/src/workers/repository/init/cache.rs) | — |
| `lib/workers/repository/init/config.ts` | partial | [`crates/renovate-core/src/workers/repository/init.rs`](../../../crates/renovate-core/src/workers/repository/init.rs) | — |
| `lib/workers/repository/init/index.ts` | partial | [`crates/renovate-core/src/workers/repository/init/index.rs`](../../../crates/renovate-core/src/workers/repository/init/index.rs) | — |
| `lib/workers/repository/init/inherited.ts` | partial | [`crates/renovate-core/src/workers/repository/init/inherited.rs`](../../../crates/renovate-core/src/workers/repository/init/inherited.rs) | — |
| `lib/workers/repository/init/merge.ts` | partial | [`crates/renovate-core/src/workers/repository/init/merge.rs`](../../../crates/renovate-core/src/workers/repository/init/merge.rs) | — |
| `lib/workers/repository/init/types.ts` | partial | [`crates/renovate-core/src/workers/repository/init.rs`](../../../crates/renovate-core/src/workers/repository/init.rs) | — |
| `lib/workers/repository/init/vulnerability.ts` | partial | [`crates/renovate-core/src/workers/repository/init/vulnerability.rs`](../../../crates/renovate-core/src/workers/repository/init/vulnerability.rs) | — |
| `lib/workers/repository/model/commit-message-factory.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/model/commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/model/custom-commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/model/semantic-commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/onboarding/branch/check.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/check.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/check.rs) | — |
| `lib/workers/repository/onboarding/branch/commit-message.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/commit_message.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/commit_message.rs) | — |
| `lib/workers/repository/onboarding/branch/config.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/config.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/config.rs) | — |
| `lib/workers/repository/onboarding/branch/create.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/create.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/create.rs) | — |
| `lib/workers/repository/onboarding/branch/index.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch/index.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch/index.rs) | — |
| `lib/workers/repository/onboarding/branch/onboarding-branch-cache.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch.rs) | — |
| `lib/workers/repository/onboarding/branch/rebase.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/branch.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/branch.rs) | — |
| `lib/workers/repository/onboarding/common.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/common.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/common.rs) | — |
| `lib/workers/repository/onboarding/pr/base-branch.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/pr.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/pr.rs) | — |
| `lib/workers/repository/onboarding/pr/config-description.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/pr.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/pr.rs) | — |
| `lib/workers/repository/onboarding/pr/index.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/pr/index.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/pr/index.rs) | — |
| `lib/workers/repository/onboarding/pr/pr-list.ts` | partial | [`crates/renovate-core/src/workers/repository/onboarding/pr/pr_list.rs`](../../../crates/renovate-core/src/workers/repository/onboarding/pr/pr_list.rs) | — |
| `lib/workers/repository/package-files.ts` | partial | [`crates/renovate-core/src/workers/repository/package_files.rs`](../../../crates/renovate-core/src/workers/repository/package_files.rs) | — |
| `lib/workers/repository/process/extract-update.ts` | partial | [`crates/renovate-core/src/workers/repository/process/extract_update.rs`](../../../crates/renovate-core/src/workers/repository/process/extract_update.rs) | — |
| `lib/workers/repository/process/fetch.ts` | partial | [`crates/renovate-core/src/workers/repository/process/fetch.rs`](../../../crates/renovate-core/src/workers/repository/process/fetch.rs) | — |
| `lib/workers/repository/process/fingerprint-fields.ts` | partial | [`crates/renovate-core/src/workers/repository/process.rs`](../../../crates/renovate-core/src/workers/repository/process.rs) | — |
| `lib/workers/repository/process/index.ts` | partial | [`crates/renovate-core/src/workers/repository/process/index.rs`](../../../crates/renovate-core/src/workers/repository/process/index.rs) | — |
| `lib/workers/repository/process/libyear.ts` | partial | [`crates/renovate-core/src/workers/repository/process/libyear.rs`](../../../crates/renovate-core/src/workers/repository/process/libyear.rs) | — |
| `lib/workers/repository/process/limits.ts` | partial | [`crates/renovate-core/src/workers/repository/process/limits.rs`](../../../crates/renovate-core/src/workers/repository/process/limits.rs) | — |
| `lib/workers/repository/process/lookup/abandonment.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup.rs) | — |
| `lib/workers/repository/process/lookup/bucket.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup.rs) | — |
| `lib/workers/repository/process/lookup/current.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup.rs) | — |
| `lib/workers/repository/process/lookup/filter-checks.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup.rs) | — |
| `lib/workers/repository/process/lookup/filter.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs) | — |
| `lib/workers/repository/process/lookup/generate.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup.rs) | — |
| `lib/workers/repository/process/lookup/index.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/index.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/index.rs) | — |
| `lib/workers/repository/process/lookup/rollback.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup.rs) | — |
| `lib/workers/repository/process/lookup/timestamps.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/timestamps.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/timestamps.rs) | — |
| `lib/workers/repository/process/lookup/types.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/types.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/types.rs) | — |
| `lib/workers/repository/process/lookup/update-type.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup.rs) | — |
| `lib/workers/repository/process/lookup/utils.ts` | partial | [`crates/renovate-core/src/workers/repository/process/lookup/utils.rs`](../../../crates/renovate-core/src/workers/repository/process/lookup/utils.rs) | — |
| `lib/workers/repository/process/sort.ts` | partial | [`crates/renovate-core/src/workers/repository/process/sort.rs`](../../../crates/renovate-core/src/workers/repository/process/sort.rs) | — |
| `lib/workers/repository/process/types.ts` | partial | [`crates/renovate-core/src/workers/repository/process.rs`](../../../crates/renovate-core/src/workers/repository/process.rs) | — |
| `lib/workers/repository/process/vulnerabilities.ts` | partial | [`crates/renovate-core/src/workers/repository/process/vulnerabilities.rs`](../../../crates/renovate-core/src/workers/repository/process/vulnerabilities.rs) | — |
| `lib/workers/repository/process/write.ts` | partial | [`crates/renovate-core/src/workers/repository/process/write.rs`](../../../crates/renovate-core/src/workers/repository/process/write.rs) | — |
| `lib/workers/repository/reconfigure/comment.ts` | partial | [`crates/renovate-core/src/workers/repository/reconfigure.rs`](../../../crates/renovate-core/src/workers/repository/reconfigure.rs) | — |
| `lib/workers/repository/reconfigure/index.ts` | partial | [`crates/renovate-core/src/workers/repository/reconfigure/index.rs`](../../../crates/renovate-core/src/workers/repository/reconfigure/index.rs) | — |
| `lib/workers/repository/reconfigure/reconfigure-cache.ts` | partial | [`crates/renovate-core/src/workers/repository/reconfigure.rs`](../../../crates/renovate-core/src/workers/repository/reconfigure.rs) | — |
| `lib/workers/repository/reconfigure/utils.ts` | partial | [`crates/renovate-core/src/workers/repository/reconfigure.rs`](../../../crates/renovate-core/src/workers/repository/reconfigure.rs) | — |
| `lib/workers/repository/reconfigure/validate.ts` | partial | [`crates/renovate-core/src/workers/repository/reconfigure.rs`](../../../crates/renovate-core/src/workers/repository/reconfigure.rs) | — |
| `lib/workers/repository/result.ts` | partial | [`crates/renovate-core/src/workers/repository/result.rs`](../../../crates/renovate-core/src/workers/repository/result.rs) | — |
| `lib/workers/repository/update/branch/artifacts.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/artifacts.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/artifacts.rs) | — |
| `lib/workers/repository/update/branch/auto-replace.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/auto_replace.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/auto_replace.rs) | — |
| `lib/workers/repository/update/branch/automerge.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/automerge.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/automerge.rs) | — |
| `lib/workers/repository/update/branch/bump-versions.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/bump_versions.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/bump_versions.rs) | — |
| `lib/workers/repository/update/branch/check-existing.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/check_existing.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/check_existing.rs) | — |
| `lib/workers/repository/update/branch/commit.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/commit.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/commit.rs) | — |
| `lib/workers/repository/update/branch/execute-post-upgrade-commands.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch.rs`](../../../crates/renovate-core/src/workers/repository/update/branch.rs) | — |
| `lib/workers/repository/update/branch/get-updated.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/get_updated.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/get_updated.rs) | — |
| `lib/workers/repository/update/branch/handle-existing.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch.rs`](../../../crates/renovate-core/src/workers/repository/update/branch.rs) | — |
| `lib/workers/repository/update/branch/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/index.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/index.rs) | — |
| `lib/workers/repository/update/branch/reuse.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/reuse.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/reuse.rs) | — |
| `lib/workers/repository/update/branch/schedule.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/schedule.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/schedule.rs) | — |
| `lib/workers/repository/update/branch/status-checks.ts` | partial | [`crates/renovate-core/src/workers/repository/update/branch/status_checks.rs`](../../../crates/renovate-core/src/workers/repository/update/branch/status_checks.rs) | — |
| `lib/workers/repository/update/pr/automerge.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/automerge.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/automerge.rs) | — |
| `lib/workers/repository/update/pr/body/changelogs.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/body.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/body.rs) | — |
| `lib/workers/repository/update/pr/body/config-description.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/body.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/body.rs) | — |
| `lib/workers/repository/update/pr/body/controls.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/body.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/body.rs) | — |
| `lib/workers/repository/update/pr/body/footer.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/body.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/body.rs) | — |
| `lib/workers/repository/update/pr/body/header.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/body.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/body.rs) | — |
| `lib/workers/repository/update/pr/body/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/body/index.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/body/index.rs) | — |
| `lib/workers/repository/update/pr/body/notes.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/body.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/body.rs) | — |
| `lib/workers/repository/update/pr/body/updates-table.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/body.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/body.rs) | — |
| `lib/workers/repository/update/pr/changelog/api.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/bitbucket-server/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/bitbucket-server/source.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/bitbucket/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/bitbucket/source.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/common.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/forgejo/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/forgejo/source.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/gitea/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/gitea/source.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/github/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/github/source.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/gitlab/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/gitlab/source.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/hbs-template.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog/index.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog/index.rs) | — |
| `lib/workers/repository/update/pr/changelog/release-notes.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/releases.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/source.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/changelog/types.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/changelog.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/changelog.rs) | — |
| `lib/workers/repository/update/pr/code-owners.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr.rs`](../../../crates/renovate-core/src/workers/repository/update/pr.rs) | — |
| `lib/workers/repository/update/pr/index.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/index.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/index.rs) | — |
| `lib/workers/repository/update/pr/labels.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/labels.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/labels.rs) | — |
| `lib/workers/repository/update/pr/participants.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr.rs`](../../../crates/renovate-core/src/workers/repository/update/pr.rs) | — |
| `lib/workers/repository/update/pr/pr-cache.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr/pr_cache.rs`](../../../crates/renovate-core/src/workers/repository/update/pr/pr_cache.rs) | — |
| `lib/workers/repository/update/pr/pr-fingerprint.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr.rs`](../../../crates/renovate-core/src/workers/repository/update/pr.rs) | — |
| `lib/workers/repository/update/pr/pr-reuse.ts` | partial | [`crates/renovate-core/src/workers/repository/update/pr.rs`](../../../crates/renovate-core/src/workers/repository/update/pr.rs) | — |
| `lib/workers/repository/updates/branch-name.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/updates/branchify.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/updates/flatten.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |
| `lib/workers/repository/updates/generate.ts` | partial | [`crates/renovate-core/src/workers/repository.rs`](../../../crates/renovate-core/src/workers/repository.rs) | — |

