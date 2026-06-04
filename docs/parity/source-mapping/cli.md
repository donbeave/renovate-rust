# Source Mapping — `cli`

[← all groups](README.md)

**Coverage:** 10/155 in-scope files mapped (full=2 partial=8 stub=0 pending=145 out-of-scope=0 opt-out=5) across 15 modules.

### `commander.d.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/commander.d.ts` | opt-out | — | Type declaration shim for Commander typings; no Rust runtime analogue is needed. |

### `config-validator.ts`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/config-validator.ts` | full | [`crates/renovate-cli/src/bin/renovate-config-validator.rs`](../../../crates/renovate-cli/src/bin/renovate-config-validator.rs) | — |

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
| `lib/workers/global/autodiscover.ts` | pending | — | — |
| `lib/workers/global/config/parse/additional-config-file.ts` | pending | — | — |
| `lib/workers/global/config/parse/cli.ts` | pending | — | — |
| `lib/workers/global/config/parse/codespaces.ts` | pending | — | — |
| `lib/workers/global/config/parse/coersions.ts` | pending | — | — |
| `lib/workers/global/config/parse/env.ts` | pending | — | — |
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
| `lib/workers/repository/changelog/types.ts` | pending | — | — |
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
| `lib/workers/repository/init/types.ts` | pending | — | — |
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
| `lib/workers/repository/process/lookup/types.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/update-type.ts` | pending | — | — |
| `lib/workers/repository/process/lookup/utils.ts` | pending | — | — |
| `lib/workers/repository/process/sort.ts` | pending | — | — |
| `lib/workers/repository/process/types.ts` | pending | — | — |
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
| `lib/workers/repository/update/pr/changelog/types.ts` | pending | — | — |
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

