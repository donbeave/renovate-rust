# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3970`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3970) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3976`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3976) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3995`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3995) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4571`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4571) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4177`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4177) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4185`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4185) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4193`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4193) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4246`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4246) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4044`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4044) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4015`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4015) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8770`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8770) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8814`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8814) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8790`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8790) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8845`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8845) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8867`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8867) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8898`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8898) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4579`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4579) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3475`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3475) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4167`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4167) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4053`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4053) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4286`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4286) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4368`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4368) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4414`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4414) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4443`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4443) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4472`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4472) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4076`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4076) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4117`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4117) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4064`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4064) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4555`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4555) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8630`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8630) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4596`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4596) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4649`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4649) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4704`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4704) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9145`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9145) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3909`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3909) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3899`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3899) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8237`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8237) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8250`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8250) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8256`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8256) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9145`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9145) |

