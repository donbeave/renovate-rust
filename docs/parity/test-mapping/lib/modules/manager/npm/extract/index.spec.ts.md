# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3965`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3965) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3971`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3971) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3990`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3990) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4566`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4566) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4172`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4172) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4180`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4180) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4188`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4188) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4241`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4241) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4039`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4039) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4010`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4010) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8765`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8765) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8809`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8809) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8785`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8785) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8840`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8840) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8862`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8862) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8893`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8893) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4574`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4574) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3470`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3470) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4162`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4162) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4048`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4048) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4281`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4281) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4363`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4363) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4409`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4409) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4438`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4438) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4467`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4467) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4071`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4071) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4112`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4112) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4059`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4059) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4550`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4550) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8625`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8625) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4591`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4591) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4644`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4644) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4699`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4699) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9140`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9140) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3904`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3904) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3894`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3894) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8232`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8232) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8245`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8245) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8251`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8251) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9140`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9140) |

