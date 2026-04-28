# Compatibility Decisions

Each entry documents a deliberate decision about how the Rust CLI matches,
or intentionally diverges from, `renovatebot/renovate`. Decisions are
append-only; if a decision is reversed, add a new entry rather than editing
the old one, so the trail stays auditable.

Format per entry:

> **CD-NNNN** - Title
> - **Date**: YYYY-MM-DD
> - **Renovate behavior**: how upstream does it.
> - **Rust behavior**: what we do.
> - **Reason**: why.
> - **Compatibility**: matched / opt-in divergence / hard divergence.
> - **References**: file paths, links.

## CD-0001 - `--version` prints a bare version line

- **Date**: 2026-04-28
- **Renovate behavior**: Commander binds `-v, --version` and prints just
  the version string followed by a newline (e.g. `0.0.0\n`), then exits 0.
- **Rust behavior**: clap is configured with `disable_version_flag = true`
  and we hand-roll a `-v`/`--version` boolean that prints
  `{CARGO_PKG_VERSION}\n` and returns `ExitCode::SUCCESS`.
- **Reason**: clap's default uses `-V` (uppercase) and prints
  `<bin> <version>`. Both differ observably from Renovate; downstream
  tooling that greps `renovate --version` would break.
- **Compatibility**: matched.
- **References**:
  - Renovate: `lib/workers/global/config/parse/cli.ts` (`.version(pkg.version, '-v, --version')`).
  - Rust: `crates/renovate-cli/src/main.rs`, `crates/renovate-cli/tests/cli.rs`.

## CD-0002 - Unknown flags exit with status 2

- **Date**: 2026-04-28
- **Renovate behavior**: Commander exits with status 1 by default for
  unknown options, but `parseEarlyFlags` calls `.allowUnknownOption()`,
  which means `renovate --bogus` does **not** error during early parsing
  in Renovate; the bogus flag is forwarded to the main config parser,
  where it's rejected later.
- **Rust behavior**: clap's standard usage-error path runs unconditionally
  for unknown flags during early parsing, exiting with status 2.
- **Reason**: We have no main config parser yet. Erroring early is
  preferable to silently accepting unknown flags during slice 1. Once the
  full option surface lands (next-slice candidate 1), we will revisit
  whether to mirror Renovate's `allowUnknownOption()` behavior during the
  early-flag pass.
- **Compatibility**: opt-in divergence (status code differs by 1, behavior
  is stricter, not looser). Tracked as a follow-up to revisit once option
  parsing exists.
- **References**:
  - Renovate: `lib/workers/global/config/parse/cli.ts` (`.allowUnknownOption()` in `parseEarlyFlags`).
  - Rust: `crates/renovate-cli/src/main.rs`, `crates/renovate-cli/tests/cli.rs`.
