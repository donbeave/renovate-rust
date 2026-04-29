# Implementation Ledger

A running log of parity slices completed in this Rust reimplementation of
[`renovatebot/renovate`](https://github.com/renovatebot/renovate). One row per
slice, newest first. Each row links to the relevant Renovate reference paths
(in the sibling `renovate/` checkout) and the Rust files that implement or
test the behavior.

The ledger is the canonical place to record:

- which Renovate behavior a slice is targeting,
- what was actually implemented,
- what was deferred,
- and any blockers (network, credentials, external services) that pushed work
  to a later slice.

If something is missing, partial, or skipped, write it down. Future loops
should be able to plan the next slice from this file alone.

## Status

| Slice | Date       | Theme                          | State    | Notes |
|-------|------------|--------------------------------|----------|-------|
| 0213  | 2026-04-29 | Per-rule `schedule` + `minimumReleaseAge` in `packageRules` | Complete | See below. |
| 0212  | 2026-04-29 | `hashedBranchLength` config option — SHA-512 branch name hashing | Complete | See below. |
| 0211  | 2026-04-28 | Refactor: split `managers_impl` into 17 focused `pipelines/` sub-modules | Complete | See below. |
| 0210  | 2026-04-28 | Refactor: extract `managers_impl.rs` + `context.rs`; `main.rs` 8,733→389 lines | Complete | See below. |
| 0209  | 2026-04-28 | `groupName` branch slug: grouped deps share one branch name | Complete | See below. |
| 0208  | 2026-04-28 | `additionalBranchPrefix` config field; fix `matchCurrentVersion` regex; scan spec map | Complete | See below. |
| 0207  | 2026-04-28 | Fix `labels`/`addLabels` not seeding `collect_rule_effects`; add test map entries | Complete | See below. |
| 0206  | 2026-04-28 | Fix `matchCurrentValue`/`matchNewValue` regex flags bug; remove `PackageNameMatcher` enum | Complete | See below. |
| 0205  | 2026-04-28 | Fix negation in `matchPackageNames` — merge deprecated fields + migrate to raw strings | Complete | See below. |
| 0204  | 2026-04-28 | Fix negation in `matchDepNames` — migrate to raw strings + `match_regex_or_glob_list` | Complete | See below. |
| 0203  | 2026-04-28 | Fix negation in `matchSourceUrls`, `matchRegistryUrls`, `matchRepositories`; Cargo `current_version_timestamp` | Complete | See below. |
| 0202  | 2026-04-28 | crates.io release timestamps via REST API — `minimumReleaseAge` for Cargo | Complete | See below. |
| 0201  | 2026-04-28 | Refactor: split `main.rs` and `repo_config.rs` into focused modules | Complete | See below. |
| 0200  | 2026-04-28 | `matchDatasources` + `matchDepTypes` glob/regex/negation via `match_regex_or_glob_list` | Complete | See below. |
| 0199  | 2026-04-28 | `matchManagers` glob/regex/negation + custom manager prefix + `string_match` utility module | Complete | See below. |
| 0198  | 2026-04-28 | PyPI `current_version_timestamp` for `matchCurrentAge` + AGPL-3.0 LICENSE + README goals section | Complete | See below. |
| 0197  | 2026-04-28 | npm `current_version_timestamp` population for `matchCurrentAge` exact pins | Complete | See below. |
| 0196  | 2026-04-28 | Fix: SemVer build metadata falsely triggered Cargo update detection | Complete | See below. |
| 0195  | 2026-04-28 | Parity tracking: create `renovate-source-map.md` + update loop prompt with maintenance rules | Complete | See below. |
| 0194  | 2026-04-28 | `matchCurrentAge` packageRule matcher + `satisfies_date_range()` + per-version timestamps (npm/pypi) | Complete | See below. |
| 0193  | 2026-04-28 | `--platform=local` fix: scan CWD, skip token requirement, `LocalClient` reads via `git ls-files` | Complete | See below. |
| 0192  | 2026-04-28 | `groupName`-based output grouping: UpdateAvailable deps grouped under header lines | Complete | See below. |
| 0191  | 2026-04-28 | PyPI release timestamp support + parse_age_duration tests + is_within_release_age tests | Complete | See below. |
| 0190  | 2026-04-28 | `minimumReleaseAge` + npm release timestamp + `is_within_release_age()` + `parse_age_duration()` | Complete | See below. |
| 0189  | 2026-04-28 | `commitMessageAction` + `commitMessagePrefix` + `rangeStrategy` in RepoConfig; pr_title() now configurable | Complete | See below. |
| 0188  | 2026-04-28 | Schedule text DSL parser: before/after/between/every weekday/on Monday/first day of month | Complete | See below. |
| 0187  | 2026-04-28 | `schedule` module: POSIX cron evaluation + schedule-gate in CLI pipeline | Complete | See below. |
| 0186  | 2026-04-28 | `branch::pr_title()` + `prTitle` field in DepReport output (semantic commit support) | Complete | See below. |
| 0185  | 2026-04-28 | `RuleEffects` + `collect_rule_effects()` + `groupName`/`automerge`/`labels` in DepReport | Complete | See below. |
| 0184  | 2026-04-28 | Pass manager context to all 72 `is_dep_ignored` call sites in main.rs | Complete | See below. |
| 0183  | 2026-04-28 | `DepContext` unified matcher + `matches_context()` on PackageRule | Complete | See below. |
| 0182  | 2026-04-28 | `matchRegistryUrls` + `matchRepositories` packageRule matchers | Complete | See below. |
| 0181  | 2026-04-28 | `matchCategories` + `matchBaseBranches` packageRule matchers | Complete | See below. |
| 0180  | 2026-04-28 | `manager_categories()` lookup table (27 ecosystems) | Complete | See below. |
| 0178  | 2026-04-28 | Add branchName to DepReport output | Complete | See below. |
| 0177  | 2026-04-28 | Branch name generation — sanitize_dep_name + branch_topic + branch_name | Complete | See below. |
| 0176  | 2026-04-28 | `matchSourceUrls` + `matchCurrentValue` + `matchNewValue` packageRule matchers | Complete | See below. |
| 0175  | 2026-04-28 | `extends` preset parsing + built-in expansion (config:recommended, :ignoreModulesAndTests) | Complete | See below. |
| 0174  | 2026-04-28 | `disabled_by_default` manager flag — azure-pipelines/git-submodules/html/nix/pre-commit/travis | Complete | See below. |
| 0173  | 2026-04-28 | `git-submodules` `.gitmodules` extractor + dispatch | Complete | See below. |
| 0172  | 2026-04-28 | `package.json` `renovate` key config discovery | Complete | See below. |
| 0170  | 2026-04-28 | `matchDepNames` + `matchDatasources` packageRule matchers | Complete | See below. |
| 0169  | 2026-04-28 | `ignoreVersions` global+packageRule + glob/regex `matchPackageNames` + `matchPackagePrefixes` | Complete | See below. |
| 0168  | 2026-04-28 | `matchDepTypes` packageRule + npm dep type filtering | Complete | See below. |
| 0167  | 2026-04-28 | `enabledManagers` repo config option | Complete | See below. |
| 0166  | 2026-04-28 | NuGet cross-file dedup for .NET solutions | Complete | See below. |
| 0165  | 2026-04-28 | Go module cross-file dedup for Go workspaces | Complete | See below. |
| 0164  | 2026-04-28 | Maven cross-file dedup for multi-module projects | Complete | See below. |
| 0163  | 2026-04-28 | PyPI cross-file dedup for pip_requirements + pip-compile | Complete | See below. |
| 0162  | 2026-04-28 | Cargo cross-file dedup + `crates_io::fetch_versions_batch` | Complete | See below. |
| 0161  | 2026-04-28 | npm cross-file dedup + `fetch_versions_batch` API | Complete | See below. |
| 0160  | 2026-04-28 | JSR datasource + endoflife-date datasource | Complete | See below. |
| 0159  | 2026-04-28 | Conda datasource (Anaconda API) + pixi conda dep activation | Complete | See below. |
| 0158  | 2026-04-28 | Hermit package manager extractor + datasource (file-list based) | Complete | See below. |
| 0157  | 2026-04-28 | `pip-compile` pipeline for `.in` source files | Complete | See below. |
| 0155  | 2026-04-28 | `cdnurl` + stub registrations for `git-submodules`/`hermit`/`pip-compile`/`custom` | Complete | See below. |
| 0154  | 2026-04-28 | PEP 723 Python inline script metadata extractor | Complete | See below. |
| 0153  | 2026-04-28 | OCB (OpenTelemetry Collector Builder) Go module extractor | Complete | See below. |
| 0152  | 2026-04-28 | Sveltos `ClusterProfile`/`Profile` Helm chart extractor | Complete | See below. |
| 0151  | 2026-04-28 | Renovate config presets extractor + `helm-requirements` alias | Complete | See below. |
| 0150  | 2026-04-28 | Glasskube package manifest extractor + packages datasource | Complete | See below. |
| 0149  | 2026-04-28 | Crossplane package manifest extractor | Complete | See below. |
| 0148  | 2026-04-28 | Bazel WORKSPACE `http_archive()` extractor (GitHub Tags + Releases) | Complete | See below. |
| 0147  | 2026-04-28 | Tekton CI/CD resource extractor (step image deps) | Complete | See below. |
| 0146  | 2026-04-28 | Kubernetes manifest Docker image extractor | Complete | See below. |
| 0145  | 2026-04-28 | ArgoCD Application manifest extractor (Helm + Git sources) | Complete | See below. |
| 0144  | 2026-04-28 | Bun lockfile manager + nodenv/nvm/pyenv manager aliases | Complete | See below. |
| 0143  | 2026-04-28 | Heroku/Render `runtime.txt` Python version extractor | Complete | See below. |
| 0142  | 2026-04-28 | Helmsman DSF extractor (Helm chart version tracking) | Complete | See below. |
| 0141  | 2026-04-28 | Cloud Native Buildpacks `project.toml` extractor + BuildpacksRegistry datasource | Complete | See below. |
| 0140  | 2026-04-28 | Unity3D `ProjectVersion.txt` extractor + Unity releases datasource | Complete | See below. |
| 0139  | 2026-04-28 | Pixi `pixi.toml` extractor (PyPI deps actionable, Conda skipped) | Complete | See below. |
| 0138  | 2026-04-28 | Bitrise CI step extractor + Bitrise steplib datasource | Complete | See below. |
| 0137  | 2026-04-28 | Homebrew formula extractor (GitHub Archive/Release + NPM routing) | Complete | See below. |
| 0136  | 2026-04-28 | Azure Bicep `.bicep` extractor + bicep-types-az datasource | Complete | See below. |
| 0135  | 2026-04-28 | Perl `cpanfile` extractor + MetaCPAN datasource | Complete | See below. |
| 0134  | 2026-04-28 | Bazel `MODULE.bazel` extractor + Bazel Central Registry datasource | Complete | See below. |
| 0133  | 2026-04-28 | Python `setup.py` PyPI dependency extractor (balanced-bracket scanner) | Complete | See below. |
| 0132  | 2026-04-28 | Apache Ant `build.xml` Maven dependency extractor (XML, coords + attributes) | Complete | See below. |
| 0131  | 2026-04-28 | Terragrunt `terragrunt.hcl` extractor (GitHub Tags + Terraform Registry) | Complete | See below. |
| 0130  | 2026-04-28 | Puppet `Puppetfile` extractor + Puppet Forge datasource | Complete | See below. |
| 0129  | 2026-04-28 | OSGi feature model Maven bundle extractor (JSON5, GAV parsing) | Complete | See below. |
| 0128  | 2026-04-28 | XcodeGen `project.yml` Swift Package extractor (GitHub Tags) | Complete | See below. |
| 0127  | 2026-04-28 | Typst `.typ` package extractor + Typst registry datasource | Complete | See below. |
| 0126  | 2026-04-28 | TFLint plugin `.tflint.hcl` extractor (GitHub Releases) | Complete | See below. |
| 0125  | 2026-04-28 | Crow CI `.crow/*.yml` Docker image extractor | Complete | See below. |
| 0124  | 2026-04-28 | Rancher Fleet extractor (Helm + GitRepo CRD dual-mode) | Complete | See below. |
| 0123  | 2026-04-28 | HTML cdnjs extractor + CDNJS datasource | Complete | See below. |
| 0122  | 2026-04-28 | Kotlin Script `*.main.kts` Maven dependency extractor | Complete | See below. |
| 0121  | 2026-04-28 | Dev Container features extractor upgrade (node/go/python/ruby version deps) | Complete | See below. |
| 0120  | 2026-04-28 | Home Assistant `manifest.json` PyPI extractor | Complete | See below. |
| 0119  | 2026-04-28 | Batect wrapper script version extractor (GitHub Releases) | Complete | See below. |
| 0118  | 2026-04-28 | Haskell Cabal `*.cabal` extractor + Hackage datasource | Complete | See below. |
| 0117  | 2026-04-28 | FVM `.fvmrc`/`.fvm/fvm_config.json` Flutter version extractor | Complete | See below. |
| 0116  | 2026-04-28 | Jsonnet Bundler `jsonnetfile.json` extractor (GitHub Tags) | Complete | See below. |
| 0115  | 2026-04-28 | Vendir `vendir.yml` Helm chart extractor | Complete | See below. |
| 0114  | 2026-04-28 | Copier `.copier-answers.yml` template extractor | Complete | See below. |
| 0113  | 2026-04-28 | Batect `batect.yml` Docker image extractor | Complete | See below. |
| 0112  | 2026-04-28 | Meteor `package.js` `Npm.depends()` extractor (npm datasource) | Complete | See below. |
| 0111  | 2026-04-28 | Cake `.cake` build script extractor (NuGet datasource) | Complete | See below. |
| 0110  | 2026-04-28 | Conan `conanfile.txt`/`.py` extractor + Conan Center datasource | Complete | See below. |
| 0109  | 2026-04-28 | `.ruby-version` version file (GitHub Tags, underscore tag normalization) | Complete | See below. |
| 0108  | 2026-04-28 | Clojure `deps.edn` / `bb.edn` extractor | Complete | See below. |
| 0107  | 2026-04-28 | Azure Pipelines Tasks datasource (GitHub mirror JSON) | Complete | See below. |
| 0106  | 2026-04-28 | Nix flakes `flake.lock` input extractor | Complete | See below. |
| 0105  | 2026-04-28 | FluxCD `gotk-components.yaml` system manifest extractor | Complete | See below. |
| 0104  | 2026-04-28 | SBT `build.sbt` / `project/build.properties` extractor | Complete | See below. |
| 0103  | 2026-04-28 | Ansible tasks Docker image extractor | Complete | See below. |
| 0102  | 2026-04-28 | Leiningen `project.clj` extractor (Clojars + Maven Central) | Complete | See below. |
| 0101  | 2026-04-28 | Jenkins plugins datasource (Update Center JSON) | Complete | See below. |
| 0100  | 2026-04-28 | CircleCI orbs extractor + Orb GraphQL datasource | Complete | See below. |
| 0099  | 2026-04-28 | GitLab CI `include:` project reference extractor | Complete | See below. |
| 0098  | 2026-04-28 | Travis CI `.travis.yml` Node.js version extractor | Complete | See below. |
| 0097  | 2026-04-28 | Bazelisk `.bazelversion` version file support | Complete | See below. |
| 0096  | 2026-04-28 | Scalafmt `.scalafmt.conf` version extractor | Complete | See below. |
| 0095  | 2026-04-28 | Mint `Mintfile` extractor (GitHub Tags datasource) | Complete | See below. |
| 0094  | 2026-04-28 | Gleam `gleam.toml` extractor (Hex.pm datasource) | Complete | See below. |
| 0093  | 2026-04-28 | Devbox `devbox.json` extractor + `search.devbox.sh` datasource | Complete | See below. |
| 0092  | 2026-04-28 | Helm `values.yaml` Docker image extractor | Complete | See below. |
| 0091  | 2026-04-28 | mise-en-place `mise.toml` tool version extractor | Complete | See below. |
| 0090  | 2026-04-28 | Quadlet `.container`/`.image`/`.volume` Docker image extractor | Complete | See below. |
| 0089  | 2026-04-28 | Vela CI `.vela.yml` Docker image extractor | Complete | See below. |
| 0088  | 2026-04-28 | Dev Container `devcontainer.json` Docker image extractor | Complete | See below. |
| 0087  | 2026-04-28 | Woodpecker CI `.woodpecker.yml` Docker image extractor | Complete | See below. |
| 0086  | 2026-04-28 | Maven Wrapper `.mvn/wrapper/maven-wrapper.properties` extractor | Complete | See below. |
| 0085  | 2026-04-28 | Gradle Wrapper extractor + Gradle Version datasource | Complete | See below. |
| 0084  | 2026-04-28 | Refactor: extract `docker_hub_reports` helper to eliminate Docker pipeline duplication | Complete | See below. |
| 0083  | 2026-04-28 | Jenkins `plugins.txt` / `plugins.yml` extractor | Complete | See below. |
| 0082  | 2026-04-28 | Bitbucket Pipelines `*-pipelines.yml` Docker image extractor | Complete | See below. |
| 0081  | 2026-04-28 | Drone CI `.drone.yml` Docker image extractor | Complete | See below. |
| 0080  | 2026-04-28 | Helmfile `helmfile.yaml` extractor | Complete | See below. |
| 0079  | 2026-04-28 | Azure Pipelines extractor (Docker containers + tasks) | Complete | See below. |
| 0078  | 2026-04-28 | Google Cloud Build `cloudbuild.yaml` extractor | Complete | See below. |
| 0077  | 2026-04-28 | Kustomize `images:` and `helmCharts:` extractor | Complete | See below. |
| 0076  | 2026-04-28 | Gradle version catalog `[plugins]` section extraction | Complete | See below. |
| 0075  | 2026-04-28 | Gradle `plugins {}` block extraction | Complete | See below. |
| 0074  | 2026-04-28 | Extend asdf tool table (bun, deno, zig, elixir, scala) + bun-version file | Complete | See below. |
| 0073  | 2026-04-28 | Add `stats` (update counts) to JSON output | Complete | See below. |
| 0072  | 2026-04-28 | `packageRules` matchFileNames glob filtering | Complete | See below. |
| 0071  | 2026-04-28 | `packageRules` matchCurrentVersion filtering | Complete | See below. |
| 0070  | 2026-04-28 | JSON output mode (`--output-format=json`) | Complete | See below. |
| 0069  | 2026-04-28 | `packageRules` allowedVersions semver range filtering | Complete | See below. |
| 0068  | 2026-04-28 | Wire matchUpdateTypes blocking into all manager dep report pipelines | Complete | See below. |
| 0067  | 2026-04-28 | `packageRules` matchUpdateTypes filtering | Complete | See below. |
| 0066  | 2026-04-28 | `UpdateType` classification + update type labels in CLI output | Complete | See below. |
| 0065  | 2026-04-28 | `packageRules` parsing + `enabled: false` filtering | Complete | See below. |
| 0064  | 2026-04-28 | GitHub Actions `runs-on` runner version extraction | Complete | See below. |
| 0063  | 2026-04-28 | GitHub Actions container/services Docker image extraction | Complete | See below. |
| 0035  | 2026-04-28 | NuGet `.csproj`/`.props` extractor + NuGet API datasource | Complete | See below. |
| 0034  | 2026-04-28 | Composer `composer.json` extractor + Packagist datasource | Complete | See below. |
| 0033  | 2026-04-28 | Go modules `go.mod` extractor + Go proxy datasource | Complete | See below. |
| 0032  | 2026-04-28 | Poetry `pyproject.toml` extractor + poetry manager | Complete | See below. |
| 0031  | 2026-04-28 | GitHub Actions `uses:` extractor + GitHub tags datasource | Complete | See below. |
| 0030  | 2026-04-28 | Maven POM property resolution (`<properties>`)  | Complete | See below. |
| 0029  | 2026-04-28 | Glob-based `ignorePaths` matching (globset)     | Complete | See below. |
| 0028  | 2026-04-28 | Run summary totals + `--quiet` mode            | Complete | See below. |
| 0027  | 2026-04-28 | Maven pom.xml extractor + Maven Central datasource | Complete | See below. |
| 0026  | 2026-04-28 | pyproject.toml (PEP 621/735) extractor + pep621 manager | Complete | See below. |
| 0025  | 2026-04-28 | Per-repo renovate.json config parsing + application | Complete | See below. |
| 0024  | 2026-04-28 | docker-compose image extractor (line-scan, no YAML dep) | Complete | See below. |
| 0023  | 2026-04-28 | HTTP retry with exponential backoff + Retry-After | Complete | See below. |
| 0022  | 2026-04-28 | GitLab platform client                           | Complete | See below. |
| 0021  | 2026-04-28 | Docker Hub datasource + Dockerfile pipeline complete | Complete | See below. |
| 0020  | 2026-04-28 | Manager regex caching + Dockerfile FROM extractor | Complete | See below. |
| 0019  | 2026-04-28 | Parallel repository processing (JoinSet + Semaphore) | Complete | See below. |
| 0018  | 2026-04-28 | pip_requirements extractor + PyPI datasource | Complete | See below. |
| 0017  | 2026-04-28 | Human-readable update report output      | Complete | See below. |
| 0016  | 2026-04-28 | npm registry datasource + npm versioning | Complete | See below. |
| 0015  | 2026-04-28 | npm package.json extractor + ledger catchup | Complete | See below. |
| 0014  | 2026-04-28 | Concurrent crates.io lookups (JoinSet + Semaphore) | Complete | commit d760d28 |
| 0013  | 2026-04-28 | update_summary + shared HttpClient | Complete | commit c5722df |
| 0012  | 2026-04-28 | crates.io sparse datasource + cargo semver versioning | Complete | commit db326e3 |
| 0011  | 2026-04-28 | Cargo.toml dependency extractor | Complete | commit ceecc6e |
| 0010  | 2026-04-28 | Package manager detection + GitHub file tree API | Complete | commit 6bc862a |
| 0009  | 2026-04-28 | Repository config discovery via GitHub Contents API | Complete | commit b8651c0 |
| 0008  | 2026-04-28 | AnyPlatformClient factory + startup token validation | Complete | commit d51301f |
| 0007  | 2026-04-28 | tokio async runtime + HttpClient + GitHub platform stub | Complete | See below. |
| 0006  | 2026-04-28 | Global config file loading (JSON/JSON5)       | Complete | See below. |
| 0005  | 2026-04-28 | GlobalConfig struct + CLI→config builder      | Complete | See below. |
| 0004  | 2026-04-28 | Option surface first-cut + env vars           | Complete | See below. |
| 0003  | 2026-04-28 | Logger init (LOG_LEVEL, LOG_FORMAT, NO_COLOR) | Complete | See below. |
| 0002  | 2026-04-28 | `migrateArgs` parity           | Complete | See below. |
| 0001  | 2026-04-28 | Workspace + early CLI flags    | Complete | See below. |

## Slice 0035 - NuGet `.csproj`/`.props` extractor + NuGet API datasource

### Renovate reference
- `lib/modules/manager/nuget/extract.ts` — `extractPackageFile`
- `lib/modules/datasource/nuget/index.ts`
- Patterns: `/\\.(?:cs|fs|vb)proj$/`, `/\\.(?:props|targets)$/`

### What landed
- `crates/renovate-core/src/extractors/nuget.rs` — SAX-style MSBuild XML
  extractor using `quick-xml` (already a dependency). Handles both
  `Event::Empty` (self-closing `<PackageReference ... />`) and `Event::Start`
  + child elements (`<Version>...</Version>`, `<VersionOverride>...</VersionOverride>`).
  - Supported elements: `PackageReference` (Include/Update), `PackageVersion`,
    `DotNetCliToolReference`, `GlobalPackageReference`.
  - `VersionOverride` wins over `Version` attribute when both present.
  - Skip reasons: `PropertyRef` (`$(Variable)`), `VersionRange` (complex range
    with upper bound or exclusive lower), `NoVersion` (no version specified).
  - NuGet version range normalization: `[1.2.3]` → `1.2.3`, `[1.2.3,]`/`[1.2.3,)` →
    `1.2.3` (updateable min-only ranges), `(1.2.3,)` and ranges with upper bound → skip.
  - 13 unit tests.
- `crates/renovate-core/src/datasources/nuget.rs` — NuGet flat-container API:
  - `GET {api_base}/{lowercase_id}/index.json` → `{"versions": [...]}`.
  - Package ID lowercased in URL (NuGet API requirement).
  - Versions in ascending order; search in reverse for latest stable.
  - `is_stable`: version must not contain `-` (pre-release hyphen separator).
  - `fetch_updates_concurrent` with bounded JoinSet.
  - 5 tests (2 unit, 3 wiremock).
- `managers.rs` — `nuget` manager with patterns for `.csproj`, `.fsproj`,
  `.vbproj`, `.props`, `.targets`.
- `cli/main.rs` — NuGet pipeline with `build_dep_reports_nuget` helper.

### What was intentionally deferred
- `packages.config` XML format (legacy NuGet).
- `dotnet-tools.json` tool manifest.
- `global.json` SDK version.
- Custom NuGet feeds (via `nuget.config`).
- Directory.Packages.props multi-project detection.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (386 passed)

## Slice 0034 - Composer `composer.json` extractor + Packagist datasource

### Renovate reference
- `lib/modules/manager/composer/extract.ts` — `extractPackageFile`
- `lib/modules/datasource/packagist/index.ts`
- Pattern: `/(^|/)([\w-]*)composer\.json$/`

### What landed
- `crates/renovate-core/src/extractors/composer.rs` — JSON extractor using
  `serde_json` (already a dependency):
  - Sections: `require` (Regular), `require-dev` (Dev).
  - Skip reasons: `PlatformPackage` (`php`, `ext-*`, `lib-*`, `composer-*`,
    `hhvm`, any package name without `/`), `DevBranch` (version starts with
    `dev-` or ends with `-dev`).
  - Output sorted by name for deterministic ordering (HashMap is unordered).
  - 9 unit tests including Renovate's composer1.json fixture.
- `crates/renovate-core/src/datasources/packagist.rs` — Packagist metadata
  API v2 datasource:
  - `GET {api_base}/p2/{vendor}/{package}.json`.
  - Versions are newest-first in p2; `is_stable()` filters pre-releases
    (`-alpha`, `-beta`, `-RC`, `dev-*`, `*-dev`).
  - `fetch_updates_concurrent` with bounded JoinSet.
  - 5 tests: stability filtering, mock HTTP (returns first stable, 404, RC
    skipped to find stable).
- `managers.rs` — `composer` manager with pattern `(^|/)([\w-]*)composer\.json$`.
- `cli/main.rs` — composer pipeline wired with `build_dep_reports_composer`.

### What was intentionally deferred
- VCS repository dependencies (git URL form).
- Custom Satis/Packagist repositories.
- `composer.lock` lockfile parsing.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (367 passed)

## Slice 0033 - Go modules `go.mod` extractor + Go proxy datasource

### Renovate reference
- `lib/modules/manager/gomod/extract.ts` — `extractPackageFile`
- `lib/modules/manager/gomod/line-parser.ts` — `parseLine`
- `lib/modules/datasource/go/index.ts` — `GoDatasource`
- Pattern: `/(^|/)go\\.mod$/`

### What landed
- `crates/renovate-core/src/extractors/gomod.rs` — two-pass line scanner:
  1. First pass collects `replace X => ../local` directives.
  2. Second pass extracts `require` directives (single-line and block form).
  - Skip reasons: `PseudoVersion` (timestamp-hash pseudo-versions matching
    `^v\d+\.\d+\.\d+-\d{14}-[0-9a-f]+$`) and `LocalReplace` (module path in
    local replace set).
  - `// indirect` comments tracked; deps are included regardless.
  - `exclude (…)` blocks are skipped entirely.
  - 9 unit tests including the Renovate fixture.
- `crates/renovate-core/src/datasources/gomod.rs` — Go module proxy datasource:
  - `GET {proxy_base}/{module}/@latest` → `{"Version":"v1.8.1","Time":"…"}`.
  - `encode_module_path`: capital letters → `!` + lowercase (Go proxy protocol).
  - `fetch_updates_concurrent` with bounded JoinSet.
  - `GO_PROXY_BASE = "https://proxy.golang.org"`.
  - 4 tests: encoding, 2 wiremock HTTP tests.
- `managers.rs` — `gomod` manager added: `(^|/)go\.mod$`.
- `cli/main.rs` — gomod pipeline wired with `build_dep_reports_gomod` helper.

### What was intentionally deferred
- `go` version directive (`go 1.21`) — `GolangVersionDatasource`.
- `toolchain` directive.
- Non-local `replace` directives (module-to-module remapping).
- `go.sum` checksum verification.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (353 passed)

## Slice 0032 - Poetry `pyproject.toml` extractor + poetry manager

### Renovate reference
- `lib/modules/manager/poetry/extract.ts` — `extractPackageFile`
- `lib/modules/manager/poetry/index.ts` — `managerFilePatterns`,
  `supersedesManagers: ['pep621']`
- Pattern: `/(^|/)pyproject\\.toml$/`

### What landed
- `crates/renovate-core/src/extractors/poetry.rs` — Poetry pyproject.toml
  extractor using the `toml` crate (already a dependency):
  - Sections: `[tool.poetry.dependencies]` (Regular), `[tool.poetry.dev-dependencies]`
    (Dev), `[tool.poetry.group.*.dependencies]` (Group).
  - String form: `requests = "^2.28.0"` → version `^2.28.0`.
  - Inline table form: `django = {version = "4.2.7", optional = true}`.
  - Skip reasons: `PythonVersion` (python key), `GitSource` ({git = "…"}),
    `LocalPath` ({path = "…"}), `UrlInstall` ({url = "…"}).
  - Wildcard `"*"` → empty constraint (unconstrained dep).
  - Names normalized per PEP 503.
  - `nested_table` helper traverses arbitrary key chains in TOML.
  - 11 unit tests including fixture from Renovate's test suite.
- `crates/renovate-core/src/managers.rs` — `poetry` manager added with
  pattern `(^|/)pyproject\.toml$` (same file as pep621, different sections).
- `crates/renovate-core/src/extractors.rs` — `pub mod poetry` added.
- `crates/renovate-cli/src/main.rs`:
  - `poetry_extractor` import added; pep621 pipeline updated to use
    `pep621_extractor` alias.
  - Poetry pipeline wired: extract → filter actionable → PyPI datasource →
    `build_dep_reports_poetry` helper → `FileReport`.

### What was intentionally deferred
- `[tool.poetry]` version key (`packageFileVersion`).
- Poetry-specific version range semantics (`^`, `~` map to PEP 440 for now).
- Platform-conditional array form (`[{version = "…", platform = "…"}, …]`).
- `poetry.lock` lockfile parsing.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (340 passed)

## Slice 0031 - GitHub Actions `uses:` extractor + GitHub tags datasource

### Renovate reference
- `lib/modules/manager/github-actions/extract.ts` — `extractPackageFile`
- `lib/modules/manager/github-actions/parse.ts` — `parseUsesLine`, `isSha`,
  `isShortSha`, `versionLikeRe`
- `lib/modules/datasource/github-tags/index.ts`

### What landed
- `crates/renovate-core/src/extractors/github_actions.rs` — line-scanner (no
  YAML parser needed) extracting `uses:` entries from workflow files.
  - `USES_LINE` regex matches `uses:` lines with optional list prefix.
  - `parse_uses`: classifies as local (`./`), Docker (`docker://`), full SHA
    (40/64 hex), short SHA (6–7 hex), or actionable (version tag).
  - `owner_repo`: strips sub-path to get canonical `owner/repo` lookup key.
  - `strip_comment`: removes trailing `# comment` from YAML values.
  - Quoted actions (`"actions/checkout@v4"`) handled via `trim_matches`.
  - 10 unit tests including fixture with mixed dep types.
- `crates/renovate-core/src/datasources/github_tags.rs` — GitHub tags API.
  - `GET /repos/{owner/repo}/tags?per_page=100` → JSON array of tag names.
  - Returns first version-like tag (`v…` or digit-prefixed) — GitHub returns
    tags in reverse creation order so index 0 is most recent.
  - `api_base_from_endpoint(endpoint)` maps platform endpoint → GitHub API URL
    (GHE support: pass custom endpoint, falls back to `api.github.com`).
  - `fetch_updates_concurrent`: bounded JoinSet, same pattern as other DS.
  - 3 wiremock tests + 3 unit tests for `api_base_from_endpoint`.
- `crates/renovate-core/src/extractors.rs` — `pub mod github_actions` added.
- `crates/renovate-core/src/datasources.rs` — `pub mod github_tags` added.
- `crates/renovate-cli/src/main.rs` — GitHub Actions pipeline wired:
  - Derives `gh_api_base` from `config.endpoint`.
  - Builds an authenticated `HttpClient::with_token` for GitHub API calls.
  - Extracts `uses:`, filters actionable, fetches tags, emits `FileReport`.
  - `build_dep_reports_github_actions` helper follows existing pattern.

### What was intentionally deferred
- `action.yml` / composite action extraction.
- Gitea/Forgejo/GitHub Enterprise action lookup variants.
- SHA-pinned deps with renovate-pin comments (ratchet support).
- Docker `uses:` entries (separate Docker Hub datasource needed).

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (329 passed)

## Slice 0030 - Maven POM property resolution (`<properties>`)

### Renovate reference
- `lib/modules/manager/maven/extract.ts` — `applyProps` / `applyPropsInternal`
- Properties resolved before emitting each `PackageDependency`.

### What landed
- `crates/renovate-core/src/extractors/maven.rs`:
  - `extract()` split into `parse_pom()` (SAX pass, returns raw deps + property
    map) and a post-processing step that resolves `${key}` references.
  - `parse_pom()` now collects `<project><properties><key>value</key>` entries
    into a `HashMap<String, String>` alongside dep records.
  - `apply_props(value, props)` — up to 3 substitution passes for recursive
    property chains (e.g. `${alias}` → `${actual}` → `"1.2.3"`).
  - `substitute_props(value, props)` — single-pass `${key}` substitution;
    unknown keys are left as-is; unclosed `${` passed through.
  - Post-processing resolves both `dep_name` (groupId/artifactId may be props)
    and `current_value` (version). A dep whose version fully resolves loses
    the `PropertyRef` skip reason and becomes actionable. A dep with
    cross-file property refs keeps the skip reason.
  - `property_ref_skipped_when_not_defined` — renamed to reflect new behavior.
  - 6 new tests: resolved property, unresolved remains skipped, two-level
    recursive resolution, PDM-style fixture (groupId+artifactId as props),
    `substitute_props` unknown key, unclosed brace passthrough.

### What was intentionally deferred
- Cross-file (parent POM) property resolution.
- `${project.version}` / `${pom.version}` built-in properties.
- Profile-scoped `<properties>`.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (312 passed)

## Slice 0029 - Glob-based `ignorePaths` matching (globset)

### Renovate reference
- `lib/config/options/index.ts` — `ignorePaths` default:
  `['**/node_modules/**', '**/__tests__/**']`; patterns use minimatch.

### What landed
- Added `globset = "0.4.18"` workspace dependency.
- `crates/renovate-core/src/repo_config.rs`:
  - `PathMatcher` struct — pre-compiles `ignorePaths` patterns at construction
    time, separating glob patterns (contain `*`, `?`, or `[`) from plain-prefix
    patterns (trailing `/` stripped). Glob patterns compiled into a `GlobSet`
    via `globset::GlobSetBuilder`; prefix patterns checked with `starts_with`.
  - `RepoConfig::build_path_matcher() -> PathMatcher` — public method for
    efficient batch checking (build once, check many).
  - `RepoConfig::is_path_ignored` updated to call `build_path_matcher()`.
  - `RepoConfig::ignore_paths` doc comment updated to describe glob support.
  - 9 new tests: `**/node_modules/**`, `**/*.spec.ts`, `**/test/**`,
    rooted `test/**`, trailing-slash stripping, mixed glob+prefix, empty,
    integration with `RepoConfig::parse`.
- `crates/renovate-cli/src/main.rs` — file-list filter uses
  `repo_cfg.build_path_matcher()` once before the `filter()` iterator rather
  than calling `is_path_ignored` (which rebuilt the matcher) per file.

### What was intentionally deferred
- Brace expansion `{a,b}` (globset supports it via `GlobOptions`; not needed yet).
- Case-insensitive matching on Windows.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (307 passed)

## Slice 0028 - Run summary totals + `--quiet` mode

### Renovate reference
- Output UX improvement (no direct Renovate equivalent — Renovate logs via
  structured JSON, this adds an interactive summary footer).
- `--quiet` / `RENOVATE_QUIET` env var.

### What landed
- `crates/renovate-cli/src/cli.rs` — `--quiet` / `-q` / `RENOVATE_QUIET` flag
  (default `false`). Suppresses per-dependency listing; shows file-level
  summary lines only.
- `crates/renovate-cli/src/output.rs`:
  - `RunStats` struct with `repos_processed`, `repos_with_updates`,
    `repos_up_to_date`, `repos_with_errors`, `total_deps`, `total_updates`,
    `total_skipped`, `total_errors`.
  - `RunStats::add_report(&mut self, report: &RepoReport)` — accumulates counts
    from one repo's report.
  - `print_run_summary(stats: &RunStats, use_color: bool)` — prints a double-rule
    footer with repository and dep aggregate counts after the run.
  - `print_report` gains a `quiet: bool` parameter; when set, the per-dep
    `format_dep` lines are skipped while file-level counts remain.
  - 6 new tests: quiet smoke, stats accumulation over 1 and 2 repos, empty run
    summary, summary-with-updates smoke.
- `crates/renovate-cli/src/main.rs` — `quiet = cli.quiet` wired; `RunStats`
  accumulated across all repo outcomes; `print_run_summary` called after the
  join loop.
- `crates/renovate-cli/src/config_builder.rs` — `quiet: false` added to `Cli`
  constructor in tests.

### What was intentionally deferred
- `--quiet` propagation into `GlobalConfig` (not needed until quiet affects
  non-output behavior).
- JSON/machine-readable output mode (`--output-format=json`).

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (299 passed)

## Slice 0027 - Maven pom.xml extractor + Maven Central datasource

### Renovate reference
- `lib/modules/manager/maven/index.ts` — `managerFilePatterns`
- `lib/modules/manager/maven/extract.ts` — `extractAllPackageFiles`
- `lib/modules/manager/maven/dep-types.ts` — dep type taxonomy
- `lib/modules/datasource/maven/common.ts` — `MAVEN_REPO`
- `lib/modules/datasource/maven/util.ts` — `getMavenUrl`, `getDependencyParts`
- Patterns: `/(^|/|\\.)pom\\.xml$/`, `/^(((\\.mvn)|(\\.m2))/)?settings\\.xml$/`

### What landed
- Added `quick-xml = "0.39.2"` workspace dependency for streaming SAX-style XML parsing.
- `crates/renovate-core/src/extractors/maven.rs` — SAX-style POM extractor using
  quick-xml. Tracks element stack to correctly classify deps by section:
  `<dependencies>` → Regular, `<dependencyManagement><dependencies>` → Management,
  `<build><plugins><plugin>` → Plugin (default groupId `org.apache.maven.plugins`),
  `<build><extensions><extension>` → Extension, `<parent>` → Parent,
  `<profiles><profile><dependencies>` → Profile. Plugin's own nested
  `<dependencies>` are not captured as Regular deps. Property refs (`${…}`) →
  `MavenSkipReason::PropertyRef`. 12 unit tests including multiline element values,
  nested plugin dependencies, and default groupId.
- `crates/renovate-core/src/datasources/maven.rs` — Maven Central datasource.
  Fetches `maven-metadata.xml` from `https://repo.maven.apache.org/maven2/{group}/{artifact}/maven-metadata.xml`.
  Parses `<release>` → `<latest>` → last `<version>` precedence. `MavenUpdateSummary`,
  `MavenDepInput`, `fetch_updates_concurrent` (bounded JoinSet, same pattern as
  crates.io/npm). 5 unit tests + 1 mock integration test.
- `crates/renovate-core/src/managers.rs` — `maven` manager added with patterns
  `(^|/|\.)(pom\.xml)$` and `^((\.mvn|\.m2)/)?settings\.xml$`. Detection test added.
- `crates/renovate-cli/src/main.rs` — Maven pipeline wired: extract deps → filter
  by `repo_cfg.is_dep_ignored` + non-empty version → Maven Central lookup →
  `build_dep_reports_maven` helper → `FileReport`.

### What was intentionally deferred
- `settings.xml` content extraction (currently matched but not extracted).
- Maven property resolution (`${spring.version}` → actual version).
- Maven version range syntax (`[1.0,2.0)`) — treated as skip for now.
- Non-Maven-Central registries (Sonatype Nexus, JFrog, GitHub Packages).
- `<distributionManagement>` and `<relocation>` elements.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (293 passed)

## Slice 0026 - pyproject.toml (PEP 621/735) extractor + pep621 manager

### Renovate reference
- `lib/modules/manager/pep621/extract.ts` — `extractPackageFile`
- `lib/modules/manager/pep621/schema.ts` — `PyProject`
- Pattern: `/(^|/)pyproject\\.toml$/`

### What landed
- `crates/renovate-core/src/extractors/pep621.rs` — parses `pyproject.toml`
  using the `toml` crate (already a dependency); extracts deps from:
  `[project].dependencies` (Regular), `[project.optional-dependencies].*`
  (Optional), `[dependency-groups].*` (Group, PEP 735). Handles PEP 508
  strings: strips env markers (`;`), strips extras (`[...]`), normalizes
  names per PEP 503. Classifies direct references (`@`) and group-include
  tables as skip reasons. 12 unit tests including the PDM fixture.
- `crates/renovate-core/src/managers.rs` — `pep621` manager added with
  `(^|/)pyproject\.toml$` pattern.
- `crates/renovate-core/src/extractors.rs` — `pub mod pep621` added.
- `crates/renovate-cli/src/main.rs` — pep621 pipeline wired: extract deps
  → filter by `repo_cfg.is_dep_ignored` → PyPI datasource → report.

### What was intentionally deferred
- `[tool.poetry.dependencies]` (Poetry) — separate manager.
- `[tool.pdm.dev-dependencies]` (PDM tool section) — separate slice.
- `build-system.requires` — build tool deps.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (276 passed)

## Slice 0025 - Per-repo renovate.json config parsing + application

### Renovate reference
- `lib/config/options/index.ts` — `enabled`, `ignoreDeps`, `ignorePaths`
- `lib/config/app-strings.ts` — `configFileNames`

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `RepoConfig { enabled, ignore_deps, ignore_paths }` struct with manual
    `Default` impl (`enabled = true` per Renovate defaults).
  - `RepoConfig::parse(content)` — parses JSON/JSON5 via the `json5` crate;
    falls back to `RepoConfig::default()` on parse failure.
  - `is_dep_ignored(name)` — exact string match against `ignoreDeps`.
  - `is_path_ignored(path)` — prefix match against `ignorePaths`.
  - `RepoConfigResult::Found { config: RepoConfig, .. }` — content field
    replaced with the parsed struct.
  - 9 unit tests: defaults, `enabled: false`, ignoreDeps, ignorePaths,
    JSON5 comments, malformed JSON fallback, exact dep match, path prefix
    match.
- `crates/renovate-cli/src/main.rs`:
  - `repo_cfg` extracted from discovery result and applied:
    - `!repo_cfg.enabled` → skip entire repo (early return)
    - File list filtered through `repo_cfg.is_path_ignored` before manager
      detection
    - Cargo, npm, and pip dep actionable lists filtered through
      `repo_cfg.is_dep_ignored`

### What was intentionally deferred
- `extends` preset resolution.
- `packageRules` matching.
- Glob/minimatch `ignorePaths` support (currently prefix-only).
- Dockerfile/compose `ignoreDeps` (image names are different from dep names).

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (266 passed)

## Slice 0024 - docker-compose image extractor

### Renovate reference
- `lib/modules/manager/docker-compose/extract.ts` — `extractPackageFile`

### What landed
- `crates/renovate-core/src/extractors/docker_compose.rs` — line-scan
  extractor for docker-compose files; no YAML dependency required.
  Tracks service blocks by indentation to detect `build:` directives
  (skip) and `image:` values.  Strips single/double quote wrapping.
  Classifies variable interpolation (`${VAR}`) as `VariableRef` skip.
  Delegates image parsing to `dockerfile::classify_image_ref`.  11 unit
  tests including Renovate fixture cases.
- `crates/renovate-core/src/extractors/dockerfile.rs` — exposes public
  `classify_image_ref(image_ref)` wrapper (calls `classify_from` with
  empty stage-names slice) so compose module can reuse it.
- `crates/renovate-core/src/extractors.rs` — `pub mod docker_compose` added.
- `crates/renovate-cli/src/main.rs` — docker-compose pipeline wired into
  `process_repo`; uses the same Docker Hub datasource as the Dockerfile
  pipeline.

### What was intentionally deferred
- Full YAML parsing (needed for YAML anchors/aliases with image values).
- `extends:` service composition.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (258 passed)

## Slice 0023 - HTTP retry with exponential backoff + Retry-After

### Renovate reference
- `lib/util/http/retry-after.ts` — `wrapWithRetry`, `getRetryAfter`
- Max retries: 2 (Renovate) / 3 (our implementation, slightly more generous)

### What landed
- `crates/renovate-core/src/http.rs`:
  - `get_retrying(&self, url)` — retry loop: retries on 429/503/504, up to
    `MAX_RETRIES = 3` times. Respects `Retry-After` header (numeric seconds
    form); falls back to exponential backoff `BASE_DELAY_MS × 2^attempt`
    (capped at 30s). Hard cap of 60s on `Retry-After` delays. Returns final
    response regardless of status — callers check it.
  - `#[cfg(test)]` `BASE_DELAY_MS = 10` so retry tests run fast.
  - `get_json` updated to use `get_retrying` internally.
  - `is_retryable`, `retry_delay`, `parse_retry_after` helpers.
  - 7 new wiremock-based tests: 429→200 retry, stop after max retries, no
    retry on 404, 503→200 via `get_json`, `Retry-After` header parsing.
- Updated all non-test HTTP call sites to `get_retrying`:
  - `datasources/crates_io.rs`, `npm.rs`, `pypi.rs`, `docker_hub.rs`
  - `platform/gitlab.rs` (all 3 `send()` calls replaced)
  - `platform/github.rs` benefits via the `get_json` path already updated.

### Deferred
- HTTP-date form of `Retry-After` header (uncommon in practice).
- Per-host rate-limit tracking (Renovate's throttle rules).
- Jitter on exponential backoff.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (248 passed)

## Slice 0022 - GitLab platform client

### Renovate reference
- `lib/modules/platform/gitlab/index.ts` — `initPlatform`, `getRawFile`,
  `getRepoInfo`
- Default endpoint: `https://gitlab.com/api/v4`

### What landed
- `crates/renovate-core/src/platform/gitlab.rs` — `GitlabClient` implementing
  `PlatformClient`:
  - `get_current_user` → `GET /user` (returns `username` field).
  - `get_raw_file` → `GET /projects/{ns%2Frepo}/repository/files/{encoded_path}?ref=HEAD`;
    decodes base64 content (GitLab may include newlines in the base64 payload
    — these are stripped before decoding).
  - `get_file_list` → paginates `GET /projects/{id}/repository/tree?recursive=true&per_page=100&page={n}`,
    filters to `type == "blob"` entries, stops on last page or 50-page cap.
  - 9 wiremock-based tests: auth success/401, file fetch, 404, path-slash
    encoding, blob-only filtering, pagination.
- `crates/renovate-core/src/platform.rs`:
  - `pub mod gitlab` added.
  - `AnyPlatformClient::Gitlab(GitlabClient)` variant added.
  - `AnyPlatformClient::create` handles `Platform::Gitlab` (with optional
    custom endpoint for self-hosted GitLab).
  - All three dispatch methods (`get_current_user`, `get_raw_file`,
    `get_file_list`) have the `Gitlab` arm added.

### What was intentionally deferred
- `PRIVATE-TOKEN` vs `Authorization: Bearer` header selection — currently
  the Bearer form is used for both PATs and OAuth tokens (GitLab accepts
  both; a later slice can detect which to use from the token format).
- GitLab merge request creation / update (PR pipeline).
- Group-level namespaces with subgroups.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (241 passed)

## Slice 0021 - Docker Hub datasource + Dockerfile pipeline complete

### Renovate reference
- `lib/modules/datasource/docker/index.ts` — `_getDockerHubTags`
- `lib/modules/datasource/docker/schema.ts` — `DockerHubTagsPage`
- `lib/modules/datasource/docker/common.ts` — `getRegistryRepository`

### What landed
- `crates/renovate-core/src/datasources/docker_hub.rs`:
  - `parse_image_name` — resolves `ubuntu` → `library/ubuntu`, detects
    non-Docker-Hub registries (any component with `.` or `:` prefix).
  - `fetch_tags` — paginates `hub.docker.com/v2/repositories/{ns}/{repo}/tags`
    up to 2 pages (200 tags) with `ordering=last_updated`.
  - `split_version_tag` / `cmp_version` / `docker_update_summary` — variant-
    suffix-aware component-wise version comparison: `"18-alpine"` only
    competes with other `-alpine` tags; `"22.04.1"` > `"22.04"`.
  - `fetch_updates_concurrent` — bounded JoinSet + Semaphore batch fetch.
  - 15 unit tests + 3 wiremock-based integration tests.
- `crates/renovate-core/src/datasources.rs` — `pub mod docker_hub` added.
- `crates/renovate-cli/src/main.rs` — Dockerfile section upgraded: builds
  `DockerDepInput` list, calls `fetch_updates_concurrent`, maps results to
  `DepReport`.  Non-Docker-Hub images (GHCR, ECR, custom registries) are
  surfaced as `Skipped { reason: "non-docker-hub registry" }`.

### What was intentionally deferred
- Docker registry v2 token auth (for private images / non-Hub registries).
- ECR, GHCR, Google Artifact Registry datasources.
- Digest pinning updates (`@sha256:…` detection).
- `--platform` flag handling for multi-arch images.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (232 passed)

## Slice 0020 - Manager regex caching + Dockerfile FROM extractor

### Renovate reference
- `lib/modules/manager/dockerfile/extract.ts` — `extractPackageFile`
- Pattern: `/(^|/)(Dockerfile|Containerfile)(\.[^/]*)?$/`

### What landed
- `crates/renovate-core/src/managers.rs` — replaced per-call regex
  compilation with `LazyLock<Vec<(&str, Vec<Regex>)>>` (`COMPILED`);
  patterns are now compiled exactly once at first use.  The `detect()` function
  became simpler and faster.
- `crates/renovate-core/src/extractors/dockerfile.rs` — Parses `FROM`
  instructions with multi-line continuation (`\`), strips `--platform=`
  flags, splits `AS alias`, tracks build stage names to detect
  `BuildStageRef` skip reasons.  Also handles `scratch` and ARG variable
  (`$VAR`) skip reasons.  Registry port in image name (`host:5000/image`)
  is not confused with a tag colon.  16 unit tests.
- `crates/renovate-cli/src/main.rs` — Dockerfile section wired into
  `process_repo`; reports images without registry lookup (Docker Hub
  datasource is a separate slice).

### What was intentionally deferred
- Docker Hub / GHCR registry datasource — planned for a follow-on slice.
- ARG value substitution before image classification.
- COPY `--from=stage` parsing.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (216 passed)

## Slice 0019 - Parallel repository processing

### What landed
- `crates/renovate-cli/src/main.rs` refactored:
  - Extracted `process_repo(client, http, repo_slug, config)` async function
    returning `(Option<RepoReport>, had_error)`.  The `None` case means the
    repo was skipped without producing a report.
  - Added `REPO_CONCURRENCY = 4` constant and a `JoinSet<(slug, report, bool)>`
    bounded by `Arc<Semaphore>`, mirroring Renovate's worker queue model.
  - Reports are printed serially in the join loop (completion order) to avoid
    interleaved stdout from concurrent tasks.
  - Added three `build_dep_reports_{cargo,npm,pip}` helper functions to remove
    the duplicated skip-reason + update-map rendering logic.
  - `manager_files(detected, name)` helper DRYs the matched-files lookup.
  - Both `AnyPlatformClient` and `GlobalConfig` already derived `Clone` —
    no changes needed there; `HttpClient` (reqwest::Client Arc) also clones
    cheaply so each task gets its own handles into the shared connection pool.

### Deferred
- Configurable concurrency via CLI flag (`--queue-concurrency`).
- Per-repo error isolation (a panicking task currently only logs, not
  hard-exits).

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (200 passed)

## Slice 0018 - pip_requirements extractor + PyPI datasource

### Renovate reference
- `lib/modules/manager/pip_requirements/extract.ts` — `extractPackageFile`
- `lib/modules/datasource/pypi/index.ts` — `PypiDatasource`
- `lib/modules/datasource/pypi/types.ts` — `PypiJSON`
- `lib/modules/versioning/pep440/index.ts` — PEP 440 semantics

### What landed
- `crates/renovate-core/src/extractors/pip.rs` — parses `requirements.txt`
  lines: strips comments, environment markers, hash directives; classifies
  skip reasons (GitSource, UrlInstall, SubRequirement); normalizes package
  names per PEP 503. 15 unit tests including real-world fixture cases.
- `crates/renovate-core/src/versioning/pep440.rs` — `exact_pin_version` detects
  `==X.Y.Z` pins; `pep440_update_summary` flags update when pin differs from
  registry latest; ranges/unconstrained never flagged. 9 unit tests.
- `crates/renovate-core/src/datasources/pypi.rs` — fetches from
  `https://pypi.org/pypi/{name}/json`; uses `info.version` as latest stable;
  filters yanked releases; bounded concurrent fetches via JoinSet + Semaphore.
  6 wiremock-based tests.
- `crates/renovate-core/src/extractors.rs`, `datasources.rs`, `versioning.rs`
  — `pub mod pip/pep440/pypi` declarations added.
- `crates/renovate-cli/src/main.rs` — pip_requirements processing wired into
  the per-repo loop alongside Cargo and npm.

### What was intentionally deferred
- PEP 440 full range semantics (`~=`, `!=`, multiple specifiers) — currently
  only exact pins (`==x.y.z`) are flagged as updatable; ranges report latest
  without update_available.
- Custom index-url support (`--index-url`, `--extra-index-url` in requirements
  files) — registry always defaults to pypi.org.
- `pip_setup` and `pipenv` managers — separate slices.

### Blockers
None.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (200 passed)

## Slice 0017 - Human-readable update report output

### Renovate reference
- Loop prompt: "Default interactive output should be colorful, intuitive, and
  easy to understand at a glance: group by repository and dependency, use
  semantic color consistently."
- Renovate's own output is structured logging; the UX improvement here is a
  Rust-native enhancement.

### What landed
- `crates/renovate-cli/src/output.rs` — `DepStatus`, `DepReport`, `FileReport`,
  `RepoReport` data model; `print_report(report, use_color)` renderer;
  `should_use_color()` (checks `NO_COLOR` env + stdout TTY). Color uses raw
  ANSI escape codes — no new dependencies. Green = up-to-date/success,
  yellow = update available, red = error, dim = skipped/metadata.
  10 unit tests.
- `crates/renovate-cli/src/main.rs` — per-dep `tracing::info!` calls replaced
  with structured `RepoReport` collection; `print_report` called once per repo
  at the end of the repo loop. Debug-level tracing kept for extraction counts.

### What was intentionally deferred
- `--quiet` flag suppression of the report (deferred to CLI flag slice).
- Full `LOG_FORMAT=json` structured report output for CI integration.
- Dep counts in the file header vs. full dep listing (currently always verbose).

### Blockers
None.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (168 passed)

## Slice 0016 - npm registry datasource + npm versioning

### Renovate reference
- `lib/modules/datasource/npm/index.ts` — `NpmDatasource`
- `lib/modules/datasource/npm/get.ts` — `getDependency`
- `lib/modules/datasource/npm/types.ts` — `NpmResponse` / `NpmResponseVersion`
- `lib/modules/versioning/npm/index.ts` — node-semver semantics

### What landed
- `crates/renovate-core/src/versioning/npm.rs` — `NpmUpdateSummary`,
  `parse_constraint`, `resolve_latest_compatible`, `npm_update_summary`,
  `is_exact_pin`. Key difference from Cargo versioning: npm bare `"1.2.3"`
  is an exact pin (`=1.2.3`), not a compatible range. Detects updates by
  comparing the current pin against the registry's `latest` dist-tag.
  15 unit tests covering pin detection, range resolution, and update summary.
- `crates/renovate-core/src/datasources/npm.rs` — `fetch_versions` (fetches
  packument from `{registry}/{encoded_name}`, filters deprecated versions,
  sorts oldest-first), `fetch_updates_concurrent` (bounded JoinSet + Semaphore,
  same pattern as crates.io). Scoped package names encoded with `%2F`.
  7 wiremock-based tests.
- `crates/renovate-core/src/versioning.rs` and `datasources.rs` — `pub mod npm`
  declarations added.
- `crates/renovate-cli/src/main.rs` — npm processing wired into per-repo loop
  alongside existing Cargo processing: detect npm manager → fetch each
  `package.json` → extract deps → concurrent registry lookups → log results.

### What was intentionally deferred
- npmrc / scoped registry overrides — npm packages can use custom registries
  per scope; deferred to a later slice.
- `deprecated` flag surfaced in update log output — currently filtered silently.
- Retry and rate-limit logic in `HttpClient`.

### Blockers
None.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (158 passed)

## Slice 0007 - tokio async runtime + HttpClient + GitHub platform stub

### Renovate reference
- `lib/modules/platform/github/index.ts` — `initPlatform(config)` which
  calls `GET /user` to verify the token.
- `lib/util/http/index.ts` — Renovate's internal HTTP client with user-agent
  and retry logic.

### What landed
- `tokio` and `reqwest` added to workspace deps; `wiremock` added as dev dep.
- `main()` converted to `#[tokio::main] async fn main()`.
- `crates/renovate-core/src/http.rs` — `HttpClient` wrapping `reqwest::Client`
  with `renovate-rust/<version>` User-Agent and optional bearer-token auth.
  `get_json<T>()` sends GET, maps non-2xx to `HttpError::Status`.
- `crates/renovate-core/src/platform.rs` — `PlatformClient` trait with
  `get_current_user() -> Result<CurrentUser, PlatformError>`; `PlatformError`
  with `Http`, `Unauthorized`, `Unexpected` variants.
- `crates/renovate-core/src/platform/github.rs` — `GithubClient` implementing
  `PlatformClient`; supports custom endpoint for GHE.
- 4 wiremock-based tests (success, 401→Unauthorized, bearer header verified,
  GHE custom endpoint). Tests spin up a real TCP mock server — no live network.
- 78 total tests, all passing.

### What was intentionally deferred
- Token validation in the main pipeline (the builder doesn't call
  `get_current_user()` yet — that comes when the worker pipeline lands).
- Retry/rate-limit logic in `HttpClient`.
- GitLab, Bitbucket, etc. platform clients.

### Blockers
None.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (78 passed)

## Slice 0006 - Global config file loading

### Renovate reference
- `lib/workers/global/config/parse/file.ts` — `getConfig(env)`:
  `RENOVATE_CONFIG_FILE ?? 'config.js'`, format detection, parse errors
  → fatal+exit 1.
- `lib/workers/global/config/parse/util.ts` — `getParsedContent(file)`:
  per-extension routing (`.renovaterc` → JSON, `.json5` → JSON5, `.js` →
  ESM/CJS import).

### What landed
- `serde`, `serde_json`, `json5`, `tempfile` added to workspace deps.
- `#[derive(serde::Deserialize)]` + `#[serde(rename_all = "camelCase", default)]`
  on `GlobalConfig` and all enum types so JSON config files deserialize
  directly into canonical types.
- `crates/renovate-core/src/config/file.rs` with:
  - `ConfigFileError` (thiserror) — path-not-found, unsupported-format,
    IO, parse.
  - `resolve_config_path(env, cwd)` — returns the path to load (or `None`
    if no env var set); errors when an explicit path doesn't exist.
  - `load(path)` — routes `.json` / `.renovaterc` to `serde_json`, `.json5`
    to the `json5` crate; rejects `.js`/`.cjs`/`.mjs` with a clear error.
  - `merge_over_base(base, file_config)` — field-by-field merge; Option
    fields use `or` semantics; non-Option fields from file always win
    (CLI override happens after).
- `config_builder::build(cli, base)` refactored to take a `base`
  `GlobalConfig` so CLI args are applied as the final layer.
- `main.rs` wires the full pipeline: `defaults → file (RENOVATE_CONFIG_FILE)
  → CLI` with structured logging at each step.
- 11 unit tests in `file.rs` (resolve, load JSON, load JSON5, load .js
  rejection, parse error, merge semantics). 74 total tests, all passing.
- Compatibility decision CD-0003 documented (no JS support, no config.js
  default, YAML deferred).

### What was intentionally deferred
- YAML (`.yaml`, `.yml`) support — deferred pending a stable maintained
  `serde_yaml` successor.
- `.renovaterc` (no extension) file auto-discovery without
  `RENOVATE_CONFIG_FILE` set — deferred to a future slice.
- `processEnv` key export from config file.
- `migrateAndValidateConfig` porting (config migration + validation).

### Blockers
None.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (74 passed)

## Slice 0005 - GlobalConfig struct + CLI→config builder

### Renovate reference
- `lib/config/options/index.ts` — option defaults and allowed values.
- `lib/workers/global/config/parse/cli.ts` `getConfig` — dryRun "true"→"full",
  requireConfig "true"→"required"/"false"→"optional" coercions with warn.
- `lib/constants/platforms.ts` — `PLATFORM_HOST_TYPES`.

### What landed
- `crates/renovate-core/src/config.rs` — `GlobalConfig` struct with typed
  fields and a `Default` impl matching Renovate's option defaults.
- `crates/renovate-core/src/config/platform.rs` — `Platform` canonical enum
  with `Display` impl (kebab-case strings matching upstream).
- `crates/renovate-core/src/config/run.rs` — `DryRun`, `RequireConfig`,
  `ForkProcessing`, `RecreateWhen` canonical enums with `Display`.
- `crates/renovate-cli/src/config_builder.rs` — `build(&Cli) -> GlobalConfig`:
  maps CLI types to core types, emits `tracing::warn` for legacy boolean
  variants (`DryRunArg::LegacyTrue` → `Full`, etc.) matching Renovate's
  deprecation warnings.
- Wired in `main.rs`: after arg parsing, `config_builder::build(&cli)` runs
  and emits a debug log with the resolved platform/dry_run.
- 10 unit tests in `config_builder.rs` covering all coercion paths and defaults.
- 63 total tests, all passing.

### Architecture note
`renovate-core` owns the **canonical** types (no legacy variants); the CLI
crate owns the CLI-facing types with legacy variants; `config_builder` bridges
the two. This avoids dragging clap types into the core library.

### Blockers
None.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (63 passed)

## Slice 0004 - Option surface first-cut + env vars

### Renovate reference
- `lib/config/options/index.ts` — option definitions for `platform`,
  `token`, `endpoint`, `dryRun`, `requireConfig`, `forkProcessing`,
  `platformAutomerge`, `recreateWhen`, `allowedCommands`,
  `allowCommandTemplating`, `hostRules`, `registryAliases`.
- `lib/config/options/env.ts` — `getEnvName` maps camelCase names to
  `RENOVATE_UPPER_SNAKE_CASE` env vars.
- `lib/constants/platforms.ts` — `PLATFORM_HOST_TYPES` constant.
- `lib/workers/global/config/parse/cli.ts` — `getConfig` coercions for
  `dryRun` ("true"→"full", "false"/"null"→null) and `requireConfig`
  ("true"→"required", "false"→"optional").

### What landed
- `crates/renovate-cli/src/cli.rs` — new module holding the `Cli` struct
  and associated `ValueEnum` types. `main.rs` is now thin (logging,
  migration, parse, dispatch).
- Registered flags: `--platform` (`Platform` enum with all 11 values),
  `--token`, `--endpoint`, `--dry-run` (`DryRunArg` enum with
  extract/lookup/full plus legacy true/false/null variants), `--require-config`
  (`RequireConfigArg` with required/optional/ignored + legacy true/false),
  `--fork-processing`, `--platform-automerge`, `--recreate-when`,
  `--allowed-commands`, `--allow-command-templating`, `--host-rules`,
  `--registry-aliases`.
- Every flag backed by its `RENOVATE_*` env var via clap's `env` feature.
- Legacy "true"/"false" variants in `DryRunArg` and `RequireConfigArg`
  so `--dry-run=true` (produced by `migrateArgs`) and `--require-config=true`
  are accepted without error. Conversion to canonical values is deferred to
  the config layer (next slice).
- 15 new integration tests completing the migrateArgs end-to-end chain
  plus env var coverage. 53 tests total, all passing.

### What was intentionally deferred
- `DryRunArg::canonical()` / `RequireConfigArg::canonical()` conversion
  methods and their callers — the config layer isn't yet wired.
- JSON5 parsing for `--allowed-commands` and `--host-rules` / `--registry-aliases`
  (accepted as raw strings; a `coercions` parity slice will parse them).
- Remaining option surface (hundreds of per-repo options); the next
  option-surface slice will add the most commonly used ones.

### Blockers
None.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (53 passed)

## Slice 0003 - Logger init

### Renovate reference
- `lib/logger/index.ts` — `init()`, `logLevel()`, `LOG_LEVEL` env, default `"info"`.
- `lib/logger/bunyan.ts` — `validateLogLevel`, `createLogger`,
  `LOG_FORMAT=json` vs pretty-stdout, `LOG_FILE`/`LOG_FILE_LEVEL`/`LOG_FILE_FORMAT`.
- `lib/logger/types.ts` — `BunyanLogLevel` alias for Bunyan's
  `LogLevelString`: `"trace" | "debug" | "info" | "warn" | "error" | "fatal"`.

### What landed
- `crates/renovate-cli/src/logging.rs` with:
  - `parse_log_level(&str) -> ParseLevelResult` — maps Renovate's 6 level
    names to `tracing::Level`; `fatal` → `Level::ERROR` (Bunyan-specific,
    no tracing equivalent above `error`); unknown → `Invalid`.
  - `should_use_ansi()` — detects TTY on stderr and respects `NO_COLOR`.
  - `init() -> InitResult` — reads `LOG_LEVEL` (default `info`) and
    `LOG_FORMAT` (default pretty). Sets up `tracing-subscriber` `fmt`
    subscriber writing to stderr; uses `.json()` when `LOG_FORMAT=json`.
- Invalid `LOG_LEVEL` exits 1 with a JSON-formatted fatal message
  matching Renovate's `validateLogLevel` behavior.
- `tracing-subscriber` `json` feature enabled in workspace `Cargo.toml`.
- `main.rs` — logging initialized first, before argv migration and arg
  parsing, matching Renovate's startup order.
- 7 unit tests (level parsing for all 6 valid names + invalid cases).
- 5 integration tests (invalid level → exit 1; debug/fatal/JSON/NO_COLOR
  → exit 0).

### What was intentionally deferred
- `LOG_FILE` / `LOG_FILE_LEVEL` / `LOG_FILE_FORMAT` support — the file
  logging path is orthogonal to stdout and can land as its own slice.
- `LOG_FORMAT=pretty` explicit format variant and colored human output
  improvements — the fmt subscriber's default is already human-readable;
  formatting polish comes later.
- `LOG_CONTEXT` env var for structured request IDs.

### Blockers
None.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features` (38 passed)

## Slice 0002 - `migrateArgs` parity

### Renovate reference
- `lib/workers/global/config/parse/cli.ts` - `migrateArgs` function
  (substring rewrites + `--git-fs*` filter, applied before
  `parseEarlyFlags` and `getConfig`).
- `lib/workers/global/config/parse/cli.spec.ts` - the table-driven test at
  lines 125-143 (`--azure-auto-complete`, `--git-lab-automerge`,
  `--recreate-closed*`, `--endpoints=`) plus the `--dry-run` /
  `--require-config` regex cases at lines 175-208.

### What landed
- `crates/renovate-cli/src/migrate.rs` with `migrate_args(&[String]) -> Vec<String>`.
- Faithful port of upstream's 19 substring rewrites + 2 anchored regexes +
  `--git-fs*` filter, applied in upstream's exact order. JavaScript
  `String.prototype.replace(string, string)` first-occurrence semantics
  preserved via Rust `str::replacen(_, _, 1)`.
- 22 unit tests covering every transformation, ordering edge cases (chained
  `--renovate-fork` → `--include-forks` → `--fork-processing=enabled`),
  the first-occurrence-only behavior for JSON-key rewrites inside
  `--host-rules` values, and the no-op pass-through path.
- Wired into `crates/renovate-cli/src/main.rs`: `std::env::args()` is
  passed through `migrate_args` before clap parses, mirroring Renovate's
  `parseEarlyFlags` / `getConfig` pipeline order.
- 1 integration test (`git_fs_legacy_flags_are_silently_dropped`) proves
  the wiring is live: a `--git-fs-something` arg that would otherwise be
  rejected by clap as unknown (exit 2) now disappears and the CLI exits 0.

### What was intentionally deferred
- End-to-end integration tests for the rewritten flags (`--dry-run`,
  `--include-forks=true`, etc.). They cannot be exercised at the CLI
  boundary until the option surface lands - clap would still reject the
  rewritten forms as unknown. Unit tests cover the transformation
  correctness; the integration tests will follow when `--dry-run` &c. are
  recognized by the parser.

### Blockers
None for the implementation. Push to `origin/main` is blocked in the
current execution environment because no SSH key, `gh auth`, or git
credential helper is configured. Slice was committed locally; user can
push manually or the next loop iteration will retry once credentials are
available.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`

(Results recorded in the slice's commit body.)

## Slice 0001 - Workspace + early CLI flags

### Renovate reference
- `lib/renovate.ts` - CLI entry orchestration.
- `lib/workers/global/config/parse/cli.ts` - `parseEarlyFlags`,
  `getCliName`, `migrateArgs`, `getConfig`. Notes the `-v, --version`
  Commander binding and the bare-version output contract.
- `package.json` - confirms the `renovate` binary name.

### What landed
- Cargo workspace with two crates:
  - `crates/renovate-cli` builds the `renovate` binary.
  - `crates/renovate-core` placeholder for shared domain types.
- Rust toolchain pinned via `rust-toolchain.toml` (1.95.0, rustfmt + clippy).
- Strict workspace lints in `Cargo.toml`:
  - `forbid(unsafe_code)` and selected clippy warns (no whole-group enables).
  - `print_stdout` / `print_stderr` denied workspace-wide; the cli crate
    re-allows them with a `reason` attribute so the only legitimate
    user-facing surface is funneled through one crate.
- `rustfmt.toml` (edition 2024, 100-col, Unix newlines).
- `cargo-nextest` profiles in `.config/nextest.toml` (default + ci).
- Minimal CLI:
  - `-v` / `--version` prints the bare version line (`<version>\n`),
    matching Renovate's commander output rather than clap's default
    `<bin> <version>` form.
  - `--help` works (clap default, exit 0).
  - Positional `repositories` accepted (no-op for now).
  - Unknown flags exit with clap's usage error (exit code 2).
- Integration tests via `assert_cmd` covering version output, help, unknown
  flags, and the no-args path. These pin behavior that downstream tooling is
  most likely to grep.

### What was intentionally deferred
- The full Renovate option surface from `lib/config/options/index.ts`. Clap
  derive structs will be generated in a later slice once we decide whether
  to keep one giant flat `Cli` struct or split by subcommand/category.
- `migrateArgs` rewriting (deprecated flag aliasing). Will land alongside
  the option surface so we can write parity tests against Renovate's
  `parseEarlyFlags` examples directly.
- Color/no-color policy and human-output styling. clap's anstyle defaults
  already respect `NO_COLOR` and TTY detection; we'll formalize the policy
  when the first user-facing rendering arrives.
- Logging (`tracing` / `tracing-subscriber`) - dependencies declared in
  the workspace but not yet initialized in `main`.

### Blockers
None. No network or credentials were required for this slice.

### Verification
- `cargo build --workspace --all-features`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`

(Results recorded in the slice's commit body.)

## Slice 0037 - Bundler `Gemfile` extractor + RubyGems datasource

### Renovate reference
- `lib/modules/manager/bundler/extract.ts` — `extractPackageFile`
- `lib/modules/manager/bundler/index.ts` — `defaultConfig`, pattern `/(^|/)Gemfile$/`
- `lib/modules/datasource/rubygems/index.ts` — `RubygemsDatasource`
- API: `GET https://rubygems.org/api/v1/versions/{gemname}.json`

### What landed
- `crates/renovate-core/src/extractors/bundler.rs` — line-scanner Gemfile extractor:
  - Handles `gem 'name'`, `gem 'name', '~> 7.0'`, and multi-constraint forms
    (`gem 'pg', '>= 0.18', '< 2.0'` → `">= 0.18, < 2.0"`).
  - Git source detection: `git:`, `github:`, `gitlab:` options → `GitSource`.
  - Path source detection: `path:` option → `PathSource`.
  - Block depth tracking for `group :development, :test do … end` blocks.
  - Double and single quoted gem names handled without backreferences (RE2 limit).
- `crates/renovate-core/src/datasources/rubygems.rs` — RubyGems REST client:
  - `GET /api/v1/versions/{gem}.json` → array newest-first, filter `prerelease == false`.
  - `lower_bound_version()` strips leading operators (`~>`, `>=`, etc.) to extract the
    pinned lower bound for `update_available` comparison.
  - Concurrent lookups via `JoinSet` + `Arc<Semaphore>`.
- `crates/renovate-core/src/managers.rs` — added `bundler` with pattern `(^|/)Gemfile$`.
- `crates/renovate-cli/src/main.rs` — wired bundler pipeline section +
  `build_dep_reports_bundler` helper.

### What was intentionally deferred
- `Gemfile.lock` lockfile parsing and locked-version pinning.
- Gemspec (`.gemspec`) files.
- Custom Gemfile sources (non-rubygems.org registries).
- `ruby` version directive as a `RubyVersionDatasource` lookup.
- git-ref source branch/tag pinning.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 340 passed

## Slice 0038 - Terraform `.tf`/`.tofu` extractor + Terraform Registry datasource

### Renovate reference
- `lib/modules/manager/terraform/index.ts` — `defaultConfig`, patterns `**/*.tf`, `**/*.tofu`
- `lib/modules/manager/terraform/extractors/terraform-block/required-provider.ts`
- `lib/modules/manager/terraform/extractors/others/modules.ts`
- `lib/modules/datasource/terraform-provider/index.ts` — v2 API
- `lib/modules/datasource/terraform-module/index.ts` — v1 API

### What landed
- `crates/renovate-core/src/extractors/terraform.rs` — brace-depth state machine extractor:
  - `terraform { required_providers { ... } }` — provider deps with `source` + `version`.
  - `module "name" { source = "...", version = "..." }` — module deps.
  - Inline string form: `provider = "~> 5.0"` in required_providers.
  - Skip reasons: `ExternalSource` (git/https/local path), `NoVersionConstraint` (no version field).
  - `lower_bound_version()` strips operators for accurate `update_available` comparison.
  - Does NOT use a full HCL parser — handles common single-line patterns only.
- `crates/renovate-core/src/datasources/terraform.rs` — Terraform Registry clients:
  - Provider: `GET /v2/providers/{ns}/{type}?include=provider-versions` (newest-first in `included`).
  - Module: `GET /v1/modules/{ns}/{name}/{provider}/versions` (first entry is newest).
  - Bare provider names (e.g. `random`) default to `hashicorp` namespace.
  - Concurrent bounded lookups via `JoinSet` + `Arc<Semaphore>`.
- `crates/renovate-core/src/managers.rs` — added `terraform` with patterns `\.tf$`, `\.tofu$`.
- `crates/renovate-cli/src/main.rs` — wired terraform pipeline section +
  `build_dep_reports_terraform` helper.

### What was intentionally deferred
- `.terraform.lock.hcl` lockfile parsing.
- Provider `required_version` constraint (Terraform CLI version).
- `terraform_workspace` resource type.
- Docker image references inside Terraform resources.
- Helm chart references in `helm_release` resources.
- HCL string interpolation and heredocs.
- OpenTofu registry differences.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 357 passed

## Next slice candidates

Pick whichever can be completed in one loop:

## Slice 0039 - Helm `Chart.yaml` extractor + Helm repository index.yaml datasource

### Renovate reference
- `lib/modules/manager/helmv3/extract.ts` — `extractPackageFile`
- `lib/modules/manager/helmv3/index.ts` — patterns `Chart.ya?ml`, `requirements.ya?ml`
- `lib/modules/datasource/helm/index.ts` — `HelmDatasource`, index.yaml fetching

### What landed
- `crates/renovate-core/src/extractors/helm.rs` — line-scanner Chart.yaml extractor:
  - Handles `dependencies:` YAML list with `name`, `version`, `repository` fields.
  - `stable` alias resolved to `https://charts.helm.sh/stable`.
  - Skip reasons: `OciRegistry` (`oci://`), `UnresolvableAlias` (`@alias`), `NoRepository`.
  - Collapsible-if Clippy fix applied (Rust 2024 `&&` let-chain form).
- `crates/renovate-core/src/datasources/helm.rs` — Helm index.yaml datasource:
  - `GET {repoUrl}/index.yaml` → line-scanner to find chart's first (newest) version.
  - State machine: `Entries` → `Chart` → `Version` (no external YAML library needed).
  - Concurrent lookups via `JoinSet` + `Arc<Semaphore>`.
- `crates/renovate-core/src/managers.rs` — added `helmv3` with patterns `Chart.ya?ml`,
  `requirements.ya?ml`.
- `crates/renovate-cli/src/main.rs` — wired helm pipeline section + helper.

### What was intentionally deferred
- `Chart.lock` lockfile parsing.
- `values.yaml` image tag extraction (separate `helm-values` manager).
- OCI registry chart lookups.
- Custom `@alias` resolution from user config.
- `requirements.yaml` (Helm v2) distinct handling.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 374 passed

## Next slice candidates

Pick whichever can be completed in one loop:

## Slice 0040 - Gradle `.gradle`/`.gradle.kts` extractor + TOML version catalog

### Renovate reference
- `lib/modules/manager/gradle/index.ts` — `defaultConfig`, file patterns
- `lib/modules/manager/gradle/utils.ts`  — `parseDependencyString`
- `lib/modules/manager/gradle/extract/catalog.ts` — TOML catalog parsing
- Datasource: `MavenDatasource` (Maven Central, already implemented)

### What landed
- `crates/renovate-core/src/extractors/gradle.rs` — dual-format Gradle extractor:
  - `extract_build_file()`: regex scanner for Groovy/Kotlin DSL string-notation deps.
    Matches 20+ configuration keywords (implementation, api, classpath, kapt, ksp, …).
    Deduplicates by `group:artifact` (same dep under different configs → one entry).
    Skip reasons: `VariableReference` (`$var`), `DynamicVersion` (`1.+`, SNAPSHOT).
  - `extract_version_catalog()`: TOML parser for `libs.versions.toml` / `.versions.toml`.
    Supports inline string form (`guava = "group:artifact:version"`) and table form
    with inline or `version.ref` lookups into `[versions]`.
  - Both functions produce `GradleExtractedDep` with Maven coordinate `dep_name`.
- Manager pattern `gradle` with `.gradle`, `.gradle.kts`, `.versions.toml` patterns.
- Pipeline routes TOML files to `extract_version_catalog`, others to `extract_build_file`.
- Reuses `datasources::maven` for Maven Central version lookups — no new datasource.

### What was intentionally deferred
- Map notation: `implementation group: 'com.example', name: 'mylib', version: '1.0'`.
- `gradle.properties` version variable resolution.
- Multi-project builds and cross-file variable sharing.
- Gradle plugin declarations (`plugins { id("...") version "..." }`).
- `gradle-consistent-versions` plugin support.
- `gradle/libs.versions.toml` `[bundles]` and `[plugins]` sections.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 388 passed

## Next slice candidates

Pick whichever can be completed in one loop:

## Slice 0041 - Maven versioning module + Maven datasource integration

### Renovate reference
- `lib/modules/versioning/maven/compare.ts` — tokenizer and qualifier ordering
- `lib/modules/versioning/maven/index.ts`   — `compare`, `isStable`

### What landed
- `crates/renovate-core/src/versioning/maven.rs` — full Maven version comparison:
  - `tokenize(v)`: splits on `.`, `-`, and digit↔letter transitions; strips leading `v`.
  - `is_null(token)`: number 0, and qualifiers `""`, `final`, `ga`, `release`, `latest`, `sr`.
  - `qualifier_order()`: alpha(1) < beta(2) < milestone(3) < rc/cr/preview(4) < snapshot(5)
    < release/ga/final/""(6) < sp(7). Unknown qualifiers compare lexicographically between
    snapshot and sp.
  - `compare(l, r) -> Ordering`: token-by-token comparison with null-fill.
  - `is_stable(v) -> bool`: true when no pre-release qualifier present.
  - `maven_update_summary(current, latest)`: produces `MavenUpdateSummary` using proper
    Maven ordering — SNAPSHOT and pre-releases won't falsely trigger updates.
- `crates/renovate-core/src/datasources/maven.rs` — wired to `maven_update_summary`.
  Previously used naive string comparison; now correctly handles pre-release ordering.

### Key correctness improvements
- `5.0.0-RC1` vs `5.0.0`: RC < release, so `5.0.0` is an update (was already correct
  by string diff, now correct by semantics).
- `5.3.28-SNAPSHOT` vs `5.3.28`: SNAPSHOT < release — `5.3.28-SNAPSHOT` being the
  "latest" from the registry would NOT trigger a false update to itself.
- `1.0.RELEASE` == `1.0` == `1.0.GA`: release-equivalent tokens treated as equal.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 399 passed

## Next slice candidates

Pick whichever can be completed in one loop:

## Slice 0042 - Elixir Mix `mix.exs` extractor + Hex.pm datasource

### Renovate reference
- `lib/modules/manager/mix/extract.ts` — `extractPackageFile`, regex patterns
- `lib/modules/manager/mix/index.ts`   — pattern `/(^|/)mix\\.exs$/`
- `lib/modules/datasource/hex/index.ts` — `HexDatasource`
- API: `GET https://hex.pm/api/packages/{name}` → `{"latest_stable_version": "x.y.z"}`

### What landed
- `crates/renovate-core/src/extractors/mix.rs` — `mix.exs` extractor:
  - Locates the `deps do … end` block using a depth-aware character scanner.
  - Matches `{:name, "constraint"}` tuples via regex; optional `only:`, `runtime:`, etc.
  - Skip reasons: `GitSource` (`git:`, `github:`), `LocalPath` (`path:`), `NoVersion`.
- `crates/renovate-core/src/datasources/hex.rs` — Hex.pm REST client:
  - `GET /api/packages/{name}` → `latest_stable_version` (avoids pre-release).
  - `lower_bound()` strips `~>`, `>=`, etc. for update comparison.
  - Concurrent bounded lookups via `JoinSet` + `Arc<Semaphore>`.
- `crates/renovate-core/src/managers.rs` — added `mix` with pattern `(^|/)mix\.exs$`.
- Mix pipeline inlined in `main.rs` (no separate build-report helper needed for this
  iteration).

### What was intentionally deferred
- `mix.lock` lockfile parsing.
- GitHub/git source deps (would use `github_tags` datasource).
- Hex organization packages (`:my_package` atom form in `:hex` option).
- Umbrella project sub-app deps resolution.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 414 passed

## Next slice candidates

Pick whichever can be completed in one loop:

## Slice 0043 - HashiCorp versioning module + Terraform datasource integration

### Renovate reference
- `lib/modules/versioning/hashicorp/convertor.ts` — `hashicorp2npm` conversion rules
- `lib/modules/versioning/hashicorp/index.ts`    — `isValid`, `matches`, `getSatisfyingVersion`

### What landed
- `crates/renovate-core/src/versioning/hashicorp.rs` — HashiCorp constraint parser:
  - Parses comma-separated constraints: `~> 5.0`, `>= 2.0.0`, `= 3.1.4`, `!= ...`, etc.
  - `lower_bound(constraint)` → `Option<String>`: extracts the pinned lower bound.
  - `parse_version(v)` pads 1- or 2-component versions to 3 components for semver compare.
  - `hashicorp_update_summary(current, latest)`: semver-orders `latest > lower_bound`.
  - Handles `~> 5` (major-only: lower bound `5.0.0`), `~> 5.0` (`5.0.0`), `~> 5.0.1` (`5.0.1`).
- `crates/renovate-core/src/datasources/terraform.rs` — wired to `hashicorp_update_summary`.
  Removed the old `lower_bound_version` string-comparison helper; tests updated.

### Key correctness improvements
- `~> 5.0` with latest `5.7.3`: semver comparison `5.7.3 > 5.0.0` → update_available.
- `~> 5.7.3` with latest `5.7.3`: same lower bound → no update.
- `>= 4.0.0, < 5.0.0` with latest `4.5.0`: lower bound `4.0.0`, `4.5.0 > 4.0.0` → update.
- Old string comparison `l != lower` was correct most of the time but semantically wrong for
  multi-component constraints where the lower bound string didn't match the latest string.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 429 passed

## Next slice candidates

Pick whichever can be completed in one loop:

## Slice 0044 - Swift Package Manager `Package.swift` extractor

### Renovate reference
- `lib/modules/manager/swift/extract.ts` — `extractPackageFile`
- `lib/modules/manager/swift/index.ts`   — pattern `/(^|/)Package\\.swift/`
- Datasource: `GithubTagsDatasource` (already implemented, reused)

### What landed
- `crates/renovate-core/src/extractors/spm.rs` — `Package.swift` extractor:
  - Regex matches `.package(url:, from:)`, `.package(url:, exact:)`,
    `.upToNextMajor(from:)`, `.upToNextMinor(from:)`, and range forms.
  - `parse_git_url()` extracts `owner/repo` from GitHub/GitLab URLs.
  - Skip reasons: `LocalPath` (`path:` form), `NonGitHost` (Bitbucket, SSH, etc.).
  - GitLab packages recognized but not currently looked up (no gitlab_tags datasource yet).
- `crates/renovate-core/src/datasources/github_tags.rs` — exported `GITHUB_API` constant.
- `crates/renovate-core/src/managers.rs` — added `swift` with pattern `(^|/)Package\.swift$`.
- Swift pipeline in `main.rs` reuses `github_tags_datasource::fetch_updates_concurrent`.

### What was intentionally deferred
- GitLab package version lookup (no `gitlab_tags` datasource yet).
- `Package.resolved` lockfile parsing.
- SSH git URL parsing.
- `.package(url:, branch:)` and `.package(url:, revision:)` forms.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 441 passed

## Next slice candidates

Pick whichever can be completed in one loop:

## Slice 0045 - GitLab tags datasource + SPM GitLab package lookups

### Renovate reference
- `lib/modules/datasource/gitlab-tags/index.ts` — `GitlabTagsDatasource`
- API: `GET {host}/api/v4/projects/{url_encoded_path}/repository/tags?per_page=100`

### What landed
- `crates/renovate-core/src/datasources/gitlab_tags.rs` — GitLab REST tags client:
  - URL-encodes `owner/repo` path (`/` → `%2F`) for the GitLab API.
  - Filters for version-like tags (starts with `v` + digit, or bare digit).
  - Strips leading `v` from returned tag for comparison with `current_value`.
  - Concurrent bounded lookups via `JoinSet` + `Arc<Semaphore>`.
- `crates/renovate-core/src/datasources.rs` — added `pub mod gitlab_tags`.
- `crates/renovate-core/src/datasources/github_tags.rs` — exported `GITHUB_API` constant.
- `crates/renovate-cli/src/main.rs` — SPM pipeline updated to do concurrent GitHub
  and GitLab lookups, merging results into a unified `spm_map` by `owner_repo`.

### What was intentionally deferred
- Self-hosted GitLab instance support (uses `GITLAB_API = https://gitlab.com`).
- GitLab tags for GitHub Actions (separate pipeline from SPM).
- Tag filtering by semver validity (currently passes any tag starting with v+digit).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 445 passed

## Next slice candidates

Pick whichever can be completed in one loop:

## Slice 0046 - CocoaPods `Podfile` extractor + CocoaPods trunk datasource

### Renovate reference
- `lib/modules/manager/cocoapods/extract.ts` — `parseLine`, `extractPackageFile`
- `lib/modules/manager/cocoapods/index.ts`   — pattern `/(^|/)Podfile$/`
- `lib/modules/datasource/pod/index.ts`      — `PodDatasource`
- API: `GET https://trunk.cocoapods.org/api/v1/pods/{name}`

### What landed
- `crates/renovate-core/src/extractors/cocoapods.rs` — Podfile line-scanner:
  - Matches `pod 'Name'` and `pod 'Name', 'version'` in both quote styles.
  - Inline comment stripping (`# comment`).
  - Skip reasons: `LocalPath` (`:path =>`), `GitSource` (`:git =>`), `PodspecSource`.
  - Subspec support: `Firebase/Analytics` name preserved in dep.
- `crates/renovate-core/src/datasources/cocoapods.rs` — CocoaPods trunk REST client:
  - `GET /api/v1/pods/{name}` → `{"versions":[{"name":"5.6.4",...}]}`
  - Filters pre-releases (versions containing `-`).
  - Subspec names use base pod name: `Firebase/Analytics` → lookup `Firebase`.
  - `lower_bound()` strips `~>`, `>=`, etc. for update comparison.
- Manager pattern `cocoapods` with `(^|/)Podfile$`.

### What was intentionally deferred
- `:git => 'url', :tag => 'X'` deps via GitHub/GitLab tags datasource.
- Custom CDN sources (non-trunk registries).
- `Podfile.lock` lockfile parsing.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 460 passed

## Slice 0047 - Generic semver versioning module

### Renovate reference
- `lib/modules/versioning/semver/index.ts` — `SemVer`
- Applies to: pub.dev, Packagist/Composer, RubyGems, Hex.pm, CocoaPods

### What landed
- `crates/renovate-core/src/versioning/semver_generic.rs` — shared semver helper:
  - `semver_update_summary(current_value, latest)`: strips operators, pads to 3 semver
    components, uses `semver::Version` comparison to avoid false-positive updates.
  - `lower_bound()`: strips `^`, `~>`, `>=`, `>`, `<=`, `<`, `=`, `!` from constraints.
  - `parse_padded()`: pads `"6.4"` → `"6.4.0"` before `semver::Version::parse`.
  - Fix: `lower_bound("^6.4") = "6.4"`, `latest = "6.4.0"` — string compare was a
    false positive; semver compare correctly reports "no update needed".
- Registered in `versioning.rs` as `pub mod semver_generic`.
- Wired into 5 datasources replacing ad-hoc `lower_bound` + string-compare:
  - `datasources/pub_dev.rs`
  - `datasources/packagist.rs`
  - `datasources/rubygems.rs` (removed `lower_bound_version` helper)
  - `datasources/hex.rs` (removed `lower_bound` helper)
  - `datasources/cocoapods.rs` (removed `lower_bound` helper)

### What was intentionally deferred
- NuGet: uses pinned versions (no constraint ranges) — string equality suffices.
- Full semver range semantics (`^1.2.3` allows `1.x.x` but not `2.x.x`) — Renovate
  tracks this separately; for update-check purposes lower-bound comparison is correct.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 469 passed

## Slice 0048 - `setup.cfg` extractor (Setuptools declarative config)

### Renovate reference
- `lib/modules/manager/setup-cfg/extract.ts` — `extractPackageFile`
- `lib/modules/manager/setup-cfg/index.ts` — pattern `/(^|/)setup\\.cfg$/`
- Datasource: PyPI (reuses existing `datasources/pypi.rs`)
- Versioning: pep440 (reuses existing `versioning/pep440.rs`)

### What landed
- `crates/renovate-core/src/extractors/setup_cfg.rs` — INI-format scanner:
  - Tracks current `[section]` and `record =` key to classify dep type:
    - `[options]` + `install_requires` → `install`
    - `[options]` + `setup_requires` → `setup`
    - `[options]` + `tests_require` → `test`
    - `[options.extras_require]` + any key → `extra`
  - Handles multi-line continuation (indented lines after `key =`).
  - Strips inline comments (`# …`) and environment markers (`; python_version …`).
  - Skip reasons: `NoVersion` (unconstrained dep), `GitSource` (`git+…`).
  - Normalizes package names to lowercase with `-` (PEP 503).
- Manager pattern `setup-cfg` with `(^|/)setup\\.cfg$` added to `managers.rs`.
- Pipeline wired in `main.rs`: extracts deps → PyPI lookups → `setup-cfg` FileReport.

### What was intentionally deferred
- `setup.py` parsing (imperative Python — no reliable static parser).
- `install_requires` declared as a list in `setup.py` calls.
- `-r file.txt` sub-requirement references within setup.cfg.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 478 passed

## Slice 0049 - pre-commit `.pre-commit-config.yaml` extractor

### Renovate reference
- `lib/modules/manager/pre-commit/extract.ts` — `extractPackageFile`
- `lib/modules/manager/pre-commit/index.ts` — pattern `/(^|/)\\.pre-commit-config\\.ya?ml$/`
- Datasources: GitHub Tags, GitLab Tags (reuses existing datasources)

### What landed
- `crates/renovate-core/src/extractors/pre_commit.rs` — YAML line scanner:
  - Tracks `repos:` list with proper indent-level detection to distinguish
    entry-level `- repo:` items from nested `- id:` hook items.
  - `local` and `meta` repos emitted without a rev (so they appear as skipped).
  - Skip reasons: `LocalHook` (`local`), `MetaHook` (`meta`),
    `InvalidUrl`, `UnknownRegistry`.
  - Git host detection: `github.com` → `GitHost::GitHub`,
    `*.gitlab.*` → `GitHost::GitLab`.
  - Strips `.git` suffix from dep names; strips surrounding quotes from rev values.
- Manager pattern `pre-commit` with `(^|/)\.pre-commit-config\.ya?ml$`.
- Pipeline wired in `main.rs`:
  - GitHub hooks → `github_tags` datasource (reuses `gh_http` + `gh_api_base`).
  - GitLab hooks → `gitlab_tags` datasource.
  - Both paths use `HashMap<String, (update_available, latest, error_msg)>` pattern
    (same as SPM mixed-host pipeline).

### What was intentionally deferred
- `additional_dependencies` for `language: node`, `language: python`,
  `language: golang` hooks — requires npm/PyPI/Go module datasource wiring per-hook.
- `rev` frozen-comment parsing (`# frozen: v1.2.3` alongside a digest `rev:`).
- Custom/self-hosted Git registries with host-rule lookup.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 487 passed

## Slice 0050 - NuGet versioning module

### Renovate reference
- `lib/modules/versioning/nuget/version.ts` — `compare`, `parseVersion`
- `lib/modules/versioning/nuget/index.ts` — `isStable`
- NuGet versioning spec: `Major.Minor.Patch[.Revision][-PreRelease]`

### What landed
- `crates/renovate-core/src/versioning/nuget.rs` — 4-part version comparison:
  - `parse()`: splits on `-` for pre-release, splits numeric part on `.`, pads to
    4 components (Revision defaults to 0).
  - `compare(a, b) -> Ordering`: numeric component comparison, then stable > pre-release.
  - `is_stable(v) -> bool`: true when no pre-release label.
  - `nuget_update_summary(current, latest)`: returns update summary using proper
    4-part comparison; fixes false-positive where `"13.0.3"` != `"13.0.3.0"`.
- Registered in `versioning.rs` as `pub mod nuget`.
- Wired into `datasources/nuget.rs` replacing the old `l != dep.current_value`
  string compare in `fetch_update_summary`.

### What was intentionally deferred
- NuGet range constraints (`[1.0,)`, `[1.0,2.0)`, `(,2.0)`). The extractor
  currently passes pinned versions only; range constraint parsing would require
  a NuGet range parser to extract the lower bound.
- Floating/wildcard versions (`1.*`, `1.2.*`).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 503 passed

## Slice 0051 - GitHub Releases datasource + asdf `.tool-versions` extractor

### Renovate reference
- `lib/modules/datasource/github-releases/index.ts` — `GithubReleasesDatasource`
- `lib/modules/manager/asdf/extract.ts` — `extractPackageFile`
- `lib/modules/manager/asdf/upgradeable-tooling.ts` — tool-to-datasource map
- API: `GET https://api.github.com/repos/{owner}/{repo}/releases?per_page=100`
- Pattern: `(^|/)\.tool-versions$`

### What landed
- `crates/renovate-core/src/datasources/github_releases.rs` — GitHub Releases client:
  - Filters `prerelease: true` and `draft: true` releases.
  - Releases are newest-first; returns first stable `tag_name`.
  - Uses `semver_generic::semver_update_summary` for version comparison (handles
    `v` prefix stripping).
- `crates/renovate-core/src/extractors/asdf.rs` — `.tool-versions` line scanner:
  - Regex `^([\w_-]+)\s+(\S+)` parses `tool version` pairs; strips inline comments.
  - Static `TOOL_TABLE` maps 20 common tools to (GitHub repo, `tag_strip`):
    - **GithubTags**: awscli, erlang, flux2, golang, kubectl, perl, php, python, rust
    - **GithubReleases**: argocd, consul, helm, k9s, kind, minikube, packer, terraform,
      terragrunt, vault, waypoint
  - Unknown tools emit `skip_reason: UnsupportedTool`.
- Manager pattern `asdf` with `(^|/)\.tool-versions$`.
- Pipeline in `main.rs`:
  - Partitions actionable deps by datasource type.
  - Unique-repo dedup: each `repo|tag_strip` key is looked up once, not once per dep.
  - `tag_strip` prefix stripped from tag before semver comparison with stored version.
  - Uses existing `gh_http` (authenticated) and `gh_api_base` from GitHub Actions setup.

### What was intentionally deferred
- nodejs (NodeVersionDatasource), ruby (RubyVersionDatasource), java
  (JavaVersionDatasource) — require specialized version datasources.
- Tools using non-standard version formats that require additional conversion
  (e.g. erlang `OTP-26.0` tag → asdf stores `26.0` — currently handled by tag_strip).
- `.tool-versions` files with multiple versions per line (only first captured).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 513 passed

## Slice 0052 - Ruby `.gemspec` extractor

### Renovate reference
- `lib/modules/manager/bundler/extract.ts` — handles gemspec deps inline
- Datasource: RubyGems (reuses existing `datasources/rubygems.rs`)
- Versioning: semver_generic (reuses existing `versioning/semver_generic.rs`)
- Pattern: `(^|/)[^/]*\.gemspec$`

### What landed
- `crates/renovate-core/src/extractors/gemspec.rs` — line scanner:
  - Regex: `(?i)^\s*(?:\w+\.)?add(?:_(runtime|development))?_dependency\s+['"]name['"](rest)`
  - Captures all three method forms: `add_dependency`, `add_runtime_dependency`,
    `add_development_dependency` with any receiver prefix (`spec.`, `s.`, `gem.`).
  - Multi-constraint versions joined: `">= 6.0", "< 8.0"` → `">= 6.0, < 8.0"`.
  - Skip reasons: `NoVersion` (unconstrained), `GitSource` (`git:`/`github:` option),
    `PathSource` (`path:` option).
  - `is_dev: bool` field set for development dependencies.
- Manager pattern `gemspec` with `(^|/)[^/]*\.gemspec$`.
- Pipeline wired in `main.rs` routing to RubyGems datasource + semver_generic.

### What was intentionally deferred
- `gemspec` directive in `Gemfile` (Bundler reads the .gemspec file and includes
  its deps — would require cross-file resolution).
- Ruby version requirements (`spec.required_ruby_version`).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 521 passed

## Slice 0053 - Pipenv `Pipfile` extractor

### Renovate reference
- `lib/modules/manager/pipenv/extract.ts` — `extractPackageFile`
- `lib/modules/manager/pipenv/index.ts` — pattern `/(^|/)Pipfile$/`
- Datasource: PyPI (reuses existing `datasources/pypi.rs`)
- Versioning: pep440 (reuses existing `versioning/pep440.rs`)

### What landed
- `crates/renovate-core/src/extractors/pipfile.rs` — TOML-based extractor:
  - Uses `toml::from_str::<toml::Table>()` (toml v1.x API — `Value::from_str` only
    parses a single TOML value, not a full document).
  - Parses `[packages]` (runtime) and `[dev-packages]` (dev) sections.
  - Handles two entry forms: string (`requests = ">=2.25"`) and table
    (`django = {version = ">=4.0", extras = [...]}`).
  - Skip reasons: `Wildcard` (`"*"` or `{version = "*"}`), `GitDependency` (`git`
    key), `LocalDependency` (`path`/`file` key).
  - Normalizes names (lowercase, `-` for `_`/`.`).
- Manager pattern `pipenv` with `(^|/)Pipfile$`.
- Pipeline wired in `main.rs` via PyPI datasource + `build_dep_reports_pipfile`.

### What was intentionally deferred
- `Pipfile.lock` lockfile parsing.
- Private PyPI index sources from `[[source]]` sections.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 532 passed

## Slice 0054 - Version-file managers (.terraform-version, .go-version, etc.)

### Renovate reference
- `lib/modules/manager/terraform-version/` — `.terraform-version`
- `lib/modules/manager/terragrunt-version/` — `.terragrunt-version`
- Pattern per file: `(^|/)\.terraform-version$`, etc.
- Datasources: GitHub Releases (terraform, terragrunt, nodejs) + GitHub Tags (golang, python)

### What landed
- `crates/renovate-core/src/extractors/version_file.rs` — single-version-file extractor:
  - `VERSION_FILE_DEFS` static table: manager name → (tool, `AsdfDatasource`)
  - `extract(content, manager_name)` returns one `VersionFileDep`: reads the first
    non-empty, non-comment line; strips leading `v`; skips NVM aliases (`lts/*`,
    `latest`, `stable`, `node`).
  - Reuses `AsdfDatasource` enum (GithubTags/GithubReleases) from `extractors/asdf.rs`.
  - 6 file types: `.terraform-version`, `.terragrunt-version`, `.go-version`,
    `.python-version`, `.node-version`, `.nvmrc`.
- 6 manager patterns added to `managers.rs`.
- Single pipeline loop in `main.rs` iterates all 6 manager names, fetches the
  version file, calls `version_file::extract()`, routes to github_tags or
  github_releases, strips tag prefix, compares with `semver_generic`.

### What was intentionally deferred
- `.ruby-version` — requires a specialized Ruby version datasource (ruby-lang.org).
- `.bun-version` — routes to npm datasource (need npm version lookup for bun).
- NVM partial-version aliases (e.g. `20` meaning latest 20.x).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 547 passed

## Slice 0055 - GitLab CI `.gitlab-ci.yml` Docker image extractor

### Renovate reference
- `lib/modules/manager/gitlabci/extract.ts` — `extractPackageFile`
- `lib/modules/manager/gitlabci/index.ts` — pattern `/\.gitlab-ci\.ya?ml$/`
- Datasource: Docker Hub (reuses existing `datasources/docker_hub.rs`)

### What landed
- `crates/renovate-core/src/extractors/gitlabci.rs` — YAML line scanner:
  - Three image forms: inline (`image: node:18`), block (`image:\n  name: ref`),
    services list (`services:\n  - postgres:15`).
  - Reuses `classify_image_ref()` from `extractors/dockerfile.rs` for Docker
    image parsing (handles registry prefixes, `scratch`, variable references, etc.).
  - Key bug fixed during dev: `image:\s+(\S+.*)` requires a space after colon
    so `image:` alone (block form) is detected by a separate `IMAGE_KEY_ONLY` regex.
  - Skips `$VAR`-form variable images.
- Manager pattern `gitlabci` with `(^|/)\.gitlab-ci\.ya?ml$`.
- Pipeline mirrors the Dockerfile pipeline: Docker Hub dep inputs, `update_map`,
  non-Docker-Hub registries get `Skipped { reason: "non-docker-hub registry" }`.

### What was intentionally deferred
- GitLab CI components (`include: component`).
- `extends:` inheritance (job templates sharing an image).
- GitLab-hosted container registry images (non-Docker-Hub).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 553 passed

## Slice 0056 - CircleCI `.circleci/config.yml` Docker image extractor

### Renovate reference
- `lib/modules/manager/circleci/extract.ts` — `extractPackageFile`
- Pattern: `(^|/)\.circleci/.+\.ya?ml$`
- Datasource: Docker Hub (reuses existing `datasources/docker_hub.rs`)

### What landed
- `crates/renovate-core/src/extractors/circleci.rs` — line scanner:
  - Detects `docker:` key, then collects `- image: ref` list items.
  - Reuses `classify_image_ref()` and `DockerfileExtractedDep` from Dockerfile extractor.
  - Skips `$VAR` variable images; other skip reasons (scratch, arg variable) inherit
    from the Dockerfile extractor's classify function.
  - Deferred: `orbs:` section (requires CircleCI Orb API datasource), `machine:`
    executor (CircleCI VM images, not Docker Hub).
- Manager pattern `circleci` with `(^|/)\.circleci/.+\.ya?ml$`.
- Pipeline mirrors the GitLab CI pipeline (Docker Hub lookups, same update reporting).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 558 passed

## Slice 0057 - Buildkite pipeline plugin extractor

### Renovate reference
- `lib/modules/manager/buildkite/extract.ts`
- Patterns: `buildkite\.ya?ml`, `(^|/)\.buildkite/.+\.ya?ml$`
- Datasource: GitHub Tags (reuses existing `datasources/github_tags.rs`)

### What landed
- `crates/renovate-core/src/extractors/buildkite.rs` — line scanner:
  - Regex: `^\s*(?:-\s+(?:\?\s+)?)?['"]?(?P<dep>[^#\s'"]+)#(?P<ver>[^:'"]+)['"]?`
  - Handles 3 plugin forms:
    - 1-part shorthand (`docker-compose#v5.1.0`) → `buildkite-plugins/docker-compose-buildkite-plugin`
    - 2-part shorthand (`buildkite/matrix-joiner#v1.0.0`) → `buildkite/matrix-joiner-buildkite-plugin`
    - Full GitHub URL (`https://github.com/org/plugin.git#v2.3.0`) → `org/plugin`
  - Non-semver versions (like branch names) get `InvalidVersion` skip reason.
  - Bitbucket registry URLs deferred (no BitbucketTagsDatasource yet).
- Manager patterns `buildkite` with two file patterns.
- Pipeline uses `github_tags::fetch_updates_concurrent` via `gh_http`/`gh_api_base`.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 563 passed

## Slice 0058 - Cargo `[workspace.dependencies]` support

### Renovate reference
- Cargo workspace root `Cargo.toml` with `[workspace.dependencies]` section
- Same `crates_io` datasource as regular `Cargo.toml` deps

### What landed
- Extended `extractors/cargo.rs`:
  - Added `RawWorkspace { dependencies: Option<BTreeMap<String, RawDep>> }` struct.
  - Added `workspace: Option<RawWorkspace>` field to `RawManifest`.
  - `extract()` now also processes `workspace.dependencies` entries with `DepType::Regular`.
  - Uses `manifest.workspace.and_then(|ws| ws.dependencies)` (Clippy let-chain form).
  - 2 new tests: `workspace_dependencies_extracted` and `workspace_and_member_deps_both_extracted`.

### What this fixes
- Workspace root `Cargo.toml` files that define shared deps in `[workspace.dependencies]`
  were previously returning 0 deps — the member crates correctly skipped inherited deps
  (`WorkspaceInherited`), but the canonical versions in the workspace root were never extracted.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 565 passed

## Slice 0059 - Cargo `[target.'cfg(...)'.dependencies]` support

### What landed
- Extended `extractors/cargo.rs`:
  - Added `RawTargetDeps { dependencies, dev_dependencies, build_dependencies }` struct.
  - Added `target: Option<BTreeMap<String, RawTargetDeps>>` field to `RawManifest`.
  - `extract()` now iterates all target platform blocks and collects their deps
    using the same `convert_dep()` path as regular deps.
  - 1 new test: `target_cfg_dependencies_extracted`.
- Closes gap: `[target.'cfg(windows)'.dependencies]`, `[target.'cfg(unix)'.dev-dependencies]`,
  etc. were previously silently ignored.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 566 passed

## Slice 0060 - npm `resolutions` (yarn) and `overrides` (npm 8+) support

### What landed
- Extended `extractors/npm.rs`:
  - Added `Resolutions` and `Overrides` variants to `NpmDepType`.
  - Added `resolutions` and `overrides` fields to `PackageJson` struct.
  - Both sections are flat `{ "pkg": "version" }` maps, same format as `dependencies`.
  - Included in the same dep-classification loop — no special handling needed.
  - 2 new tests: `extracts_yarn_resolutions`, `extracts_npm_overrides`.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 568 passed

## Slice 0061 - pep621 `[build-system].requires` support

### Renovate reference
- `lib/modules/manager/pep621/extract.ts` — line 76: `const buildSystemRequires = def['build-system']?.requires`
- Dep type: `build-system.requires`

### What landed
- Extended `extractors/pep621.rs`:
  - Added `BuildSystem` variant to `Pep621DepType`.
  - Added `[build-system].requires` extraction after existing sections.
  - `parse_pep508_entry` reused — same PEP 508 format as regular deps.
  - 2 new tests: `no_project_section_returns_build_system_only`,
    `build_system_requires_with_project_deps`.
- Updated module doc table to include the new section.

### What this fixes
- Build tool dependencies like `setuptools>=61.0`, `poetry-core>=1.0.0`,
  `hatchling`, `wheel` were silently ignored even though they're pinned and
  can fall behind.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 569 passed

## Slice 0062 - Ansible Galaxy `requirements.yml` GitHub-URL roles extractor

### Renovate reference
- `lib/modules/manager/ansible-galaxy/roles.ts` — `extractRoles`
- `lib/modules/manager/ansible-galaxy/extract.ts`
- Pattern: `(^|/)(galaxy|requirements)(\.ansible)?\.ya?ml$`
- Datasource: GitHub Tags for GitHub-URL roles (Galaxy API deferred)

### What landed
- `crates/renovate-core/src/extractors/ansible_galaxy.rs` — YAML line scanner:
  - Scans `roles:` and `collections:` sections.
  - Extracts `name:`, `src:`, `version:` fields from each list item.
  - `classify_source()`: GitHub URL (`https://github.com/` or `git@github.com:`)
    → `AnsibleGalaxySource::GitHub { owner_repo }`, else `Galaxy`.
  - Skip reasons: `NoVersion` (no `version:` field), `GalaxyHosted`
    (requires GalaxyDatasource not yet implemented).
  - `.git` suffix stripped from repo URLs.
- Manager pattern `ansible-galaxy`.
- Pipeline routes GitHub-sourced roles to `github_tags` datasource.

### What was intentionally deferred
- Galaxy-hosted roles (`geerlingguy.apache`) → requires `GalaxyDatasource`.
- Galaxy collections (`community.general`) → same.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`: 575 passed

## Slice 0063 - GitHub Actions container/services Docker image extraction

### Renovate reference
- `lib/modules/manager/github-actions/schema.ts` — `WorkFlowJobs.container` (string | `{ image }`)
  and `WorkFlowJobs.services` (mapping of string | `{ image }`)
- `lib/modules/manager/github-actions/extract.ts` — `extractWithYAMLParser`

### What landed
- `crates/renovate-core/src/extractors/github_actions.rs`:
  - Added `extract_docker_images(content) -> Vec<DockerfileExtractedDep>` — line-scan state machine.
  - `GaDockerState` enum: `Default | InContainerBlock { indent } | InServices { svc_indent, service_level }`.
  - Handles four forms:
    1. Inline container: `container: node:18`
    2. Block container: `container:\n  image: node:18`
    3. Block service: `services:\n  redis:\n    image: redis:5`
    4. Inline service string: `services:\n  postgres: postgres:10`
  - `$VAR` references skipped automatically.
  - Reuses `classify_image_ref()` from the dockerfile extractor.
  - `transition_default()` helper avoids duplication on block-exit reprocessing.
  - 8 new unit tests (includes upstream `workflow_1.yml` fixture scenario).
- `crates/renovate-cli/src/main.rs` — GitHub Actions pipeline extended:
  - Calls `extract_docker_images` alongside `extract` for each workflow file.
  - Routes container/services images through the Docker Hub datasource pipeline.
  - Combines action dep reports and Docker dep reports into a single `FileReport`.

### What was intentionally deferred
- Non-Docker-Hub private registry images (already handled by the existing
  `NonDockerHub` skip reason in the Docker Hub datasource).
- `runs-on` labels (GitHub-hosted runner versions — different datasource).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 661 passed

## Slice 0064 - GitHub Actions `runs-on` runner version extraction

### Renovate reference
- `lib/modules/datasource/github-runners/index.ts` — static `releases` table
- `lib/modules/manager/github-actions/extract.ts` — `extractRunner()`, `runnerVersionRegex`
- `GithubRunnersDatasource.isValidRunner(name, version)`

### What landed
- `crates/renovate-core/src/datasources/github_runners.rs` — fully offline, static datasource:
  - `RunnerVersion { version, stable, deprecated }` — per-version record.
  - `RUNNERS: &[(&str, &[RunnerVersion])]` — table for `ubuntu`, `macos`, `windows`
    (ported from Renovate's `GithubRunnersDatasource.releases`).
  - `is_valid_runner(name, version) -> bool` — checks if a runner+version exists.
  - `variant_suffix(version) -> &str` — strips leading `X.Y` numeric prefix to get `-arm`, `-xlarge`, etc.
  - `latest_stable(name, current_version) -> Option<&str>` — finds newest stable, non-deprecated
    version with the same variant suffix.
  - `update_summary(name, version) -> RunnerUpdateSummary` — combines update + deprecated flags.
  - 12 unit tests.
- `crates/renovate-core/src/extractors/github_actions.rs`:
  - `GhRunnerDep { runner_name, current_value }` — extracted runner dep.
  - `extract_runner_labels(content) -> Vec<GhRunnerDep>` — line-scanner for `runs-on:`.
    - Handles inline single value (`runs-on: ubuntu-22.04`) and inline array form
      (`runs-on: [ubuntu-22.04, self-hosted]`).
    - Skips `latest`, `${{...}}` variables, self-hosted, unknown runners.
  - `parse_runner_label(s) -> Option<(&str, &str)>` — splits `ubuntu-22.04` into name + version.
  - 8 unit tests.
- `crates/renovate-cli/src/main.rs` — GitHub Actions pipeline extended:
  - Calls `extract_runner_labels()` for each workflow file.
  - Computes update summary via `update_summary()` (no network needed).
  - Reports `UpdateAvailable`, `UpToDate`, or `Skipped { "deprecated runner" }`.

### What was intentionally deferred
- Block-form `runs-on:` arrays (multi-line list items after `runs-on:`).
- Matrix expression expansion (`${{ matrix.os }}`).
- `self-hosted` runner labels with custom labels.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 685 passed

## Slice 0065 - `packageRules` parsing + `enabled: false` filtering

### Renovate reference
- `lib/config/options/index.ts` — `packageRules` option schema
- `matchPackageNames`, `matchPackagePatterns`, `matchManagers`, `enabled`

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `PackageRule { match_package_names, match_package_patterns, match_managers, enabled, has_name_constraint }` — compiled rule struct.
  - `has_name_constraint` field: `true` when the raw config specified any name or pattern constraint (even if invalid patterns failed to compile). Prevents a fully-invalid `matchPackagePatterns` from accidentally matching all packages.
  - `name_matches(dep_name) -> bool` — OR-s `matchPackageNames` (exact) and compiled `matchPackagePatterns` (regex).
  - `manager_matches(manager) -> bool` — empty `matchManagers` matches all managers.
  - `RepoConfig.package_rules: Vec<PackageRule>` — parsed from `packageRules` in `renovate.json`.
  - `is_dep_ignored(name)` — extended to also check `packageRules` with `enabled: false`.
  - `is_dep_ignored_for_manager(name, manager)` — manager-aware variant (respects `matchManagers`).
  - Added `regex` crate import to `repo_config.rs` for pattern compilation.
  - 8 new unit tests.

### What was intentionally deferred
- `matchUpdateTypes` (major/minor/patch filtering)
- `allowedVersions` constraint filtering
- `matchDepTypes` filtering
- `extends` / preset expansion

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 693 passed

## Slice 0066 - `UpdateType` classification + update type labels in CLI output

### Renovate reference
- `lib/config/types.ts` — `UpdateType` enum (`major`, `minor`, `patch`, ...)

### What landed
- `crates/renovate-core/src/versioning/semver_generic.rs`:
  - `UpdateType { Major, Minor, Patch }` — enum for bump classification.
  - `classify_semver_update(current: &str, latest: &str) -> Option<UpdateType>` — compares
    semver versions (with `lower_bound()` and `parse_padded()`) to determine bump magnitude.
    Returns `None` for non-semver strings, same versions, or when parsing fails.
  - 7 new unit tests covering major/minor/patch/same-version/v-prefix/range/non-semver cases.
- `crates/renovate-cli/src/output.rs`:
  - `format_dep()` now calls `classify_semver_update(current, latest)` for `UpdateAvailable` deps.
  - Appends colored bump label: red `major`, yellow `minor`, green `patch`.
  - No change to `DepStatus` struct — classification is computed at display time.

### What was intentionally deferred
- `matchUpdateTypes` in `packageRules` filtering (infrastructure is now in place).
- Non-semver update type classification (Docker tags, runner versions, etc.).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 700 passed

## Slice 0067 - `packageRules` matchUpdateTypes filtering

### Renovate reference
- `lib/config/options/index.ts` — `matchUpdateTypes` option
- Allowed values: `major`, `minor`, `patch`, `pin`, `pinDigest`, `digest`,
  `lockFileMaintenance`, `rollback`, `bump`, `replacement`

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `PackageRule.match_update_types: Vec<UpdateType>` — parsed from `matchUpdateTypes`.
    Known types (`major`, `minor`, `patch`) are compiled to `UpdateType`; unknown strings
    (e.g., `pin`, `digest`) are silently skipped (empty list → matches all update types).
  - `PackageRule::update_type_matches(update_type) -> bool` — checks if the given type
    is in the rule's `match_update_types` list (empty = all).
  - `RepoConfig::is_update_blocked(name, update_type, manager) -> bool` — returns `true`
    when any matching rule with `enabled: false` covers this update type.
  - Added `use crate::versioning::semver_generic::UpdateType;` import.
  - 5 new unit tests.

### What was intentionally deferred
- Wiring `is_update_blocked()` into all 32+ dep-report building sites in `main.rs`.
  The API is ready; the wiring can be done incrementally or in a bulk slice.
- `pin`, `pinDigest`, `digest`, etc. update type classifications (non-semver bump types).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 705 passed

## Slice 0068 - Wire matchUpdateTypes blocking into all manager dep report pipelines

### What landed
- `crates/renovate-cli/src/main.rs`:
  - `apply_update_blocking_to_report(report, repo_cfg)` — post-processes all `FileReport`s
    after every manager's scan is complete. For each `UpdateAvailable` dep, classifies
    the semver bump type (via `classify_semver_update`) and converts to `Skipped` if any
    `packageRules` entry with `enabled: false` and `matchUpdateTypes` blocks it.
  - Called once before the `(Some(repo_report), had_error)` return, covering all ~30
    manager pipelines in a single pass.
  - Skip reason includes the blocked update type for debuggability:
    `"blocked by packageRules (matchUpdateTypes: major)"`.

### What was intentionally deferred
- Non-semver version strings (Docker tags, runner labels) — `classify_semver_update`
  returns `None` and the dep is unaffected.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 705 passed

## Slice 0069 - `packageRules` allowedVersions semver range filtering

### Renovate reference
- `lib/config/options/index.ts` — `allowedVersions` option
- "A version range or regex pattern capturing allowed versions for dependencies."

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `PackageRule.allowed_versions: Option<String>` — raw range string from config.
  - `RepoConfig::is_version_restricted(name, manager, proposed_version) -> bool`:
    - Parses `proposed_version` via `parse_padded()`.
    - For each matching rule with `allowedVersions` set, parses the range as a
      `semver::VersionReq` and checks if the proposed version satisfies it.
    - Returns `true` (restricted) when the proposed version is outside the allowed range.
    - Skips: regex patterns (leading `/`), unparseable constraints, non-semver versions.
  - 5 new unit tests.
- `crates/renovate-cli/src/main.rs`:
  - `apply_update_blocking_to_report()` now also checks `is_version_restricted()` before
    `is_update_blocked()`. Restricted deps are marked `Skipped { reason: "blocked by packageRules (allowedVersions)" }`.

### What was intentionally deferred
- Regex `allowedVersions` patterns (`/^1\./`) — would require regex matching against
  version strings, different from semver range matching.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 710 passed

## Slice 0070 - JSON output mode (`--output-format=json`)

### What landed
- `crates/renovate-cli/src/cli.rs`:
  - `OutputFormat { Human, Json }` — `ValueEnum` for `--output-format`.
  - `--output-format` flag with `RENOVATE_OUTPUT_FORMAT` env var support.
- `crates/renovate-cli/src/output.rs`:
  - `serde::{Serialize, Deserialize}` derived on `DepStatus`, `DepReport`, `FileReport`, `RepoReport`.
  - `DepStatus` uses `#[serde(tag = "status", rename_all = "camelCase")]` for JSON tag discriminant.
  - `DepReport` uses `#[serde(flatten)]` so status fields appear inline.
  - `print_json_reports(reports: &[RepoReport])` — serializes to pretty JSON.
- `crates/renovate-cli/src/main.rs`:
  - When `--output-format=json`, collects all `RepoReport`s into `all_reports` and emits
    them as a JSON array at the end; suppresses the human summary.
  - `serde` and `serde_json` added to the CLI crate's `Cargo.toml`.

### JSON format
```json
[
  {
    "repoSlug": "owner/repo",
    "files": [
      {
        "path": "package.json",
        "manager": "npm",
        "deps": [
          {"name": "lodash", "status": "updateAvailable", "current": "4.17.20", "latest": "4.17.21"},
          {"name": "react", "status": "upToDate", "latest": "18.3.1"}
        ]
      }
    ]
  }
]
```

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 710 passed

## Slice 0071 - `packageRules` matchCurrentVersion filtering

### Renovate reference
- `lib/config/options/index.ts` — `matchCurrentVersion` option
- "A version range to match the current dep version against."

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `PackageRule.match_current_version: Option<String>` — raw range string.
  - `PackageRule::current_version_matches(current_value) -> bool`:
    - Strips leading operators from `current_value` (via `lower_bound()`), pads to 3 components.
    - Parses `matchCurrentVersion` as `semver::VersionReq` and checks if current satisfies it.
    - Passes through (returns `true`) for regex patterns, unset constraints, unparseable values.
  - `is_update_blocked()` signature extended with `current_value: &str` parameter.
    Now checks all four conditions: name, manager, update type, current version.
  - Updated all test call sites with the new `current_value` argument.
  - 4 new unit tests: blocks below range, passes current with caret constraint,
    absent matchCurrentVersion matches all, current above range not blocked.

### What was intentionally deferred
- Regex `matchCurrentVersion` patterns (`/^1\./`) — silently treated as matching.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 714 passed

## Slice 0072 - `packageRules` matchFileNames glob filtering

### Renovate reference
- `lib/config/options/index.ts` — `matchFileNames` option

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `PackageRule.match_file_names: Vec<String>` — raw file name patterns.
  - `PackageRule::file_name_matches(path) -> bool` — delegates to `PathMatcher::new(&self.match_file_names).is_ignored(path)`. Reuses the existing glob/prefix matching infrastructure from `ignorePaths`.
  - `RepoConfig::is_update_blocked_for_file(name, current, type, manager, file_path)` — extends `is_update_blocked` with file-path-aware matching.
  - `RepoConfig::is_version_restricted_for_file(...)` — extends `is_version_restricted` with file-path-aware matching.
  - `is_update_blocked()` and `is_version_restricted()` now delegate to the `_for_file` variants with an empty path (matches all files).
  - 4 new unit tests.
- `crates/renovate-cli/src/main.rs`:
  - `apply_update_blocking_to_report()` now uses the `_for_file` variants, passing `file.path` to respect `matchFileNames` constraints.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 718 passed

## Slice 0073 - Add stats (update counts) to JSON output

### What landed
- `crates/renovate-cli/src/output.rs`:
  - `DepStats { total, updateAvailable, upToDate, skipped, errors }` — serializable struct.
  - `DepStats::from_deps(deps) -> DepStats` — computes counts from a dep slice.
  - `JsonFileReport<'a>` and `JsonRepoReport<'a>` — wrapper structs used only for JSON serialization that include `stats` fields computed from the deps.
  - `print_json_reports()` now emits the wrapper structs with computed per-file and per-repo stats.
  - 2 new unit tests.

### JSON output example
```json
[{
  "repoSlug": "owner/repo",
  "stats": {"total": 42, "updateAvailable": 5, "upToDate": 30, "skipped": 6, "errors": 1},
  "files": [{
    "path": "package.json", "manager": "npm",
    "stats": {"total": 10, "updateAvailable": 2, "upToDate": 8, "skipped": 0, "errors": 0},
    "deps": [...]
  }]
}]
```

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 720 passed

## Slice 0074 - Extend asdf tool table (bun, deno, zig, elixir, scala) + bun-version file

### What landed
- `crates/renovate-core/src/extractors/asdf.rs`:
  - Added 6 new tools to `TOOL_TABLE`:
    - `bun` → GitHub Releases `oven-sh/bun`, tag_strip `bun-v`
    - `deno` → GitHub Releases `denoland/deno`, tag_strip `v`
    - `zig` → GitHub Tags `ziglang/zig`, tag_strip `` (bare tags)
    - `elixir` → GitHub Tags `elixir-lang/elixir`, tag_strip `v`
    - `java` → GitHub Releases `adoptium/temurin17-binaries`, tag_strip `jdk-`
    - `scala` → GitHub Tags `scala/scala`, tag_strip `v`
- `crates/renovate-core/src/managers.rs`:
  - Added `bun-version` manager with pattern `(^|/)\.bun-version$`.
- `crates/renovate-core/src/extractors/version_file.rs`:
  - Added `bun-version` entry to `VERSION_FILE_DEFS` using GitHub Releases `oven-sh/bun` with `bun-v` tag strip.
- `crates/renovate-cli/src/main.rs`:
  - Added `bun-version` to the version file processing manager loop.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 720 passed

## Slice 0075 - Gradle `plugins {}` block extraction

### Renovate reference
- `lib/modules/manager/gradle/parser/plugins.ts` — plugin block parser

### What landed
- `crates/renovate-core/src/extractors/gradle.rs`:
  - `PLUGIN_DEP` regex: `\bid\s*[\(]?\s*['"]([^'"]+)['"]\s*[\)]?\s+version\s+['"]([^'"]+)['"]`
    matches both `id 'plugin.id' version 'X.Y'` and `id("plugin.id") version "X.Y"` forms.
  - `parse_plugin_dep(plugin_id, version) -> Option<GradleExtractedDep>` — converts plugin ID to
    Maven marker coordinate `{id}:{id}.gradle.plugin` (the standard artifact name for Gradle plugins).
  - `extract_build_file()` now calls both `STRING_DEP` and `PLUGIN_DEP` scanners.
  - `GradleDepKind` enum added (Dependency / Plugin) for future dep-type filtering.
  - 4 new tests: single-quote, double-quote-parens, mixed plugins + deps, variable version skip.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run --workspace --all-features`: 724 passed

## Slice 0079 - Azure Pipelines extractor (Docker containers + tasks)

### Renovate reference
- `lib/modules/manager/azure-pipelines/extract.ts`
- `lib/modules/manager/azure-pipelines/schema.ts`
- Patterns: `/(^|/).azuredevops/.+\.ya?ml$/`, `/azure.*pipelines?.*\.ya?ml$/`

### What landed
- `crates/renovate-core/src/extractors/azure_pipelines.rs`:
  - `AzPipelineTaskDep { name, version }` — pipeline task dep from `task: Name@Version`.
  - `AzPipelinesDep { Container(DockerfileExtractedDep), Task(AzPipelineTaskDep) }` enum.
  - `extract(content)` — line-scanner with state tracking for `resources.containers` block.
  - Container images: state machine tracks `in_resources → in_containers → in_container_item`,
    extracts `image:` values and runs through `classify_image_ref()`.
  - Pipeline tasks: universal scan of all lines for `[- ]task: Name@Version` (inline and key forms);
    tasks appear inside `steps:` at any nesting level (top-level, jobs, stages, deployments).
  - 8 unit tests: single container, multiple containers, tasks, nested stage/job tasks,
    variable ref skip, task without `@` ignored, empty file, non-container resources.
- `crates/renovate-core/src/managers.rs`: `azure-pipelines` manager with 2 patterns.
- `crates/renovate-core/src/extractors.rs`: `pub mod azure_pipelines`.
- `crates/renovate-cli/src/main.rs`: Azure Pipelines pipeline loop — Docker images go through
  Docker Hub datasource; tasks emitted as skipped with `"azure-pipelines-tasks datasource pending"`.

### What was intentionally deferred
- `azure-pipelines-tasks` datasource (requires Azure DevOps API or GitHub data mirror).
- `resources.repositories` extraction (git tags datasource).
- Template file references.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 745 passed

## Slice 0080 - Helmfile `helmfile.yaml` extractor

### Renovate reference
- `lib/modules/manager/helmfile/extract.ts`
- `lib/modules/manager/helmfile/schema.ts`
- Patterns: `/(^|/)helmfile\.ya?ml(?:\.gotmpl)?$/`, `/(^|/)helmfile\.d/.+\.ya?ml(?:\.gotmpl)?$/`

### What landed
- `crates/renovate-core/src/extractors/helmfile.rs`:
  - Two-pass line scanner: Pass 1 collects `repositories:` name→URL map; Pass 2 collects `releases:`.
  - Handles both 0-indent and 2-indent YAML list item styles.
  - `resolve_release()` handles: local path (excluded), Go templates (skip UnresolvableAlias),
    OCI direct (`oci://`), OCI-backed repo alias, `alias/chart-name` form, plain name lookup.
  - Reuses `HelmExtractedDep` + `HelmSkipReason` from `extractors/helm.rs`.
  - `stable` alias built-in (resolves to `STABLE_REPO` without repo entry).
  - 10 unit tests.
- `crates/renovate-core/src/managers.rs`: `helmfile` manager with 2 patterns.
- `crates/renovate-core/src/extractors.rs`: `pub mod helmfile`.
- `crates/renovate-cli/src/main.rs`: Helmfile pipeline reuses `helm_datasource::fetch_updates_concurrent`
  and `build_dep_reports_helm` helper — no duplication.

### What was intentionally deferred
- Multi-document YAML (multiple `---` separated documents in one helmfile).
- `helmfile.lock` lockfile parsing.
- `values:` inline values injection.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 755 passed

## Slice 0081 - Drone CI `.drone.yml` Docker image extractor

### Renovate reference
- `lib/modules/manager/droneci/extract.ts`
- Pattern: `/(^|/)\.drone\.yml$/`

### What landed
- `crates/renovate-core/src/extractors/droneci.rs`:
  - Simplest CI extractor: scans every `image:` key in the file at any nesting depth.
  - Strips `- ` list-item prefix before matching (handles both `- image:` and `image:` forms).
  - Passes each value through `classify_image_ref()` — `$VAR` refs become `ArgVariable` skip.
  - 6 unit tests: single step image, service image, multiple images, variable ref skip,
    private registry, empty file.
- `crates/renovate-core/src/managers.rs`: `droneci` manager with pattern `(^|/)\.drone\.yml$`.
- `crates/renovate-core/src/extractors.rs`: `pub mod droneci`.
- `crates/renovate-cli/src/main.rs`: Drone CI pipeline using Docker Hub datasource.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 761 passed

## Slice 0082 - Bitbucket Pipelines `*-pipelines.yml` Docker image extractor

### Renovate reference
- `lib/modules/manager/bitbucket-pipelines/extract.ts`
- `lib/modules/manager/bitbucket-pipelines/util.ts`
- Pattern: `**/*-pipelines.yml`

### What landed
- `crates/renovate-core/src/extractors/bitbucket_pipelines.rs`:
  - `extract()` — index-based line scanner (needed for look-ahead on image objects).
  - **Simple `image:` line**: scans `image: ref` and `- image: ref` forms.
  - **Image object**: when `image:` has no inline value, looks ahead for `name:` key
    in the next non-empty line.
  - **Docker pipe**: `- pipe: docker://image:tag` → extracts Docker image.
  - Non-docker pipes (`atlassian/pipe-name:version`) → skipped (BitbucketTags datasource pending).
  - 8 unit tests.
- `crates/renovate-core/src/managers.rs`: `bitbucket-pipelines` manager with pattern.
- `crates/renovate-core/src/extractors.rs`: `pub mod bitbucket_pipelines`.
- `crates/renovate-cli/src/main.rs`: Bitbucket Pipelines pipeline using Docker Hub datasource.

### What was intentionally deferred
- `pipe:` non-docker references (BitbucketTags datasource).
- `image.username`/`image.password` authentication fields.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 769 passed

## Slice 0083 - Jenkins `plugins.txt` / `plugins.yml` extractor

### Renovate reference
- `lib/modules/manager/jenkins/extract.ts`
- Pattern: `/(^|/)plugins\.(txt|ya?ml)$/`

### What landed
- `crates/renovate-core/src/extractors/jenkins.rs`:
  - `JenkinsPluginDep { artifact_id, version, skip_reason }` struct.
  - `JenkinsSkipReason { UnspecifiedVersion, UnsupportedVersion }` enum.
  - `extract_txt(content)` — line scanner for `plugin-id:version` format;
    strips `#`-prefixed comments; skips `latest`/`experimental` with `UnsupportedVersion`.
  - `extract_yml(content)` — line scanner for YAML `plugins:` list with `artifactId:` + `version:`
    (also handles `source.version:` nested form via `version:` key).
  - 9 unit tests (5 txt, 4 yml).
- `crates/renovate-core/src/managers.rs`: `jenkins` manager with pattern `(^|/)plugins\.(txt|ya?ml)$`.
- `crates/renovate-core/src/extractors.rs`: `pub mod jenkins`.
- `crates/renovate-cli/src/main.rs`: Jenkins pipeline — all deps emitted as skipped
  (jenkins-plugins datasource pending), actionable deps also skipped with reason.

### What was intentionally deferred
- `jenkins-plugins` datasource (Jenkins Update Center JSON API).
- `renovate.ignore: true` annotation in YAML format.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 778 passed

## Slice 0084 - Refactor: `docker_hub_reports` helper eliminates Docker pipeline duplication

### What landed
- `crates/renovate-cli/src/main.rs`:
  - Added `docker_hub_reports(http, deps) -> Vec<DepReport>` async helper that encapsulates the
    full Docker Hub pipeline: filter actionable, build `DockerDepInput` list, `fetch_updates_concurrent`,
    build update_map, iterate all deps mapping skip/update/up-to-date/error to `DepReport`.
  - Replaced 6 identical inline Docker pipeline blocks (GitLab CI, CircleCI, Cloud Build, Drone CI,
    Bitbucket Pipelines, Azure Pipelines containers) with `docker_hub_reports` calls.
  - For GitLab CI and CircleCI (which wrap `DockerfileExtractedDep` in a type): map `.dep.clone()` before
    calling the helper.
  - For Azure Pipelines: separate container images from task deps, use helper for containers,
    append task deps with "datasource pending" status.
  - **Net: −437 lines / +100 lines = 337 fewer lines, 10→5 Docker pipeline call sites.**

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 778 passed

## Slice 0085 - Gradle Wrapper extractor + Gradle Version datasource

### Renovate reference
- `lib/modules/manager/gradle-wrapper/extract.ts`
- `lib/modules/manager/gradle-wrapper/utils.ts` — `extractGradleVersion`
- `lib/modules/datasource/gradle-version/index.ts`
- Pattern: `/(^|/)gradle/wrapper/gradle-wrapper\.properties$/`
- API: `https://services.gradle.org/versions/all`

### What landed
- `crates/renovate-core/src/extractors/gradle_wrapper.rs`:
  - `GradleWrapperDep { version }` struct.
  - `extract(content)` — scans for `distributionUrl=` key, calls `parse_distribution_url()`.
  - `parse_distribution_url()` — unescapes `\:` → `:`, extracts filename from URL path,
    strips `gradle-` prefix and `-bin`/`-all` suffix via `rfind('-')`.
  - 5 unit tests.
- `crates/renovate-core/src/datasources/gradle_version.rs`:
  - `GradleVersionSummary { update_available, current_version, latest }` struct.
  - `fetch_latest(http, current_version)` — GETs `services.gradle.org/versions/all` JSON,
    filters stable releases (no snapshot/nightly/broken), sorts by numeric version descending,
    compares with current.
  - `cmp_gradle_version()` — splits on `.`, parses segments as `u32`, lexicographic compare;
    handles `8.10 > 8.4` correctly (vs. string comparison).
  - 1 unit test for sorting.
- `crates/renovate-core/src/managers.rs`: `gradle-wrapper` manager pattern.
- `crates/renovate-cli/src/main.rs`: Gradle Wrapper pipeline (single dep `"gradle"`, version lookup).

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 784 passed

## Slice 0086 - Maven Wrapper `.mvn/wrapper/maven-wrapper.properties` extractor

### Renovate reference
- `lib/modules/manager/maven-wrapper/extract.ts`
- Pattern: `/(^|/)\.mvn/wrapper/maven-wrapper\.properties$/`
- Datasource: Maven Central (reuses existing `datasources::maven::fetch_latest`)

### What landed
- `crates/renovate-core/src/extractors/maven_wrapper.rs`:
  - `MavenWrapperDep { dep_name, package_name, version }` struct.
  - `extract(content)` — scans for `distributionUrl=`, `wrapperUrl=`, `wrapperVersion=` keys.
  - `extract_version_from_url()` — finds the version path segment (between artifact name and filename)
    using `is_version_like()` (starts with digit, contains `.`).
  - `is_version_like()` — simple heuristic for version segments.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `maven-wrapper` manager pattern.
- `crates/renovate-core/src/extractors.rs`: `pub mod maven_wrapper`.
- `crates/renovate-cli/src/main.rs`: Maven Wrapper pipeline — for each dep, calls
  `maven_datasource::fetch_latest(&dep.package_name, http)` (no new datasource needed —
  looks up `org.apache.maven:apache-maven` and `org.apache.maven.wrapper:maven-wrapper`).

### What was intentionally deferred
- `mvnw`/`mvnw.cmd` script parsing (shell/batch scripts with version in comment).
- `.mvn/wrapper/MavenWrapperDownloader.java` parsing.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 789 passed

## Slice 0087 - Woodpecker CI `.woodpecker.yml` Docker image extractor

### Renovate reference
- `lib/modules/manager/woodpecker/extract.ts`
- Pattern: `/^\.woodpecker(?:/[^/]+)?\.ya?ml$/`

### What landed
- `crates/renovate-core/src/extractors/woodpecker.rs`:
  - Universal `image:` key scanner (same approach as Drone CI).
  - Works at any nesting depth — covers steps, services, pipeline, clone blocks.
  - Handles `- image:` list-item inline and `image:` key forms.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `woodpecker` manager pattern.
- `crates/renovate-core/src/extractors.rs`: `pub mod woodpecker`.
- `crates/renovate-cli/src/main.rs`: Woodpecker pipeline using `docker_hub_reports` helper.

### Verification
- `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace`: 794 passed

## Slice 0099 - GitLab CI `include:` project reference extractor

### Renovate reference
- `lib/modules/manager/gitlabci-include/extract.ts`
- Pattern: `/(^|/)\.gitlab-ci\.ya?ml$/` (shared with `gitlabci`)
- Datasource: GitLab Tags (`datasources::gitlab_tags`)

### What landed
- `crates/renovate-core/src/extractors/gitlabci_include.rs`:
  - `GitlabIncludeDep { project, ref_value }` struct.
  - `extract(content)` — line-scanner that detects the `include:` block, iterates list items,
    collects `project:` + `ref:` pairs; flushes each item when a new `- ` list bullet is seen.
  - Inline comment stripping (`# ...`), `include:` block exit on next top-level key.
  - 5 unit tests: single ref, multiple refs, ref missing → skip, non-include blocks ignored, empty.
- `crates/renovate-core/src/managers.rs`: `gitlabci-include` manager entry (same pattern as `gitlabci`).
- `crates/renovate-core/src/extractors.rs`: `pub mod gitlabci_include`.
- `crates/renovate-cli/src/main.rs`: `gitlabci-include` pipeline — builds `GitlabTagsDepInput`
  (with `dep_name` = project path), calls `fetch_updates_concurrent`, emits `DepReport` per dep.

### What was intentionally deferred
- `include: component:` style references (GitLab CI components, different datasource).
- `include: remote:` and `include: template:` forms.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 853 passed

## Slice 0100 - CircleCI orbs extractor + Orb GraphQL datasource

### Renovate reference
- `lib/modules/manager/circleci/extract.ts` — `extractDefinition` (orb handling)
- `lib/modules/datasource/orb/index.ts` — `OrbDatasource`
- API: `POST https://circleci.com/graphql-unstable` (GraphQL)
- Versioning: npm semver (same as orb versions are semver)

### What landed
- `crates/renovate-core/src/extractors/circleci.rs`:
  - Added `CircleCiOrbDep { alias, package_name, version }` struct.
  - Added `extract_orbs(content)` function — line-scanner for `orbs:` top-level block,
    parses `alias: owner/name@version` entries; skips inline orb map values.
  - 5 new unit tests: extract 2 orbs, skip missing `@`, block ends at next top-level key, empty, no orbs block.
- `crates/renovate-core/src/datasources/orb.rs` (new):
  - `OrbDepInput { package_name, current_value }`, `OrbUpdateSummary`, `OrbUpdateResult` structs.
  - `fetch_latest(http, package_name, current_value)` — POSTs GraphQL query to
    `https://circleci.com/graphql-unstable`, extracts `data.orb.versions[0].version`.
  - `fetch_updates_concurrent(http, deps, concurrency)` — semaphore-bounded concurrent fetcher.
- `crates/renovate-core/src/http.rs`: Added `post_json<T>(url, body)` method to `HttpClient`.
- `crates/renovate-cli/src/main.rs`: Extended circleci pipeline — runs `extract_orbs` in parallel
  with Docker image extraction; combines both into one `FileReport` per file.

### What was intentionally deferred
- Inline orbs (YAML map values with `commands:`, `jobs:` etc. — not a version reference).
- `machine:` executor VM image versions (CircleCI-specific, not Docker Hub).

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 858 passed

## Slice 0101 - Jenkins plugins datasource (Update Center JSON)

### Renovate reference
- `lib/modules/datasource/jenkins-plugins/index.ts` — `JenkinsPluginsDatasource`
- API: `GET https://updates.jenkins.io/current/update-center.actual.json`

### What landed
- `crates/renovate-core/src/datasources/jenkins_plugins.rs` (new):
  - `JenkinsPluginUpdateSummary { current_value, latest, update_available }` struct.
  - `fetch_latest(http, plugin_name, current_value)` — fetches and parses the Update Center
    JSON; uses `OnceLock` to cache the full response for the process lifetime (~1.5 MB).
  - Parses `{"plugins": {"name": {"version": "x.y.z"}}}` shape.
  - 1 unit test for JSON deserialization.
- `crates/renovate-cli/src/main.rs`: Replaced the Jenkins pipeline "datasource pending" stub
  with real `fetch_latest` calls; skips deps with `skip_reason` or no version.

### What was intentionally deferred
- `plugin-versions.json` (all historical versions) — not needed for "is update available?"
- Custom Update Center URLs (non-`updates.jenkins.io` registries).

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 859 passed

## Slice 0102 - Leiningen `project.clj` extractor (Clojars + Maven Central)

### Renovate reference
- `lib/modules/manager/leiningen/extract.ts` — `extractPackageFile`, `expandDepName`
- `lib/modules/datasource/clojure/index.ts` — `ClojureDatasource` (Maven + Clojars)
- Pattern: `/(^|/)project\\.clj$/`

### What landed
- `crates/renovate-core/src/extractors/leiningen.rs` (new):
  - `LeinDepType { Dependencies, ManagedDependencies, Plugins, PomPlugins, Coords }` enum.
  - `LeinDep { dep_name, current_value, dep_type }` struct.
  - `strip_comments(content)` — strips `;` Clojure line comments without touching string literals.
  - `balanced_brackets(s)` — returns slice from `[` to matching `]`, respecting string contents.
  - `expand_dep_name(symbol)` — `org.clojure/clojure` → `org.clojure:clojure`; `ring` → `ring:ring`.
  - `extract(content)` — scans for `:dependencies`, `:managed-dependencies`, `:plugins`,
    `:pom-plugins`, `:coords` keywords; extracts `[symbol "version"]` pairs from their vectors.
  - 8 unit tests covering all dep types, bare names, comment stripping, empty file.
- `crates/renovate-core/src/datasources/maven.rs`:
  - Added `CLOJARS_BASE` constant.
  - Added `fetch_latest_from_registry(dep_name, http, registry)` — same as `fetch_latest` but
    accepts a registry base URL.
- `crates/renovate-core/src/managers.rs`: `leiningen` manager with pattern `(^|/)project\.clj$`.
- `crates/renovate-cli/src/main.rs`: Leiningen pipeline — tries Clojars first per dep, falls
  back to Maven Central if not found on Clojars.

### What was intentionally deferred
- `~varName` version interpolation (runtime variable substitution).
- Custom `:repositories` entries.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 867 passed

## Slice 0103 - Ansible tasks Docker image extractor

### Renovate reference
- `lib/modules/manager/ansible/extract.ts`
- Pattern: `/(^|/)tasks/[^/]+\\.ya?ml$/`
- Datasource: Docker Hub (same as Dockerfile)

### What landed
- `crates/renovate-core/src/extractors/ansible.rs` (new):
  - `extract(content)` — scans for `image:` key lines (with optional quotes), strips
    inline comments, skips variable refs (`${...}` and `$`-prefixed values).
  - Delegates to `classify_image_ref` for Docker image parsing (same as Dockerfile extractor).
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `ansible` manager with pattern `(^|/)tasks/[^/]+\.ya?ml$`.
- `crates/renovate-core/src/extractors.rs`: `pub mod ansible`.
- `crates/renovate-cli/src/main.rs`: Ansible pipeline using `docker_hub_reports` helper.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 872 passed

## Slice 0104 - SBT `build.sbt` / `project/build.properties` extractor

### Renovate reference
- `lib/modules/manager/sbt/extract.ts`
- Patterns: `/\.sbt$/`, `/project/[^/]*\.scala$/`, `/project/build\.properties$/`
- Datasource: Maven Central

### What landed
- `crates/renovate-core/src/extractors/sbt.rs` (new):
  - `SbtDepStyle { Java, Scala }` — `%` vs `%%` operator distinction.
  - `SbtDepType { Library, Plugin, SbtVersion }` — dep classification.
  - `SbtDep { group_id, artifact_id, current_value, style, dep_type }` with `dep_name()` helper.
  - `extract(content)` — scans `.sbt`/`.scala` files line by line; strips `//` comments; detects
    `addSbtPlugin` lines; uses regex `"group" %%? "artifact" % "version"` to extract deps.
  - `extract_build_properties(content)` — extracts `sbt.version=x.y.z` from `build.properties`.
  - 7 unit tests.
- `crates/renovate-core/src/managers.rs`: `sbt` manager with `.sbt`, `project/*.scala`,
  and `project/build.properties` patterns.
- `crates/renovate-cli/src/main.rs`: SBT pipeline using Maven Central lookups.

### What was intentionally deferred
- `scalaVersion` variable substitution (e.g. `scalaVersion := "2.13.12"` affecting `%%` deps).
- Sbt Plugin Registry (`https://repo.scala-sbt.org/scalasbt/sbt-plugin-releases`).
- Multi-project builds with sub-project references.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 879 passed

## Slice 0105 - FluxCD `gotk-components.yaml` system manifest extractor

### Renovate reference
- `lib/modules/manager/flux/extract.ts` — `extractSystemManifest`
- `lib/modules/manager/flux/common.ts` — `systemManifestHeaderRegex`
- Pattern: `/(^|/)gotk-components\.ya?ml$/`
- Datasource: GitHub Releases (`fluxcd/flux2`)

### What landed
- `crates/renovate-core/src/extractors/flux.rs` (new):
  - `FluxSystemDep { version, components }` struct.
  - `FLUX2_REPO = "fluxcd/flux2"` constant.
  - `extract(content)` — applies `# Flux Version: <ver>` regex to whole file content
    (not line-by-line) so the optional `# Components:` on the next line is captured.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `flux` manager with pattern `(^|/)gotk-components\.ya?ml$`.
- `crates/renovate-cli/src/main.rs`: Flux pipeline — calls `fetch_latest_release("fluxcd/flux2")`
  and emits one `DepReport`.

### What was intentionally deferred
- `HelmRelease`, `GitRepository`, `OCIRepository` CRD resources (require YAML schema parsing).
- Kustomize image refs inside Flux resource manifests.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 884 passed

## Slice 0106 - Nix flakes `flake.lock` input extractor

### Renovate reference
- `lib/modules/manager/nix/extract.ts`
- `lib/modules/manager/nix/schema.ts`
- Pattern: `/(^|/)flake\.nix$/`
- Datasource: GitRefsDatasource (GitHub Tags for github-type inputs)

### What landed
- `crates/renovate-core/src/extractors/nix.rs` (new):
  - `FlakeInputType` enum (github, gitlab, git, tarball, sourcehut, indirect, path, etc.).
  - `FlakeLocked`, `FlakeOriginal`, `FlakeNode`, `FlakeLock` deserialization structs.
  - `NixSkipReason` enum (Indirect, LocalPath, NoRev, Transitive, UnsupportedType).
  - `NixFlakeDep { input_name, locked_rev, current_ref, package_name, input_type, skip_reason }`.
  - `extract(flake_lock_content)` — parses `flake.lock` JSON (version 7), collects only
    root-referenced inputs, skips indirect/path/transitive, builds package URLs.
  - `build_package_name()` — constructs `https://github.com/owner/repo` etc. per type.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `nix` manager with pattern `(^|/)flake\.nix$`.
- `crates/renovate-cli/src/main.rs`: Nix pipeline — when `flake.nix` is detected, reads
  sibling `flake.lock`; GitHub-type inputs use GitHub Tags datasource; others are skipped.

### What was intentionally deferred
- GitLab, git, tarball, sourcehut types (need git-refs-style datasource).
- Nixpkgs channel versioning (nixpkgsVersioning).
- `flake.nix` content update (updating `rev:` inline) — requires file mutation.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 889 passed

## Slice 0107 - Azure Pipelines Tasks datasource (GitHub mirror JSON)

### Renovate reference
- `lib/modules/datasource/azure-pipelines-tasks/index.ts`
- JSON mirrors: `https://raw.githubusercontent.com/renovatebot/azure-devops-marketplace/main/*.json`
- Format: `Record<string, string[]>` — task name (lowercase) → list of versions

### What landed
- `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` (new):
  - `AzureTaskUpdateSummary { current_value, latest, update_available }` struct.
  - `fetch_latest(http, task_name, current_value)` — fetches built-in tasks JSON, falls
    back to marketplace tasks JSON; uses `OnceLock` for process-wide caching.
  - Case-insensitive task name lookup (normalized to lowercase).
  - `cmp_version()` — numeric component-wise comparison for version selection.
  - 1 unit test for version comparison.
- `crates/renovate-cli/src/main.rs`: Replaced "azure-pipelines-tasks datasource pending"
  stub with real `fetch_latest` calls; `NotFound` → `Skipped` status.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 890 passed

## Slice 0108 - Clojure `deps.edn` / `bb.edn` extractor

### Renovate reference
- `lib/modules/manager/deps-edn/extract.ts`
- Pattern: `/(^|/)(?:deps|bb)\.edn$/`
- Datasource: Clojure (Maven Central + Clojars)

### What landed
- `crates/renovate-core/src/extractors/deps_edn.rs` (new):
  - `DepsEdnDep { dep_name, current_value }` struct.
  - `expand_name()` — `org.clojure/clojure` → `org.clojure:clojure`; bare `ring` → `ring:ring`.
  - `extract(content)` — line-scanner that strips `;` comments, tracks `dep-name {` on each line,
    matches `:mvn/version "version"` and pairs with last-seen dep symbol; skips `:git/`/`:local/` deps.
  - `find_last_dep_sym()` — char-by-char scan finding rightmost `symbol {` on a line.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `deps-edn` manager with `(^|/)(?:deps|bb)\.edn$` pattern.
- `crates/renovate-cli/src/main.rs`: deps-edn pipeline using Clojars-then-Maven-Central fallback.

### What was intentionally deferred
- `:git/url` + `:git/sha` deps (GitRefsDatasource).
- `com.github.owner/repo` → GitHub Tags mapping.
- Variable substitution (`${version}`).

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 895 passed

## Slice 0109 - `.ruby-version` version file (GitHub Tags, underscore normalization)

### Renovate reference
- `lib/modules/manager/ruby-version/index.ts`
- `lib/modules/datasource/ruby-version/index.ts`
- Pattern: `/(^|/)\.ruby-version$/`
- Datasource: GitHub Tags (`ruby/ruby`) — tags use `v3_3_0` format

### What landed
- `crates/renovate-core/src/extractors/version_file.rs`:
  - Added `("ruby-version", "ruby", GithubTags { repo: "ruby/ruby", tag_strip: "v" })` entry.
  - Added `.ruby-version` → `"ruby-version"` to `manager_for_file()`.
  - 2 new unit tests.
- `crates/renovate-core/src/managers.rs`: `ruby-version` manager with `(^|/)\.ruby-version$` pattern.
- `crates/renovate-cli/src/main.rs`: Added `"ruby-version"` to the version-file manager list;
  adds underscore→dot normalization (`v3_3_0` → `3.3.0`) for ruby tags.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 897 passed

## Slice 0110 - Conan `conanfile.txt`/`.py` extractor + Conan Center datasource

### Renovate reference
- `lib/modules/manager/conan/extract.ts`
- `lib/modules/datasource/conan/index.ts` — `getConanCenterReleases`
- Patterns: `/(^|/)conanfile\.(txt|py)$/`
- API: GitHub API `conan-io/conan-center-index/contents/recipes/{name}/config.yml`

### What landed
- `crates/renovate-core/src/extractors/conan.rs` (new):
  - `ConanDepType { Requires, BuildRequires, PythonRequires }` enum.
  - `ConanSkipReason { CustomChannel, RangeVersion }` enum.
  - `ConanDep { name, current_value, dep_type, skip_reason }` struct.
  - `extract_txt()` — section-aware `[requires]`/`[build_requires]` scanner.
  - `extract_py()` — line scanner for `requires`/`build_requires` assignments.
  - `parse_dep_line()` — shared `name/version[@channel]` regex parser.
  - 5 unit tests.
- `crates/renovate-core/src/datasources/conan.rs` (new):
  - `fetch_latest(http, package_name, current_value)` — fetches `config.yml` from
    `conan-io/conan-center-index` on GitHub via raw Accept header; parses version keys.
  - `get_raw_with_accept()` added to `HttpClient`.
  - 2 unit tests.
- `crates/renovate-core/src/managers.rs`: `conan` with `(^|/)conanfile\.(txt|py)$` pattern.
- `crates/renovate-cli/src/main.rs`: Conan pipeline with `ConanError::NotFound` → Skipped.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 904 passed

## Slice 0111 - Cake `.cake` build script extractor (NuGet datasource)

### Renovate reference
- `lib/modules/manager/cake/index.ts`
- Pattern: `/\.cake$/`
- Datasource: NuGet

### What landed
- `crates/renovate-core/src/extractors/cake.rs` (new):
  - `CakeDep { package_name, current_value, registry_url }` struct.
  - `find_comment_start()` — smart `//` comment detection that ignores `://` URL separators.
  - `extract()` — handles `#addin`/`#tool`/`#module`/`#load`/`#l` directives with `nuget:` prefix;
    skips `file://` local refs; strips `/* */` block comments; parses `package=` and `version=`.
  - 7 unit tests.
- `crates/renovate-core/src/managers.rs`: `cake` manager with `\.cake$` pattern.
- `crates/renovate-cli/src/main.rs`: Cake pipeline using `nuget_datasource::fetch_updates_concurrent`;
  deps without version → `Skipped("no-version")`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 911 passed

## Slice 0112 - Meteor `package.js` `Npm.depends()` extractor

### Renovate reference
- `lib/modules/manager/meteor/extract.ts`
- Pattern: `/(^|/)package\.js$/`
- Datasource: npm

### What landed
- `crates/renovate-core/src/extractors/meteor.rs` (new):
  - `MeteorDep { name, current_value }` struct.
  - `extract(content)` — DOTALL regex captures `Npm.depends({...})` block, then
    extracts `name: "version"` pairs with `PAIR_RE`.
  - 3 unit tests.
- `crates/renovate-core/src/managers.rs`: `meteor` manager with `(^|/)package\.js$` pattern.
- `crates/renovate-cli/src/main.rs`: Meteor pipeline using npm datasource.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 914 passed

## Slice 0113 - Batect `batect.yml` Docker image extractor

### Renovate reference
- `lib/modules/manager/batect/extract.ts`
- Patterns: `/(^|/)batect(-bundle)?\.ya?ml$/`
- Datasource: Docker Hub

### What landed
- `crates/renovate-core/src/extractors/batect.rs` (new):
  - `extract(content)` — scans `containers:` block for `image:` keys; stops at next top-level key.
  - Delegates to `classify_image_ref` for Docker image parsing.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `batect` manager with batect.yml pattern.
- `crates/renovate-cli/src/main.rs`: Batect pipeline using `docker_hub_reports` helper.

### What was intentionally deferred
- `include[*]` with `type: git` (git bundle includes, requires git-tags datasource with custom URLs).

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 919 passed

## Slice 0114 - Copier `.copier-answers.yml` template extractor

### Renovate reference
- `lib/modules/manager/copier/extract.ts`
- Pattern: `/(^|/)\.copier-answers(\..+)?\.ya?ml/`
- Datasource: git-tags (GitHub Tags for GitHub URLs)

### What landed
- `crates/renovate-core/src/extractors/copier.rs` (new):
  - `CopierDep { src_path, github_repo, current_value }` struct.
  - `extract(content)` — scans for `_src_path:` and `_commit:` YAML keys;
    strips `git+` prefix; extracts `owner/repo` from GitHub URLs.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `copier` manager with `.copier-answers*.yml` pattern.
- `crates/renovate-cli/src/main.rs`: Copier pipeline — GitHub repos use GitHub Tags; others → Skipped.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 924 passed

## Slice 0115 - Vendir `vendir.yml` Helm chart extractor

### Renovate reference
- `lib/modules/manager/vendir/extract.ts` — `extractHelmChart`
- Pattern: `/(^|/)vendir\.yml$/`
- Datasource: Helm

### What landed
- `crates/renovate-core/src/extractors/vendir.rs` (new):
  - `VendirHelmDep { chart_name, version, repo_url }` struct.
  - `extract(content)` — state-machine scanner detecting `helmChart:` blocks; extracts
    `name:`, `version:`, and `url:` (under `repository:`).
  - 4 unit tests.
- `crates/renovate-core/src/managers.rs`: `vendir` manager with `(^|/)vendir\.yml$` pattern.
- `crates/renovate-cli/src/main.rs`: Vendir pipeline using Helm datasource.

### What was intentionally deferred
- Docker image deps in Vendir `contents` items (non-helmChart types).
- Git refs (`type: git`) — requires a git-tags-with-URL datasource.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 928 passed

## Slice 0116 - Jsonnet Bundler `jsonnetfile.json` extractor

### Renovate reference
- `lib/modules/manager/jsonnet-bundler/extract.ts`
- Pattern: `/(^|/)jsonnetfile\.json$/`
- Datasource: git-tags (GitHub Tags for GitHub remotes)

### What landed
- `crates/renovate-core/src/extractors/jsonnet_bundler.rs` (new):
  - `JsonnetDep { remote, github_repo, version }` struct.
  - `extract(content)` — parses JSON, iterates `dependencies[]`, extracts `source.git.remote` + `version`.
  - `github_repo()` — converts GitHub HTTPS/SSH URLs to `owner/repo` form.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `jsonnet-bundler` with `jsonnetfile.json` pattern.
- `crates/renovate-cli/src/main.rs`: Jsonnet Bundler pipeline using GitHub Tags; non-GitHub → Skipped.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 933 passed

## Slice 0117 - FVM `.fvmrc`/`.fvm/fvm_config.json` Flutter version extractor

### Renovate reference
- `lib/modules/manager/fvm/extract.ts`
- Patterns: `/(^|/)\.fvm/fvm_config\.json$/`, `/(^|/)\.fvmrc$/`
- Datasource: flutter-version (we use GitHub Tags `flutter/flutter`)

### What landed
- `crates/renovate-core/src/extractors/fvm.rs` (new):
  - `FvmDep { version }` struct.
  - `extract(content)` — deserializes JSON; reads `flutter` or `flutterSdkVersion` key.
  - 5 unit tests.
- `crates/renovate-core/src/managers.rs`: `fvm` manager with `.fvmrc` and `.fvm/fvm_config.json` patterns.
- `crates/renovate-cli/src/main.rs`: FVM pipeline using GitHub Tags `flutter/flutter`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 938 passed

## Slice 0118 - Haskell Cabal `*.cabal` extractor + Hackage datasource

### Renovate reference
- `lib/modules/manager/haskell-cabal/extract.ts`
- `lib/modules/datasource/hackage/index.ts`
- Pattern: `/\.cabal$/`
- Datasource: Hackage (`https://hackage.haskell.org/package/{name}.json`)

### What landed
- `crates/renovate-core/src/extractors/cabal.rs` (new):
  - `CabalDep { package_name, current_value }` struct.
  - `extract(content)` — finds `build-depends:` fields (case-insensitive), collects
    continuation lines, strips `--` comments, splits on commas, extracts package names.
  - 5 unit tests.
- `crates/renovate-core/src/datasources/hackage.rs` (new):
  - `fetch_latest(http, package_name)` — GETs `{name}.json`, filters deprecated versions,
    sorts by PVP version components, returns latest.
  - `cmp_pvp()` — PVP-aware numeric component comparison.
  - 1 unit test.
- `crates/renovate-core/src/managers.rs`: `haskell-cabal` with `\.cabal$` pattern.
- `crates/renovate-cli/src/main.rs`: Cabal pipeline with exact-version update detection.

### What was intentionally deferred
- Complex version constraint ranges for update comparison (e.g. `>= 4.7 && < 5`).

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 944 passed

## Slice 0167 - `enabledManagers` repo config option

### Renovate reference
- `lib/config/options/index.ts` — `enabledManagers` option
- When set, only the listed manager names are active; all others are skipped.

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `enabled_managers: Vec<String>` field added to `RepoConfig`.
  - Parsed from `"enabledManagers"` key in `renovate.json` / `.renovaterc`.
  - `is_manager_enabled(name)` helper method.
  - 2 unit tests.
- `main.rs`: after `managers::detect()`, filters the detected list using
  `repo_cfg.is_manager_enabled()` when `enabled_managers` is non-empty.
  - Single filter point; all 100+ `manager_files()` calls inherit the restriction.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1142 passed

## Slice 0166 - NuGet cross-file deduplication for .NET solutions

### What landed
- `crates/renovate-core/src/datasources/nuget.rs`:
  - `fetch_latest_batch(package_ids, api_base, concurrency)` — concurrent batch.
  - `summary_from_cache(current_value, latest)` — summary from cache.
  - `NuGetLatestEntry` type alias.
- `main.rs` nuget pipeline refactored to three passes.
  - Significant for .NET solutions with many `.csproj`/`.fsproj` files.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1140 passed

## Slice 0165 - Go module cross-file deduplication for Go workspaces

### What landed
- `crates/renovate-core/src/datasources/gomod.rs`:
  - `fetch_latest_batch(module_paths, proxy_base, concurrency)` — concurrent batch.
  - `summary_from_cache(current_value, latest)` — update summary from cache.
  - `GoModLatestEntry` type alias.
- `main.rs` gomod pipeline refactored to three passes.
  - Significant for Go workspaces with multiple `go.mod` files.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1140 passed

## Slice 0164 - Maven cross-file deduplication for multi-module projects

### What landed
- `crates/renovate-core/src/datasources/maven.rs`:
  - `fetch_latest_batch(dep_names, concurrency)` — fetches latest version for
    a batch of unique `groupId:artifactId` coordinates concurrently.
  - `summary_from_cache(current_version, latest)` — update summary from cache.
  - `MavenLatestEntry` type alias.
- `main.rs` maven pipeline refactored to three passes:
  1. Fetch all pom.xml files and extract deps.
  2. Collect unique coordinates, call `fetch_latest_batch` once.
  3. Build per-file reports from cached latest versions.

### Impact
- Reduces Maven Central requests from O(files × coords) to O(unique coords).
- Significant for Java multi-module projects with dozens of POMs.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1140 passed

## Slice 0163 - PyPI cross-file deduplication for pip_requirements + pip-compile

### What landed
- `crates/renovate-core/src/datasources/pypi.rs`:
  - `fetch_versions_batch(names, api_base, concurrency)` — batch PyPI fetch.
  - `summary_from_cache(specifier, entry)` — PEP 440 summary from cached versions.
  - `PypiVersionsEntry` type alias.
  - `PypiError::NotFound` variant.
- `main.rs`: pip_requirements and pip-compile pipelines merged into a single
  two-pass block:
  1. Fetch all requirement files from both managers.
  2. Collect unique package names, call `fetch_versions_batch` once.
  3. Build per-file reports using `summary_from_cache`.

### Impact
- Repos with both `requirements.txt` and `requirements-dev.txt` sharing packages
  (e.g. `django`) make one PyPI API call instead of two.
- pip-compile `.in` files are processed in the same dedup batch.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1140 passed

## Slice 0162 - Cargo cross-file request deduplication

### What landed
- `crates/renovate-core/src/datasources/crates_io.rs`:
  - `fetch_versions_batch(names, index_base, concurrency)` — batch version fetch.
  - `summary_from_cache(constraint, versions)` — summary from cached versions.
  - `CrateVersionsEntry` type alias.
  - `CratesIoError::NotFound` variant.
- `main.rs` cargo pipeline refactored to three passes (same pattern as npm):
  1. Fetch all `Cargo.toml` files and extract deps.
  2. Collect unique crate names, call `fetch_versions_batch` once.
  3. Build per-file reports using `summary_from_cache`.

### Impact
- Reduces crates.io index requests from O(files × crates) to O(unique crates).
- Significant for Rust workspaces with multiple crates sharing dependencies.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1140 passed

## Slice 0161 - npm cross-file request deduplication

### Motivation
In monorepos with multiple `package.json` files, the previous pipeline made
one npm registry call per package per file. If `lodash` appears in 10 workspaces,
that was 10 identical requests to registry.npmjs.org. This slice eliminates
that redundancy.

### What landed
- `crates/renovate-core/src/datasources/npm.rs`:
  - `fetch_versions_batch(names, registry, concurrency)` — fetches versions
    for a set of unique package names; returns `HashMap<name, (versions, latest_tag)>`.
  - `summary_from_cache(constraint, entry)` — computes `NpmUpdateSummary`
    from a pre-fetched entry without a network call.
  - Added `NpmError::NotFound` variant for cache-miss reporting.
- `main.rs` npm pipeline refactored to three passes:
  1. Fetch all `package.json` files and extract deps.
  2. Collect unique package names, call `fetch_versions_batch` once.
  3. Build per-file reports using `summary_from_cache` from the shared cache.
- `NpmVersionsEntry` type alias exported for external use.

### Impact
- Reduces npm registry calls from O(files × packages) to O(unique packages).
- Most beneficial in monorepos; single-file repos see no change in behavior.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1140 passed

## Slice 0160 - JSR datasource + endoflife-date datasource

### Renovate reference
- `lib/modules/datasource/jsr/index.ts` — JSR.io package registry
- `lib/modules/datasource/endoflife-date/index.ts` — endoflife.date API

### What landed
- `crates/renovate-core/src/datasources/jsr.rs` (new):
  - Fetches `https://jsr.io/@scope/name/meta.json` for JSR package versions.
  - Validates `@scope/name` format before making requests.
  - Filters yanked versions; returns `latest` field from meta.
  - 2 unit tests.
- `crates/renovate-core/src/datasources/endoflife.rs` (new):
  - Fetches `https://endoflife.date/api/{product}.json` for lifecycle info.
  - Custom `eol` field deserializer handles both `bool` and date-string values.
  - Returns EOL status + latest non-EOL cycle version.
  - 1 integration-style test with wiremock.

### Notes
- Both datasources are available for user-configured `custom` manager rules.
- Neither is wired to a specific manager pipeline yet (no upstream manager
  directly uses them by default). Can be used via custom regex manager.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1140 passed

## Slice 0159 - Conda datasource (Anaconda API) + pixi conda dep activation

### Renovate reference
- `lib/modules/datasource/conda/index.ts`
- Default registry: `https://api.anaconda.org/package/`
- Default channel: `conda-forge`

### What landed
- `crates/renovate-core/src/datasources/conda.rs` (new):
  - `fetch_latest(package_name, current_value, http)` queries Anaconda API.
  - Supports `channel::package` syntax (e.g. `bioconda::bwa`).
  - Returns `CondaUpdateSummary { versions, latest, update_available }`.
  - 2 unit tests for channel parsing.
- `crates/renovate-core/src/extractors/pixi.rs` (updated):
  - Removed `PixiSkipReason::CondaNotSupported` — conda deps are now actionable.
  - `parse_conda_dep` only skips when version is missing/empty.
  - Updated test: `extracts_conda_deps_as_actionable`.
- `main.rs` pixi pipeline:
  - Conda actionable deps now route to `conda_datasource::fetch_latest`.
  - Conda and PyPI dep reports built and combined in a single `FileReport`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1137 passed

## Slice 0158 - Hermit package manager extractor + datasource

### Renovate reference
- `lib/modules/manager/hermit/extract.ts`
- `lib/modules/datasource/hermit/index.ts`
- Registry: `https://github.com/cashapp/hermit-packages` (default)

### What landed
- `crates/renovate-core/src/extractors/hermit.rs` (new):
  - `extract_from_file_list(files: &[String])` — scans file list for `bin/.*.pkg` files.
  - Versioned: `bin/.git-2.47.0.pkg` → name=`git`, version=`2.47.0`
  - Channel: `bin/.kubectl@stable.pkg` → name=`kubectl`, channel=`@stable`, ChannelPin skip.
  - 5 unit tests.
- `crates/renovate-core/src/datasources/hermit.rs` (new):
  - Fetches `https://api.github.com/repos/{owner}/{repo}/releases/tags/index`
  - Downloads `index.json` asset, finds package by name, returns versions + channels.
  - `parse_github_repo()` helper extracts owner/repo from GitHub URL.
- `main.rs`: Hermit pipeline uses `filtered_files` (already fetched) directly.
  - No extra HTTP call for file content — filenames encode all needed info.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1135 passed

## Slice 0157 - `pip-compile` pipeline for `.in` source files

### Renovate reference
- `lib/modules/manager/pip-compile/extract.ts`
- Full upstream behavior: tracks `.in`→`requirements.txt` output relationships (deferred).
- Datasource: PyPI

### What landed
- `main.rs`: `pip-compile` pipeline block reads `requirements*.in` files with `pip_extractor::extract`.
  - Routes to PyPI datasource same as `pip_requirements` pipeline.
  - Simplified vs upstream (no output-file linking, no lockfile maintenance).
  - Full multi-file source/output graph deferred to a future slice.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run --workspace --all-features`: 1129 passed

## Slice 0155 - `cdnurl` pipeline + remaining manager stub registrations

### Renovate reference
- `lib/modules/manager/cdnurl/extract.ts` — same cloudflare URL regex as html, no SRI
- `lib/modules/manager/git-submodules/` — requires local git ops (deferred)
- `lib/modules/manager/hermit/` — requires directory listing (deferred)
- `lib/modules/manager/pip-compile/` — multi-file delegation (deferred)
- `lib/modules/manager/custom/` — runtime user-defined patterns (deferred)

### What landed
- `managers.rs`: registered `cdnurl`, `git-submodules`, `hermit`, `pip-compile`, `custom`.
  - `cdnurl`: empty default patterns (user-configured), pipeline reuses html extractor.
  - Others: stub entries with practical file patterns so names are valid in config.
- `main.rs`: `cdnurl` pipeline block reuses `renovate_core::extractors::html::extract` + cdnjs datasource (same as `html`, but manager field = `"cdnurl"`).
- All upstream manager names are now registered — manager name coverage = 100%.

### Deferred
- `git-submodules`: needs local git operations to resolve submodule commit hashes.
- `hermit`: needs directory listing for `.*.pkg` files.
- `pip-compile`: needs multi-file extraction delegating to pip_requirements/pep621/pip_setup.
- `custom`: needs runtime regex/jsonpath pattern support.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1049 passed

## Slice 0154 - PEP 723 Python inline script metadata extractor

### Renovate reference
- `lib/modules/manager/pep723/extract.ts`, `utils.ts`, `schema.ts`
- Default patterns: `[]` (user-configured). We add `scripts/*.py`, `*.script.py` conventions.
- Datasource: PyPI

### What landed
- `crates/renovate-core/src/extractors/pep723.rs` (new):
  - Regex `# /// script\n(...)# ///` extracts the metadata block.
  - Strips `# ` prefix per line to reconstruct TOML, then parses with `toml::from_str`.
  - Note: must use `toml::from_str::<Value>()` (serde path), NOT `s.parse::<Value>()` (which only parses TOML value literals, not documents).
  - PEP 508 parser extracts name + version specifier; name normalized per PEP 503.
  - 6 unit tests: versioned deps, unpinned, direct ref, name normalization, pinned, no block.
- Registered in `extractors.rs`, `managers.rs`.
- `crates/renovate-cli/src/main.rs`: pipeline uses `pypi_datasource::fetch_updates_concurrent`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1049 passed

## Slice 0153 - OCB (OpenTelemetry Collector Builder) Go module extractor

### Renovate reference
- `lib/modules/manager/ocb/extract.ts`, `schema.ts`
- Default patterns: `[]` (user-configured). We add `otelcol-builder.ya?ml`, `ocb.ya?ml`.
- Datasource: `go` (Go module proxy)

### What landed
- `crates/renovate-core/src/extractors/ocb.rs` (new):
  - Line-based YAML scanner detects `dist:` / module-section structure.
  - `dist.otelcol_version` → collector dep (bare version; pipeline prepends `v` for proxy lookup).
  - `exporters[]`, `extensions[]`, `receivers[]`, `processors[]`, `providers[]`, `connectors[]` → `gomod: path version` entries.
  - `OcbDep { dep_name, current_value, dep_type, skip_reason }`.
  - 4 unit tests covering full example, section types, missing version, unknown content.
- Registered in `extractors.rs`, `managers.rs`.
- `crates/renovate-cli/src/main.rs`: pipeline uses `gomod_datasource::fetch_updates_concurrent`.
  - Collector dep: `v`-prefix normalized before proxy lookup.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1043 passed

## Slice 0152 - Sveltos `ClusterProfile`/`Profile` Helm chart extractor

### Renovate reference
- `lib/modules/manager/sveltos/extract.ts`
- Patterns: `sveltos/*.ya?ml` (local convention)
- Datasources: Helm

### What landed
- `crates/renovate-core/src/extractors/sveltos.rs` (new):
  - Detects `apiVersion: (config|lib).projectsveltos.io/` documents.
  - Line-based state machine scans `helmCharts:` blocks for `repositoryURL`, `chartName`, `chartVersion`.
  - Handles multi-document YAML (`---` separators) and inline `- key: value` list items.
  - `SveltosDep { chart_name, current_value, registry_url }` maps directly to `HelmDepInput`.
  - 3 unit tests: single chart, multiple charts, skip non-sveltos.
- Registered in `extractors.rs`, `managers.rs` with `sveltos/*.ya?ml` pattern.
- `crates/renovate-cli/src/main.rs`: pipeline uses `helm_datasource::fetch_updates_concurrent`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1039 passed

## Slice 0151 - Renovate config presets extractor + `helm-requirements` alias

### Renovate reference
- `lib/modules/manager/renovate-config-presets/extract.ts`
- Patterns: `renovate.json(5)`, `.renovaterc`, `.github/renovate.json(5)`, `.gitlab/renovate.json(5)`
- Datasources: GitHub Tags, GitLab Tags

### What landed
- `crates/renovate-core/src/extractors/renovate_config_presets.rs` (new):
  - `PRESET_STR_RE` (r##"..."##) — matches `"github>owner/repo#tag"` patterns.
  - Extracts `github>` presets → GitHub Tags, `gitlab>` presets → GitLab Tags.
  - Skips presets without `#tag` (UnspecifiedVersion) and internal presets.
  - 5 unit tests.
- Registered in `extractors.rs`, `managers.rs` with all standard Renovate config filenames.
- `helm-requirements` registered as manager name alias for `requirements.ya?ml`
  (no new pipeline — helmv3 already processes these files).
- `crates/renovate-cli/src/main.rs`: pipeline uses `github_tags`/`gitlab_tags` datasources.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1036 passed

## Slice 0150 - Glasskube package manifest extractor + packages datasource

### Renovate reference
- `lib/modules/manager/glasskube/extract.ts`
- `lib/modules/datasource/glasskube-packages/index.ts`
- Default patterns: `[]`; we add `(^|/)glasskube/.+\.ya?ml$`
- Datasource: Glasskube packages registry (`packages.dl.glasskube.dev`)

### What landed
- `crates/renovate-core/src/extractors/glasskube.rs` (new):
  - `GLASSKUBE_RE` — content detection for `apiVersion: glasskube.dev/`.
  - Multi-doc YAML split on `\n---`, extracts `packageInfo.name` + `version`.
  - 3 unit tests.
- `crates/renovate-core/src/datasources/glasskube_packages.rs` (new):
  - `fetch_latest(http, package_name, current_value)`.
  - `GET https://packages.dl.glasskube.dev/packages/{name}/versions.yaml`.
  - Regex-based YAML parser for `latestVersion` and `versions[].version`.
- Registered in all 4 files. Manager count: 111.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1031 passed

## Slice 0149 - Crossplane package manifest extractor

### Renovate reference
- `lib/modules/manager/crossplane/extract.ts`
- Default patterns: `[]`; we add `(^|/)crossplane/.+\.ya?ml$`
- Datasource: `docker` (OCI packages — currently skipped for `xpkg.upbound.io`)

### What landed
- `crates/renovate-core/src/extractors/crossplane.rs` (new):
  - `CROSSPLANE_RE` — content detection for `apiVersion: pkg.crossplane.io/v*`.
  - Multi-document YAML split on `\n---` boundary.
  - `KIND_RE` + `PACKAGE_RE` — extracts kind and `spec.package` OCI image.
  - `xpkg.upbound.io` packages → `UnsupportedRegistry` skip (OCI registry pending).
  - 4 unit tests.
- Registered in `extractors.rs`, `managers.rs` with `crossplane/` directory pattern.
- `crates/renovate-cli/src/main.rs`: pipeline reports all deps with skip reasons.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1028 passed

## Slice 0148 - Bazel WORKSPACE `http_archive()` extractor (GitHub Tags + Releases)

### Renovate reference
- `lib/modules/manager/bazel/extract.ts`
- `lib/modules/manager/bazel/rules/git.ts`
- Patterns: `WORKSPACE(\.bazel|\.bzlmod)?`, `\.WORKSPACE\.bazel`, `\.bzl`
- Datasources: GitHub Tags (archive URLs), GitHub Releases (release download URLs)

### What landed
- `crates/renovate-core/src/extractors/bazel.rs` (new):
  - Brace-counting `extract_block()` finds `http_archive(...)` call boundaries.
  - `name = "..."` extraction with fallback inline pattern.
  - `urls = [...]` scanning with `GH_ARCHIVE_RE` / `GH_RELEASE_RE` matching
    (same patterns as Homebrew extractor — both handle GitHub tarball/zip URLs).
  - 5 unit tests.
- Registered in `extractors.rs`, `managers.rs` with WORKSPACE and `.bzl` patterns.
- `crates/renovate-cli/src/main.rs`: pipeline dispatches GitHub Archive → `github_tags`,
  GitHub Release → `github_releases_datasource`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1024 passed

## Slice 0147 - Tekton CI/CD resource extractor (step image deps)

### Renovate reference
- `lib/modules/manager/tekton/extract.ts`
- Default patterns: `[]`; we add `(^|/)tekton/.+\.ya?ml$`
- Datasource: `docker` (Docker Hub step images)

### What landed
- `crates/renovate-core/src/extractors/tekton.rs` (new):
  - `TEKTON_RE` — content detection for `apiVersion: tekton.dev/`.
  - Delegates to `kubernetes::extract()` for `image:` line scanning (Tekton steps
    use the same format as K8s containers).
  - Exports `KubernetesDep` and `KubernetesSkipReason` for unified pipeline handling.
  - 3 unit tests.
- Registered in `extractors.rs`, `managers.rs` with `(^|/)tekton/.+\.ya?ml$`.
- `crates/renovate-cli/src/main.rs`: pipeline identical to Kubernetes block.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1019 passed

## Slice 0146 - Kubernetes manifest Docker image extractor

### Renovate reference
- `lib/modules/manager/kubernetes/extract.ts`
- Default patterns: `[]`; we add `k8s/`, `kubernetes/`, `manifests/` directory conventions
- Datasource: `docker` (Docker Hub)

### What landed
- `crates/renovate-core/src/extractors/kubernetes.rs` (new):
  - `API_RE` + `KIND_RE` — content-based K8s detection (`apiVersion:` + `kind:`).
  - `IMAGE_RE` — extracts `image:` values with optional quotes and list prefix.
  - `split_image_tag()` — correctly splits `image:tag` (handles ports in registry URLs).
  - `is_non_docker_hub()` — detects private/GCR/ECR registries (skipped with `non-docker-hub`).
  - Skips `@sha256:` digest-pinned images and `:latest` tags.
  - 5 unit tests.
- Registered in `extractors.rs`, `managers.rs` with three common K8s directory patterns.
- `crates/renovate-cli/src/main.rs`: pipeline uses `docker_datasource::fetch_updates_concurrent`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1016 passed

## Slice 0145 - ArgoCD Application manifest extractor (Helm + Git sources)

### Renovate reference
- `lib/modules/manager/argocd/extract.ts`
- `lib/modules/manager/argocd/util.ts`
- Default patterns: `[]`; we add `(^|/)argocd/.+\.ya?ml$` and `(^|/)argo-cd/.+\.ya?ml$`
- Datasources: `helm` (chart sources), `git-tags` (GitHub Git sources)

### What landed
- `crates/renovate-core/src/extractors/argocd.rs` (new):
  - `ARGOCD_RE` — detects `apiVersion: argoproj.io/` (skips non-ArgoCD files).
  - `SOURCE_RE` — detects `source:` / `sources:` blocks.
  - `KV_RE` — extracts `repoURL`, `chart`, `targetRevision` fields.
  - Line-based scanner with flush-on-block-end pattern.
  - Helm sources → `ArgocdSource::Helm { registry_url, chart_name }`.
  - GitHub/Git sources → `ArgocdSource::Git { repo_url }`.
  - 4 unit tests.
- Registered in `extractors.rs`, `managers.rs` with two ArgoCD directory patterns.
- `crates/renovate-cli/src/main.rs`: pipeline dispatches Helm to `helm_datasource`,
  Git to `github_tags_datasource`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1011 passed

## Slice 0144 - Bun lockfile manager + nodenv/nvm/pyenv manager aliases

### Renovate reference
- `lib/modules/manager/bun/index.ts` — bun lockfile detection
- `lib/modules/manager/nodenv/index.ts`, `nvm/index.ts`, `pyenv/index.ts`
- Patterns: `bun.lockb?`, `.node-version`, `.nvmrc`, `.python-version`

### What landed
- Registered 4 new managers in `managers.rs`:
  - `bun`: detects `bun.lockb?` files; pipeline reads sibling `package.json`.
  - `nodenv`: `.node-version` alias for existing `node-version` pipeline.
  - `nvm`: `.nvmrc` alias for existing `nvmrc` pipeline.
  - `pyenv`: `.python-version` alias for existing `python-version` pipeline.
- `crates/renovate-cli/src/main.rs`:
  - `bun` pipeline: finds sibling `package.json` from lockfile path, extracts with
    `npm_extractor`, routes to npm datasource — no lockfile parsing needed.
  - Added `nodenv`, `nvm`, `pyenv` to the version-file manager loop (same routing
    as their counterparts via `AsdfDatasource`).

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1007 passed

## Slice 0143 - Heroku/Render `runtime.txt` Python version extractor

### Renovate reference
- `lib/modules/manager/runtime-version/extract.ts`
- Pattern: `(^|/)runtime\.txt$`
- Datasource: GitHub Releases on `python/cpython` (upstream uses DockerDatasource)

### What landed
- `crates/renovate-core/src/extractors/runtime_version.rs` (new):
  - `PYTHON_RE` matches `python-X.Y.Z` (exact 3-part semver).
  - Returns `RuntimeVersionDep { dep_name: "python", current_value: "X.Y.Z" }`.
  - 4 unit tests.
- Registered in `extractors.rs`, `managers.rs` with `(^|/)runtime\.txt$`.
- `crates/renovate-cli/src/main.rs`: pipeline uses GitHub Releases on `python/cpython`
  (strips `v` prefix for comparison).

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1007 passed

## Slice 0142 - Helmsman DSF extractor (Helm chart version tracking)

### Renovate reference
- `lib/modules/manager/helmsman/extract.ts`
- Default patterns: `[]`; we add `(^|/)helmsman\.ya?ml$` and `(^|/)helmsman\.d/.+\.ya?ml$`
- Datasource: `helm`

### What landed
- `crates/renovate-core/src/extractors/helmsman.rs` (new):
  - Line-based scanner with 2-section state machine (`helmRepos`, `apps`).
  - Parses `helmRepos:` alias→URL map.
  - For each `apps:` entry extracts `chart` and `version`.
  - Resolves `stable/redis` → `(alias="stable", chart_name="redis")` → registry URL.
  - `HelmsmanSkipReason::UnspecifiedVersion`, `InvalidChart`, `NoRepository`.
  - 4 unit tests.
- Registered in `extractors.rs`, `managers.rs` with two common Helmsman patterns.
- `crates/renovate-cli/src/main.rs`: pipeline uses `helm_datasource::fetch_updates_concurrent`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 1003 passed (crossed 1000 tests)

## Slice 0141 - Cloud Native Buildpacks `project.toml` extractor + BuildpacksRegistry datasource

### Renovate reference
- `lib/modules/manager/buildpacks/extract.ts` + `schema.ts`
- `lib/modules/datasource/buildpacks-registry/index.ts`
- Pattern: `(^|/)project\.toml$`
- Datasources: `buildpacks-registry` (actionable), `docker` (skipped)

### What landed
- `crates/renovate-core/src/extractors/buildpacks.rs` (new):
  - TOML parsing of `[io.buildpacks]` section.
  - `builder = "image:tag"` → `BuildpacksSource::Docker` (skipped, `docker-image`).
  - `[[io.buildpacks.group]]` with `id`+`version` → `BuildpacksSource::Registry` (actionable).
  - `uri = "urn:cnb:registry:ns/name@v"` → `BuildpacksSource::Registry` (actionable).
  - `uri = "docker://..."` or `uri = "image:tag"` → `BuildpacksSource::Docker` (skipped).
  - Unsupported schemes → `BuildpacksSkipReason::UnsupportedUri`.
  - 5 unit tests.
- `crates/renovate-core/src/datasources/buildpacks_registry.rs` (new):
  - `fetch_latest(http, package_name, current_value)`.
  - `GET https://registry.buildpacks.io/api/v1/buildpacks/{ns}/{name}`.
  - Versions returned newest-first; picks `results[0]`.
- Registered in `datasources.rs`, `extractors.rs`, `managers.rs`.
- `crates/renovate-cli/src/main.rs`: pipeline dispatches registry deps to datasource.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 999 passed

## Slice 0140 - Unity3D `ProjectVersion.txt` extractor + Unity releases datasource

### Renovate reference
- `lib/modules/manager/unity3d/extract.ts`
- `lib/modules/datasource/unity3d/index.ts`
- Pattern: `(^|/)ProjectSettings/ProjectVersion\.txt$`
- Datasource: Unity Releases API (`services.api.unity.com`)

### What landed
- `crates/renovate-core/src/extractors/unity3d.rs` (new):
  - `WITH_REV_RE` matches `m_EditorVersionWithRevision: 2022.3.10f1 (ff3792e53c62)`.
  - `VERSION_RE` fallback for plain `m_EditorVersion: 2022.3.10f1`.
  - `Unity3dVersionKind::WithRevision` / `Plain` controls datasource lookup mode.
  - 3 unit tests.
- `crates/renovate-core/src/datasources/unity3d.rs` (new):
  - `fetch_latest_lts(http, with_revision)` — fetches first page of LTS stream.
  - Returns `Unity3dUpdateSummary` with `latest` and `latest_with_revision` fields.
- Registered in `datasources.rs`, `extractors.rs`, `managers.rs`.
- `crates/renovate-cli/src/main.rs`: pipeline dispatches based on `kind` field.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 994 passed

## Slice 0139 - Pixi `pixi.toml` extractor (PyPI deps actionable, Conda skipped)

### Renovate reference
- `lib/modules/manager/pixi/extract.ts` + `schema.ts`
- Pattern: `(^|/)pixi\.toml$`
- Datasources: `pypi` (actionable), `conda` (skipped — not yet implemented)

### What landed
- `crates/renovate-core/src/extractors/pixi.rs` (new):
  - `extract(content)`: parses `[pypi-dependencies]` (PyPI, actionable),
    `[dependencies]` (Conda, skipped with `CondaNotSupported`),
    and `[feature.*.{pypi-dependencies,dependencies}]` sections.
  - `extract_from_pyproject(content)`: same for `[tool.pixi]` in `pyproject.toml`.
  - Supports both string specs (`">=1.0"`) and table specs (`{ version = ">=1.0" }`).
  - 6 unit tests.
- Registered in `extractors.rs`, `managers.rs` with `(^|/)pixi\.toml$`.
- `crates/renovate-cli/src/main.rs`: pipeline uses `pypi_datasource::fetch_updates_concurrent`
  for PyPI deps; Conda deps emitted as `Skipped { reason: "conda-not-supported" }`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 991 passed

## Slice 0138 - Bitrise CI step extractor + Bitrise steplib datasource

### Renovate reference
- `lib/modules/manager/bitrise/extract.ts` + `utils.ts`
- `lib/modules/datasource/bitrise/index.ts`
- Pattern: `(^|/)bitrise\.ya?ml$`
- Datasources: `bitrise` (steplib index), `git-tags` (git:: steps)

### What landed
- `crates/renovate-core/src/extractors/bitrise.rs` (new):
  - Regex-based line scanner; no serde_yaml dependency.
  - `DEFAULT_REGISTRY_RE` extracts `default_step_lib_source`.
  - `STEPS_KEY_RE` / `LIST_ITEM_RE` detect `steps:` blocks and list items.
  - `extract_yaml_key()` handles `:://` and `::` in step ref strings.
  - Routes `git::url@v` → `BitriseSource::Git`, `path::` → `BitriseSource::Local`,
    `url::step@v` → `BitriseSource::Steplib { registry_url: Some(...) }`,
    `step@v` → `BitriseSource::Steplib { registry_url: None }`.
  - 8 unit tests covering all ref forms and edge cases.
- `crates/renovate-core/src/datasources/bitrise.rs` (new):
  - `fetch_latest(http, step_name, current_value, registry_url)`.
  - Fetches `GET /repos/{owner}/{repo}/releases/tags/index` from GitHub API.
  - Parses `index.json` asset (`{ Name, Versions, Channels }`).
  - Process-level `Mutex<Option<...>>` cache for the default steplib URL.
- Registered in `datasources.rs`, `extractors.rs`, `managers.rs`.
- `crates/renovate-cli/src/main.rs`: pipeline routes `git::` steps to
  `github_tags_datasource`, steplib steps to `bitrise_datasource`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 985 passed

## Slice 0137 - Homebrew formula extractor (GitHub Archive/Release + NPM routing)

### Renovate reference
- `lib/modules/manager/homebrew/extract.ts`
- Pattern: `(^|/)Formula/[^/]+\.rb$`
- Datasources: GitHub Tags (archive), GitHub Releases (release download), NPM

### What landed
- `crates/renovate-core/src/extractors/homebrew.rs` (new):
  - Parses `class Name < Formula`, `url "..."`, `sha256 "..."` via `LazyLock<Regex>`.
  - Skips with `InvalidSha256` when sha256 is not exactly 64 hex chars.
  - Routes GitHub archive URLs (`/archive/refs/tags/` or `/archive/`) to `GitHubUrlType::Archive`.
  - Routes GitHub release URLs (`/releases/download/`) to `GitHubUrlType::Release`.
  - Routes `registry.npmjs.org` URLs to `HomebrewSource::Npm`.
  - Otherwise emits `HomebrewSkipReason::UnsupportedUrl`.
  - 5 unit tests covering all URL forms and skip cases.
- Registered in `extractors.rs` and `managers.rs` with `(^|/)Formula/[^/]+\.rb$`.
- `crates/renovate-cli/src/main.rs`: pipeline dispatches GitHub Archive to `github_tags::fetch_latest_tag`,
  GitHub Release to `github_releases::fetch_latest_release`, NPM to `npm_datasource::fetch_updates_concurrent`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`: clean
- `cargo nextest run -p renovate-core`: 975 passed

## Slice 0136 - Azure Bicep `.bicep` extractor + bicep-types-az datasource

### Renovate reference
- `lib/modules/manager/bicep/extract.ts`
- `lib/modules/datasource/azure-bicep-resource/index.ts`
- Pattern: `/\.bicep$/`
- Datasource: `raw.githubusercontent.com/Azure/bicep-types-az/main/generated/index.json`

### What landed
- `crates/renovate-core/src/datasources/azure_bicep.rs` (new):
  - `fetch_latest(http, resource_type, current_value)`.
  - `get_or_fetch_index()` — process-level `OnceLock` cache of the full index (avoids re-fetching per dep).
  - Parses `resources` map (keys `type@version`) into `HashMap<type, Vec<version>>`.
  - Latest = lexicographic max (ISO 8601 dates sort correctly).
- `crates/renovate-core/src/extractors/bicep.rs` (new):
  - `RESOURCE_RE` matches `resource Name 'Namespace.Provider/Type@version'` on non-comment lines.
  - 5 unit tests.
- Registered in `datasources.rs`, `extractors.rs`, `managers.rs` with `\.bicep$`.
- `crates/renovate-cli/src/main.rs`: pipeline using `azure_bicep::fetch_latest`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 969 passed

## Slice 0135 - Perl `cpanfile` extractor + MetaCPAN datasource

### Renovate reference
- `lib/modules/manager/cpanfile/extract.ts` + `parser.ts`
- `lib/modules/datasource/cpan/index.ts`
- Pattern: `/(^|/)cpanfile$/`
- Datasource: MetaCPAN (`fastapi.metacpan.org`)

### What landed
- `crates/renovate-core/src/datasources/cpan.rs` (new):
  - `fetch_latest(http, module_name, current_value)` — `GET /v1/module/{name}`.
  - Simpler single-endpoint approach vs Renovate's Elasticsearch POST.
- `crates/renovate-core/src/extractors/cpanfile.rs` (new):
  - Phase-tracking line scanner: `PHASE_BLOCK_RE` detects `on 'phase' => sub {`.
  - Brace-depth counter exits phase blocks on `}`.
  - `REQUIRES_RE` extracts module names + versions from `requires`/`recommends`/etc.
  - Handles comma and fat-arrow (`=>`) separators; bare numeric versions.
  - Skip reasons: `UnspecifiedVersion`, `PerlCore` (skips `perl` itself).
  - 8 unit tests.
- Registered in `datasources.rs`, `extractors.rs`, `managers.rs`.
- `crates/renovate-cli/src/main.rs`: pipeline using `cpan::fetch_latest`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 964 passed

## Slice 0134 - Bazel `MODULE.bazel` extractor + Bazel Central Registry datasource

### Renovate reference
- `lib/modules/manager/bazel-module/extract.ts`
- `lib/modules/datasource/bazel/index.ts`
- Pattern: `/(^|/|\.)MODULE\.bazel$/`
- Datasource: `https://raw.githubusercontent.com/bazelbuild/bazel-central-registry/main`

### What landed
- `crates/renovate-core/src/datasources/bazel.rs` (new):
  - `fetch_latest(http, module_name, current_value)` — fetches `metadata.json` from GitHub raw.
  - Filters out yanked versions before finding the latest by semver comparison.
- `crates/renovate-core/src/extractors/bazel_module.rs` (new):
  - `BAZEL_DEP_BLOCK_RE` matches `bazel_dep(...)` calls (including multiline via `(?s)`).
  - `ATTR_RE` extracts `name=` and `version=` from call arguments.
  - `DEV_DEP_RE` detects `dev_dependency = True`.
  - Comment stripping before regex matching.
  - 7 unit tests.
- `crates/renovate-core/src/datasources.rs`: added `pub mod bazel`.
- `crates/renovate-core/src/extractors.rs`: added `pub mod bazel_module`.
- `crates/renovate-core/src/managers.rs`: added `bazel-module` with `(^|/|\.)MODULE\.bazel$`.
- `crates/renovate-cli/src/main.rs`: pipeline using `bazel::fetch_latest`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 956 passed

## Slice 0133 - Python `setup.py` PyPI dependency extractor

### Renovate reference
- `lib/modules/manager/pip_setup/extract.ts`
- Pattern: `/(^|/)setup\.py$/`
- Datasource: PyPI

### What landed
- `crates/renovate-core/src/extractors/pip_setup.rs` (new):
  - `KEY_START_RE` regex finds `install_requires=[`, `tests_require=[`, `setup_requires=[`, `extras_require={` blocks.
  - `extract_balanced()` walks bytes tracking quote state to find end of bracket block.
  - `STRING_RE` collects all single/double-quoted string literals from the block.
  - Delegates to `pip::extract` for PEP 508 parsing (reuses existing logic).
  - 6 unit tests covering install_requires, tests_require, extras_require, empty lists.
- `crates/renovate-core/src/extractors.rs`: added `pub mod pip_setup`.
- `crates/renovate-core/src/managers.rs`: added `pip_setup` with `(^|/)setup\.py$`.
- `crates/renovate-cli/src/main.rs`: pipeline reuses `build_dep_reports_pip` and PyPI datasource.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 949 passed

## Slice 0132 - Apache Ant `build.xml` Maven dependency extractor

### Renovate reference
- `lib/modules/manager/ant/extract.ts`
- Pattern: `**/build.xml`
- Datasource: Maven

### What landed
- `crates/renovate-core/src/extractors/ant.rs` (new):
  - Uses `quick-xml` `Event::Empty | Event::Start` to scan all XML elements.
  - `local_name()` strips namespace prefix (`artifact:dependency` → `dependency`).
  - Handles two forms: `groupId`/`artifactId`/`version` attributes and `coords="g:a:v"`.
  - Collects `<remoteRepository url="...">` registry URLs and attaches to all deps.
  - Skip reasons: `PropertyRef`, `MissingVersion`.
  - 6 unit tests.
- `crates/renovate-core/src/extractors.rs`: added `pub mod ant`.
- `crates/renovate-core/src/managers.rs`: added `ant` with `(^|/)build\.xml$`.
- `crates/renovate-cli/src/main.rs`: pipeline using `maven_datasource::fetch_updates_concurrent`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 943 passed

## Slice 0131 - Terragrunt `terragrunt.hcl` extractor

### Renovate reference
- `lib/modules/manager/terragrunt/extract.ts` + `modules.ts`
- Pattern: `/(^|/)terragrunt\.hcl$/`
- Datasources: GitHub Tags, Terraform Module Registry

### What landed
- `crates/renovate-core/src/extractors/terragrunt.rs` (new):
  - Brace-counting scanner finds `terraform { source = "..." }` blocks.
  - `analyse_source()` dispatches on source string:
    - `github.com/owner/repo?ref=tag` → `TerragruntSource::GitHub`.
    - `git::https://github.com/...?ref=tag` → `TerragruntSource::GitHub`.
    - `git::https://other?ref=tag` → `TerragruntSource::Git`.
    - `tfr://registry/org/name/cloud?version=x` → `TerragruntSource::TerraformRegistry`.
    - 3-part registry paths → Terraform Registry (no version).
    - Local (`../`, `./`) → skipped.
  - 5 unit tests.
- `crates/renovate-core/src/extractors.rs`: added `pub mod terragrunt`.
- `crates/renovate-core/src/managers.rs`: added `terragrunt` with `(^|/)terragrunt\.hcl$`.
- `crates/renovate-cli/src/main.rs`: GitHub → `github_tags::fetch_latest_tag`; Terraform Registry → `terraform_datasource::fetch_updates_concurrent`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 937 passed

## Slice 0130 - Puppet `Puppetfile` extractor + Puppet Forge datasource

### Renovate reference
- `lib/modules/manager/puppet/extract.ts` + `puppetfile-parser.ts`
- `lib/modules/datasource/puppet-forge/index.ts`
- Pattern: `/(^|/)Puppetfile$/`
- Datasources: PuppetForgeDatasource, GithubTagsDatasource

### What landed
- `crates/renovate-core/src/datasources/puppet_forge.rs` (new):
  - `fetch_latest(http, module_name, current_value, registry_url)`.
  - `GET {forge}/v3/modules/{author}-{name}?exclude_fields=current_release`.
  - Normalizes `author/name` → `author-name` for API slug.
  - Finds latest by semver comparison over `releases[]`.
- `crates/renovate-core/src/extractors/puppet.rs` (new):
  - Line-by-line parser: `FORGE_RE` for forge URL changes, `MOD_START_RE` for mod declarations, `SYMBOL_KV_RE` for `:key => 'value'` Ruby symbol hashes.
  - Handles multi-line `mod` declarations ending with `,`.
  - Forge modules → `PuppetSource::PuppetForge { forge_url }`.
  - Git modules with GitHub URL → `PuppetSource::GitHub(repo)`.
  - Git modules without tag → skipped.
  - 7 unit tests.
- `crates/renovate-core/src/datasources.rs`: added `pub mod puppet_forge`.
- `crates/renovate-core/src/extractors.rs`: added `pub mod puppet`.
- `crates/renovate-core/src/managers.rs`: added `puppet` with `(^|/)Puppetfile$`.
- `crates/renovate-cli/src/main.rs`: routes Forge → `puppet_forge::fetch_latest`; GitHub → `github_tags::fetch_latest_tag`; generic git → skipped.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 932 passed

## Slice 0129 - OSGi feature model Maven bundle extractor

### Renovate reference
- `lib/modules/manager/osgi/extract.ts`
- Pattern: `/(^|/)src/main/features/.+\.json$/`
- Datasource: Maven

### What landed
- `crates/renovate-core/src/extractors/osgi.rs` (new):
  - Parses OSGi Compendium R8 feature model JSON files (via `json5` crate — supports JS comments).
  - Validates `feature-resource-version` must be major version 1.
  - Extracts bundle IDs from `bundles` array (string or `{id: ...}` object).
  - Also checks `execution-environment:JSON|false`.framework.
  - Scans extra sections for arrays with GAV-like entries.
  - Normalizes `/`-separated to `:`-separated GAVs.
  - Skip reasons: `ContainsVariable`, `InvalidValue`.
  - 7 unit tests including JSON5 comment support.
- `crates/renovate-core/src/extractors.rs`: added `pub mod osgi`.
- `crates/renovate-core/src/managers.rs`: added `osgi` with `src/main/features/.+\.json$`.
- `crates/renovate-cli/src/main.rs`: pipeline using `maven_datasource::fetch_updates_concurrent`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 925 passed

## Slice 0128 - XcodeGen `project.yml` Swift Package extractor

### Renovate reference
- `lib/modules/manager/xcodegen/extract.ts`
- Pattern: `**/project.yml`
- Datasources: GitHub Tags, GitLab Tags, Git Tags

### What landed
- `crates/renovate-core/src/extractors/xcodegen.rs` (new):
  - 3-state scanner: `Scanning` → `InPackages` → `InPackageEntry`.
  - Extracts `url:` / `github:` source and version fields (`from`, `exactVersion`, `version`, etc.).
  - `build_source()` detects GitHub/GitLab from URL prefix.
  - Skip reasons: `LocalPath`, `NoSemverVersion`, `MissingSource`.
  - 7 unit tests.
- `crates/renovate-core/src/extractors.rs`: added `pub mod xcodegen`.
- `crates/renovate-core/src/managers.rs`: added `xcodegen` with `(^|/)project\.yml$`.
- `crates/renovate-cli/src/main.rs`: pipeline routes GitHub sources to `github_tags::fetch_latest_tag`; non-GitHub sources skipped for now.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 918 passed

## Slice 0127 - Typst `.typ` package extractor + Typst registry datasource

### Renovate reference
- `lib/modules/manager/typst/extract.ts`
- `lib/modules/datasource/typst/index.ts`
- Pattern: `/\.typ$/`
- Datasource: `https://packages.typst.org/preview/index.json`

### What landed
- `crates/renovate-core/src/datasources/typst.rs` (new):
  - `fetch_latest(http, package_name, current_value)` fetches the flat JSON index.
  - Filters entries by package name, selects latest by semver comparison.
  - Error variants: `Http`, `Parse`, `NotFound`.
- `crates/renovate-core/src/extractors/typst.rs` (new):
  - `TypstDep { package_name, namespace, current_value, skip_reason }`.
  - `IMPORT_RE` matches `#import "@namespace/pkg:version"` patterns.
  - `preview` namespace → actionable; `local` → skipped; other → unsupported.
  - Line comments (`//`) are skipped.
  - 7 unit tests.
- `crates/renovate-core/src/datasources.rs`: added `pub mod typst`.
- `crates/renovate-core/src/extractors.rs`: added `pub mod typst`.
- `crates/renovate-core/src/managers.rs`: added `typst` with `\.typ$`.
- `crates/renovate-cli/src/main.rs`: pipeline block; skip non-preview deps.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 911 passed

## Slice 0126 - TFLint plugin `.tflint.hcl` extractor

### Renovate reference
- `lib/modules/manager/tflint-plugin/extract.ts`
- Pattern: `/\.tflint\.hcl$/`
- Datasource: GitHub Releases

### What landed
- `crates/renovate-core/src/extractors/tflint_plugin.rs` (new):
  - `TflintPluginDep { name, dep_name, current_value, skip_reason }`.
  - `PLUGIN_BLOCK` regex matches `plugin "name" {` block starts.
  - Brace-counting state machine extracts `source` and `version` from each block.
  - `build_dep` parses `github.com/owner/repo` source URL → `dep_name`.
  - Skip reasons: `MissingSource`, `UnsupportedDatasource`, `UnspecifiedVersion`.
  - 5 unit tests.
- `crates/renovate-core/src/extractors.rs`: added `pub mod tflint_plugin`.
- `crates/renovate-core/src/managers.rs`: added `tflint-plugin` with `\.tflint\.hcl$`.
- `crates/renovate-cli/src/main.rs`: pipeline block using `github_releases_datasource`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 904 passed

## Slice 0125 - Crow CI `.crow/*.yml` Docker image extractor

### Renovate reference
- `lib/modules/manager/crow/extract.ts`
- Pattern: `/^\.crow(?:/[^/]+)?\.ya?ml$/`
- Datasource: Docker

### What landed
- `crates/renovate-core/src/extractors/crow.rs` (new):
  - Scans `pipeline:`, `steps:`, `clone:`, `services:` sections for `image:` values.
  - Array items at indent=0 correctly remain in the current section.
  - 7 unit tests covering pipeline, services, array-style steps, clone, variable refs.
- `crates/renovate-core/src/extractors.rs`: added `pub mod crow`.
- `crates/renovate-core/src/managers.rs`: added `crow` with `^\.crow(?:/[^/]+)?\.ya?ml$`.
- `crates/renovate-cli/src/main.rs`: pipeline block using `docker_hub_reports`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 899 passed

## Slice 0124 - Rancher Fleet extractor (Helm + GitRepo CRD dual-mode)

### Renovate reference
- `lib/modules/manager/fleet/extract.ts`
- Pattern: `/(^|/)fleet\.ya?ml/`
- Datasources: Helm, GitTagsDatasource

### What landed
- `crates/renovate-core/src/extractors/fleet.rs` (new):
  - `FleetDeps { helm_deps, git_deps }` return type.
  - `is_fleet_yaml_path(path)` detects `fleet.yaml`/`fleet.yml` by filename.
  - `extract_fleet_yaml(content)` — indentation-level scanner:
    - Parses top-level `helm:` block (chart, repo, version).
    - Parses `targetCustomizations:` list; each item merges with the base helm block.
    - Skip reasons: `MissingChart`, `NoRepository`, `OciRegistry`, `LocalOrAlias`, `UnspecifiedVersion`.
  - `extract_gitrepo(content)` — multi-document YAML scanner:
    - Processes `---` separated docs.
    - Only processes docs with `kind: GitRepo`.
    - Extracts `spec.repo` and `spec.revision`.
    - Skip reasons: `MissingRepo`, `UnspecifiedVersion`.
  - 8 unit tests.
- `crates/renovate-core/src/extractors.rs`: added `pub mod fleet`.
- `crates/renovate-core/src/managers.rs`: added `fleet` with `(^|/)fleet\.ya?ml` pattern.
- `crates/renovate-cli/src/main.rs`:
  - Pipeline block: uses `is_fleet_yaml_path()` to dispatch parsing mode.
  - Helm deps → `helm_datasource::fetch_updates_concurrent`.
  - Git deps → `github_tags::fetch_latest_tag` (GitHub repo URL parsed from spec.repo).

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 892 passed

## Slice 0123 - HTML cdnjs extractor + CDNJS datasource

### Renovate reference
- `lib/modules/manager/html/extract.ts`
- Pattern: `/\.html?$/`
- Datasource: CDNJS (`api.cdnjs.com`)

### What landed
- `crates/renovate-core/src/datasources/cdnjs.rs` (new):
  - `fetch_latest(http, library, current_value)` → `GET https://api.cdnjs.com/libraries/{library}?fields=versions`.
  - Finds highest version using semver comparison.
  - Error variants: `Http`, `NotFound`, `Parse`.
- `crates/renovate-core/src/extractors/html.rs` (new):
  - `HtmlCdnjsDep { dep_name, current_value, asset }`.
  - `TAG_RE` matches `<script>` and `<link>` tags.
  - `CDNJS_URL_RE` extracts `depName/version/asset` from cdnjs.cloudflare.com URLs.
  - 6 unit tests.
- `crates/renovate-core/src/datasources.rs`: added `pub mod cdnjs`.
- `crates/renovate-core/src/extractors.rs`: added `pub mod html`.
- `crates/renovate-core/src/managers.rs`: added `html` with `\.html?$` pattern.
- `crates/renovate-cli/src/main.rs`: pipeline block fetches HTML, extracts cdnjs deps, fetches latest versions.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 883 passed

## Slice 0122 - Kotlin Script `*.main.kts` Maven dependency extractor

### Renovate reference
- `lib/modules/manager/kotlin-script/extract.ts`
- Pattern: `/^.+\.main\.kts$/`
- Datasource: Maven

### What landed
- `crates/renovate-core/src/extractors/kotlin_script.rs` (new):
  - `KotlinScriptDep { dep_name, current_value, registry_urls }`.
  - `DEPENDS_ON_RE` regex extracts `@file:DependsOn("group:artifact:version")`.
  - `REPOSITORY_RE` regex extracts `@file:Repository("url")` declarations.
  - `registry_urls` attached to each dep (not yet used for lookup — Maven Central default).
  - 5 unit tests.
- `crates/renovate-core/src/extractors.rs`: added `pub mod kotlin_script`.
- `crates/renovate-core/src/managers.rs`: added `kotlin-script` with `^.+\.main\.kts$` pattern.
- `crates/renovate-cli/src/main.rs`: pipeline block fetches `*.main.kts`, extracts deps, looks up Maven Central updates.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 877 passed

## Slice 0121 - Dev Container features extractor upgrade

### Renovate reference
- `lib/modules/manager/devcontainer/extract.ts`
- Patterns: `^.devcontainer/devcontainer.json$`, `^.devcontainer.json$`
- Datasources: Docker, NodeVersionDatasource, GolangVersionDatasource, PythonVersionDatasource, RubyVersionDatasource

### What landed
- `crates/renovate-core/src/extractors/devcontainer.rs` upgraded:
  - Rewrote using `serde_json` for proper JSON parsing.
  - Added `DevContainerDeps { docker_deps, version_deps }` return type.
  - Extracts `image` field as Docker dep (unchanged).
  - Extracts each `features` key as OCI Docker dep.
  - For known features (node, go, python, ruby), also extracts `version` field as `VersionFileDep` using same GitHub Tags/Releases datasources as `version_file.rs`.
  - 7 unit tests covering all cases.
- `crates/renovate-cli/src/main.rs`:
  - Updated devcontainer pipeline block to handle new `DevContainerDeps` struct.
  - Docker deps → `docker_hub_reports`.
  - Version deps → GitHub Tags/Releases lookup (same pattern as version-file manager).
  - Ruby tag underscore normalization applied.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 872 passed

## Slice 0120 - Home Assistant `manifest.json` PyPI extractor

### Renovate reference
- `lib/modules/manager/homeassistant-manifest/extract.ts`
- Pattern: `/(^|/)manifest\.json$/`
- Datasource: PyPI

### What landed
- `crates/renovate-core/src/extractors/homeassistant.rs` (new):
  - Deserializes `HaManifest { requirements: Option<Vec<String>> }` via serde_json.
  - Delegates to `pip::extract` by joining requirements with `\n` — reuses PEP 508 parser.
  - 4 unit tests: exact version, range version, invalid JSON, empty requirements.
- `crates/renovate-core/src/extractors.rs`: added `pub mod homeassistant`.
- `crates/renovate-core/src/managers.rs`: added `homeassistant-manifest` with pattern `(^|/)manifest\.json$`.
- `crates/renovate-cli/src/main.rs`:
  - Import `homeassistant as homeassistant_extractor`.
  - Pipeline block: fetches `manifest.json`, extracts deps, fetches PyPI updates, pushes `FileReport`.
  - Reuses `build_dep_reports_pip` since the extractor returns `Vec<PipExtractedDep>`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run -p renovate-core`: 871 passed

## Slice 0119 - Batect wrapper script version extractor

### Renovate reference
- `lib/modules/manager/batect-wrapper/extract.ts`
- Pattern: `/(^|/)batect$/`
- Datasource: GitHub Releases (`batect/batect`)

### What landed
- `crates/renovate-core/src/extractors/batect_wrapper.rs` (new):
  - `BatectWrapperDep { version }` struct.
  - `extract(content)` — finds `VERSION="x.y.z"` via regex; 3 unit tests.
- `crates/renovate-core/src/managers.rs`: `batect-wrapper` with `(^|/)batect$` pattern.
- `crates/renovate-cli/src/main.rs`: Uses `github_releases_datasource::fetch_latest_release("batect/batect")`.

### Verification
- `cargo fmt --all && cargo clippy --all-targets --all-features`
- `cargo nextest run --workspace`: 947 passed

## Next slice candidates

Pick whichever can be completed in one loop:

1. **Renovate option surface (first cut)**: port the option definitions
   from `lib/config/options/index.ts` into a strongly-typed Rust schema
   and wire them into clap.
2. **Cargo lock parsing**: parse `Cargo.lock` for pinned transitive dependency versions.
3. **`bazel` / `MODULE.bazel` extractor**: Bazel module deps (requires Bazel Central Registry datasource).
4. **`tekton` extractor**: Tekton pipeline bundle references.
5. **`devcontainer` features** — version extraction for Node, Go, Python, Ruby features.
6. **`argocd`** — ArgoCD Application YAML Helm chart version extraction.
7. **`homebrew`** — Homebrew formula GitHub version tracking.
8. **`pixi`** — Pixi `pixi.toml` package extraction (PyPI + Conda).
9. **`cdnurl`** — CDN URL version extraction (semver in URL paths).
7. **`homebrew`** — Homebrew formula GitHub version tracking.
8. **`batect-wrapper`** — Batect wrapper script version tracking (GitHub Releases).
9. **`pixi`** — Pixi `pixi.toml` package extraction (PyPI + Conda).
7. **`haskell-cabal`** — Cabal `*.cabal` Hackage package version tracking.
8. **`homebrew`** — Homebrew formula version tracking.
9. **`glasskube`** — Glasskube package manifest version tracking.
7. **`fvm`** — Flutter Version Manager `.fvmrc` / `fvm_config.json` version tracking.
8. **`helm-requirements`** — Helm v2 `requirements.yaml` chart tracking.
9. **`haskell-cabal`** — Cabal `*.cabal` package version tracking (Hackage datasource).
7. **`pixi`** — Pixi `pixi.toml` package extraction (PyPI + Conda).
8. **`renovate-config-presets`** — `renovate.json` extends preset version tracking.
9. **`nodenv`** — `.node-version` tracking (already covered by node-version manager).
7. **`vendir`** — Vendir `vendir.yml` Helm chart + Docker image sync tracking.
8. **`osgi`** — OSGi `bnd.bnd` / `*.bndrun` Maven artifact extraction.
9. **`pip-compile`** — `requirements.in` tracking upstream constraint files.
7. **`vendir`** — Vendir `vendir.yml` sync bundle version tracking.
8. **`pixi`** — Pixi `pixi.toml` conda/PyPI package extraction.
9. **`cross`** — Rust Cross `Cross.toml` Docker image extraction.
7. **`cpanfile`** — Perl `cpanfile` dependency extraction (MetaCPAN API).
8. **`pixi`** — Pixi `pixi.toml` conda/PyPI package extraction.
9. **`batect`** — Batect `batect.yml` Docker image extraction.
7. **`cpanfile`** — Perl `cpanfile` dependency extraction (MetaCPAN API).
8. **`pixi`** — Pixi `pixi.toml` conda/PyPI package extraction.
9. **`crow`** — C++ Crow framework `Makefile`/`CMakeLists.txt` dependency tracking.
7. **`cpanfile`** — Perl `cpanfile` dependency extraction (MetaCPAN API).
8. **`cake`** — C# Cake build scripts (`*.cake` / `*.csx`) dependency extraction.
9. **`pixi`** — Pixi `pixi.toml` conda/PyPI package extraction.
9. **`pixi`** — Pixi `pixi.toml` conda package extraction.
9. **`ruby-version`** — `.ruby-version` file version tracking (GitHub Releases).
10. **`conan`** — C/C++ Conan package manager (`conanfile.txt`/`conanfile.py`).
10. **`helm-requirements`** — `requirements.yaml` (Helm v2) chart dependencies.

## Slice 0170 — `matchDepNames` + `matchDatasources` packageRule matchers

### Renovate reference
- `lib/util/package-rules/dep-names.ts` — `DepNameMatcher`
- `lib/util/package-rules/matchers.ts` — matcher chain (AND conditions)
- `lib/config/options/index.ts` — `matchDepNames`, `matchDatasources`

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `PackageRule` struct: added `match_dep_names: Vec<PackageNameMatcher>`,
    `has_dep_name_constraint: bool`, `match_datasources: Vec<String>`.
  - `PackageRule::dep_name_matches()` — mirrors `DepNameMatcher`; returns true
    when `matchDepNames` is not set (no constraint) or when the dep name
    matches at least one exact/regex/glob entry.
  - `PackageRule::datasource_matches()` — returns true when `matchDatasources`
    is empty or contains the provided datasource ID.
  - All `is_dep_ignored*`, `is_update_blocked*`, `is_version_restricted*`,
    `is_version_ignored*` methods updated to AND in `dep_name_matches()`.
  - `RawPackageRule` deserialization updated with `matchDepNames` and
    `matchDatasources` fields.
  - 7 new unit tests covering exact/regex/glob matching, AND logic, and
    empty-list match-all semantics.

### Deferred
- `matchDatasources` is parsed and exposed via `datasource_matches()` but not
  yet wired into the `is_dep_ignored*` / `is_update_blocked*` methods since
  those methods don't currently receive the datasource as a parameter.
  Future slice: add `datasource` parameter to the high-level filter methods
  and wire `datasource_matches()` into the AND chain.
- `packageName` vs `depName` distinction: currently both matchers check against
  the same `dep_name` value since we don't track a separate `packageName` at
  the rule-evaluation boundary.  When Docker/Helm extractors expose distinct
  `packageName` fields, the call sites should pass `packageName` to
  `name_matches()` and `depName` to `dep_name_matches()`.

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1160 passed

## Next slice candidates

1. **Wire `matchDatasources` into filter methods** — add `datasource` parameter
   to `is_dep_ignored_for_manager` and related methods so `datasource_matches()`
   is enforced.
2. **`schedule` config option** — parse the `schedule` string vec from
   `renovate.json`, store in `RepoConfig`, and expose for future PR-timing use.
3. **`groupName` in packageRules** — add `groupName: Option<String>` to
   `PackageRule` for future PR grouping output.
4. **`automerge` + `labels`** — parse metadata fields into `RepoConfig` so
   they appear in the output JSON even if not yet acted upon.
5. **`git-submodules` extractor** — parse `.gitmodules` INI file to extract
   submodule URLs; dispatch via the already-registered `git-submodules` manager.

## Slice 0171 — `RepoConfig` metadata fields: schedule, automerge, labels, groupName, baseBranches, and more

### Renovate reference
- `lib/config/options/index.ts` — `schedule`, `timezone`, `automerge`,
  `automergeType`, `labels`, `addLabels`, `assignees`, `reviewers`,
  `branchPrefix`, `baseBranches`, `prConcurrentLimit`, `prHourlyLimit`,
  `groupName`, `separateMajorMinor`, `separateMinorPatch`, `semanticCommits`
- `lib/config/defaults.ts` — default values for each option

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `RepoConfig` struct: added 16 new fields — `schedule`, `timezone`,
    `automerge`, `automerge_type`, `labels`, `add_labels`, `assignees`,
    `reviewers`, `branch_prefix` (default `"renovate/"`), `base_branches`,
    `pr_concurrent_limit` (default 0), `pr_hourly_limit` (default 2),
    `group_name`, `separate_major_minor` (default true),
    `separate_minor_patch` (default false), `semantic_commits`.
  - `PackageRule` struct: added 4 per-rule metadata fields — `group_name`,
    `automerge`, `schedule`, `labels`.
  - `RawPackageRule` deserialization updated with all new fields.
  - `Raw` deserialization updated with all new repo-level fields.
  - `Default for RepoConfig` updated to match Renovate defaults.
  - 18 new unit tests covering parsing and defaults of all new fields.

### Deferred
- None of the new metadata fields are yet used to drive PR creation or
  filtering.  They are parsed, stored, and exposed, ready for use in the
  PR-creation / output-formatting slice.
- `branchName` / `branchNameStrict` / commit message composition options
  will be a separate slice.
- `extends` preset resolution is a separate slice (requires fetching
  preset configs from GitHub).

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1178 passed

## Next slice candidates

1. **Wire `matchDatasources` into filter methods** — add `datasource` param.
2. **`extends` preset resolution** — fetch and merge `config:recommended`
   and other presets from GitHub into the local `RepoConfig`.
3. **`package.json` `renovate` key** — parse `renovate.json`-equivalent
   config from `package.json` (currently skipped).
4. **Commit message composition** — `branchName`, `commitMessage`,
   `commitMessagePrefix`, `commitMessageAction`, `commitMessageSuffix`.
5. **`prConcurrentLimit` / `prHourlyLimit` enforcement** — count open PRs
   and respect the limits when proposing updates.

## Slice 0172 — `package.json` `renovate` key config discovery

### Renovate reference
- `lib/config/app-strings.ts` — `configFileNames` (includes `package.json`)
- `lib/workers/repository/init/merge.ts` — `detectConfigFile()` checks
  `pJson.renovate` before treating `package.json` as a Renovate config source
- Upstream emits a deprecation warning when `package.json` is the config source

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `RepoConfig::parse_from_package_json(content)` — parses a `package.json`
    string, extracts the `"renovate"` key, and delegates to `parse()`.
    Returns `None` when no `renovate` key exists or JSON is invalid.
  - `discover()` updated: after exhausting `CONFIG_FILE_CANDIDATES`, fetches
    `package.json` and calls `parse_from_package_json`.  If a `renovate` key
    is found, logs a deprecation warning and returns `RepoConfigResult::Found`.
  - Updated `CONFIG_FILE_CANDIDATES` doc comment to note the special handling.
  - Updated existing async tests (`returns_needs_onboarding_*`,
    `returns_not_found_*`) to also mock `package.json` with 404.
  - 4 new async integration tests and 4 unit tests (6 tests total for new code).

### Notes
- Using `package.json` for Renovate config is deprecated upstream.
  The warning nudges users to migrate to a dedicated file.

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1184 passed

## Next slice candidates

1. **Wire `matchDatasources` into filter methods** — add `datasource` param to
   `is_dep_ignored_for_manager` and related methods.
2. **`extends` preset resolution** — fetch `config:recommended` and other
   presets from GitHub and merge them into the local `RepoConfig`.
3. **Commit message composition** — `branchName`, `commitMessage`,
   `commitMessagePrefix`, `commitMessageAction`, `commitMessageSuffix`.
4. **`prConcurrentLimit` / `prHourlyLimit` enforcement** — count open PRs and
   respect the limits when proposing updates.
5. **`git-submodules` extractor** — parse `.gitmodules` INI content to extract
   submodule name/path/URL; dispatch via the already-registered manager.

## Slice 0173 — `git-submodules` `.gitmodules` extractor + dispatch

### Renovate reference
- `lib/modules/manager/git-submodules/extract.ts` — INI parsing + URL normalization
- `lib/modules/manager/git-submodules/index.ts` — defaultConfig: enabled=false,
  datasource: git-refs, versioning: git
- Renovate fixtures: `.gitmodules.{1-8}` in `__fixtures__/`

### What landed
- `crates/renovate-core/src/extractors/git_submodules.rs` (new):
  - `GitSubmoduleDep { name, path, url, branch }` struct
  - `extract(content)` — hand-written INI state machine parser
  - `normalize_url()` — converts SSH `git@host:org/repo` → `https://host/org/repo`,
    strips Azure DevOps user prefix, strips `.git` suffix from HTTPS URLs,
    passes relative paths through unchanged
  - `branch = .` normalized to `None` (Renovate: means "current branch")
  - 11 unit tests covering all fixture scenarios
- `crates/renovate-core/src/extractors.rs`: registered `pub mod git_submodules`
- `crates/renovate-cli/src/main.rs`: dispatches `git-submodules` manager,
  fetches `.gitmodules`, calls extractor, builds dep reports

### Deferred
- `currentDigest` (the submodule's current commit SHA) requires reading the
  git tree via the platform API — deferred to a future slice.
- `branch = .` ("current branch"): requires knowing the repo's default branch —
  deferred.
- Manager is disabled by default in Renovate; our `is_manager_enabled` check
  doesn't yet honour per-manager default configs.  Future: add a
  `disabled_by_default` field to `ManagerDef`.

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1195 passed

## Next slice candidates

1. **`currentDigest` for git-submodules** — use GitHub Trees API to get
   submodule commit SHAs.
2. **Wire `matchDatasources` into filter methods** — add `datasource` param.
3. **`extends` preset resolution** — fetch `config:recommended` and merge.
4. **Commit message composition** — `branchName`, `commitMessage` templates.
5. **`prConcurrentLimit` / `prHourlyLimit` enforcement**.

## Slice 0174 — `disabled_by_default` manager flag + `is_manager_enabled` fix

### Renovate reference
- `lib/modules/manager/azure-pipelines/index.ts` — `defaultConfig.enabled: false`
- `lib/modules/manager/git-submodules/index.ts` — `defaultConfig.enabled: false`
- `lib/modules/manager/html/index.ts` — `defaultConfig.enabled: false`
- `lib/modules/manager/nix/index.ts` — `defaultConfig.enabled: false`
- `lib/modules/manager/pre-commit/index.ts` — `defaultConfig.enabled: false`
- `lib/modules/manager/travis/index.ts` — `defaultConfig.enabled: false`

### Bug fixed
Previously `is_manager_enabled` returned `true` when `enabledManagers` was
empty, causing disabled-by-default managers (git-submodules, html, nix,
pre-commit, travis, azure-pipelines) to run for every repository.  These
managers should only run when explicitly listed in `enabledManagers`.

### What landed
- `crates/renovate-core/src/managers.rs`:
  - `DISABLED_BY_DEFAULT: &[&str]` — static list of opt-in-only manager names
  - `is_disabled_by_default(name) -> bool` — public lookup function
- `crates/renovate-core/src/repo_config.rs`:
  - `is_manager_enabled(name, disabled_by_default) -> bool` — updated signature;
    when `enabledManagers` is empty, disabled-by-default managers return false
- `crates/renovate-cli/src/main.rs`: updated the single filter call site to
  pass `is_disabled_by_default(m.name)` as the second argument
- 5 new tests covering disabled-by-default semantics (3 scenario tests)

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1198 passed

## Next slice candidates

1. **`currentDigest` for git-submodules** — GitHub Trees API to get SHA.
2. **Wire `matchDatasources` into filter methods**.
3. **`extends` preset resolution** — `config:recommended` and built-in presets.
4. **Commit message / branch name composition** — template evaluation.
5. **`separateMajorMinor` split behavior** — report major vs minor/patch separately.

## Slice 0175 — `extends` preset parsing + built-in expansion

### Renovate reference
- `lib/config/presets/internal/config.preset.ts` — `config:recommended`,
  `config:base`, `config:best-practices`
- `lib/config/presets/internal/default.preset.ts` — `:ignoreModulesAndTests`,
  `:semanticCommits`, `:semanticCommitsDisabled`
- `lib/config/options/index.ts` — `extends`

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `RepoConfig.extends: Vec<String>` — stores the raw extends array
  - `Raw.extends: Vec<String>` — deserialized from renovate.json
  - `resolve_extends_ignore_paths(extends)` — expands known presets to
    `ignorePaths`: handles `:ignoreModulesAndTests`, `config:recommended`,
    `config:base`, `config:best-practices`; deduplicates; unknown presets logged
  - `parse()` updated: preset paths prepended to user-configured `ignorePaths`;
    `:semanticCommits` / `:semanticCommitsDisabled` sets `semanticCommits`
    when not explicitly overridden
  - `Default::default()`: extends = vec![]
  - 10 new unit tests

### Preset effects implemented
- `config:recommended`, `config:base`, `config:best-practices`,
  `:ignoreModulesAndTests` → adds 8 ignore paths: `**/node_modules/**`,
  `**/bower_components/**`, `**/vendor/**`, `**/examples/**`,
  `**/__tests__/**`, `**/test/**`, `**/tests/**`, `**/__fixtures__/**`
- `:semanticCommits` → `semanticCommits = "enabled"`
- `:semanticCommitsDisabled` → `semanticCommits = "disabled"`

### Deferred
- Remote presets (`github>org/repo`, `local>path`, `npm>package`) require
  network fetching — future slice.
- Most built-in presets beyond ignore paths / semantic commits are not yet
  expanded (group:monorepos, workarounds:all, etc.) — future slices.

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1208 passed

## Next slice candidates

1. **Remote preset resolution** — `github>org/repo//preset` fetching.
2. **More built-in preset expansion** — group:monorepos, schedule presets.
3. **`currentDigest` for git-submodules** — GitHub Trees API.
4. **Wire `matchDatasources` into filter methods**.
5. **Commit message / branch name composition** — template evaluation.

## Slice 0176 — `matchSourceUrls` + `matchCurrentValue` + `matchNewValue` packageRule matchers

### Renovate reference
- `lib/util/package-rules/sourceurls.ts` — `SourceUrlsMatcher`
- `lib/util/package-rules/current-value.ts` — `CurrentValueMatcher`
- `lib/util/package-rules/new-value.ts` — `NewValueMatcher`
- `lib/config/options/index.ts` — option definitions

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `PackageRule` struct: added `match_source_urls: Vec<PackageNameMatcher>`,
    `has_source_url_constraint: bool`, `match_current_value: Option<PackageNameMatcher>`,
    `match_new_value: Option<PackageNameMatcher>`
  - `PackageRule::source_url_matches(source_url)` — same exact/regex/glob logic
    as `dep_name_matches`; matches against the dep's source URL
  - `PackageRule::current_value_matches(current_value)` — single-pattern match
    against the raw version string in the manifest
  - `PackageRule::new_value_matches(new_value)` — single-pattern match against
    the proposed new version string
  - `RawPackageRule` deserialization extended
  - `PackageRule` construction updated
  - 9 new unit tests (3 scenarios × 3 matchers)

### Deferred
- `matchSourceUrls` is not yet wired into `is_dep_ignored*` / `is_update_blocked*`
  methods — those don't currently receive `sourceUrl` as a parameter.
- `matchCurrentValue` and `matchNewValue` are not yet called at the update-
  proposal level since we don't thread `currentValue`/`newValue` through the
  high-level filter functions.
- Future slice: add `DepFilterContext { source_url, current_value, new_value }`
  and wire all three into the filter chain.

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1217 passed

## Next slice candidates

1. **`DepFilterContext` struct** — bundle `source_url`, `current_value`, `new_value`
   into a context struct so all matchers can be wired into filter methods.
2. **Remote preset resolution** — `github>org/repo//preset` fetching.
3. **More built-in preset expansion** — group:monorepos, schedule presets.
4. **`currentDigest` for git-submodules** — GitHub Trees API.
5. **Commit message / branch name composition** — template evaluation.

## Slice 0177 — Branch name generation: `sanitize_dep_name` + `branch_topic` + `branch_name`

### Renovate reference
- `lib/workers/repository/updates/flatten.ts` — `sanitizeDepName()`
- `lib/workers/repository/updates/branch-name.ts` — `generateBranchName()`,
  `cleanBranchName()`
- `lib/config/options/index.ts` — `branchTopic` default template, `branchName`

### What landed
- `crates/renovate-core/src/branch.rs` (new module):
  - `sanitize_dep_name(name)` — strips `@types/`, `@`, replaces `/` `:` with
    `-`, collapses consecutive dashes, lowercases
  - `branch_topic(dep, major, minor, is_patch, separate_minor_patch)` — computes
    the default `{depSanitized}-{major}.x` topic (or `{major}.{minor}.x` when
    `separateMinorPatch=true` and `is_patch=true`)
  - `branch_name(prefix, additional_prefix, topic)` — concatenates to
    `{prefix}{additional}{topic}` and cleans invalid git ref chars
  - 25 unit tests covering all behaviors
- `crates/renovate-core/src/lib.rs`: registered `pub mod branch`
- All doctest examples pass

### Deferred
- Grouped updates (`groupName` / `groupSlug`) use a different branch naming
  path — deferred to a later slice.
- Full Handlebars template evaluation for custom `branchTopic` / `branchName`
  strings — deferred; current implementation handles the default template.
- Integration into the output `DepReport` — branch name is now computable but
  not yet added to the JSON/terminal output.

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1234 passed

## Next slice candidates

1. **Add `branch_name` to `DepReport` output** — show proposed branch names.
2. **DepFilterContext struct** — thread source_url, current_value, new_value
   through the filter chain so matchSourceUrls etc. are enforced.
3. **Remote preset resolution** — `github>org/repo//preset` fetching.
4. **`currentDigest` for git-submodules** — GitHub Trees API.
5. **More built-in preset expansion** — schedule presets, group:monorepos.

## Slice 0184 — Pass manager context to all dep-ignore call sites in main.rs

### What was implemented
- Upgraded all 72 `repo_cfg.is_dep_ignored(&d.name)` call sites in `main.rs` to `is_dep_ignored_for_manager(&d.name, "<manager>")` using an automated Python script
- Each of the 80+ manager sections now passes the correct manager string when filtering deps, so `matchManagers` and `matchCategories` packageRule matchers fire correctly in production
- Managers covered: cargo, pub, nuget, composer, bun, pip-compile, pip_setup, setup-cfg, homeassistant-manifest, html, cdnurl, typst, cpanfile, pipenv, pep621, poetry, gomod, ant, maven, kotlin-script, osgi, github-actions, gemspec, terragrunt, tflint-plugin, fleet, pre-commit, ansible-galaxy, asdf, bazel-module, bicep, buildkite, nix, meteor, cake, conan, haskell-cabal, jsonnet-bundler, vendir, xcodegen, puppet, deps-edn, leiningen, bitrise, pixi, kubernetes, bazel, tekton, argocd, homebrew, helmsman, buildpacks, glasskube, renovate-config-presets, pep723, hermit

---

## Slice 0183 — `DepContext` unified matcher + `PackageRule::matches_context()`

### Renovate reference
- `lib/util/package-rules/index.ts` — `matchesRule(inputConfig, packageRule)` iterating all matchers
- `lib/config/types.ts` — `PackageRuleInputConfig`

### What was implemented
- `DepContext<'a>` struct in `repo_config.rs` — carries all matching context (dep_name, package_name, manager, datasource, dep_type, file_path, source_url, registry_urls, repository, base_branch, current_value, new_value, update_type)
- `DepContext::for_dep()` convenience constructor + builder methods (`with_manager`, `with_datasource`, `with_dep_type`, `with_file_path`)
- `PackageRule::matches_context(ctx: &DepContext)` — unified entry point calling ALL matchers in correct AND-chain, following upstream semantics (missing context field + rule constraint = no match)
- `RepoConfig::is_dep_ignored_ctx(ctx: &DepContext)` — full-context ignore check
- Rewrote `is_dep_ignored`, `is_dep_ignored_for_manager`, `is_dep_ignored_with_dep_type` as wrappers delegating to `is_dep_ignored_ctx`
- Rewrote `is_update_blocked_for_file` and `is_version_restricted_for_file` to use `matches_context`
- **Behavior fix**: a rule with `matchManagers: ["npm"]` no longer fires when called via `is_dep_ignored(name)` without manager context (upstream-correct; old tests updated)
- Added `manager_categories` import to `repo_config.rs` so category derivation happens inside `matches_context`

### Tests added (5 new)
- `dep_context_tests::dep_context_with_manager_fires_correct_rule`
- `dep_context_tests::dep_context_datasource_gates_rule`
- `dep_context_tests::dep_context_categories_from_manager`
- `dep_context_tests::dep_context_repository_gates_rule`
- `dep_context_tests::dep_context_builder_methods`

### Deferred
- Update `main.rs` call sites to pass richer `DepContext` (datasource, registry URLs, repository, base branch) for more accurate filtering

---

## Slice 0182 — `matchRegistryUrls` + `matchRepositories` packageRule matchers

### Renovate reference
- `lib/util/package-rules/registryurls.ts` — `matchRegistryUrls`
- `lib/util/package-rules/repositories.ts` — `matchRepositories`

### What was implemented
- `PackageRule::match_registry_urls: Vec<PackageNameMatcher>` + `has_registry_url_constraint` flag
- `PackageRule::registry_url_matches(urls: &[&str]) -> bool` — any dep URL matches any pattern (ANY-of-any semantics), empty = matches all, no URLs with constraint = false
- `PackageRule::match_repositories: Vec<PackageNameMatcher>` + `has_repository_constraint` flag
- `PackageRule::repository_matches(repo: &str) -> bool` — single repo name matched against exact/regex/glob, empty = matches all
- `RawPackageRule` serde fields for `matchRegistryUrls` and `matchRepositories`

### Tests added (9 new)
- `registry_url_repository_tests::match_registry_urls_exact_hit`
- `registry_url_repository_tests::match_registry_urls_any_of_dep_urls`
- `registry_url_repository_tests::match_registry_urls_glob`
- `registry_url_repository_tests::match_registry_urls_empty_matches_all`
- `registry_url_repository_tests::match_registry_urls_no_dep_urls_fails_when_constraint_set`
- `registry_url_repository_tests::match_repositories_exact_hit`
- `registry_url_repository_tests::match_repositories_glob`
- `registry_url_repository_tests::match_repositories_regex`
- `registry_url_repository_tests::match_repositories_empty_matches_all`

### Deferred
- Wiring `registry_url_matches` and `repository_matches` into the CLI dep-report filtering (requires registry URL and repository context through the pipeline)

---

## Slice 0181 — `matchCategories` + `matchBaseBranches` packageRule matchers

### Renovate reference
- `lib/util/package-rules/categories.ts` — `matchCategories`
- `lib/util/package-rules/base-branch.ts` — `matchBaseBranches`
- `lib/modules/manager/*/index.ts` — each manager exports a `categories` array

### What was implemented
- `PackageRule::match_categories: Vec<String>` field (from `matchCategories` in JSON)
- `PackageRule::categories_match(categories: &[&str]) -> bool` — empty = matches all; non-empty = dep's manager must belong to at least one listed category
- `PackageRule::match_base_branches: Vec<String>` field (from `matchBaseBranches` in JSON)
- `PackageRule::base_branch_matches(branch: &str) -> bool` — empty = matches all; supports exact strings and glob patterns (`release/*`, `feature/**`)
- `RawPackageRule` deserialization extended with both new fields
- `PackageRule` construction in `parse()` passes both through

### Tests added (7 new)
- `categories_base_branch_tests::match_categories_exact_hit`
- `categories_base_branch_tests::match_categories_any_of_many`
- `categories_base_branch_tests::match_categories_empty_matches_all`
- `categories_base_branch_tests::match_base_branches_exact_hit`
- `categories_base_branch_tests::match_base_branches_glob`
- `categories_base_branch_tests::match_base_branches_empty_matches_all`
- `categories_base_branch_tests::match_base_branches_multiple_entries`

### Deferred
- Wiring `categories_match` and `base_branch_matches` into the CLI dep-report filtering phase (requires propagating manager name → categories and current base branch through the pipeline)

---

## Slice 0180 — `manager_categories()` lookup table

### Renovate reference
- `lib/modules/manager/*/index.ts` — each manager's `categories` export

### What was implemented
- `pub fn manager_categories(manager_name: &str) -> &'static [&'static str]` in `managers.rs`
- 27 ecosystem categories mapped: js, python, java, golang, rust, ruby, php, dotnet, docker, kubernetes, terraform/iac, ci, dart, swift, haskell, elixir, perl, ansible, bazel, nix

---

## Slice 0178 — Add `branchName` field to `DepReport` output

### Renovate reference
- `lib/workers/repository/updates/branch-name.ts` — `generateBranchName()`
- `lib/config/options/index.ts` — `branchName`, `branchTopic` defaults

### What landed
- `crates/renovate-cli/src/output.rs`:
  - `DepReport` struct: added `branch_name: Option<String>` with serde rename
    `"branchName"`, skipped when `None`
  - `DepReport::new(name, status)` convenience constructor (branch_name=None)
- `crates/renovate-cli/src/main.rs`:
  - 145 `output::DepReport { ... }` literals updated with `branch_name: None,`
  - Post-processing loop (`apply_package_rules_to_report`): computes branch name
    for all remaining `UpdateAvailable` deps using `branch::branch_topic()` and
    `branch::branch_name()` with `parse_padded(latest)` for version decomposition
    and `repo_cfg.separate_minor_patch` / `repo_cfg.branch_prefix`

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo nextest run --workspace --all-features`: 1234 passed

## Next slice candidates

1. **`renovate-test-map.md`** — build and maintain the Renovate↔Rust test map.
2. **DepFilterContext** — thread source_url, current_value, new_value.
3. **Remote preset resolution** — `github>org/repo//preset` fetching.
4. **`currentDigest` for git-submodules** — GitHub Trees API.
5. **More built-in preset expansion** — schedule presets, group:monorepos.

## Slice 0180 — `schedule:*` and `:automerge*` preset expansion

### Renovate reference
- `lib/config/presets/internal/schedule.preset.ts` — all `schedule:*` presets
- `lib/config/presets/internal/default.preset.ts` — `:automergeAll`, `:automergeMinor`, etc.

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - `resolve_extends_schedule(extends)` — maps `schedule:daily`, `schedule:weekly`,
    `schedule:monthly`, `schedule:earlyMondays`, `schedule:nonOfficeHours`,
    `schedule:officeHours`, `schedule:quarterly`, `schedule:weekdays`,
    `schedule:weekends`, `schedule:yearly` to cron string arrays; last matching
    preset wins; user-configured `schedule` takes precedence
  - `resolve_extends_automerge(extends)` — maps `:automergeAll`, `:automergeMinor`,
    `:automergeMajor`, `:automergeBranch`, `:automergePr`, `:autoMerge` to
    `automerge: true`; `:automergeDisabled` to `false`
  - Both functions wired into `parse()` alongside existing `semantic_commits` and
    `ignore_paths` preset resolution
  - 15 new unit tests

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo fmt --all --check` ✓
- `cargo clippy --workspace --all-features -D warnings` ✓
- `cargo nextest run --workspace --all-features`: 1249 passed

## Slice 0193 - `--platform=local` fix: scan CWD, skip token requirement, LocalClient

### Renovate reference
- `lib/modules/platform/local/index.ts` — local platform reads from filesystem

### What landed
- `crates/renovate-core/src/platform/local.rs` (new):
  - `LocalClient { base_dir }` struct
  - `get_file_list()` — runs `git ls-files --cached --others --exclude-standard`;
    falls back to recursive `walk_dir()` skipping `.git`, `target`, `node_modules`
  - `get_raw_file()` — reads from `base_dir/path` via `std::fs::read_to_string`
  - `get_current_user()` — returns `CurrentUser { login: "local" }`
- `crates/renovate-core/src/platform.rs`:
  - Added `Local(LocalClient)` variant to `AnyPlatformClient`
  - Added `AnyPlatformClient::local(base_dir)` constructor
  - Dispatch methods include Local arm
- `crates/renovate-cli/src/main.rs`:
  - When `platform == Local && repositories.is_empty()` → inject `"local/{dirname}"`
  - When `platform == Local` → skip token requirement; create `AnyPlatformClient::local(&cwd)`
  - Display slug uses CWD basename for readable output

### Verification
- `renovate --platform=local --dry-run=full` correctly scans CWD and shows deps

---

## Slice 0194 - `matchCurrentAge` packageRule matcher

### Renovate reference
- `lib/util/package-rules/current-age.ts` — `CurrentAgeMatcher`
- `lib/util/pretty-time.ts` — `satisfiesDateRange`

### What landed
- `crates/renovate-core/src/schedule.rs`:
  - `satisfies_date_range(timestamp, range)` — evaluates `"> 3 days"`,
    `"< 1 month"`, `">= 2 weeks"` etc. against an ISO 8601 timestamp
  - 9 tests for the function (operators, edge cases, naive timestamps)
- `crates/renovate-core/src/repo_config.rs`:
  - `match_current_age: Option<String>` field on `PackageRule`
  - `current_version_timestamp: Option<&'a str>` field on `DepContext`
  - `PackageRule::current_age_matches(timestamp)` method
  - Wired into `matches_context()` AND-chain
  - JSON parsing: `matchCurrentAge` deserialized from config
  - 6 tests: parse, no-constraint match-all, with/without timestamp, DepContext integration
- `crates/renovate-core/src/datasources/npm.rs`:
  - `version_timestamps: HashMap<String, String>` added to `NpmVersionsEntry`
  - Populated from packument `time` field (filtering meta-keys)
- `crates/renovate-core/src/datasources/pypi.rs`:
  - `version_timestamps: HashMap<String, String>` added to `PypiVersionsEntry`
  - Collected per-version from `releases[v][].upload_time` during fetch
- `crates/renovate-cli/src/output.rs`:
  - `current_version_timestamp: Option<String>` added to `DepReport` (not serialized)
- Wired `dep.current_version_timestamp.as_deref()` into `DepContext` in
  `apply_update_blocking_to_report`

### Deferred
- Populating `current_version_timestamp` in each dep report from the cached
  timestamps (requires resolving the current version string from specifier first)

---

## Slice 0195 - Parity tracking: source map + prompt rules

### What landed
- `docs/parity/renovate-source-map.md` (new): maps 60+ TypeScript source files
  to their Rust counterparts with `full/partial/stub/not-started/out-of-scope`
  status; covers CLI, platform, datasources, extractors, versioning, utilities
- `docs/parity/renovate-test-map.md`: added 15 new rows for `matchCurrentAge`
  tests (current-age.ts, pretty-time.ts) and LocalClient integration test
- `prompts/claude-loop-renovate-rust.md`: added explicit rules for maintaining
  all four parity tracking files in every loop iteration; added rule that prompt
  changes must be committed separately

---

---

## Slice 0199 - `matchManagers` glob/regex/negation + custom manager prefix

### What landed
- `crates/renovate-core/src/string_match.rs` (new): `match_regex_or_glob` and
  `match_regex_or_glob_list` utilities mirroring Renovate's `lib/util/string-match.ts`.
  Supports `/regex/flags` literals (with `(?i)` embedding), glob patterns
  (`*`, `?`, `**`, `{a,b}`), and negation patterns (`!pattern`).
  13 unit tests.
- `crates/renovate-core/src/repo_config.rs`: `PackageRule::manager_matches`
  rewritten to use `match_regex_or_glob_list`.  Custom managers (`"regex"`,
  `"jsonata"`) are matched as `"custom.<id>"` per Renovate convention.
  4 new integration tests: glob, `/regex/`, negation, custom prefix.
- `crates/renovate-core/src/lib.rs`: `pub mod string_match` registered.

### Deferred
- Applying `match_regex_or_glob_list` to `matchDepTypes` and `matchDatasources`
  (currently exact-only; glob/negation on those is rare in practice).

### Renovate reference
- `lib/util/package-rules/managers.ts`
- `lib/util/string-match.ts`

---

---

## Slice 0213 - Per-rule `schedule` and `minimumReleaseAge` in `packageRules`

### Renovate reference
- `lib/config/options/index.ts` — `schedule` and `minimumReleaseAge` in `packageRules`
- `lib/util/package-rules/index.ts` — rule application merges schedule + minimumReleaseAge

### What landed
- `PackageRule`: added `minimum_release_age: Option<String>` (serde: `minimumReleaseAge`)
- `RuleEffects`: added `minimum_release_age: Option<String>` — collected by `collect_rule_effects`;
  the last matching rule that sets it wins (same semantics as `schedule`)
- `repo_config.rs::collect_rule_effects`: propagates `minimum_release_age` from matching rules
- `pipeline_utils.rs::apply_update_blocking_to_report`: two new checks after `collect_rule_effects`:
  1. **Per-rule schedule**: if `effects.schedule` is non-empty and we're outside the window →
     skip with reason `"outside schedule window (packageRule)"`
  2. **Effective minimumReleaseAge**: uses per-rule override when set, falls back to global;
     replaces the old pre-effects global-only check so both code paths are unified
- 6 new unit tests covering: schedule collected for matching dep, not-collected for non-matching,
  per-rule minimumReleaseAge parsed/collected, last-rule-wins semantics

### Behavior notes
- Per-rule schedule is checked AFTER `collect_rule_effects` so it only fires when the rule
  actually matches the dep (correct behavior — matches Renovate's evaluation order)
- `minimumReleaseAge` now supports both global and per-rule values; per-rule takes precedence

### Verification
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓
- `cargo nextest run --workspace --all-features` → 1397 tests pass (6 new)

---

## Slice 0212 - `hashedBranchLength` — SHA-512 branch name hashing

### Renovate reference
- `lib/config/options/index.ts` — `hashedBranchLength` (integer, default `null`)
- `lib/workers/repository/updates/branch-name.ts` — hash logic, `MIN_HASH_LENGTH = 6`
- `lib/util/hash.ts` — SHA-512 via Node `crypto`

### What landed
- `crates/renovate-core/src/repo_config.rs`:
  - Added `hashed_branch_length: Option<u32>` field with serde rename `hashedBranchLength`
- `crates/renovate-core/src/branch.rs`:
  - Added `sha2` import (`Sha512`, `Digest`)
  - Implemented `hashed_branch_name(prefix, additional_prefix, topic, length) -> String`
    - Computes `hash_len = length - len(prefix)`, floors at `MIN_HASH_LENGTH = 6`
    - Hash input = `additionalBranchPrefix + branchTopic` (matching Renovate's template)
    - SHA-512 digest encoded as lowercase hex, truncated to `hash_len` chars
    - Result: `prefix + hex[..hash_len]`, always exactly `length` chars (or MIN fallback)
  - 6 unit tests: exact length, different topics differ, min-length fallback, determinism,
    additional prefix changes hash, hex-only output
- `crates/renovate-cli/src/pipeline_utils.rs`:
  - `apply_update_blocking_to_report`: uses `hashed_branch_name` when
    `repo_cfg.hashed_branch_length.is_some()`, falls back to `branch_name` otherwise
- `crates/renovate-core/Cargo.toml`: added `sha2 = "0.11.0"`

### What was deferred
- Hashing for grouped updates (groupName branch slug) — same path, no separate case needed.

### Verification
- `cargo build --workspace --all-features` ✓
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓
- `cargo nextest run --workspace --all-features` → 1391 tests pass (6 new)

---

## Slice 0211 - Refactor: split managers into focused `pipelines/` sub-modules

### What landed
- `managers_impl.rs` (8,387 lines) deleted.
- `pipelines.rs` (new, 103 lines): flat module entry point declaring all 17 sub-modules;
  re-exports all shared imports (`pub(crate) use`) so sub-modules can do `use super::*`.
- `pipelines/` directory with 17 focused sub-module files:

| File | Lines | Ecosystems covered |
|------|-------|--------------------|
| `rust.rs` | 102 | Cargo |
| `php.rs` | 72 | Composer |
| `dotnet.rs` | 178 | NuGet, Cake |
| `dart.rs` | 125 | pub/pubspec, FVM |
| `go.rs` | 94 | Go modules |
| `ruby.rs` | 232 | Bundler, gemspec, CocoaPods |
| `docker.rs` | 279 | Dockerfile, docker-compose, Dev Container, Quadlet |
| `javascript.rs` | 388 | npm, Bun, Meteor, HTML CDN, CDN URLs |
| `terraform.rs` | 348 | Terraform, Terragrunt, TFLint, Azure Bicep |
| `helm.rs` | 391 | Helm, Helm Values, Helmfile, Helmsman, Fleet |
| `mobile.rs` | 500 | Swift, Mint, XcodeGen, Mix (Elixir), Gleam |
| `version_files.rs` | 488 | asdf, mise, version files, Devbox |
| `python.rs` | 727 | pip, pip-compile, setup.py/cfg, Pipfile, pep621, Poetry, PEP 723, Pixi |
| `kubernetes.rs` | 782 | Kustomize, K8s manifests, FluxCD, Tekton, ArgoCD, Crossplane, Glasskube, Sveltos |
| `jvm.rs` | 840 | Maven, Gradle, Kotlin Script, Ant, SBT, OSGi, Scalafmt, Clojure, Leiningen |
| `ci.rs` | 887 | GitHub Actions + 13 CI/CD platforms |
| `misc.rs` | 2,188 | Bazel, Nix, pre-commit, Ansible, Puppet, Jenkins, Conan, Homebrew, and 16 more |

- Module renamed from `managers_impl` to `pipelines` — cleaner, idiomatic Rust naming.
- Each sub-module uses `use super::*` to get the shared imports from `pipelines.rs`.
- SETUP in each sub-module is minimal: only includes `gh_http`/`gh_api_base` if the
  module actually uses GitHub API calls; `filtered_files` only in `misc.rs` (Hermit).
- `context.rs` updated: all originally-shared variables still accessible via `ctx`.

### No behavior changes
All 1385 tests pass. External API unchanged.

---

## Slice 0210 - Refactor: extract `managers_impl.rs` + `context.rs`; main.rs 8,733→389 lines

### What landed
- `crates/renovate-cli/src/context.rs` (new, 33 lines): `RepoPipelineCtx<'a>`
  struct capturing all shared state for the manager pipeline — immutable borrows
  (`client`, `http`, `config`, `owner`, `repo`, `repo_slug`, `repo_cfg`,
  `detected`, `filtered_files`) plus owned mutable state (`report`, `had_error`).
- `crates/renovate-cli/src/managers_impl.rs` (new, 8,387 lines): all 70+
  manager pipeline blocks extracted from `process_repo()` into a single
  `process_all_managers(ctx: &mut RepoPipelineCtx<'_>)` function. Local variable
  bindings are created at the top so every manager block is syntactically
  identical to its original form; only `repo_report.files.push` →
  `ctx.report.files.push` and `had_error = true` → `ctx.had_error = true`
  were substituted.
- `crates/renovate-cli/src/main.rs`: 8,733 → 389 lines (-8,344 lines).
  `process_repo()` now creates the context struct and delegates to
  `managers_impl::process_all_managers()`.

### No behavior changes
All 1385 tests continue to pass. External API is unchanged.

### Next steps (deferred)
- Split `managers_impl.rs` into per-ecosystem sub-modules
  (`python.rs`, `jvm.rs`, `ci.rs`, `container.rs`, etc.) for finer-grained
  ownership. This is safe to do incrementally.

---

## Slice 0201 - Refactor: split large files into focused modules

### What landed
- `crates/renovate-cli/src/report_builders.rs` (new, 948 lines): all
  `build_dep_reports_*` functions extracted from `main.rs` — 16 functions
  covering cargo, npm, github-actions, maven, pub, nuget, composer, gomod,
  poetry, pip, bundler, terraform, helm, gradle, setup-cfg, pipenv.
- `crates/renovate-cli/src/pipeline_utils.rs` (new, 236 lines):
  `apply_update_blocking_to_report`, `apply_version_ignore_to_report`,
  `manager_files`, `docker_hub_reports` extracted from `main.rs`.
- `crates/renovate-core/src/package_rule.rs` (new, 708 lines):
  `PackageRule`, `PackageNameMatcher`, `DepContext`, `PathMatcher`,
  `RuleEffects`, `compile_name_matcher`, `version_matches_ignore_list`,
  and all `impl PackageRule` matcher methods extracted from `repo_config.rs`.
- `crates/renovate-cli/src/main.rs`: 9,885 → 8,726 lines (-1,159)
- `crates/renovate-core/src/repo_config.rs`: 3,673 → 2,882 lines (-791)

### No behavior changes
All 1363 tests continue to pass.  External API is unchanged: types are
re-exported from `repo_config` for backward compatibility.

---

## Next slice candidates

1. **Populate `current_version_timestamp` from npm/pypi cache** — resolve specifier
   to current version string and look up in `version_timestamps`.
2. **crates.io release timestamp** — hit `crates.io/api/v1/crates/{name}/versions`
   to get `created_at` per version for minimumReleaseAge support.
3. **Remote preset resolution** — `github>org/repo//preset` fetching.
4. **Docker versioning scheme** — proper Docker tag version comparison.
5. **Split `process_repo()` in `main.rs`** — further refactor the 8,440-line function.
