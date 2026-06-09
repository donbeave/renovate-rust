# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3960`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3960) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3966`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3966) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3985`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3985) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4561`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4561) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4167`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4167) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4175`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4175) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4183`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4183) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4236`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4236) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4034`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4034) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4005`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4005) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8760`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8760) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8804`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8804) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8780`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8780) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8835`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8835) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8857`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8857) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8888`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8888) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4569`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4569) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3465`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3465) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4157`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4157) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4043`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4043) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4276`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4276) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4358`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4358) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4404`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4404) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4433`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4433) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4462`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4462) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4066`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4066) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4107`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4107) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4054`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4054) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4545`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4545) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8620`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8620) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4586`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4586) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4639`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4639) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4694`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4694) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9135`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9135) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3899`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3899) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3889`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3889) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8227`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8227) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8240`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8240) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8246`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8246) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9135`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9135) |

