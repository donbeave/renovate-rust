You are working on the renovate-rust repository.

## Mission

The Rust implementation must have **at minimum the same test coverage as the
original Renovate TypeScript repository**. Every `it()` test case in every
`.spec.ts` file in the Renovate reference is a requirement: it must either be
ported to an equivalent Rust test or explicitly marked `not-applicable` with a
documented reason.

This is the minimum bar. The Rust test suite may and should go further —
additional edge cases, Rust-specific invariants, performance assertions — but
those extras do not substitute for the TypeScript parity requirement.

Your goal in this loop is to drive the Rust test suite toward that minimum
bar by maintaining `docs/parity/renovate-test-map.md` as a precise, per-test
audit trail, and by writing or annotating Rust tests to close the gaps.

Run autonomously. Do not ask questions. Make the best engineering decision you
can from local evidence. Never stop because of a missing credential or external
service. Document the blocker, skip that entry, and continue.

Never modify `prompts/claude-loop-test-parity.md` while executing this loop.

---

## Workspace layout

- Normal Claude Code working directory: `~/Projects/renovate-rust-experiement`
- `renovate/` — upstream Renovate reference clone (read-only)
- `renovate-rust/` — the Rust implementation repository
- If you are already inside `renovate-rust/`, use `.` as the project root and
  `../renovate` as the reference.
- All writes, tests, and commits target `renovate-rust/`. Never edit `renovate/`.

---

## The target file

`docs/parity/renovate-test-map.md`

This file must be rebuilt to use the **per-test format** defined below.
The previous file-level table format (one row per spec file with just counts)
is being replaced entirely. Read the current file before you start each
iteration so you know which entries already exist and what phase they are in.

---

## New per-test format

The file is structured as a set of Markdown sections, one per `.spec.ts` file.
Each section follows this template exactly:

```markdown
## `lib/modules/manager/ansible-galaxy/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ansible-galaxy/extract.spec.ts  
**Total tests:** 14 | **Ported:** 9 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name |
|---|---|---|---|---|
| returns null for empty | 15 | ported | `ansible_galaxy.rs` | `test_returns_null_for_empty` |
| extracts multiple dependencies from requirements.yml | 23 | pending | — | — |
```

Rules:
- Section heading (`##`) = spec file path as inline code, relative to Renovate repo root.
- **Reference** link always points to the GitHub blob URL for that spec file.
- **Total tests** = number of `it(` occurrences in the spec file (count with grep).
- **Ported** = number of rows whose Status is `ported`.
- **Status** summary: `ported` (all done) · `partial` (some done) · `pending` (none done) · `not-applicable` (out of scope or TypeScript-internal).
- Subsection heading (`###`) = the full `describe()` path joined with ` › ` when nested.
  Example: outer describe `extractPackageFile()` only → `### \`extractPackageFile()\``
  Nested: outer `extractPackageFile()` + inner `git deps` → `### \`extractPackageFile() › git deps\``
- Table columns: **Original test name** (exact string from `it(` call) · **Line** (1-based line number of the `it(` call) · **Status** · **Rust file** (filename only, no path) · **Rust test name** (the `#[test] fn` name).
- Use `—` in Rust file and Rust test name when the test is not yet ported.
- Status values per row: `ported` · `pending` · `not-applicable` · `skipped`.
- For `not-applicable` and `skipped` rows, add a **Reason** column with a short explanation.

  Extended table format when skips exist:
  ```
  | Original test name | Line | Status | Rust file | Rust test name | Reason |
  |---|---|---|---|---|---|
  | validates TypeScript types | 42 | not-applicable | — | — | TypeScript type-system test; no runtime behavior |
  ```

---

## Four-phase workflow

Determine the current phase for each spec file by reading `renovate-test-map.md`:

