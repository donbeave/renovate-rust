You are working on the renovate-rust repository. Your sole goal in this loop is
to maintain and advance `docs/parity/renovate-test-map.md` with per-test-case
granularity so it becomes a reliable audit trail for test port coverage.

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
- **Status** summary: `ported` (all done) · `partial` (some done) · `pending` (none done) · `not-applicable` (out of scope).
- Subsection heading (`###`) = the full `describe()` path joined with ` › ` when nested.
  Example: outer describe `extractPackageFile()` only → `### \`extractPackageFile()\``
  Nested: outer `extractPackageFile()` + inner `git deps` → `### \`extractPackageFile() › git deps\``
- Table columns: **Original test name** (exact string from `it(` call) · **Line** (1-based line number of the `it(` call) · **Status** · **Rust file** (filename only, no path) · **Rust test name** (the `#[test] fn` name).
- Use `—` in Rust file and Rust test name when the test is not yet ported.
- Status values per row: `ported` · `pending` · `not-applicable`.

---

## Three-phase workflow

Determine the current phase for each spec file by reading `renovate-test-map.md`:

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

Work through spec files in this priority order:

1. Any file that already has a Phase 1 or Phase 2 section but still has
   `pending` rows — advance it to Phase 3 if mapping is done, otherwise
   finish Phase 2 mapping first.
2. Spec files listed in the current `renovate-test-map.md` that have not yet
   been converted to the new format.
3. Spec files discovered in `../renovate/lib/` that are not mentioned at all.

Within each priority group, prefer spec files where the Rust extractor already
exists (most likely to have matchable tests).

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

1. Read `docs/parity/renovate-test-map.md` to understand the current state.
2. Identify the highest-priority spec file to advance (see Iteration order above).
3. Determine which phase that file is in.
4. Execute that phase, update `renovate-test-map.md`, run quality gates, commit.
5. Move to the next file and repeat until the loop budget is exhausted.
