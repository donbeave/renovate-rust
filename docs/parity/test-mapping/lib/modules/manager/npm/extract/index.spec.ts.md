# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/40 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3943`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3943) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3949`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3949) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3968`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3968) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4544`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4544) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4150`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4150) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4158`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4158) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4166`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4166) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4219`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4219) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:4017`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4017) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:3988`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3988) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8637`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8637) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8681`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8681) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8657`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8657) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8712`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8712) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8734`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8734) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8765`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8765) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4552`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4552) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3448`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3448) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4140`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4140) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:4026`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4026) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4259`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4259) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4341`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4341) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4387`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4387) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4416`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4416) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4445`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4445) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4049`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4049) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4090`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4090) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4037`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4037) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4528`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4528) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8497`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8497) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4569`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4569) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4622`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4622) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4677`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4677) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9012`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9012) |
| 1277 | warns for invalid pnpm workspace yaml files | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with packageFile + err: any Error, and specific message 'Failed to parse pnpm-workspace.yaml file') when fs.readLocalFile returns invalid yaml content for pnpm-workspace.yaml during extractAllPackageFiles; the core business logic (bad workspace file leads to no extracted packages / empty result, parse errors are tolerated) is already exercised by ported unit tests on extract_pnpm_workspace_file and related empty/valid parse cases in extractors/npm.rs; no direct Rust logger spy equivalent without altering production instrumentation or test harness |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3882`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3882) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3872`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3872) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:8104`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8104) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:8117`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8117) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:8123`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8123) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:9012`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L9012) |

