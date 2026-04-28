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

1. **Renovate option surface (first cut)**: port the option definitions
   from `lib/config/options/index.ts` into a strongly-typed Rust schema
   and wire them into clap.
2. **`gemspec` extractor**: extend bundler manager to parse `.gemspec` files.
3. **CocoaPods** (`Podfile` extractor + CocoaPods trunk datasource).
4. **`semver` versioning module**: improve update decisions for pub.dev, NuGet,
   and composer using proper semver range matching (uses existing `semver` crate).