- **Phase 0 — Backfill:** Many Rust tests were written before the provenance
  comment convention existed. These must be audited and annotated.

  For each Rust file in `crates/`, collect all `#[test]` functions that do NOT
  already have a `// Ported:` comment immediately above them. For each such
  test, inspect the test body and try to match it to an `it()` call in the
  corresponding TypeScript spec file. If a match is found:
  1. Add the `// Ported:` comment to the Rust test.
  2. Update (or create) the corresponding row in `renovate-test-map.md` with
     Status `ported` and the Rust file/test name.

  If no TypeScript match can be found, the test is Rust-specific — leave it
  uncommented and do not add it to the test map (it goes beyond the minimum bar,
  which is fine).

  Run Phase 0 for a batch of Rust files each iteration. Prioritize files with
  the most existing `#[test]` functions and no `// Ported:` comments yet.

  Quick audit commands:
  ```sh
  # All tests in the codebase
  grep -rn "#\[test\]" crates/ --include="*.rs" | wc -l

  # Tests already attributed
  grep -rn "// Ported:" crates/ --include="*.rs" | wc -l

  # Find Rust files that still have un-attributed tests
  grep -rln "#\[test\]" crates/ --include="*.rs" | while read f; do
    if ! grep -q "// Ported:" "$f"; then echo "$f"; fi
  done
  ```

- **Phase 1 — Inventory:** The spec file has no section in the new format yet.
  Parse the spec file, enumerate all `describe()` and `it()` calls (including
  nested describes), write the section with every row set to `pending`.
  Do not skip nested describes — they become separate `###` subsections.

- **Phase 2 — Mapping:** The section exists and at least one row is still
  `pending`. Grep the Rust codebase (`crates/`) for test function names or
  assertion strings that correspond to this original test. If a clear match
  exists, fill in the Rust file and Rust test name and update Status to `ported`.
  If no match is found, leave Status as `pending`.

- **Phase 3 — Porting:** The section exists, rows remain `pending`, and you
  have completed Phase 2 mapping for this file. Now write the missing Rust test.
  Implement only what is needed to make the test pass — no broad refactors
  unless the implementation is simply missing. After writing and verifying the
  test, update the row to `ported` and fill in the Rust file and test name.

Each loop iteration processes as many files as possible. Prefer to complete all
three phases for a small group of related spec files rather than doing Phase 1
across every file in one pass. This keeps the tracking file useful and accurate
after every commit.

---

## Iteration order

Each loop iteration should do one of the following, in priority order:

1. **Phase 0 — Backfill existing tests.** If any Rust files still contain
   `#[test]` functions without a `// Ported:` comment, process a batch of them.
   This must run until every ported test is attributed before spending time on
   new Phase 1/2/3 work. Check progress with:
   ```sh
   grep -rln "#\[test\]" crates/ --include="*.rs" | while read f; do
     if ! grep -q "// Ported:" "$f"; then echo "$f"; fi
   done
   ```

2. **Phase 3 — Port missing tests.** Any spec file section that completed
   Phase 2 mapping and still has `pending` rows needs new Rust tests written.

3. **Phase 2 — Map existing coverage.** Any spec file that has a Phase 1
   section with `pending` rows — grep for Rust matches and fill in the table.

4. **Phase 1 — Inventory new spec files.** Convert remaining spec files from
   the old table format into the new per-test section format, or add sections
   for spec files not yet mentioned at all.

Within each priority group, prefer spec files where the Rust extractor already
exists (most likely to have matchable or porteable tests).

---

## When to skip a test

Some TypeScript tests exist only to verify TypeScript-internal behavior that
has no equivalent concept in Rust. These must be explicitly skipped rather than
left as `pending` forever. Use Status `not-applicable` for tests that should
never be ported, and `skipped` for tests that are deferred without a definitive
decision.

**Always mark `not-applicable` (never port):**
- Tests that verify TypeScript type shapes (`typeof`, `as const`, type guards,
  generic constraint behavior) — these test the compiler, not runtime logic.
- Tests that verify mock infrastructure, Jest helper behavior, or test fixture
  loading (e.g. `Fixtures.get` behavior itself).
- Tests for TypeScript-specific module import/export resolution.
- Tests for features that are out of scope for the Rust CLI — see the out-of-scope
  list in `prompts/claude-loop-renovate-rust.md` (hosted bots, GitHub Apps,
  webhook processors, etc.).

**Mark `skipped` (defer, revisit later):**
- Tests for features not yet implemented in Rust but that are in scope. These
  will become `pending` once implementation starts.

For every `not-applicable` or `skipped` row, the **Reason** column is
mandatory. Short phrases are fine: `TypeScript type test`, `mocking framework`,
`out of scope: hosted only`, `not yet implemented`.

When in doubt, port the test. Rust can always ignore the TypeScript-specific
mechanics and test the underlying behavior directly.

---

## Porting approach reference

