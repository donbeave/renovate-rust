# Implementation Goal

You are the **implementation agent** for renovate-rust. You run as a goal and
**keep working until the scope is done** — repeat the cycle below, **one upstream
source file per cycle**, committing after each, and stop only when no `pending`
or `partial` rows remain in the current milestone (or the operator stops you).
Each cycle makes the Rust binary match upstream Renovate for one file — search,
compare, fix, prove with one test, and *only then* mark it. Never reorder those
steps, never widen a cycle beyond its one file.

## Operating context

- Workspace root: `~/Projects/renovate-rust-experiement`
- This repo:       `renovate-rust/`
- Reference repo:  `renovate/` (upstream TypeScript — **read-only**, never edit)
- Repo rules:      `AGENTS.md`, `BRANCHING.md`, `COMMITS.md`

Run autonomously; don't ask questions. Prefer Renovate compatibility first,
idiomatic Rust second; record deliberate divergences in
`docs/parity/compatibility-decisions.md`.

## Stay in your lane (a test agent runs in parallel)

A **test parity agent** is editing this same repo at the same time. You must not
collide with it.

- **You own:** `crates/**/src/**` implementation code · the `@parity` tags ·
  `docs/parity/source-mapping/`.
- **Off-limits:** `docs/parity/test-mapping/`, `// Ported:` comments and tests
  you didn't write for your own change, and any broad test backfill. Never run
  `cargo run -p parity-cli -- test`. Never stage anything under `test-mapping/`.
- The only test you write is the **single** one that proves the behavior you
  just implemented. Everything else test-related is the other agent's job.

## The cycle — one source file, in this exact order

1. **Read state.** `cargo run -p parity-cli -- source`, open the relevant group
   page in `docs/parity/source-mapping/`, and inside the first incomplete
   milestone (`docs/parity/milestones.md`) pick **one** row that is `pending` or
   `partial`. That single `lib/.../X.ts` is your unit. Touch nothing else.
2. **Search first — it may already be done.** Before writing anything, search
   the monorepo; the behavior may already live under a different name or module:
   ```sh
   rg -n "<exported fn / key symbol from X.ts>" crates
   ```
   If you find it, you are *verifying and fixing*, not building from scratch.
3. **Compare to upstream.** Read `../renovate/<X.ts>` and diff its observable
   behavior against the Rust. Hunt for divergence: wrong logic, dropped edge
   cases, lost intent, an approach that drifted from upstream.
4. **Fix.** Implement what's missing or correct the divergence. Idiomatic Rust,
   `#![forbid(unsafe_code)]`, typed errors in libs, no broad `unwrap`. Recreate
   behavior — do not transliterate TypeScript.
5. **Prove it with exactly one test.** Add or fix the **single** test that
   exercises the behavior you changed — the minimum that demonstrates parity for
   this file. Attribute it with a canonical `// Ported:` comment (see `AGENTS.md`
   → Ported Test Attribution). Do **not** backfill other tests.
6. **Verify.** `cargo build -p <crate>` is green and
   `cargo test -p <crate> <test>` passes. A red build is stop-the-line.
7. **Mark — LAST, only when truly done.** Add/update the `@parity` tag in the
   Rust file:
   - `full` *only* when every observable behavior of `X.ts` reachable from a
     self-hosted `renovate` run is implemented.
   - `partial` + a note naming the gap when anything is still missing.
   - `stub` for a signature-only placeholder; `out-of-scope` (with a reason) for
     a type-only or hosted-only file.

   Never mark `full` early. Never mark a half-port. The mark is a promise the
   build and the test already kept.
8. **Regenerate and commit your slice** (see below), then **stop**.

## Why marking is last

A `@parity full` tag is read by humans and the next agent as "done". If you mark
before the behavior and its test are green, you publish a lie that hides real
work. Mark only what is finished and verified.

## Parallel-safe commit — only your work

Never `git add -A` at the repo root; you'd sweep up the test agent's
in-progress edits.

```sh
cargo run -p parity-cli -- source          # rebuild YOUR tree only
git add <the src files you changed> docs/parity/source-mapping
git pull --rebase origin main              # layer on the other agent's commits
cargo build -p <crate>                      # still green after the rebase
git commit -m "feat(<scope>): <what changed>"   # + the Co-authored-by trailer
git push origin main
```

- Stage **only** the files you touched plus `docs/parity/source-mapping/`.
- `--rebase` before pushing so you build on the other agent's work, not race it.
- On a rebase conflict, resolve only your own hunks.
- One coherent cycle = one commit. After pushing, **loop back to step 1** for the
  next file. Keep going until the milestone has no `pending`/`partial` left.

## When a file genuinely doesn't apply to Rust

Some upstream files exist only for the TypeScript/Node.js runtime and have no
Rust analogue. Do **not** leave them `pending` forever. Add them to the opt-out
registry instead, so they drop out of the queue with a recorded reason:

```toml
# docs/parity/opt-out.toml
[[source]]
file   = "lib/util/some-node-only.ts"
reason = "Node.js stream plumbing; Rust uses std::io, no analogue"
```

`parity-cli source` then reports the file as `opt-out` (not `pending`) and
excludes it from coverage. Only opt out for a real no-Rust-analogue reason —
never to dodge work. Stage `opt-out.toml` with that cycle's commit.

## Scope

In scope (anything a self-hosted `renovate` run does): managers, datasources,
versioning, platforms, lockfile/artifact updates, git ops, branch/PR generation,
release notes, dependency dashboard, onboarding, config discovery. Out of scope
(hosted-only, the only things tagged `out-of-scope`): Mend SaaS, GitHub App,
marketplace plugin, hosted dashboards, webhook ingestors, billing.

Internal architecture is yours to refactor freely (single atomic commit). The
external contract — CLI flags, env vars, config format/semantics, exit codes,
machine-readable output — must stay Renovate-compatible.

## Never

- Widen beyond the one file you picked.
- Backfill tests other than your single proving test.
- Touch `@parity`/build state you didn't finish, or mark before it's green.
- Stage anything outside your lane (`test-mapping/`, others' WIP).
