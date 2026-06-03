# `lib/modules/manager/npm/extract/index.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**39/41 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 38 | returns null if cannot parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3916`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3916) |
| 47 | catches invalid names | ported | [`crates/renovate-core/src/extractors/npm.rs:3922`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3922) |
| 58 | ignores vendorised package.json | ported | [`crates/renovate-core/src/extractors/npm.rs:3941`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3941) |
| 67 | throws error if non-root renovate config | ported | [`crates/renovate-core/src/extractors/npm.rs:4517`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4517) |
| 77 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:4123`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4123) |
| 86 | handles invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4131`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4131) |
| 95 | returns an array of dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:4139`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4139) |
| 122 | returns an array of dependencies with resolution comments | ported | [`crates/renovate-core/src/extractors/npm.rs:4192`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4192) |
| 161 | finds a lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:3990`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3990) |
| 180 | warns when multiple lock files found | ported | [`crates/renovate-core/src/extractors/npm.rs:3961`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3961) |
| 207 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8504`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8504) |
| 230 | uses config.npmrc if no .npmrc is returned from search | ported | [`crates/renovate-core/src/extractors/npm.rs:8548`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8548) |
| 239 | uses config.npmrc if no .npmrc exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8524`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8524) |
| 249 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8579`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8579) |
| 272 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8601`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8601) |
| 295 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8632`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8632) |
| 320 | reads registryurls from .yarnrc.yml | ported | [`crates/renovate-core/src/extractors/npm.rs:4525`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4525) |
| 348 | reads registryurls from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm.rs:3421`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3421) |
| 375 | resolves registry urls using the package name if set | ported | [`crates/renovate-core/src/extractors/npm.rs:4113`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4113) |
| 408 | finds complex yarn workspaces | ported | [`crates/renovate-core/src/extractors/npm.rs:3999`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3999) |
| 422 | extracts engines | ported | [`crates/renovate-core/src/extractors/npm.rs:4232`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4232) |
| 513 | extracts volta | ported | [`crates/renovate-core/src/extractors/npm.rs:4314`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4314) |
| 556 | extracts volta yarn unspecified-version | ported | [`crates/renovate-core/src/extractors/npm.rs:4360`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4360) |
| 597 | extracts volta yarn higher than 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:4389`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4389) |
| 639 | extracts non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4418`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4418) |
| 787 | does not set registryurls for non-npmjs | ported | [`crates/renovate-core/src/extractors/npm.rs:4022`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4022) |
| 842 | extracts npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:4063`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4063) |
| 893 | sets skipinstalls false if yarn zero-install is used | ported | [`crates/renovate-core/src/extractors/npm.rs:4010`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4010) |
| 921 | extracts packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:4501`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4501) |
| 950 | sets haspackagemanager to true when devengines detected in package file | ported | [`crates/renovate-core/src/extractors/npm.rs:8364`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8364) |
| 984 | extracts dependencies from overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4542`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4542) |
| 1063 | extracts dependencies from pnpm.overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:4595`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4595) |
| 1144 | extracts dependencies from pnpm.overrides, with version ranges in flat syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:4650`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4650) |
| 1227 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:8879`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8879) |
| 1277 | warns for invalid pnpm workspace yaml files | pending | — |
| 1294 | parses empty pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3855`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3855) |
| 1303 | extracts pnpm workspace yaml files | ported | [`crates/renovate-core/src/extractors/npm.rs:3845`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3845) |
| 1333 | extracts yarnrc.yml and adds it as packagefile | ported | [`crates/renovate-core/src/extractors/npm.rs:7971`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7971) |
| 1367 | extracts yarnrc.yml and adds it as packagefile and packagemanager to true | ported | [`crates/renovate-core/src/extractors/npm.rs:7984`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7984) |
| 1399 | extracts yarnrc.yml and adds it as packagefile and packagemanager to false if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7990`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7990) |
| 1436 | runs | ported | [`crates/renovate-core/src/extractors/npm.rs:8879`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8879) |

