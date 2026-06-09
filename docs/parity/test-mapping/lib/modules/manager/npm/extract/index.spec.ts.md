# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3959`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3959) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3965`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3965) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3984`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3984) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4560`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4560) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4166`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4166) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4174`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4174) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4182`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4182) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4235`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4235) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4033`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4033) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4004`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4004) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8759`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8759) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8803`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8803) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8779`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8779) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8834`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8834) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8856`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8856) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8887`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8887) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4568`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4568) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3464`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3464) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4156`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4156) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4042`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4042) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4275`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4275) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4357`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4357) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4403`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4403) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4432`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4432) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4461`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4461) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4065`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4065) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4106`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4106) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4053`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4053) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4544`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4544) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8619`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8619) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4585`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4585) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4638`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4638) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4693`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4693) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9134`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9134) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3898`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3898) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3888`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3888) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8226`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8226) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8239`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8239) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8245`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8245) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9134`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9134) |

