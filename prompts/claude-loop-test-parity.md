You are working on the renovate-rust repository.

## Mission

The Rust implementation must have **at minimum the same test coverage as the
original Renovate TypeScript repository**. Every `it()`, `test()`, `it.each()`,
and `test.each()` call in every `.spec.ts` file is a porting requirement: it
must either be ported to an equivalent Rust test or explicitly marked
`not-applicable` with a documented reason.

This is the minimum bar. The Rust test suite may go further — additional edge
cases, Rust-specific invariants, performance assertions — but extras do not
substitute for parity with the TypeScript baseline.

Your goal is to drive the Rust test suite toward that minimum bar by maintaining
`docs/parity/renovate-test-map.md` as a precise per-test audit trail and by
writing or annotating Rust tests to close the gaps.

Run autonomously. Do not ask questions. Make the best engineering decision from
local evidence. Never stop because of a missing credential or external service —
document the blocker, skip the entry, and continue.

Never modify `prompts/claude-loop-test-parity.md` while executing this loop.

---

## Workspace layout

- Normal Claude Code working directory: `~/Projects/renovate-rust-experiement`
- `renovate/` — upstream Renovate reference clone (read-only)
- `renovate-rust/` — the Rust implementation repository
- If already inside `renovate-rust/`, use `.` as the project root and
  `../renovate` as the reference.
- All writes, tests, and commits target `renovate-rust/`. Never edit `renovate/`.

**Before starting Phase 1 work**, verify the reference checkout exists:

```sh
ls ../renovate/lib   # or ./renovate/lib from the experiment root
```

If it is absent, skip all Phase 1 work this iteration. Focus on Phase 0
backfill and Phase 2/3 work using information already in the tracking file.
Document the missing checkout in the commit message.

---

## The target file

`docs/parity/renovate-test-map.md`

Read it at the start of every iteration. The file must begin with a summary
block updated on every commit:

```markdown
# Renovate Test Map

**Overall progress:** 340 / 1,100 actionable tests ported (31%) — updated 2026-04-29

Status key: `ported` · `pending` · `not-applicable`
```

### Count definitions

- **Total tests** (per spec file): count of `it(`, `test(`, `it.each(`,
  `test.each(`, and tagged-template `it.each\`` / `test.each\`` call sites.
- **Actionable** (per spec file and global): Total minus `not-applicable` rows.
  These are the tests that must be ported.
- **Ported**: rows with Status `ported`. `not-applicable` rows do not count
  toward Ported but do count as satisfied — a file with all rows either
  `ported` or `not-applicable` is `Status: ported`.
- The global summary fraction is `Ported / Actionable`, not `Ported / Total`.

### Recount commands

```sh
# Run from the renovate-rust project root
grep -c "| ported |" docs/parity/renovate-test-map.md
grep -c "| pending |" docs/parity/renovate-test-map.md
grep -c "| not-applicable |" docs/parity/renovate-test-map.md
```

---

## Per-test format

Sections appear in this fixed order: **Managers** (alphabetical by manager
name) → **Config** → **Workers** → **Util**. Do not reorder existing sections;
append new ones at the end of their category. This keeps diffs readable and the
file navigable.

Every section follows this template. The table **always has six columns** —
the Reason column is `—` for `ported` and `pending` rows.

```markdown
## `lib/modules/manager/ansible-galaxy/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ansible-galaxy/extract.spec.ts
**Total tests:** 14 | **Ported:** 9 | **Actionable:** 12 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 15 | ported | `ansible_galaxy.rs` | `test_returns_null_for_empty` | — |
| extracts multiple dependencies from requirements.yml | 23 | pending | — | — | — |
| validates TypeScript types | 67 | not-applicable | — | — | TypeScript type-system test; no runtime behavior |
```

### Format rules

- `##` heading = spec file path as inline code, relative to Renovate repo root.
- **Reference** = GitHub blob URL for the spec file. No line anchor on the URL
  itself — line anchors appear in the `// Ported:` comments in Rust source.
- **Total tests** = count of all `it`/`test`/`it.each`/`test.each` call sites
  (including skipped ones — see Phase 1). `not-applicable` rows are still
  included in Total.
- **Actionable** = Total minus `not-applicable` row count.
- **Ported** = count of rows with Status `ported` (excludes `not-applicable`).
- **Status** (file level):
  - `ported` — all actionable tests ported (zero `pending` rows)
  - `partial` — some actionable tests ported (Ported > 0, pending > 0)
  - `pending` — no actionable tests ported yet (Ported = 0)
  - `not-applicable` — entire spec file is out of scope
