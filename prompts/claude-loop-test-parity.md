# Renovate Rust Test Parity Prompt

You are working on the renovate-rust repository.

## Mission

The Rust implementation must have **at minimum the same behavioral test coverage
as the original Renovate TypeScript repository**. Every `it()`, `test()`,
`it.each()`, and `test.each()` call in every `.spec.ts` file is an audit
requirement: it must either be covered by an equivalent Rust test or explicitly
marked `not-applicable` with a documented reason.

The goal is functional equivalence, not a mechanical Node-to-Rust conversion.
Tests that verify Renovate runtime behavior must be covered in Rust, even if the
Rust test structure, fixtures, names, and module boundaries differ.

`not-applicable` is for a **small** set of tests that exercise TypeScript or
test-framework mechanics with no runtime behavior — and nothing else. The
project scope is a **full drop-in replacement** of the self-hosted `renovate`
CLI (see `prompts/claude-loop-renovate-rust.md` → In scope). That means the
following are **in scope and must be ported**, not marked `not-applicable`:

- datasource / version-lookup behavior tested via `httpMock` or the `got`/`Http`
  client — port it against Rust's HTTP layer with recorded responses;
- artifact and lockfile updates that invoke external package managers — port the
  behavior using Rust's process layer and test doubles;
- git operations (`simple-git`, real-repo tests) — port against Rust's git layer;
- branch/PR/MR creation and update via platform APIs — port against the platform
  clients;
- release-note fetching, dependency-dashboard contents, onboarding, config
  parsing/migration/validation, template/commit-message rendering.

The fact that the upstream test uses a Node mock, an HTTP mock, a child process,
or a real git repo is **not** a reason to mark it `not-applicable`. Port the
runtime behavior; replace the Node mechanism with the Rust equivalent.

### Anti-gaming rules (read every iteration)

`not-applicable` removes a test from the denominator. That makes it the easiest
way to fake progress, so it is the most tightly controlled status:

- The headline metric is **not** `ported / actionable`. Report `ported / total`
  alongside the `not-applicable` count and its share of total (see the summary
  block format). A number that only moves because NA grew is not progress.
- There is a **NA budget**: across the whole project, `not-applicable` is
  expected to be a small minority of total tests (well under ~25%). Renovate is
  overwhelmingly runtime-behavior tests. If the NA share is higher, treat it as a
  defect to investigate, not as a finished state.
- Every `not-applicable` row must name one of the allowed mechanics categories in
  "When to mark `not-applicable`" and give a concrete reason. "out of scope:
  platform/datasource/artifact/git/exec" is **not** allowed — those are in scope.
- Marking a test `not-applicable` because the Rust code "does not exist yet" is
  forbidden. That is `pending`.

This is the minimum bar. The Rust test suite may go further — additional edge
cases, Rust-specific invariants, performance assertions — but extras do not
substitute for parity with the TypeScript baseline.

Your goal is to drive the Rust test suite toward that minimum bar by maintaining
the compact `docs/parity/renovate-test-map.md` root index plus the linked
per-spec detail files as a precise per-test audit trail, and by writing or
annotating Rust tests to close the gaps.

Run autonomously. Do not ask questions. Make the best engineering decision from
local evidence. Never stop because of a missing credential or external service —
document the blocker, skip the entry, and continue.

Never modify `prompts/claude-loop-test-parity.md` while following this prompt.

---

## Workspace layout

- Normal working directory: `~/Projects/renovate-rust-experiement`
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

## The target files

`docs/parity/renovate-test-map.md` is the compact root index.
Per-test detail rows live in one Markdown file per upstream spec path:

```text
docs/parity/<original Renovate spec path>.md
```

Example:

```text
docs/parity/lib/modules/manager/ansible-galaxy/extract.spec.ts.md
```

Read the root index at the start of every iteration, then open the linked
detail file for the spec you will work on. The root index must stay small and
must begin with a summary block updated on every commit:

```markdown
# Renovate Test Map

**Progress:** 340 / 1,420 total tests ported (24%) · pending 980 · not-applicable 100 (7% of total) — updated 2026-04-29

This file is intentionally compact. It tracks one row per upstream Renovate `.spec.ts` file and uses only two root statuses:

- `Done` means the detail file has no `pending` rows; all actionable tests are ported, and any out-of-scope rows are documented as `not-applicable` in the detail file.
- `Not done` means the detail file still has at least one `pending` row. Open the linked detail file for per-test progress and reasons.
```

