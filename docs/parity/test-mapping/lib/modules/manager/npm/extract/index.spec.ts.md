# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3962`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3962) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3968`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3968) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3987`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3987) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4563`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4563) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4169`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4169) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4177`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4177) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4185`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4185) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4238`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4238) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4036`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4036) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4007`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4007) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8762`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8762) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8806`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8806) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8782`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8782) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8837`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8837) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8859`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8859) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8890`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8890) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4571`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4571) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3467`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3467) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4159`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4159) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4045`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4045) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4278`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4278) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4360`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4360) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4406`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4406) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4435`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4435) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4464`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4464) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4068`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4068) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4109`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4109) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4056`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4056) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4547`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4547) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8622`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8622) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4588`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4588) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4641`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4641) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4696`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4696) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9137`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9137) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3901`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3901) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3891`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3891) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8229`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8229) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8242`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8242) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8248`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8248) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9137`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9137) |