- `###` heading = full `describe()` nesting path joined with ` › `:
  - Top-level only: `### \`extractPackageFile()\``
  - Nested: `### \`extractPackageFile() › git deps\``
- Table columns (always all six, in this order):
  1. **Original test name** — exact string from the `it(` / `test(` call, or
     the `it.each` description template.
  2. **Line** — 1-based line of the call.
  3. **Status** — `ported` · `pending` · `not-applicable`.
  4. **Rust file** — filename only, no path. `—` when not ported.
  5. **Rust test name** — `#[test] fn` name. `—` when not ported.
  6. **Reason** — mandatory for `not-applicable`; `—` otherwise.

### Handling `it.each` and `test.each`

Both call forms produce **one row** in the table. Use the description template
as the test name (e.g. `parses $type dependency`). The row is `ported` only
when *all* data-table cases from the original are covered by the Rust
equivalent.

Catch both array and tagged-template forms:

```sh
# Array form: it.each([...])(  or  test.each([...])(
grep -nE "^\s+(it|test)\.each\(" path/to/extract.spec.ts

# Tagged-template form: it.each`...`  or  test.each`...`
grep -nE "^\s+(it|test)\.each\`" path/to/extract.spec.ts
```

In Rust, implement parameterized cases with `#[rstest]` / `#[case]` or a
data-driven loop. Verify `rstest` is in `Cargo.toml` before using it:

```sh
grep "rstest" Cargo.toml crates/*/Cargo.toml
```

If absent, add it: `cargo add rstest --dev -p <crate-name>`.

For `#[rstest]` tests, place the `// Ported:` comment above the `#[rstest]`
attribute (not above individual `#[case]` lines):

```rust
// Ported: "parses $type dependency" — manager/extract.spec.ts line <N>
#[rstest]
#[case("input1", "expected1")]
#[case("input2", "expected2")]
fn test_parses_dependency(#[case] input: &str, #[case] expected: &str) {
```

---

## Phases

### Phase 0 — Backfill existing Rust tests

Many Rust tests were written before the `// Ported:` comment convention.
Audit and annotate them each iteration as a batch — this runs alongside other
phases and does not block them.

#### Audit commands

```sh
# Total test functions (all test attribute variants)
grep -rn "#\[test\]\|#\[tokio::test\]\|#\[rstest\]" crates/ --include="*.rs" | wc -l

# Already attributed
grep -rn "// Ported:" crates/ --include="*.rs" | wc -l
```

#### Find unattributed tests (handles stacked attributes)

Use the Python script below — it looks back up to 5 lines before any test
attribute, which correctly handles stacked `#[cfg(...)]`, `#[allow(...)]`,
and similar attributes between the `// Ported:` comment and `#[test]`:

```sh
python3 - <<'EOF'
import glob, re
test_re = re.compile(r'#\[(test|tokio::test|rstest)\]')
ported_re = re.compile(r'// Ported:')
for path in sorted(glob.glob('crates/**/*.rs', recursive=True)):
    lines = open(path).readlines()
    for i, line in enumerate(lines):
        if test_re.search(line):
            window = lines[max(0, i - 5):i]
            if not any(ported_re.search(l) for l in window):
                print(f'{path}:{i+1}:{line.rstrip()}')
EOF
```

#### Backfill procedure

For each unattributed test:

1. Read the test body to understand what behavior it covers.
2. Find the most likely `.spec.ts` file (the one for the same manager/module).
3. Search for a matching `it()` call by looking for the same fixture data,
   assertion values, or function under test.
4. If a **confident** match is found:
   - Add `// Ported: "<it() description>" — <manager>/<spec>.spec.ts line <N>`
     on the line immediately above the `#[test]` / `#[tokio::test]` /
     `#[rstest]` attribute.
   - Update or create the corresponding row in `renovate-test-map.md`.
5. **If no confident match is found after inspecting the most likely spec file,
   classify the test as Rust-specific and move on.** Do not spend more than a
   few minutes on any one test. Leave it uncommented; it does not belong in the
   test map.

Process one Rust file (or a small batch) per iteration, commit, then continue.

---

### Phase 1 — Inventory a spec file

When a `.spec.ts` file has no section in the per-test format yet:

1. Verify the Renovate reference checkout exists (see Workspace layout).

