# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3961`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3961) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3967`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3967) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3986`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3986) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4562`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4562) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4168`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4168) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4176`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4176) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4184`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4184) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4237`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4237) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4035`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4035) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:4006`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4006) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8761`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8761) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8805`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8805) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8781`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8781) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8836`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8836) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8858`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8858) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8889`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8889) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4570`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4570) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3466`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3466) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4158`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4158) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4044`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4044) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4277`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4277) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4359`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4359) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4405`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4405) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4434`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4434) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4463`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4463) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4067`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4067) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4108`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4108) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4055`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4055) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4546`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4546) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8621`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8621) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4587`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4587) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4640`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4640) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4695`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4695) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9136`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9136) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3900`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3900) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3890`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3890) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8228`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8228) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8241`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8241) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8247`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8247) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9136`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9136) |

