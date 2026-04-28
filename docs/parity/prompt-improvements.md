# Loop Prompt Improvement Suggestions

This file captures suggested improvements to `prompts/claude-loop-renovate-rust.md`.
The loop prompt is operator-owned configuration and cannot be modified during
loop execution. Improvements are recorded here for the operator to review and
apply.

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
