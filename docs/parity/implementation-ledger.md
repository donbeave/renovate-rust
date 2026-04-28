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
| 0003  | 2026-04-28 | Logger init (LOG_LEVEL, LOG_FORMAT, NO_COLOR) | Complete | See below. |
| 0002  | 2026-04-28 | `migrateArgs` parity           | Complete | See below. |
| 0001  | 2026-04-28 | Workspace + early CLI flags    | Complete | See below. |

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
