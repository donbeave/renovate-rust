# Module: `config/migrations`

[← all modules](../../README.md)

**Coverage:** 155/161 tests ported across 58 spec files.

| Spec file | it() | ported | pending | Rust test file(s) | Status |
|---|--:|--:|--:|---|---|
| [`lib/config/migrations/base/abstract-migration.spec.ts`](../../lib/config/migrations/base/abstract-migration.spec.ts.md) | 2 | 0 | 2 | — | pending |
| [`lib/config/migrations/custom/automerge-major-migration.spec.ts`](../../lib/config/migrations/custom/automerge-major-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/automerge-migration.spec.ts`](../../lib/config/migrations/custom/automerge-migration.spec.ts.md) | 4 | 4 | 0 | `crates/renovate-core/src/config/migrate_validate.rs`<br>`crates/renovate-core/src/repo_config.rs` | ported |
| [`lib/config/migrations/custom/automerge-minor-migration.spec.ts`](../../lib/config/migrations/custom/automerge-minor-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/automerge-patch-migration.spec.ts`](../../lib/config/migrations/custom/automerge-patch-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/automerge-type-migration.spec.ts`](../../lib/config/migrations/custom/automerge-type-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts`](../../lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/base-branch-migration.spec.ts`](../../lib/config/migrations/custom/base-branch-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/binary-source-migration.spec.ts`](../../lib/config/migrations/custom/binary-source-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/branch-name-migration.spec.ts`](../../lib/config/migrations/custom/branch-name-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/branch-prefix-migration.spec.ts`](../../lib/config/migrations/custom/branch-prefix-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/compatibility-migration.spec.ts`](../../lib/config/migrations/custom/compatibility-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts`](../../lib/config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/custom-managers-migration.spec.ts`](../../lib/config/migrations/custom/custom-managers-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/datasource-migration.spec.ts`](../../lib/config/migrations/custom/datasource-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/dep-types-migration.spec.ts`](../../lib/config/migrations/custom/dep-types-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/dry-run-migration.spec.ts`](../../lib/config/migrations/custom/dry-run-migration.spec.ts.md) | 2 | 0 | 2 | — | pending |
| [`lib/config/migrations/custom/enabled-managers-migration.spec.ts`](../../lib/config/migrations/custom/enabled-managers-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/extends-migration.spec.ts`](../../lib/config/migrations/custom/extends-migration.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/config/migrate_validate.rs`<br>`crates/renovate-core/src/repo_config.rs` | ported |
| [`lib/config/migrations/custom/fetch-release-notes-migration.spec.ts`](../../lib/config/migrations/custom/fetch-release-notes-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/file-match-migration.spec.ts`](../../lib/config/migrations/custom/file-match-migration.spec.ts.md) | 4 | 4 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/go-mod-tidy-migration.spec.ts`](../../lib/config/migrations/custom/go-mod-tidy-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/host-rules-migration.spec.ts`](../../lib/config/migrations/custom/host-rules-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/ignore-node-modules-migration.spec.ts`](../../lib/config/migrations/custom/ignore-node-modules-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/ignore-npmrc-file-migration.spec.ts`](../../lib/config/migrations/custom/ignore-npmrc-file-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/include-forks-migration.spec.ts`](../../lib/config/migrations/custom/include-forks-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/match-datasources-migration.spec.ts`](../../lib/config/migrations/custom/match-datasources-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/match-managers-migration.spec.ts`](../../lib/config/migrations/custom/match-managers-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs`<br>`crates/renovate-core/src/repo_config.rs` | ported |
| [`lib/config/migrations/custom/match-strings-migration.spec.ts`](../../lib/config/migrations/custom/match-strings-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/node-migration.spec.ts`](../../lib/config/migrations/custom/node-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/package-files-migration.spec.ts`](../../lib/config/migrations/custom/package-files-migration.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/package-name-migration.spec.ts`](../../lib/config/migrations/custom/package-name-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/package-pattern-migration.spec.ts`](../../lib/config/migrations/custom/package-pattern-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/package-rules-migration.spec.ts`](../../lib/config/migrations/custom/package-rules-migration.spec.ts.md) | 8 | 8 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/packages-migration.spec.ts`](../../lib/config/migrations/custom/packages-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/path-rules-migration.spec.ts`](../../lib/config/migrations/custom/path-rules-migration.spec.ts.md) | 4 | 4 | 0 | `crates/renovate-core/src/config/migrate_validate.rs`<br>`crates/renovate-core/src/repo_config.rs` | ported |
| [`lib/config/migrations/custom/pin-versions-migration.spec.ts`](../../lib/config/migrations/custom/pin-versions-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/platform-commit-migration.spec.ts`](../../lib/config/migrations/custom/platform-commit-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/post-update-options-migration.spec.ts`](../../lib/config/migrations/custom/post-update-options-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/rebase-conflicted-prs-migration.spec.ts`](../../lib/config/migrations/custom/rebase-conflicted-prs-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/rebase-stale-prs-migration.spec.ts`](../../lib/config/migrations/custom/rebase-stale-prs-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/recreate-closed-migration.spec.ts`](../../lib/config/migrations/custom/recreate-closed-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/renovate-fork-migration.spec.ts`](../../lib/config/migrations/custom/renovate-fork-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/require-config-migration.spec.ts`](../../lib/config/migrations/custom/require-config-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/required-status-checks-migration.spec.ts`](../../lib/config/migrations/custom/required-status-checks-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/schedule-migration.spec.ts`](../../lib/config/migrations/custom/schedule-migration.spec.ts.md) | 5 | 5 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/semantic-commits-migration.spec.ts`](../../lib/config/migrations/custom/semantic-commits-migration.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/semantic-prefix-migration.spec.ts`](../../lib/config/migrations/custom/semantic-prefix-migration.spec.ts.md) | 4 | 4 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/separate-major-release-migration.spec.ts`](../../lib/config/migrations/custom/separate-major-release-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/separate-multiple-major-migration.spec.ts`](../../lib/config/migrations/custom/separate-multiple-major-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/stability-days-migration.spec.ts`](../../lib/config/migrations/custom/stability-days-migration.spec.ts.md) | 1 | 1 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/suppress-notifications-migration.spec.ts`](../../lib/config/migrations/custom/suppress-notifications-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/trust-level-migration.spec.ts`](../../lib/config/migrations/custom/trust-level-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/unpublish-safe-migration.spec.ts`](../../lib/config/migrations/custom/unpublish-safe-migration.spec.ts.md) | 7 | 7 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/update-lock-files-migration.spec.ts`](../../lib/config/migrations/custom/update-lock-files-migration.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/upgrade-in-range-migration.spec.ts`](../../lib/config/migrations/custom/upgrade-in-range-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/custom/version-strategy-migration.spec.ts`](../../lib/config/migrations/custom/version-strategy-migration.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/config/migrate_validate.rs` | ported |
| [`lib/config/migrations/migrations-service.spec.ts`](../../lib/config/migrations/migrations-service.spec.ts.md) | 6 | 4 | 2 | `crates/renovate-core/src/config/migrate_validate.rs` | partial |

