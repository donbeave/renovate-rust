# `lib/workers/repository/config-migration/branch/create.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**4/5 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 36 | applies the default commit message | ported | [`crates/renovate-core/src/branch.rs:2808`](../../../../../../../../crates/renovate-core/src/branch.rs#L2808) |
| 58 | applies supplied commit message | ported | [`crates/renovate-core/src/branch.rs:2824`](../../../../../../../../crates/renovate-core/src/branch.rs#L2824) |
| 85 | migrates renovate config in package.json | opt-out | the special case for when the migrated config is embedded in package.json 'renovate' field (read the package, extract the renovate config to a new renovate.json, strip the 'renovate' key from package.json in the commit files, and the specific files array in commitAndPush) is in the pending worker/index orchestration for createConfigMigrationBranch (per @parity comment in branch.rs; the file changes/stripping logic not yet in the Rust create surface). The message production for migration is covered by the other ported tests in the spec and the config_migration_* helpers. Opt as the package.json cleanup behavior requires the pending orchestration wiring. |
| 125 | to the default commit message | ported | [`crates/renovate-core/src/branch.rs:2847`](../../../../../../../../crates/renovate-core/src/branch.rs#L2847) |
| 154 | to the default commit message | ported | [`crates/renovate-core/src/branch.rs:2847`](../../../../../../../../crates/renovate-core/src/branch.rs#L2847) |
| 182 | uses user defined semantic commit type | ported | [`crates/renovate-core/src/branch.rs:2836`](../../../../../../../../crates/renovate-core/src/branch.rs#L2836) |

