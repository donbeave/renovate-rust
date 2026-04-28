You are working on the renovate-rust repository. Your job is to steadily build a production-quality Rust replacement for renovatebot/renovate.

Run autonomously. Do not ask me questions. Make the best engineering decision you can from local evidence, Renovate's behavior, Rust ecosystem conventions, and the constraints below. If something is ambiguous, choose the option that preserves Renovate compatibility first, improves Rust design second, and document the decision in the repo. Never stop because of missing credentials, unavailable network, or an external service requirement. Document the blocker, skip that blocked slice, and continue with another local/offline slice that can move the project forward.

Workspace layout:
- The normal Claude Code working directory is `~/Projects/renovate-rust-experiement`.
- That directory contains two sibling checkouts:
  - `renovate/` is the upstream Renovate reference clone from `https://github.com/renovatebot/renovate`.
  - `renovate-rust/` is the Rust implementation repository from `https://github.com/donbeave/renovate-rust`.
- If the current working directory contains `renovate-rust/.git`, use `./renovate-rust` as the project root and `./renovate` as the upstream reference.
- If the current working directory is already the Rust repository, use `.` as the project root and `../renovate` as the upstream reference.
- All edits, commits, and project commands must target the Rust implementation repository. Treat the upstream `renovate/` reference checkout as read-only: inspect it, but never edit, format, generate files, install dependencies, run mutating commands, or commit inside it.

Repository rules:
- Follow `AGENTS.md`, `CLAUDE.md`, `BRANCHING.md`, and `COMMITS.md`.
- Keep agent-only rules in `AGENTS.md` and shared human/agent rules in topic-specific files.
- Commit messages must follow the repository commit conventions.
- Never modify `prompts/claude-loop-renovate-rust.md` while executing this loop. Treat this prompt as operator-owned configuration. If you identify an improvement, record it as a suggestion in project docs instead of editing the prompt.

Reference repository:
- Treat renovatebot/renovate as the behavioral reference.
- Use the existing sibling checkout at `../renovate` from the Rust project root, or `./renovate` when Claude Code was started from `~/Projects/renovate-rust-experiement`.
- Do not clone Renovate; the reference checkout is expected to already exist.
- Treat the reference checkout as read-only. Only run non-mutating inspection commands there.
- Do not update the reference checkout. If it appears stale, document the assumption and continue using the local contents.
- Read Renovate's docs, source, and tests before implementing behavior. Prefer `docs/`, `lib/`, `test/`, `package.json`, and configuration schema files as primary references.
- Do not copy Renovate source code verbatim. Recreate behavior in idiomatic Rust and keep license implications in mind when porting tests or fixtures.

Primary outcome:
- Build a Rust binary named `renovate` that can be used as a drop-in replacement for common Renovate CLI workflows.
- Focus only on rebuilding the Renovate CLI as a Rust alternative. Do not try to
  rebuild the full Renovate ecosystem or hosted infrastructure now.
- Preserve Renovate-compatible CLI flags, environment variables, config file names, config semantics, exit codes, update decision logic, and machine-readable output wherever behavior is observable.
- Treat performance as a core product goal and a main reason this project exists.
  Even though this is a Renovate-compatible CLI, do not merely port the original
  design one-for-one. Think hard about how the Rust implementation can be much
  faster than the original Renovate CLI in real repositories while preserving
  observable compatibility.
- Consider performance improvements in every slice: startup time, repository
  scanning, config loading, manifest parsing, dependency extraction, version
  comparison, lockfile handling, datasource lookups, caching, memory use,
  allocation patterns, async scheduling, bounded concurrency, and avoiding
  unnecessary work.
- Improve human output where compatibility allows. Default interactive output
  should be colorful, intuitive, and easy to understand at a glance: group by
  repository and dependency, use semantic color consistently for success,
  skipped, warning, error, and pending states, and explain skipped or failed
  updates plainly. Support quiet, verbose, debug, and JSON log modes.
- Color must be controllable. Enable color by default only for suitable
  interactive terminals, disable it automatically for CI and non-TTY output, and
  provide explicit opt-outs through standard environment variables such as
  `NO_COLOR` plus CLI/config switches when compatibility allows.
- When compatibility and improved UX conflict, keep compatibility available through flags or config and document the tradeoff.

Out of scope for now:
- Hosted Renovate bots, GitHub Apps, GitHub Actions plugins, hosted dashboards,
  SaaS services, marketplace integrations, webhook processors, and other
  surrounding infrastructure.
- Features that only make sense for hosted or managed Renovate deployments unless
  they are needed to preserve ordinary CLI behavior.
- Building replacement infrastructure around the CLI before the local Rust CLI is
  useful, compatible, fast, and well-tested. Record future infrastructure ideas
  in docs if useful, but keep implementation work on the CLI.

Rust project standards:
- Use the latest stable Rust release as the project toolchain. As of 2026-04-28,
  the latest stable release is Rust 1.95.0; do not treat that date-pinned value
  as a ceiling. When network access is available, refresh with
  `rustup update stable`, verify with `rustc --version`, and update
  `rust-toolchain.toml`, `Cargo.toml` `rust-version`, docs, and CI references
  to the current stable release before starting feature work.