2. Count all test call sites:

   ```sh
   # it( and test( calls (excluding it.skip, it.only — handled separately)
   plain=$(grep -cE "^\s+(it|test)(\.each)?\(" path/to/spec.spec.ts 2>/dev/null || echo 0)

   # Tagged-template it.each` / test.each`
   tagged=$(grep -cE "^\s+(it|test)\.each\`" path/to/spec.spec.ts 2>/dev/null || echo 0)

   echo "Total: $((plain + tagged))"
   ```

3. Extract describe, it, test, and variant names with line numbers:

   ```sh
   grep -nE "^\s+(describe|it|test|it\.each|test\.each|it\.skip|test\.skip|xit|xtest)\b" \
     path/to/spec.spec.ts
   ```

4. Build the section with one row per call site, tracking `describe` nesting to
   assign the right `###` subsection. For deeply nested files, parse nesting
   level by scanning for `describe(` opening lines and their matching `})`
   closings in sequence.

5. Initial Status for each row:
   - `it(` / `test(` / `it.each(` / `test.each(` → `pending`
   - `it.skip(` / `test.skip(` / `xit(` / `xtest(` → `pending`, Reason =
     `intentionally skipped in TypeScript source — verify before porting`
     (do **not** default to `not-applicable`; a human should decide)

6. **For large spec files (> 20 tests):** commit partial Phase 1 progress after
   each batch of rows. Note how many remain in the commit message:
   `docs(parity): inventory dockerfile extract spec (40 / 75 rows)`

---

### Phase 2 — Map existing Rust coverage

When a section exists and one or more rows are `pending`:

**Phase 2 is complete for a file when:** you have run all searches below for
every `pending` row and either found a confident match or concluded no match
exists in the current Rust codebase. Commit the result — remaining `pending`
rows are genuinely unported and ready for Phase 3.

1. Run once per session (not per file) and reuse the output:

   ```sh
   grep -rn "// Ported:" crates/ --include="*.rs"
   ```

   Tests with a provenance comment are definitively ported — link them
   immediately without further searching.

2. For each remaining `pending` row, search by keyword from the test name:

   ```sh
   grep -rn "fn test_" crates/ --include="*.rs" | grep -i "<keyword>"
   ```

3. Search by fixture content, assertion values, or error strings from the
   original test. Replace `<search-term>` with a literal string from the spec:

   ```sh
   grep -rn "<search-term>" crates/ --include="*.rs"
   ```

4. Confidence threshold: link a match only when you are confident the Rust test
   covers the same behavior as the TypeScript original — same inputs, same
   expected outcome. When confidence is ~80% or lower, leave the row `pending`.

5. When you confirm a match that lacks the `// Ported:` comment, add the
   comment to the Rust test as part of this iteration.

---

### Phase 3 — Port missing tests

When a section has `pending` rows and Phase 2 mapping is complete for that file:

1. Read `prompts/claude-loop-renovate-rust.md` before writing any Rust code.
   It is the canonical reference for Rust standards, crate conventions, scope
   decisions, and quality gates.

2. Read the original TypeScript test and the fixtures it uses.

3. Locate fixtures by spec file category:

   | Spec category | Fixture path |
   |---|---|
   | Manager | `../renovate/lib/modules/manager/<manager>/__fixtures__/` |
   | Config | `../renovate/lib/config/__fixtures__/` (may be inline) |
   | Util | `../renovate/lib/util/<module>/__fixtures__/` (may be inline) |
   | Workers | `../renovate/lib/workers/<path>/__fixtures__/` (may be inline) |

   Inline fixtures (template literals defined inside the spec file) should be
   copied into Rust string constants or test data files.

4. **If the Rust function under test does not exist yet, implement it first.**
   A test that cannot compile is not a port. Write the minimum correct
   implementation needed for the behavior the test exercises — not a stub that
   hardcodes the return value, and not a broad reimplementation. Follow the
   patterns established in the existing Rust codebase and documented in the main
   loop prompt.

5. **Write the test to cover the actual behavior, not to satisfy the assertion.**
   The implementation must handle the input correctly, not just return the
   expected value for that one input.

6. Place the test in the existing Rust file for the manager/module, or create
   a new test module if none exists.

7. Add the provenance comment on the line immediately above the test attribute:

   ```rust
   // Ported: "<original it() description>" — <manager>/<spec-file>.spec.ts line <N>
   #[test]
   fn test_returns_null_for_invalid_yaml() {
   ```

   - Quoted string = exact text of the `it(` / `test(` argument.
   - Path = relative to `lib/modules/manager/` for managers; relative to `lib/`
     for other categories.
   - Line = 1-based line of the call.
   - For `#[rstest]`, place above `#[rstest]` (see Handling `it.each` above).
   - No exceptions.

