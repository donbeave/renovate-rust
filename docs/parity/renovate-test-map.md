# Renovate Test Map

Maps Renovate (TypeScript / vitest) tests to the equivalent Rust tests in
this workspace. Use this file to (a) check whether a parity behavior already
has Rust coverage and (b) plan which Renovate tests to port next.

Format per row:

| Renovate test file | Renovate `describe` / `it` | Rust test location | Status |

`Status` is one of: `ported`, `partial`, `pending`, `not-applicable`.

## CLI

| Renovate test file | Renovate `describe` / `it` | Rust test location | Status |
|--------------------|----------------------------|--------------------|--------|
| `lib/workers/global/config/parse/cli.spec.ts` | `'--version'` shows the version | `crates/renovate-cli/tests/cli.rs::version_long_flag_prints_bare_version`, `version_short_flag_matches_long_flag` | ported |
| `lib/workers/global/config/parse/cli.spec.ts` | `'--help'` shows help text | `crates/renovate-cli/tests/cli.rs::help_flag_succeeds_and_mentions_repositories` | partial (just exit/keyword smoke) |
| `lib/workers/global/config/parse/cli.spec.ts` | `migrateArgs` rewrites for legacy flags | _not yet ported_ | pending |
| `lib/workers/global/config/parse/cli.spec.ts` | `parseEarlyFlags` ignores unknown flags | _not yet ported_ | pending (see CD-0002) |

## Config

_None ported yet._

## Datasources

_None ported yet._

## Versioning

_None ported yet._

## Managers

_None ported yet._
