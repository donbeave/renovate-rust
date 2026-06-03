# Prompts

Two goal prompts, meant to run **in parallel** — one agent per prompt, both on
this same repo, each doing one focused cycle at a time. They work in Claude Code
or Codex; paste the matching block below into each agent.

| Prompt | Agent | Owns (its lane) | Never touches |
|---|---|---|---|
| [implementation.md](implementation.md) | implementation agent | `crates/**/src/**`, `@parity` tags, `docs/parity/source-mapping/` | `test-mapping/`, broad test backfill |
| [test-parity.md](test-parity.md)       | test parity agent    | `crates/**/tests/**` + `mod tests`, `// Ported:` comments, `docs/parity/test-mapping/` | `@parity`, `source-mapping/`, feature work |

The two lanes are designed **not to overlap**, so the agents don't break each
other. Each owns a different parity tree and a different kind of edit. Both:
pick **one** item → do the focused work → **mark it last** → regenerate their
own tree → commit and push **only what they touched**.

## How to run them in parallel

Run each prompt as a **`/goal`** in its own agent session — one for
implementation, one for test parity — both against this **same checkout** (no
worktrees). They run concurrently; the ownership lanes keep them out of each
other's way.

### Implementation agent — paste this

```text
/goal Follow renovate-rust/prompts/implementation.md. Keep cycling until the
current milestone has no pending/partial source files left: each cycle, read the
source-mapping state, pick one pending/partial source file, search the repo for
an existing implementation, compare it to the upstream TypeScript, fix
divergences, prove it with the single test that covers it, then mark the @parity
tag last (or, if it has no Rust analogue, opt it out in docs/parity/opt-out.toml
with a reason). Regenerate source-mapping, commit + push only the files you
changed, then move to the next file.
```

### Test parity agent — paste this

```text
/goal Follow renovate-rust/prompts/test-parity.md. Keep cycling until the current
milestone has no pending tests left: each cycle, read the test-mapping state,
pick one spec with pending tests (skip it if its implementation is missing), port
the missing test(s) so they really exercise the behavior — making a minimal
business-logic fix only if a test exposes a real divergence, or opting a test out
in docs/parity/opt-out.toml with a reason if it only checks TypeScript-runtime
behavior — then mark // Ported: last. Regenerate test-mapping, commit + push only
the files you changed, then move to the next test.
```

`/goal` is the recommended way to run these in both Claude Code and Codex: paste
the block and the agent keeps working through its scope, committing after each
item. (If a harness lacks `/goal`, the same text works as a plain message; in
non-interactive Codex you can inline the prompt with
`$(cat renovate-rust/prompts/implementation.md)`.)

## Staying conflict-free in one checkout

Both agents share one working directory — no worktrees. They stay out of each
other's way through three rules the prompts enforce:

- **Ownership lanes.** Each agent edits only its own files and its own mapping
  tree (see the table above). This is the single most important guard.
- **Scoped staging.** Never `git add -A` at the repo root. Stage only your own
  files plus your own mapping tree.
- **Rebase before push.** `git pull --rebase origin main` so each agent layers
  on the other's commits instead of racing; resolve only your own hunks.
- **Mark last.** Status (`@parity`, `// Ported:`) is set only after the build
  and test are green — never to reserve a row or pad a number.

## How priorities are set

`docs/parity/milestones.md` lists the ordered milestones. Both agents always
pick work from the **first incomplete milestone**.

## The tool

`parity-cli` (Rust) owns both parity trees and regenerates each by wiping and
rebuilding it, so removed upstream files/specs leave no stale pages:

```sh
# Run anywhere in renovate-rust/
cargo run -p parity-cli                # regenerate BOTH trees
cargo run -p parity-cli -- source      # docs/parity/source-mapping/ (TS impl → Rust, @parity tags)
cargo run -p parity-cli -- test        # docs/parity/test-mapping/   (upstream it() → // Ported:)
cargo run -p parity-cli -- gaps <mod>  # list pending upstream it()s for a module (e.g. manager/cargo)
cargo run -p parity-cli -- check       # CI guard: stale @parity tags + deleted-upstream tests
```

- **source-mapping** statuses: `full` · `partial` · `stub` · `pending` (no
  `@parity` tag) · `out-of-scope` (tag) · `opt-out` (registry).
- **test-mapping** states: `ported` (upstream test + matching `// Ported:`) ·
  `pending` · `opt-out` (registry) · `deleted` (a `// Ported:` whose upstream
  identity is gone — kept for review, never auto-removed).

**Opt-out registry — `docs/parity/opt-out.toml`.** The single place recording
items that will never be ported (TypeScript/Node-runtime specifics with no Rust
analogue), each with a reason. `parity-cli` reads it, marks those items `opt-out`
(not `pending`), excludes them from the coverage denominator, and keeps agents
from picking them. This is how the agents' loops terminate: every item ends up
ported/full or opted-out, so `pending` drains to zero.

Per-module coverage is the by-module summary in each tree's `README.md`. The
previous Python scripts have been removed; `parity-cli` is the only parity tool.

## Prompt maintenance

The two prompts are operator-owned. Agents running them must not edit them. File
improvement suggestions in `docs/parity/prompt-improvements.md` — the operator
decides whether to apply.
