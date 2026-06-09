# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3964`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3964) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3970`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3970) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3989`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3989) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4565`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4565) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4171`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4171) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4179`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4179) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4187`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4187) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4240`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4240) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4038`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4038) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4009`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4009) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8764`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8764) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8808`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8808) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8784`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8784) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8839`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8839) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8861`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8861) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8892`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8892) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4573`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4573) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3469`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3469) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4161`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4161) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4047`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4047) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4280`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4280) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4362`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4362) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4408`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4408) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4437`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4437) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4466`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4466) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4070`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4070) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4111`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4111) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4058`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4058) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4549`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4549) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8624`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8624) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4590`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4590) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4643`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4643) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4698`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4698) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9139`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9139) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3903`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3903) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3893`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3893) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8231`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8231) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8244`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8244) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8250`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8250) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9139`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9139) |