- Use the latest released, non-yanked crates.io version for every new or updated
  dependency. Verify versions from crates.io with `cargo search`, `cargo info`,
  or `cargo add`; do not rely on memory, stale examples, or old generated
  scaffolding. Prefer stable releases over prereleases unless the project
  intentionally needs a prerelease and documents why.
- Use `clap` derive APIs for the CLI, including subcommands, help, version output, shell completions when useful, env-backed options where appropriate, and Renovate-compatible aliases.
- Set up formatting, linting, and test infrastructure at the beginning of the project, before feature work grows:
  - `rustfmt` policy, committed through `rustfmt.toml` when project defaults are not enough
  - strict Clippy policy, committed through crate lints and/or `clippy.toml` when useful
  - `cargo-nextest` configuration in `.config/nextest.toml` when profiles, retries, slow timeouts, or CI behavior are needed
  - documented local quality commands in `README.md` or `CONTRIBUTING.md`
- Favor a workspace layout suitable for a serious CLI project:
  - CLI crate for argument parsing, output, and process exit behavior
  - Core crates/modules for config, managers, datasources, versioning, update planning, repository/platform integrations, and execution
  - Integration and snapshot tests that exercise the binary
- Prefer idiomatic crates and patterns:
  - `serde`/`serde_json`/`toml`/`serde_yaml` for structured config and fixtures
  - `tokio` and `reqwest` for async network behavior
  - `thiserror` for library errors and `miette` or `anyhow` for CLI diagnostics as appropriate
  - `tracing`/`tracing-subscriber` for logging
  - `camino` or `std::path` carefully for paths
  - `assert_cmd`, `assert_fs`, `predicates`, `insta`, or `snapbox` for CLI tests
  - `schemars` or equivalent when generating or validating schemas is useful
- Keep code safe and maintainable:
  - `#![forbid(unsafe_code)]`
  - strict Clippy and rustdoc expectations
  - no broad `allow` attributes without a short justification
  - logical, reasonable module structure with clear responsibilities
  - small modules with clear ownership
  - deterministic tests
  - no hidden network access in unit tests

Rust best-practice rules:
- Inspect `Cargo.toml`, workspace layout, crate boundaries, feature flags, lint config, tests, docs, and dependency policy before changing Rust code.
- Keep the Rust implementation logically structured and maintainable. During
  each implementation run, refactor code that has become messy, confusing,
  overly coupled, duplicated, or hard to extend, provided the refactor is scoped
  to the current slice and protected by tests or follow-up checks.
- Prefer designs that avoid avoidable overhead: stream large inputs where
  practical, avoid repeated filesystem scans, cache expensive derived data,
  batch independent work, use bounded parallelism for IO-heavy workflows, and
  keep hot-path data structures compact and predictable.
- Prefer borrowed inputs when ownership is not required: `&str`, `&[T]`, `&Path`, and `Option<&T>` over owned or nested-reference forms. Take ownership only when storing, moving, sending, or intentionally avoiding a caller-side clone.
- Treat `.clone()` as a design decision. Do not clone merely to satisfy the borrow checker; make ownership boundaries explicit.
- Avoid unnecessary intermediate `Vec` and `String` allocations when iterators, slices, borrowed views, or lazy fallback closures are clear enough.
- Prefer readable control flow over clever iterator chains when errors, branching, ownership, or side effects become hidden.
- Return `Result<T, E>` for expected failure. Use `Option<T>` only when absence needs no diagnostic detail.
- Prefer typed errors in library/core crates. Use `anyhow`-style errors at binary, CLI, integration-test, or prototype boundaries. Add context at IO, parsing, network, task, and user-facing boundaries.
- Reserve `panic!`, `unwrap`, and `expect` for tests, examples, unreachable invariants, or programmer errors with precise context.
- Encode invariants in types with newtypes, enums, builders, validated wrappers, and type-state where that reduces ambiguity. Avoid ambiguous `bool`, primitive, or loosely-typed `String` parameters at important boundaries.
- Keep public fields private unless representation is deliberately part of the contract. Avoid public dependency exposure, re-exports, blanket generics, and serialization derives unless they are intentional compatibility commitments.
- Add dependencies conservatively, but when a dependency is justified, use its
  latest suitable release. Account for compile time, maintenance, supply-chain
  risk, and public API cost; if the latest release is unusable, document the
  concrete blocker and choose the newest compatible release instead of silently
  pinning an older version.
- Tests should describe behavior at stable boundaries. Cover success and failure paths with minimal deterministic fixtures. Use snapshots for CLI output, generated data, serialized forms, and rendered summaries when whole-output review is clearer.
- Add focused benchmarks or measurable before/after notes for performance-sensitive
  code paths when practical. Do not claim the Rust implementation is faster
  because it is Rust; make algorithmic, IO, concurrency, or allocation choices
  that can be explained and measured.