Read `prompts/claude-loop-renovate-rust.md` before writing any Rust code in
Phase 3. It is the canonical reference for:
- How to read and use the Renovate upstream checkout as a behavioral reference.
- Rust project standards and crate conventions to follow.
- Which features are in scope and out of scope for the CLI.
- Quality gates and commit conventions.

Do not re-derive these decisions from scratch. Follow the patterns already
established in the Rust codebase and documented in that prompt.

---

## Discovering and parsing spec files

To enumerate all spec files in the Renovate reference:

```sh
find ../renovate/lib -name '*.spec.ts' | sort
```

To count `it(` occurrences (total tests) in a spec file:

```sh
grep -c "^\s*it(" path/to/extract.spec.ts
```

To extract `describe` and `it` call names with line numbers:

```sh
grep -n "^\s*\(describe\|it\)(" path/to/extract.spec.ts
```

Use these to build each Phase 1 section. Parse the nesting depth manually by
tracking open/close describe blocks if the file has nested describes.

---

## Finding Rust matches (Phase 2)

First, check for tests that already carry a provenance comment — these are
definitively ported and can be linked immediately:

```sh
grep -rn "// Ported:" crates/ --include="*.rs"
```

The comment format is:
```
// Ported: "<original it() description>" — <manager>/<spec-file>.spec.ts line <N>
```

For tests without a provenance comment, search by name or assertion content:

```sh
grep -rn "fn test_" crates/ | grep -i "keyword"
grep -rn "#\[test\]" crates/ -A 1 | grep -i "keyword"
```

Also search assertion strings — Rust tests often assert on the same fixture
values or error messages as the TypeScript originals:

```sh
grep -rn "returns null\|empty" crates/ --include="*.rs"
```

Use judgment: a Rust test does not need the same name as the TypeScript
original to be a port. Check that the behavior under test is the same.
If you confirm a match that lacks the provenance comment, add the comment
to that Rust test as part of this loop iteration.

---

## Writing ported tests (Phase 3)

- Read the original TypeScript test and any fixture files it uses.
- Find the fixture files in `../renovate/lib/modules/manager/<manager>/__fixtures__/`.
  Copy needed fixture files into the corresponding Rust test fixtures directory.
- Write the Rust test in idiomatic Rust. Do not copy TypeScript verbatim.
- Place the test in the existing Rust extractor file for that manager, or
  create a new test module if none exists.
- **Every ported test must carry a provenance comment on the line immediately
  above `#[test]`:**
  ```rust
  // Ported: "<original it() description>" — <manager>/<spec-file>.spec.ts line <N>
  #[test]
  fn test_returns_null_for_invalid_yaml() {
  ```
  - Quoted string = exact text from the `it(` call in the TypeScript spec.
  - Path = relative to `lib/modules/manager/` (e.g. `pre-commit/extract.spec.ts`).
  - Line = 1-based line of the `it(` call.
  - No exceptions. Every ported test needs this comment.
- Run `cargo nextest run --workspace --all-features` to verify the test passes.
- Run `cargo fmt --all` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` before committing.
- Do not commit failing tests.

---

## Quality gates before every commit

```sh
cargo build --workspace --all-features
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
```

Fix all failures before committing. If a failure existed before your changes,
document it in the commit message and skip only that test, not the gate.

---

## Commit rules

- Follow `AGENTS.md`, `CLAUDE.md`, and `COMMITS.md`.
- Every commit must include:
  ```
  Co-authored-by: Claude <noreply@anthropic.com>
  ```
- Commit `renovate-test-map.md` updates together with any Rust test files added
  in the same iteration. One commit per loop slice.
- Use concise commit messages:
  - `docs(parity): inventory ansible-galaxy extract spec tests`
  - `test(ansible-galaxy): port extract tests from renovate spec`
  - `docs(parity): map existing Rust coverage for cargo extract spec`

---

## Start now

1. Run the Phase 0 audit commands to count how many Rust tests still lack a
   `// Ported:` comment. If the number is non-zero, start there.
2. Read `docs/parity/renovate-test-map.md` to understand the current state.
3. Identify the highest-priority work according to the Iteration order above.
4. Execute that phase, update `renovate-test-map.md`, run quality gates, commit.
5. Repeat until the loop budget is exhausted.

At every commit, the repository must be in a state where:
- All quality gates pass.
- Every Rust test you touched or wrote has a `// Ported:` comment if it maps
  to a TypeScript spec test.
- `renovate-test-map.md` accurately reflects what you just did.
