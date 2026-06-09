# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3963`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3963) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3969`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3969) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3988`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3988) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4564`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4564) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4170`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4170) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4178`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4178) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4186`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4186) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4239`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4239) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4037`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4037) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4008`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4008) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8763`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8763) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8807`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8807) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8783`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8783) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8838`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8838) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8860`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8860) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8891`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8891) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4572`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4572) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3468`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3468) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4160`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4160) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4046`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4046) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4279`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4279) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4361`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4361) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4407`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4407) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4436`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4436) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4465`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4465) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4069`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4069) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4110`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4110) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4057`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4057) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4548`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4548) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8623`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8623) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4589`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4589) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4642`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4642) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4697`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4697) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9138`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9138) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3902`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3902) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3892`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3892) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8230`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8230) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8243`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8243) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8249`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8249) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9138`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9138) |