The root index table has only these columns:

```markdown
| Spec file | Status | Details |
|---|---|---|
| `lib/modules/manager/ansible-galaxy/extract.spec.ts` | Done | [details](lib/modules/manager/ansible-galaxy/extract.spec.ts.md) |
```

The root index must not contain per-test rows, partial counts, or statuses other
than `Done` and `Not done`.

### Count definitions

- **Total tests** (per spec file): count of `it(`, `test(`, `it.each(`,
  `test.each(`, and tagged-template `it.each\`` / `test.each\`` call sites.
- **Ported**: rows with Status `ported`.
- **Pending**: rows with Status `pending` (in scope, not yet ported).
- **Not-applicable**: rows with Status `not-applicable` (allowed mechanics
  categories only). `Total = Ported + Pending + Not-applicable`.
- **Actionable**: `Total − Not-applicable` (= Ported + Pending). Still tracked
  per file, but it is **not** the headline number.
- The headline global fraction is **`Ported / Total`**, reported next to the
  `not-applicable` count and `NA / Total` percentage. Reporting `Ported /
  Actionable` alone is forbidden: it hides progress made by shrinking the
  denominator. A file is `Status: ported` only when it has zero `pending` rows
  **and** its `not-applicable` rows all cite an allowed mechanics category.
- **NA budget:** `NA / Total` is expected to be a small minority (well under
  ~25%). If it is higher, the surplus NA rows are suspect — re-audit them
  (Phase 0.5) before treating the map as closed.

### Recount commands

```sh
# Run from the renovate-rust project root
rg -o "\| ported \|" docs/parity --glob "*.spec.ts.md" | wc -l
rg -o "\| pending \|" docs/parity --glob "*.spec.ts.md" | wc -l
rg -o "\| not-applicable \|" docs/parity --glob "*.spec.ts.md" | wc -l
```

---

## Per-test format

The root index preserves the existing spec order. Do not reorder root rows
unless you are doing a deliberate full re-sort.

Each detail file contains exactly one upstream `.spec.ts` file's tracking
section. If a spec path is `lib/modules/manager/foo/extract.spec.ts`, its detail
file is `docs/parity/lib/modules/manager/foo/extract.spec.ts.md`.

Every detail file follows this template. The table **always has six columns** —
the Reason column is `—` for `ported` and `pending` rows.

```markdown
# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

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
- Root index status:
  - `Done` — the detail file has zero `pending` rows, including files whose
    remaining rows are documented as `not-applicable`
  - `Not done` — the detail file has one or more `pending` rows
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
   - Update or create the corresponding row in the matching
     `docs/parity/<spec path>.md` detail file.
   - Update the root `docs/parity/renovate-test-map.md` row to `Done` only
     when the detail file has no `pending` rows; otherwise keep it `Not done`.
5. **If no confident match is found after inspecting the most likely spec file,
   classify the test as Rust-specific and move on.** Do not spend more than a
   few minutes on any one test. Leave it uncommented; it does not belong in the
   test map.

Process one Rust file (or a small batch) per iteration, commit, then continue.

---

### Phase 0.5 — Re-audit `not-applicable` rows against current scope

Existing `not-applicable` rows were classified under a narrower "extraction
layer only" scope and many parked real, in-scope work (datasource HTTP, platform
APIs, artifacts/exec, git, PR creation) as NA. The project scope is now a **full
drop-in replacement**. Those rows are wrong and must be reclassified.

This phase runs until the NA budget is satisfied (NA / Total well under ~25% and
every remaining NA row cites an allowed mechanics category). Treat it as
high-priority work alongside Phase 3.

#### Find suspect NA rows

```sh
# All not-applicable reasons, ranked — anything mentioning platform, datasource,
# httpMock, artifact, exec, git, PR/branch, release note, dashboard, template,
# or "yet" is almost certainly mis-classified.
grep -rhoE "\| not-applicable \|[^|]*\|[^|]*\| [^|]+\|" docs/parity --include='*.spec.ts.md' \
  | sed -E 's/.*not-applicable \|[^|]*\|[^|]*\| //; s/ *\|$//' \
  | sort | uniq -c | sort -rn
