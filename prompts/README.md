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

Open two agent sessions (two terminals, or two Claude Code / Codex windows).
Paste one block into each. They run concurrently against the same checkout.

### Implementation agent — paste this

```text
Follow renovate-rust/prompts/implementation.md. Do exactly one focused cycle:
read the source-mapping state, pick one pending/partial source file in the first
incomplete milestone, search the repo for an existing implementation, compare it
to the upstream TypeScript, fix divergences, prove it with the single test that
covers it, then mark the @parity tag last. Regenerate source-mapping and commit
+ push only the files you changed. Then stop.
```

### Test parity agent — paste this

```text
Follow renovate-rust/prompts/test-parity.md. Do exactly one focused cycle: read
the test-mapping state, pick one spec with pending tests in the first incomplete
milestone (skip it if its implementation is missing), port the missing test(s)
so they really exercise the behavior — making a minimal business-logic fix only
if a test exposes a real divergence — then mark // Ported: last. Regenerate
test-mapping and commit + push only the files you changed. Then stop.
```

In Claude Code you can reference the file with `@renovate-rust/prompts/...`; in
non-interactive Codex use `$(cat renovate-rust/prompts/implementation.md)`. If
your harness has a `/goal` (or similar) command, the same text is the goal.

## Staying conflict-free in one checkout

The single most important rule for parallel agents sharing a directory is
**file-ownership** — which the lanes above give you. On top of that, both
prompts require:

- **Scoped staging.** Never `git add -A` at the repo root. Stage only your own
  files plus your own mapping tree.
- **Rebase before push.** `git pull --rebase origin main` so each agent layers
  on the other's commits instead of racing; resolve only your own hunks.
- **Mark last.** Status (`@parity`, `// Ported:`) is set only after the build
  and test are green — never to reserve a row or pad a number.

**Safest of all (optional):** give each agent its own git worktree of the same
repo so they can't touch each other's files at all:

```sh
git worktree add ../renovate-rust-impl main   # implementation agent here
git worktree add ../renovate-rust-test main   # test parity agent here
```

Then merge sequentially (rebase the second branch on the first). The prompts
also work in a single shared checkout with the ownership + rebase rules above.

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
  `@parity` tag) · `out-of-scope`.
- **test-mapping** states: `ported` (upstream test + matching `// Ported:`) ·
  `pending` · `deleted` (a `// Ported:` whose upstream identity is gone — kept
  for review, never auto-removed).

Per-module coverage is the by-module summary in each tree's `README.md`. The
previous Python scripts have been removed; `parity-cli` is the only parity tool.

## Prompt maintenance

The two prompts are operator-owned. Agents running them must not edit them. File
improvement suggestions in `docs/parity/prompt-improvements.md` — the operator
decides whether to apply.
