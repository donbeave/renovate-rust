# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3956`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3956) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3962`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3962) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3981`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3981) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4557`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4557) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4163`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4163) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4171`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4171) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4179`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4179) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4232`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4232) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4030`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4030) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4001`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4001) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8756`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8756) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8800`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8800) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8776`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8776) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8831`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8831) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8853`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8853) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8884`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8884) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4565`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4565) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3461`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3461) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4153`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4153) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4039`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4039) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4272`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4272) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4354`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4354) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4400`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4400) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4429`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4429) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4458`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4458) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4062`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4062) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4103`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4103) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4050`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4050) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4541`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4541) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8616`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8616) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4582`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4582) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4635`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4635) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4690`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4690) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9131`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9131) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3895`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3895) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3885`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3885) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8223`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8223) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8236`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8236) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8242`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8242) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9131`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9131) |

