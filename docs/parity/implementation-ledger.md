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

## Next slice candidates

Pick whichever can be completed in one loop:

1. **Renovate option surface (first cut)**: port the option definitions
   from `lib/config/options/index.ts` into a strongly-typed Rust schema
   and wire them into clap. Will likely need to be split across two
   slices because the option list is large; start with the small set of
   flags `migrateArgs` already produces (`--dry-run`, `--require-config`,
   `--platform-automerge`, `--fork-processing`, `--recreate-when`,
   `--trust-level`, `--host-rules`, `--registry-aliases`,
   `--allowed-commands`, `--allow-command-templating`) so the migration
   wiring becomes end-to-end testable.
2. **Logger init + log levels**: wire `tracing-subscriber` with
   `LOG_LEVEL` env support and Renovate's level names (`fatal`, `error`,
   `warn`, `info`, `debug`, `trace`).
3. **Config file discovery**: port the `config.js`/`.renovaterc(.json)`
   discovery rules from `lib/workers/global/config/parse/file.ts`.
4. **`coersions` parity**: port the type coercions from
   `lib/workers/global/config/parse/coersions.ts` (string, integer,
   boolean, list, object, json) - feeds option-surface work.
