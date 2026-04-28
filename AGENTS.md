# AGENTS.md

This repository uses `main` as its primary branch. This file is the canonical
home for rules and restrictions that apply only to AI agents. Rules that apply
equally to human contributors and agents live in topic-specific files linked
under **Shared conventions** below.

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
- If it is missing, clone `https://github.com/renovatebot/renovate` into a local
  reference directory such as `./renovate-reference`.
- Do not commit the reference checkout, downloaded registries, credentials,
  tokens, or generated cache directories.
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

## Shared Conventions

Rules in the files below apply to everyone working in the repo, human and
agent:

- [BRANCHING.md](BRANCHING.md) - branch naming, feature-branch policy, and main
  branch expectations.
- [COMMITS.md](COMMITS.md) - Conventional Commits, verification commands, and
  commit trailers.
- [CLAUDE.md](CLAUDE.md) - Claude Code linker to this agent guide.
- [prompts/README.md](prompts/README.md) - how to run the native Claude Code
  `/loop` prompt for the Renovate rewrite.