8. After the test passes, update the row: Status → `ported`, fill in Rust file
   and Rust test name.

9. **For large spec files (many pending rows):** port a batch per iteration,
   commit, and continue. Do not attempt to port an entire large file in one
   loop.

---

## When to mark `not-applicable`

Use `not-applicable` when a test should **never** be ported. The Reason column
is mandatory. When in doubt, port the test — Rust can ignore TypeScript
mechanics and test the underlying runtime behavior directly.

| Situation | Reason text |
|---|---|
| Verifies TypeScript type shapes, generics, or type guards | `TypeScript type-system test; no runtime behavior` |
| Tests Jest/Vitest mock infrastructure or fixture loading helpers | `mocking framework internals` |
| Tests TypeScript module import/export resolution | `TypeScript module system` |
| Feature is explicitly out of scope for the Rust CLI | `out of scope: hosted only` / `out of scope: GitHub App` |

**Do not** default `it.skip` / `xit` rows to `not-applicable`. These are
`pending` with a note. The implementation may have been incomplete when the
TypeScript test was skipped — it may be fully portable to Rust.

**Do not** mark a test `not-applicable` just because the Rust implementation
does not exist yet — that is `pending`.

---

## Iteration order

This ordering selects *which file to work on next*. For any individual file,
phases always run in sequence: 1 (inventory) → 2 (map) → 3 (port). Never
attempt Phase 3 on a file that has not completed Phase 2.

Each iteration should include work from multiple phases (a Phase 0 batch, plus
a Phase 2 or Phase 3 task), each committed separately:

1. **Phase 0 batch** — run the unattributed test audit; process one Rust file's
   worth of backfill. Commit the result.

2. **Phase 3 work** — pick one spec file where Phase 2 is complete (all rows
   either have Rust links or have been confirmed unmatched) and `pending` rows
   remain. Port a batch of tests. Commit.

3. **Phase 2 work** — pick one spec file that has a Phase 1 section (rows
   exist) and still has unexamined `pending` rows. Run the mapping search.
   Commit.

4. **Phase 1 work** — convert one spec file from the old table format, or add a
   new section for a spec file not yet covered. Commit.

Within each step, prefer spec files where the Rust module already exists — they
are most likely to have matchable or portable tests.

If a spec file has no Rust counterpart yet, still create the Phase 1 section.
Use `—` for Rust file and Rust test name. The section documents the gap so
Phase 3 can implement it later.

---

## Quality gates before every commit

```sh
cargo build --workspace --all-features
cargo fmt --all                                                         # fix first
cargo fmt --all --check                                                 # then verify
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
```

Also run doctests when public documentation was changed:

```sh
cargo test --doc --workspace --all-features
```

Fix all failures before committing. If a failure predates your changes, note it
in the commit message and skip only that test, not the gate.

---

## Commit rules

- Follow `AGENTS.md`, `CLAUDE.md`, and `COMMITS.md`.
- Every commit must include:
  ```
  Co-authored-by: Claude <noreply@anthropic.com>
  ```
- Multiple commits per session are expected and correct — one per coherent unit
  of work (one Phase 0 batch, one spec file mapped, one batch of tests ported).
- Always commit `renovate-test-map.md` together with any Rust source files
  changed in the same unit. Exception: if Phase 0 backfill finds no test-map
  match for a Rust file, commit the comment-only change without a map update.
- Update the summary block counts on every commit.
- Example commit messages:
  - `docs(parity): inventory ansible-galaxy extract spec (14 tests)`
  - `test(ansible-galaxy): port 5 extract tests from renovate spec`
  - `docs(parity): map existing Rust coverage for cargo extract spec`
  - `test(parity): backfill Ported comments in pre-commit extractor`
  - `docs(parity): inventory dockerfile extract spec (40 / 75 rows)`

---

## Start now

1. Run the Phase 0 audit to get a count of unattributed tests.
2. Read `docs/parity/renovate-test-map.md` to understand current state.
3. Recount the summary block and update it if stale.
4. Work through the Iteration order until the loop budget runs out, committing
   after each unit of work.

At every commit:
- Quality gates pass.
- Every Rust test you touched or wrote has `// Ported:` if it maps to a
  TypeScript spec test.
- `renovate-test-map.md` summary block reflects accurate current counts.