```

#### Reclassification rule

For each NA row, ask: *is this behavior reachable from a plain `renovate` CLI run
on a local repo?* If yes → it is in scope → change Status to `pending` and clear
the Reason to `—`. Only keep `not-applicable` when the test matches an allowed
mechanics category in "When to mark `not-applicable`" below.

Reasons that MUST be reclassified to `pending` (non-exhaustive):

- "tests platform HTTP API … via httpMock" → platform clients are in scope.
- "out of scope: artifact management … external package managers" → in scope.
- "datasource … HTTP layer yet" / any "… yet" → in scope, not done yet.
- "tests simple-git / real git repos" → git layer is in scope.
- "branch update orchestration", "PR creation/update", "dependency dashboard",
  "release note fetching", "commit message / Handlebars template" → all in scope.

Reasons that may legitimately stay `not-applicable`: genuine TypeScript type-
system tests, Vitest/Jest mock-framework internals, TS module-resolution tests,
and Zod/TS-library-specific schema-internals tests (the *behavior* may still need
a Rust equivalent — only the TS-mechanics assertion is NA).

#### Procedure

1. Pick one detail file (or a small batch) with suspect NA rows.
2. Reclassify per the rule above; flip mis-scoped NA rows to `pending`.
3. Update the detail file's Total/Ported/Actionable/Status header and the root
   index row (`Done` → `Not done` if it now has `pending` rows).
4. Update the root summary block counts. Commit:
   `docs(parity): re-audit not-applicable rows in <area> (NA -N, pending +N)`.

---

### Phase 1 — Inventory a spec file

When a `.spec.ts` file has no detail file in the per-test format yet:

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

4. Build the detail file with one row per call site, tracking `describe`
   nesting to assign the right `###` subsection. For deeply nested files, parse
   nesting level by scanning for `describe(` opening lines and their matching
   `})` closings in sequence.

5. Initial Status for each row:
   - `it(` / `test(` / `it.each(` / `test.each(` → `pending`
   - `it.skip(` / `test.skip(` / `xit(` / `xtest(` → `pending`, Reason =
     `intentionally skipped in TypeScript source — verify before porting`
     (do **not** default to `not-applicable`; a human should decide)

6. **For large spec files (> 20 tests):** commit partial Phase 1 progress after
   each batch of rows. Note how many remain in the commit message:
   `docs(parity): inventory dockerfile extract spec (40 / 75 rows)`

7. Add the spec to the root index with status `Not done` unless the newly
   created detail file has zero `pending` rows, in which case use `Done`.

---

### Phase 2 — Map existing Rust coverage

When a detail file exists and one or more rows are `pending`:

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

When a detail file has `pending` rows and Phase 2 mapping is complete for that
file:

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
   implementation prompt.

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

   Also update the root index row:
   - `Done` if the detail file has no remaining `pending` rows.
   - `Not done` if any `pending` row remains.

9. **For large spec files (many pending rows):** port a batch per iteration,
   commit, and continue. Do not attempt to port an entire large file in one
   iteration.

---

## When to mark `not-applicable`

Use `not-applicable` when a test should **never** be ported. The Reason column
is mandatory and must name one of the allowed categories below. When in doubt,
port the test — Rust can ignore TypeScript mechanics and test the underlying
runtime behavior directly. This is a **small** set; see the NA budget.

**Allowed categories (the only valid `not-applicable` reasons):**

| Situation | Reason text |
|---|---|
| Verifies TypeScript type shapes, generics, or type guards | `TypeScript type-system test; no runtime behavior` |
| Tests Jest/Vitest mock infrastructure or fixture-loading helpers | `mocking framework internals` |
| Tests TypeScript module import/export resolution | `TypeScript module system` |
| Tests a Zod/TS-library schema-internal with no Rust runtime analogue | `TS-library-specific; behavior covered by serde elsewhere` |
| Feature exists only for hosted/managed Renovate | `out of scope: hosted only` / `out of scope: GitHub App` |

**NOT `not-applicable` — these are in scope, mark `pending` and port them:**

- platform HTTP/API interactions (tested via `httpMock`, `got`, `Http`,
  `GithubHttp`, etc.) — port against the Rust platform/HTTP clients;
- datasource lookups and version-lookup pipelines — in scope;
- artifact/lockfile updates and external package-manager / `child_process` exec;
- git operations (`simple-git`, real-repo tests) — port against the Rust git layer;
- branch/PR/MR creation and update, commit messages, Handlebars/template
  rendering, release notes, dependency dashboard, onboarding, config
  parsing/migration/validation.

The Node mechanism (mock, HTTP stub, child process, real git) is never itself a
reason to skip — replace it with the Rust equivalent and port the behavior.

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

