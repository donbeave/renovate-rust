# Claude Code /loop Prompt: Renovate in Rust

Use this prompt with Claude Code's native `/loop` from the root of this repository. Run it on a 15 minute interval until the Rust implementation is a practical drop-in replacement for `renovatebot/renovate`.

Usage instructions live in [prompts/README.md](README.md). The intended command is:

```text
/loop 15m Follow @prompts/claude-loop-renovate-rust.md
```

```text
You are working in the renovate-rust repository. Your job is to steadily build a production-quality Rust replacement for renovatebot/renovate.

Run autonomously. Do not ask me questions. Make the best engineering decision you can from local evidence, Renovate's behavior, Rust ecosystem conventions, and the constraints below. If something is ambiguous, choose the option that preserves Renovate compatibility first, improves Rust design second, and document the decision in the repo. Only stop when blocked by missing credentials, unavailable network, or an external service requirement that cannot be avoided.

Repository rules:
- Follow `AGENTS.md`, `CLAUDE.md`, `BRANCHING.md`, and `COMMITS.md`.
- Keep agent-only rules in `AGENTS.md` and shared human/agent rules in topic-specific files.
- Commit messages must follow the repository commit conventions.
- Never modify `prompts/claude-loop-renovate-rust.md` while executing this loop. Treat this prompt as operator-owned configuration. If you identify an improvement, record it as a suggestion in project docs instead of editing the prompt.

Reference repository:
- Treat renovatebot/renovate as the behavioral reference.
- Locate it in this order:
  1. `$RENOVATE_REFERENCE_REPO`, if set
  2. `../renovate`
  3. `./renovate-reference`
- If no reference checkout exists, clone it locally with:
  `git clone https://github.com/renovatebot/renovate ./renovate-reference`
- If a reference checkout exists, update it when possible with `git -C <reference> fetch --all --tags --prune`, but do not let network failure block local progress.
- Read Renovate's docs, source, and tests before implementing behavior. Prefer `docs/`, `lib/`, `test/`, `package.json`, and configuration schema files as primary references.
- Do not copy Renovate source code verbatim. Recreate behavior in idiomatic Rust and keep license implications in mind when porting tests or fixtures.

Primary outcome:
- Build a Rust binary named `renovate` that can be used as a drop-in replacement for common Renovate CLI workflows.
- Preserve Renovate-compatible CLI flags, environment variables, config file names, config semantics, exit codes, update decision logic, and machine-readable output wherever behavior is observable.
- Improve human output where compatibility allows. Default output should be clear, grouped by repository and dependency, explain skipped or failed updates plainly, respect `NO_COLOR` and CI, and support quiet, verbose, debug, and JSON log modes.
- When compatibility and improved UX conflict, keep compatibility available through flags or config and document the tradeoff.

Rust project standards:
- Use stable Rust and current mainstream crates.
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
  - small modules with clear ownership
  - deterministic tests
  - no hidden network access in unit tests

Quality gates:
- Before committing code changes, run the strongest applicable local checks that fit the iteration:
  - `cargo build --workspace --all-features`
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo nextest run --workspace --all-features`
  - `cargo test --doc --workspace --all-features` when doctests exist or public docs changed
  - `cargo doc --workspace --all-features --no-deps` when public APIs changed
- Use `cargo-nextest`, not `cargo test`, for unit and integration tests. If `cargo nextest` is missing, install it or document the blocker, then continue with another local task only if installation is impossible.
- If the project does not yet have the required Rust scaffolding, create it first, including `Cargo.toml`, a pinned or documented toolchain policy, rustfmt/clippy/nextest configuration where useful, and CI-ready commands.
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
- Human output should be calmer and clearer than Renovate's default output.
- Show what repository is being processed, what dependencies were found, what updates are proposed, and why updates were skipped.
- Use color only when appropriate; respect `NO_COLOR`, non-TTY output, and CI.
- Keep machine-readable modes stable and easy to parse.
- Prefer concise summaries with expandable verbose/debug detail.

Autonomy rules:
- Do not ask the user which crate, architecture, or behavior to implement next.
- Do not wait for permission to add files, refactor local Rust code, or create tests.
- Do not push to remotes.
- Do not rewrite unrelated user changes. If the worktree contains unrelated changes, leave them alone and commit only your own files.
- If another agent or user changed files, inspect and integrate with those changes rather than reverting them.
- If blocked, write a short note in `docs/parity/implementation-ledger.md` describing the blocker and choose another local slice.

Commit rules:
- Commit at the end of each successful loop when there are meaningful changes and checks pass.
- Stage only files changed for this loop.
- Use concise commit messages such as:
  - `Add Rust workspace scaffolding`
  - `Port Renovate config discovery tests`
  - `Implement clap CLI compatibility flags`
  - `Add parity ledger for Renovate tests`
- Include test/check results in the commit body when useful.

Start now:
1. Confirm the reference Renovate checkout exists or clone it.
2. Inspect this repository and its docs.
3. Pick the next best parity slice.
4. Implement it, test it, document it, and commit it.
```
