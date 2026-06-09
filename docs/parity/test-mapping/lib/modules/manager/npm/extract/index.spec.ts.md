# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3966`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3966) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3972`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3972) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3991`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3991) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4567`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4567) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4173`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4173) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4181`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4181) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4189`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4189) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4242`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4242) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4040`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4040) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4011`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4011) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8766`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8766) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8810`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8810) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8786`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8786) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8841`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8841) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8863`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8863) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8894`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8894) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4575`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4575) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3471`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3471) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4163`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4163) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4049`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4049) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4282`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4282) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4364`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4364) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4410`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4410) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4439`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4439) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4468`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4468) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4072`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4072) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4113`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4113) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4060`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4060) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4551`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4551) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8626`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8626) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4592`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4592) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4645`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4645) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4700`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4700) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9141`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9141) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3905`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3905) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3895`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3895) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8233`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8233) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8246`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8246) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8252`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8252) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9141`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9141) |

