# AGENTS.md

This repository uses `main` as its primary branch. This file is the canonical
home for rules and restrictions that apply only to AI agents. Rules that apply
equally to human contributors and agents live in topic-specific files linked
under **Shared conventions** below.

Agents may commit requested work directly to `main`. Do not create feature
branches or pull requests unless the operator explicitly asks for them.

## Pull Request Merging (agent-only)

**Agents must never merge a pull request without explicit per-PR confirmation
from the human operator.**

- Open the PR, share the URL, and stop.
- Prior "just do it", "do everything autonomously", or similar authorization
  applies only to the active workstream. It does not carry forward to later PRs.
- Do not use branch-protection bypass or admin merge unless the operator
  explicitly authorizes it for the specific PR.
- If merge authorization is ambiguous, ask before merging.

### Verify PR metadata before merging

When the operator confirms a PR can be merged, verify that the PR title and
description still match the actual diff before invoking the merge.

- Read the current PR metadata.
- Read the actual diff being merged.
- Update stale title, summary, or test-plan text before merging.
- Surface meaningful metadata corrections briefly in the final reply.

If overriding a squash or merge commit title, preserve the GitHub PR reference
in the title, for example: `feat(cli): add config discovery (#42)`.

## Commit Attribution (agent-only)

Every commit created by an AI agent in this repository must include exactly one
`Co-authored-by` trailer identifying the agent tool that made the commit.

Trailers by agent:

- Claude Code:

  ```text
  Co-authored-by: Claude <noreply@anthropic.com>
  ```

- OpenAI Codex:

  ```text
  Co-authored-by: Codex <codex@openai.com>
  ```

- Sourcegraph Amp:

  ```text
  Co-authored-by: Amp <amp@ampcode.com>
  ```

Do not stack multiple agent trailers on one commit. If you are uncertain which
agent is creating the commit, ask.

## Renovate Reference Repository (agent-only)

Treat the upstream Renovate repository as the behavioral reference for this
project.

- Prefer the local checkout at `../renovate`.
- Do not clone Renovate; the sibling reference checkout is expected to already
  exist.
- Treat the reference checkout as read-only. Do not edit files, install
  dependencies, generate artifacts, run mutating commands, or commit there.
- Do not commit downloaded registries, credentials, tokens, or generated cache
  directories.
- Use Renovate docs, source, and tests to drive Rust parity work.

## Autonomous Implementation (agent-only)

For long-running implementation work, especially work driven by the native
Claude Code `/loop` prompt, make local engineering decisions without waiting for
the operator.

- Prefer Renovate compatibility first and Rust idioms second when they conflict.
- Keep each iteration small, buildable, tested, and documented.
- If blocked by network, credentials, or external service access, document the
  blocker and continue with another local slice.
- Do not edit `prompts/claude-loop-renovate-rust.md` while executing the loop.
  Treat it as operator-owned configuration.
- Never rewrite unrelated user changes.

## Refactoring Philosophy (agent-only)

**Large-scale refactoring is always acceptable.** Do not take the easiest path
out of fear of fundamental change. The goal is a better technical solution, not
the smallest possible diff.

- Refactor internal implementation freely: module structure, data types,
  pipeline architecture, abstraction layers, naming, error handling, async
  design — anything internal may change if it improves correctness, clarity,
  or performance.
- Preserve external compatibility: CLI commands, flags, environment variables,
  config file format, config semantics, exit codes, and machine-readable output
  must remain consistent with the original Renovate CLI where it makes sense to
  do so. Renovate compatibility is the external contract; internal design is
  fully under our control.
- Do not accumulate technical debt by avoiding refactors. If a prior design
  decision turns out to be wrong, fix it completely rather than working around
  it with hacks or compatibility shims.
- When a refactor touches many files, commit it as a single atomic change with
  a clear description of the motivation.

## Test Coverage Parity Goal (agent-only)

The Rust implementation must reach **at minimum the same test coverage as the
original Renovate TypeScript repository**. Every `it()` test case in every
`.spec.ts` file in the Renovate reference is a porting requirement.

- A TypeScript test may be satisfied by a Rust test that covers the same
  behavior, even if the Rust test has a different name or structure.
- A TypeScript test may be marked `not-applicable` when: the feature is
  explicitly out of scope for the Rust CLI; or the test exercises TypeScript-
  internal behavior with no runtime equivalent (type guards, mock infrastructure,
  generic constraint tests, module resolution). Always record the reason in the
  test map's Reason column. When in doubt, port the test.
- Additional Rust tests beyond the TypeScript baseline are welcome and
  encouraged, but they do not count toward closing parity gaps.
- Track coverage in `docs/parity/renovate-test-map.md` using the per-test
  format. The file is the source of truth for parity status.

## Ported Test Attribution (agent-only)

Every Rust test that was ported from a Renovate TypeScript spec file **must**
include the following comment on the line immediately above `#[test]`:

```rust
// Ported: "<original it() description>" — <manager>/<spec-file>.spec.ts line <N>
#[test]
fn test_returns_null_for_invalid_yaml() {
```

Rules:
- The quoted string is the exact text passed to `it(` in the TypeScript spec.
- The path is relative to `lib/modules/manager/` — e.g. `pre-commit/extract.spec.ts`.
- The line number is the 1-based line of the `it(` call in the spec file.
- This comment is required on every ported test, no exceptions.
- When searching for existing ported tests during Phase 2 mapping, grep for
  `// Ported:` to locate already-attributed tests quickly:
  ```sh
  grep -rn "// Ported:" crates/
  ```

## Shared Conventions

Rules in the files below apply to everyone working in the repo, human and
agent:

- [BRANCHING.md](BRANCHING.md) - direct-to-main workflow and branch exceptions.
- [COMMITS.md](COMMITS.md) - Conventional Commits, verification commands, and
  commit trailers.