1b. **Phase 0.5 re-audit** — while the NA budget is exceeded (NA / Total over
   ~25%, which it currently is), prioritize this: reclassify one batch of
   mis-scoped `not-applicable` rows to `pending`. Commit. This comes before new
   Phase 3 porting — fix the baseline before adding to it.

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

If a spec file has no Rust counterpart yet, still create the Phase 1 detail
file and root index row. Use `—` for Rust file and Rust test name. The detail
file documents the gap so Phase 3 can implement it later.

---

## Verification during implementation parity

Do not run verification commands automatically before or after every commit.
Run checks only when the operator explicitly asks for them, or when a task
instruction names a specific command.

During the implementation/test-parity phase, focus on porting runtime behavior
and covering Renovate `.spec.ts` rows. Keep changed code compiling and run
focused Rust tests or crate-level compile/test commands for the current slice
when useful. Do not run Rustfmt or Clippy as routine hygiene, and do not include
formatting-only churn in parity commits. Formatting and lint cleanup are
deferred until the terminal quality pass after source and test parity are
otherwise closed.

For ordinary parity work, prefer targeted compile/test checks such as:

```sh
cargo test -p <crate> <ported_test_filter> -- --nocapture
cargo test -p <crate> <ported_test_filter> --lib
```

Only when the operator explicitly requests full Rust verification, or when the
terminal quality pass is reached, use the full gate set:

```sh
cargo build --workspace --all-features
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
```

Run doctests only when explicitly requested or during the terminal quality pass:

```sh
cargo test --doc --workspace --all-features
```

If requested checks fail, fix failures that block the current parity slice. If a
formatting or Clippy failure predates the parity work, record it for the final
hardening pass instead of diverting the implementation loop.

---

## Commit rules

- Follow `AGENTS.md`, `CLAUDE.md`, and `COMMITS.md`.
- Every commit must include exactly one `Co-authored-by` trailer for the active
  agent, as defined in `AGENTS.md`. Use the Claude trailer when Claude Code
  creates the commit and the Codex trailer when Codex creates the commit.
- Multiple commits per session are expected and correct — one per coherent unit
  of work (one Phase 0 batch, one spec file mapped, one batch of tests ported).
- Always commit the matching detail file and `renovate-test-map.md` root index
  together with any Rust source files changed in the same unit. Exception: if
  Phase 0 backfill finds no test-map match for a Rust file, commit the
  comment-only change without a map update.
- After every commit, push all committed local changes to the matching remote
  branch.
- Update the summary block counts on every commit.
- Example commit messages:
  - `docs(parity): inventory ansible-galaxy extract spec (14 tests)`
  - `test(ansible-galaxy): port 5 extract tests from renovate spec`
  - `docs(parity): map existing Rust coverage for cargo extract spec`
  - `test(parity): backfill Ported comments in pre-commit extractor`
  - `docs(parity): inventory dockerfile extract spec (40 / 75 rows)`

---

## Completion

Test parity is **not** complete when the headline percentage looks high. It is
complete only as part of the shared terminal state in
`prompts/claude-loop-renovate-rust.md`. For the test side specifically:

- Every `.spec.ts` root row is `Done`; every detail file has zero `pending` rows.
- Every `not-applicable` row cites an allowed mechanics category, and `NA / Total`
  is within budget after a Phase 0.5 re-audit.
- **Source ↔ test cross-check:** no spec is `Done` while a source file it
  exercises is `partial`/`stub`/`not-started` in `renovate-source-map.md`. If the
  two maps disagree, stop and reconcile.
- The differential parity harness is green for the behaviors these tests cover.

If any of these is false, the loop continues. A single committed unit, a clean
worktree, or a turn limit is never completion — keep working until the operator
stops you or the full terminal state holds.

## Start now

1. Run the Phase 0 audit to get a count of unattributed tests.
2. Read `docs/parity/renovate-test-map.md` to understand current state, then
   open the linked detail file for the spec you will work on.
3. Recount the summary block and update it if stale.
4. Work through the Iteration order until the available work budget runs out, committing
   after each unit of work.

At every commit:
- Operator-requested checks pass, or pre-existing/blocking failures are
  documented.
- Every Rust test you touched or wrote has `// Ported:` if it maps to a
  TypeScript spec test.
- `renovate-test-map.md` summary block reflects accurate current counts.
- The root index row for every touched spec is `Done` only when its detail file
  has no `pending` rows; otherwise it is `Not done`.
