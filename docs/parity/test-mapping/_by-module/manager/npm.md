# Module: `manager/npm`

[← all modules](../../README.md)

**Coverage:** 353/439 tests ported across 32 spec files.

| Spec file | it() | ported | pending | Rust test file(s) | Status |
|---|--:|--:|--:|---|---|
| [`lib/modules/manager/npm/artifacts.spec.ts`](../../lib/modules/manager/npm/artifacts.spec.ts.md) | 23 | 10 | 13 | `crates/renovate-core/src/extractors/npm.rs`<br>`crates/renovate-core/src/extractors/npm_post_update/artifact_runner.rs` | partial |
| [`lib/modules/manager/npm/detect.spec.ts`](../../lib/modules/manager/npm/detect.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/extract/common/catalogs.spec.ts`](../../lib/modules/manager/npm/extract/common/catalogs.spec.ts.md) | 4 | 4 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/extract/common/package-file.spec.ts`](../../lib/modules/manager/npm/extract/common/package-file.spec.ts.md) | 7 | 7 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/extract/index.spec.ts`](../../lib/modules/manager/npm/extract/index.spec.ts.md) | 41 | 39 | 2 | `crates/renovate-core/src/extractors/npm.rs` | partial |
| [`lib/modules/manager/npm/extract/npm.spec.ts`](../../lib/modules/manager/npm/extract/npm.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/extract/pnpm.spec.ts`](../../lib/modules/manager/npm/extract/pnpm.spec.ts.md) | 16 | 5 | 11 | `crates/renovate-core/src/extractors/npm.rs` | partial |
| [`lib/modules/manager/npm/extract/post/locked-versions.spec.ts`](../../lib/modules/manager/npm/extract/post/locked-versions.spec.ts.md) | 21 | 12 | 9 | `crates/renovate-core/src/extractors/npm.rs` | partial |
| [`lib/modules/manager/npm/extract/post/monorepo.spec.ts`](../../lib/modules/manager/npm/extract/post/monorepo.spec.ts.md) | 5 | 5 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/extract/utils.spec.ts`](../../lib/modules/manager/npm/extract/utils.spec.ts.md) | 3 | 3 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/extract/yarn.spec.ts`](../../lib/modules/manager/npm/extract/yarn.spec.ts.md) | 9 | 9 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/extract/yarnrc.spec.ts`](../../lib/modules/manager/npm/extract/yarnrc.spec.ts.md) | 7 | 6 | 1 | `crates/renovate-core/src/extractors/npm.rs` | partial |
| [`lib/modules/manager/npm/npmrc.spec.ts`](../../lib/modules/manager/npm/npmrc.spec.ts.md) | 9 | 9 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/post-update/index.spec.ts`](../../lib/modules/manager/npm/post-update/index.spec.ts.md) | 33 | 0 | 33 | — | pending |
| [`lib/modules/manager/npm/post-update/node-version.spec.ts`](../../lib/modules/manager/npm/post-update/node-version.spec.ts.md) | 11 | 11 | 0 | `crates/renovate-core/src/extractors/npm.rs`<br>`crates/renovate-core/src/extractors/npm_post_update/node_version.rs` | ported |
| [`lib/modules/manager/npm/post-update/npm.spec.ts`](../../lib/modules/manager/npm/post-update/npm.spec.ts.md) | 35 | 32 | 3 | `crates/renovate-core/src/extractors/npm_post_update.rs`<br>`crates/renovate-core/src/extractors/npm_post_update/npm.rs`<br>`crates/renovate-core/src/extractors/npm_post_update/utils.rs` | partial |
| [`lib/modules/manager/npm/post-update/pnpm.spec.ts`](../../lib/modules/manager/npm/post-update/pnpm.spec.ts.md) | 31 | 31 | 0 | `crates/renovate-core/src/extractors/npm_post_update.rs`<br>`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs`<br>`crates/renovate-core/src/extractors/npm_post_update/utils.rs` | ported |
| [`lib/modules/manager/npm/post-update/rules.spec.ts`](../../lib/modules/manager/npm/post-update/rules.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/post-update/yarn.spec.ts`](../../lib/modules/manager/npm/post-update/yarn.spec.ts.md) | 29 | 21 | 8 | `crates/renovate-core/src/extractors/npm_post_update.rs`<br>`crates/renovate-core/src/extractors/npm_post_update/utils.rs`<br>`crates/renovate-core/src/extractors/npm_post_update/yarn.rs` | partial |
| [`lib/modules/manager/npm/range.spec.ts`](../../lib/modules/manager/npm/range.spec.ts.md) | 5 | 5 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/dependency/index.spec.ts`](../../lib/modules/manager/npm/update/dependency/index.spec.ts.md) | 24 | 24 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/dependency/pnpm.spec.ts`](../../lib/modules/manager/npm/update/dependency/pnpm.spec.ts.md) | 24 | 24 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/dependency/yarn.spec.ts`](../../lib/modules/manager/npm/update/dependency/yarn.spec.ts.md) | 26 | 25 | 1 | `crates/renovate-core/src/extractors/npm.rs` | partial |
| [`lib/modules/manager/npm/update/locked-dependency/common/parent-version.spec.ts`](../../lib/modules/manager/npm/update/locked-dependency/common/parent-version.spec.ts.md) | 5 | 0 | 5 | — | pending |
| [`lib/modules/manager/npm/update/locked-dependency/index.spec.ts`](../../lib/modules/manager/npm/update/locked-dependency/index.spec.ts.md) | 20 | 20 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts`](../../lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts.md) | 4 | 4 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`](../../lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/locked-dependency/yarn-lock/get-locked.spec.ts`](../../lib/modules/manager/npm/update/locked-dependency/yarn-lock/get-locked.spec.ts.md) | 2 | 2 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`](../../lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`](../../lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/update/package-version/index.spec.ts`](../../lib/modules/manager/npm/update/package-version/index.spec.ts.md) | 6 | 6 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |
| [`lib/modules/manager/npm/utils.spec.ts`](../../lib/modules/manager/npm/utils.spec.ts.md) | 7 | 7 | 0 | `crates/renovate-core/src/extractors/npm.rs` | ported |

