# Prompt Improvement Suggestions

This file captures suggested improvements to the operator-owned prompts
(`prompts/implementation.md`, `prompts/test-parity.md`). Agents running them
cannot modify them; improvements are recorded here for the operator to review
and apply. (Earlier entries reference the now-retired `claude-loop-*` prompts.)

---

## Suggestion 1 — Test map maintenance in the parity workflow

**Date:** 2026-04-28
**Context:** Operator requested explicit maintenance of `docs/parity/renovate-test-map.md`
as a Renovate ↔ Rust test mapping table.

**Proposed addition to the "Parity workflow" section:**

After step 5 ("Write Rust tests that encode Renovate-compatible behavior"),
add:

> 5b. When the slice ports behavior that has Renovate test coverage, add rows to
>     `docs/parity/renovate-test-map.md` mapping each Renovate `it()` to the
>     equivalent Rust `#[test]` function (file path + function name + status).
>     Rows that cannot be ported yet go in the "Pending" section.

**Philosophy to add to the workflow:**

> **Test parity philosophy:** The goal is functional equivalence, not structural
> copy. One Renovate `it()` may map to multiple Rust tests; multiple `it()` blocks
> may collapse into one. Test organization follows Rust conventions. Fixtures are
> recreated as Rust literals — not copied verbatim. What matters is that each
> Renovate observable behavior has corresponding Rust coverage, even if the test
> structure differs.

---

## Suggestion 2 — Output compatibility philosophy

**Date:** 2026-04-28
**Context:** Operator clarified output compatibility expectations.

**Proposed addition to the "Output and UX requirements" section:**

> **Output compatibility scope:** Machine-readable output (JSON `--output=json`)
> must remain stable and compatible where it represents observable Renovate
> behavior (dep names, versions, PR/branch names, update types, skip reasons).
> Human-readable terminal output may improve over Renovate's defaults: calmer,
> more colorful, better UX. Color must be controllable (auto-detect TTY, respect
> `NO_COLOR`, support explicit `--no-color`). CI-friendly output (no ANSI color,
> compact format) should be the automatic default in non-TTY contexts.

---

## Suggestion 3 — "Not a 1:1 port" clarification

**Date:** 2026-04-28
**Context:** Operator emphasized that architectural improvements are welcome.

**Proposed addition to the "Refactoring philosophy" section:**

> This is a fresh Rust implementation, not a line-by-line port. We fix Renovate's
> historical design decisions where we can: better abstractions, cleaner error
> handling, faster algorithms, more idiomatic Rust. The external contract
> (observable behavior, CLI flags, output formats) must remain compatible; the
> internal implementation is entirely under our control.

---

## Applied 2026-05-25 — close the parity-illusion loophole (operator-requested)

**Context:** Audit found the test map reporting 100% (3068/3068 actionable) while
the source map had only 3 `full` rows (354 `partial`, 138 `not-started`). Cause:
8,609 of 11,677 detail rows (74%) were marked `not-applicable` under an
extraction-only scope, and the headline metric was `ported / actionable`, so the
denominator shrank as work was parked in NA. Operator confirmed the true scope is
a **full drop-in replacement** and asked to make the prompts run until that is
objectively achieved.

**Changes applied to both prompts (operator-authorized prompt edits):**

1. `claude-loop-renovate-rust.md` Definition Of Done replaced with a five-point
   machine-checkable **terminal state** (source-map full · test-map zero pending
   with budgeted NA · source↔test cross-check · differential harness green ·
   quality gates pass). Explicit "what is NOT completion" list.
2. Added an explicit **In scope** enumeration (datasources, version decisions,
   lockfile/artifact updates, platform branch/PR ops) and a scope guard; `Out of
   scope` narrowed to hosted/managed infrastructure only.
3. Defined source-map `partial` rigorously (must carry a `Missing:` note) and
   declared `partial`/`stub`/`not-started` all non-terminal.
4. Added the **Differential parity harness** section + `differential-harness.md`
   doc (run upstream Renovate and renovate-rust on shared offline fixtures, diff
   observable output).
5. `claude-loop-test-parity.md`: headline metric changed to `ported / total`
   with NA surfaced + NA budget (<~25%); added **Phase 0.5** to re-audit and
   reclassify mis-scoped `not-applicable` rows to `pending`; tightened the
   allowed NA categories; added a Completion section tied to the shared terminal
   state and the source↔test cross-check.
6. `README.md`: removed the "stop after one unit / 10 turns" test-parity
   invocations, added a "Running until truly done" section, aligned verification
   guidance (gates + harness run at the completion check).

**Follow-up work for the loop (not a prompt change):** execute Phase 0.5 to
reclassify the existing 8,609 NA rows, then drive source rows to `full` and build
out the differential harness fixtures.

---

## Applied 2026-05-31 — module ledger redesign (operator-authorized)

**Audit findings:** Despite Phase 0.5 instructions to *reduce* NA, the agent
loop flipped ~2,000 more tests from `pending` → `not-applicable` in the past
week (`parity: reclassify N mock-based spec files as NA` commit pattern). NA
share reached 53.7%, more than double the documented budget. Of 354K monthly
insertions, 125K (37%) were inside `docs/parity/`. Per-`it()` accounting at
11,658 rows was the single biggest source of wasted iteration time and
gaming. The previous prompts (535 + 711 lines) were too large to be re-read
honestly each iteration.

**Changes applied:**

1. New parity scheme:
   - `docs/parity/modules.md` is the new (and only) ledger. ~292 rows, one
     per upstream module. Two human-edited columns (Impl, Notes) owned by the
     implementation agent. Coverage is computed.
   - `scripts/parity_coverage.py` walks upstream spec files and Rust
     `// Ported:` comments, deduplicates by `(spec, it() description)`, and
     emits the ledger plus `gaps`/`orphans` reports.
   - The `not-applicable` mechanism is removed entirely. Coverage = deduped
     `ported / upstream_it()`. Per-module target ≥ 80%. No denominator
     gaming surface.
2. New prompts replace the old loop prompts:
   - `prompts/implementation.md` (~110 lines) — implementation agent.
   - `prompts/test-parity.md` (~110 lines) — test parity agent.
   - Old files moved to `prompts/_archived-claude-loop-*.md` with banners.
   - Single canonical invocation per agent (was four near-duplicates).
3. Two agents stop colliding:
   - Implementation owns `crates/**/src/*.rs` plus the Impl/Notes columns.
   - Test parity owns `crates/**/tests/*.rs` + `mod tests` blocks plus
     `// Ported:` comments. Never touches the ledger.
   - Coverage column is owned by the script.
4. New `docs/parity/milestones.md` gives ordered milestones (M0–M5) starting
   with `cargo + crates.io + semver end-to-end`. Both agents always pick work
   from the first incomplete milestone.
5. `docs/parity/renovate-test-map.md` and `renovate-source-map.md` carry
   deprecation banners and are no longer edited.
6. `AGENTS.md` updated: removed `not-applicable` rules, pointed at the new
   ledger and scripts.

**Initial coverage numbers from the new script** (deduped, honest):
5,004 / 11,667 = 42.9% (the old `47.3%` raw count inflated by 484 duplicate
`// Ported:` comments). Strong areas: versioning 90%, managers 60%. Weakest:
platforms 9%, workers 19%, util 39%.