- Public examples should work as doctests where practical and should use `?` rather than `unwrap` unless demonstrating panic behavior.
- Comments should explain invariants, compatibility, platform behavior, safety, performance tradeoffs, or external constraints; do not restate obvious code.
- Fix Clippy warnings before suppressing them. If suppression is justified, prefer `#[expect(clippy::lint_name, reason = "...")]` or the local equivalent so stale suppressions are caught later.
- Do not enable broad Clippy `restriction`, `pedantic`, or `nursery` groups wholesale. Select strict lints intentionally.
- Do not claim performance wins without measurement unless the change removes an obvious allocation, clone, or blocking operation from ordinary execution.

Quality gates:
- Before committing code changes, run the strongest applicable local checks that fit the iteration:
  - `cargo build --workspace --all-features`
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo nextest run --workspace --all-features`
  - `cargo test --doc --workspace --all-features` when doctests exist or public docs changed
  - `cargo doc --workspace --all-features --no-deps` when public APIs changed
- Use `cargo-nextest`, not `cargo test`, for unit and integration tests. If `cargo nextest` is missing, install it or document the blocker, then continue with another local task only if installation is impossible.
- If the project does not yet have the required Rust scaffolding, create it first, including `Cargo.toml`, a latest-stable Rust toolchain policy, rustfmt/clippy/nextest configuration where useful, and CI-ready commands.
- Never commit failing formatting, Clippy, build, or tests unless the repo was already failing before your changes and the failure is documented in the commit message and progress notes.

Parity workflow:
1. Inspect the current repo state and the latest commits.
2. Inspect Renovate reference docs/tests/source for one missing behavior slice.
3. Choose the highest-value slice that can be completed in this loop without breaking existing work.
4. Add or update a parity note before or during implementation. Maintain docs such as:
   - `docs/parity/implementation-ledger.md`
   - `docs/parity/renovate-test-map.md`
   - `docs/parity/compatibility-decisions.md`
   Create them if they do not exist.
5. Write Rust tests that encode Renovate-compatible behavior. When practical, translate Renovate test cases into Rust tests using original Rust test code and local fixtures.
6. Implement the behavior in idiomatic Rust.
7. Run checks, fix failures, and tighten the implementation.
8. Commit the completed slice with a concise message.

Iteration sizing:
- Each 15 minute loop should leave the repository better than it started.
- Each loop must build something concrete, add or update tests for that behavior, run formatting, run Clippy, and fix any issues found before committing.
- Prefer a complete vertical slice over broad partial scaffolding, except for the initial loop where creating the Rust workspace, formatting, Clippy, and nextest foundation is the highest-value slice.
- Good slices include:
  - CLI flag or config compatibility
  - config discovery and merge behavior
  - a datasource client behind a trait with mocked tests
  - a package manager parser
  - versioning/range semantics
  - repository scanning behavior
  - output/logging improvements
  - parity test infrastructure
- If a slice is too large, implement the smallest test-backed part and record the next step.

Compatibility details to keep checking against Renovate:
- CLI names, aliases, help text expectations, and exit codes
- config file discovery and precedence
- environment variable names and parsing
- onboarding behavior
- repository/platform detection
- package manager detection
- datasource lookup behavior
- versioning and range update decisions
- lockfile and manifest update strategy
- branch, commit, PR title/body naming behavior
- dry-run behavior
- logging levels and JSON logs
- error handling and partial failure behavior

Output and UX requirements:
- Human output should be calmer, more colorful, and clearer than Renovate's
  default output while still being intuitive for first-time users.
- Show what repository is being processed, what dependencies were found, what updates are proposed, and why updates were skipped.
- Use color to improve scanning in interactive terminals, but never make color
  the only signal. Respect `NO_COLOR`, explicit no-color CLI/config settings,
  non-TTY output, and CI.
- Keep machine-readable modes stable and easy to parse.
- Prefer concise summaries with expandable verbose/debug detail.

Autonomy rules:
- Do not ask the user which crate, architecture, or behavior to implement next.
- Do not wait for permission to add files, refactor local Rust code, or create tests.
- Do not let short-term feature work pile up unmaintainable structure. If a
  small refactor is needed to keep the implementation reasonable, do it in the
  same loop before committing.
- Do not push to remotes.
- Do not rewrite unrelated user changes. If the worktree contains unrelated changes, leave them alone and commit only your own files.
- If another agent or user changed files, inspect and integrate with those changes rather than reverting them.
- If blocked, write a short note in `docs/parity/implementation-ledger.md` describing the blocker and choose another local slice.

Commit rules:
- Commit at the end of each successful loop when there are meaningful changes and checks pass.
- Stage only files changed for this loop.
- Use concise commit messages such as:
  - `build: add Rust workspace scaffolding`
  - `test(config): port Renovate config discovery cases`
  - `feat(cli): implement compatible CLI flags`
  - `docs(parity): add Renovate test ledger`
- Include test/check results in the commit body when useful.

Start now:
1. Confirm the existing reference Renovate checkout is available.
2. Inspect this repository and its docs.
3. Pick the next best parity slice.
4. Implement it, test it, document it, and commit it.
