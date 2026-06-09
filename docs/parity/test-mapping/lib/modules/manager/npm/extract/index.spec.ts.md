# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3958`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3958) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3964`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3964) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3983`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3983) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4559`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4559) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4165`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4165) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4173`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4173) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4181`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4181) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4234`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4234) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4032`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4032) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4003`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4003) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8758`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8758) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8802`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8802) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8778`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8778) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8833`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8833) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8855`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8855) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8886`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8886) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4567`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4567) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3463`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3463) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4155`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4155) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4041`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4041) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4274`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4274) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4356`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4356) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4402`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4402) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4431`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4431) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4460`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4460) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4064`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4064) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4105`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4105) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4052`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4052) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4543`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4543) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8618`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8618) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4584`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4584) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4637`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4637) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4692`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4692) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9133`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9133) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3897`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3897) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3887`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3887) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8225`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8225) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8238`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8238) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8244`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8244) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9133`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9133) |

