#!/usr/bin/env python3
"""Module-level Renovate parity coverage report.

Walks upstream Renovate spec files and Rust // Ported: comments, groups the
counts by module, and emits a per-module coverage report. Replaces the manual
per-it() ledger.

Modes:
    parity_coverage.py report          # ledger summary (default)
    parity_coverage.py ledger > FILE   # write docs/parity/modules.md
    parity_coverage.py gaps <module>   # list upstream it() calls without
                                       # a // Ported: counterpart
    parity_coverage.py orphans         # // Ported: comments that resolve to
                                       # no upstream spec file
    parity_coverage.py verify          # check every // Ported: comment's
                                       # description and line number against
                                       # the actual upstream it() call
    parity_coverage.py json            # raw JSON dump of the analysis

The script is read-only. It never edits Rust source, spec files, or markdown.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

# Default expected layout: ../renovate next to renovate-rust/.
RUST_ROOT = Path(__file__).resolve().parent.parent
REPO_ROOT = RUST_ROOT.parent
RENOVATE_ROOT = REPO_ROOT / "renovate"
RENOVATE_LIB = RENOVATE_ROOT / "lib"

# ---------------------------------------------------------------------------
# Spec discovery and module classification
# ---------------------------------------------------------------------------

# Module taxonomy. Each rule maps a spec path (relative to lib/) to a module
# id and a human-readable group. Order matters: first match wins.
MODULE_RULES: list[tuple[re.Pattern[str], str, str]] = [
    (re.compile(r"^modules/manager/([^/]+)/"),     "manager/{0}",     "managers"),
    (re.compile(r"^modules/manager/[^/]+\.spec\.ts$"), "manager/_common", "managers"),
    (re.compile(r"^modules/datasource/([^/]+)/"),  "datasource/{0}",  "datasources"),
    (re.compile(r"^modules/datasource/[^/]+\.spec\.ts$"), "datasource/_common", "datasources"),
    (re.compile(r"^modules/platform/([^/]+)/"),    "platform/{0}",    "platforms"),
    (re.compile(r"^modules/platform/[^/]+\.spec\.ts$"), "platform/_common", "platforms"),
    (re.compile(r"^modules/versioning/([^/]+)/"),  "versioning/{0}",  "versioning"),
    (re.compile(r"^modules/versioning/[^/]+\.spec\.ts$"), "versioning/_common", "versioning"),
    # Workers: group by the first subdir under workers/.
    (re.compile(r"^workers/([^/]+)/"),             "worker/{0}",      "workers"),
    (re.compile(r"^workers/"),                     "worker/_root",    "workers"),
    # Config + util grouped by first subdir.
    (re.compile(r"^config/([^/]+)/"),              "config/{0}",      "config"),
    (re.compile(r"^config/"),                      "config/_root",    "config"),
    (re.compile(r"^util/([^/]+)/"),                "util/{0}",        "util"),
    (re.compile(r"^util/"),                        "util/_root",      "util"),
    (re.compile(r"^logger/"),                      "logger",          "infra"),
    (re.compile(r"^instrumentation/"),             "instrumentation", "infra"),
    (re.compile(r"^constants/"),                   "constants",       "infra"),
    (re.compile(r"^data/"),                        "data",            "infra"),
    (re.compile(r"^types/"),                       "types",           "infra"),
    # Top-level files like proxy.spec.ts, renovate.spec.ts.
    (re.compile(r"^[^/]+\.spec\.ts$"),             "cli/_root",       "cli"),
]


def classify(spec_rel: str) -> tuple[str, str]:
    """Return (module_id, group) for a spec path relative to renovate/lib/."""
    for rule, mod_template, group in MODULE_RULES:
        m = rule.match(spec_rel)
        if m:
            return mod_template.format(*m.groups()), group
    return "other", "other"


# ---------------------------------------------------------------------------
# Upstream spec scanning
# ---------------------------------------------------------------------------

# Matches a leading-indent call to it(...), test(...), it.each(...),
# test.each(...), with .skip / .only / .failing / .concurrent / .each variants.
# Captures one call site per match. For .each( and .each` we count one site
# regardless of how many data rows it has — that's how the existing test map
# already counted them.
IT_CALL_RE = re.compile(
    r"""
    ^[ \t]+                       # must be indented (avoids commented top-level)
    (?:it|test)                    # base
    (?:\.(?:each|skip|only|failing|concurrent|todo))?  # optional variant chain
    (?:\.(?:each|skip|only|failing|concurrent|todo))?  # up to two chained
    \s*
    [\(\`]                         # opens with ( or backtick (tagged each)
    """,
    re.VERBOSE,
)

# Also catch xit / xtest (skipped, counts as pending).
XIT_RE = re.compile(r"^[ \t]+(?:xit|xtest)\s*\(")


@dataclass
class SpecFile:
    rel_path: str                 # relative to renovate/lib/
    module_id: str
    group: str
    it_count: int


def scan_specs() -> list[SpecFile]:
    if not RENOVATE_LIB.is_dir():
        sys.exit(f"renovate reference not found at {RENOVATE_LIB}")
    specs: list[SpecFile] = []
    for path in sorted(RENOVATE_LIB.rglob("*.spec.ts")):
        rel = path.relative_to(RENOVATE_LIB).as_posix()
        count = 0
        with path.open(encoding="utf-8", errors="replace") as fh:
            for line in fh:
                if IT_CALL_RE.match(line) or XIT_RE.match(line):
                    count += 1
        module_id, group = classify(rel)
        specs.append(SpecFile(rel, module_id, group, count))
    return specs


# ---------------------------------------------------------------------------
# Rust // Ported: comment scanning
# ---------------------------------------------------------------------------

# Captures "<spec ref>" out of `// Ported: "..." — <ref>` (em dash) or
# `// Ported: "..." - <ref>` (regular dash). <ref> ends at "line N", end of
# string, or another em/hyphen.
PORTED_RE = re.compile(
    r"""
    //\s*Ported:\s*
    (?P<desc>
        "(?:\\.|[^"\\])*"           # double-quoted, allow \" escapes
      | '(?:\\.|[^'\\])*'           # single-quoted, allow \' escapes
    )
    \s*[—–-]\s*                     # separator (em, en, or hyphen)
    (?P<ref>[^\s][^\n]*)            # the spec reference (greedy, normalized later)
    """,
    re.VERBOSE,
)

# Permissive fallback. Triggers when:
#  - the description has unescaped inner quotes, e.g. `"equals("$x", "$y")"`,
#    so PORTED_RE stops at the first inner ", or
#  - the reference is on a continuation line (`// ─ <ref>` on the next line).
# Captures everything between the first quote after "Ported:" and the em dash.
PORTED_RE_LOOSE = re.compile(
    r"""
    //\s*Ported:\s*
    (?P<q>["'])
    (?P<desc>.*?)               # any chars up to...
    (?P=q)                      # ...the same quote
    (?:\s*\([^)]*\))?           # optional ` (parenthetical)` after the desc
    \s*[—–-]\s*
    (?P<ref>[^\n]*)
    """,
    re.VERBOSE | re.DOTALL,
)

# Continuation marker on a separate line: `//   — <ref>` or `//   - <ref>`.
PORTED_CONT_RE = re.compile(r"^\s*//\s*[—–-]\s*(?P<ref>\S.*)$")

LINE_NO_RE = re.compile(r"\s+lines?\s+(\d+)")


def _normalize_ref(ref: str) -> tuple[str, int | None]:
    """Return (path, cited_line). Strips trailing ` line N` and any
    parenthetical commentary."""
    ref = ref.strip()
    cited_line: int | None = None
    m = LINE_NO_RE.search(ref)
    if m:
        cited_line = int(m.group(1))
        ref = ref[:m.start()] + ref[m.end():]
    # Cut at parenthetical commentary like ` (parse modules with phases)`.
    ref = re.split(r"\s+\(", ref, maxsplit=1)[0]
    return ref.strip().rstrip(",").rstrip(), cited_line


def _unquote(s: str) -> str:
    """Strip surrounding single or double quotes from a captured token."""
    if len(s) >= 2 and s[0] == s[-1] and s[0] in {'"', "'"}:
        return s[1:-1]
    return s


@dataclass
class PortedComment:
    rust_path: Path
    rust_line: int
    description: str            # includes the surrounding quotes for display
    spec_ref: str               # normalized spec reference (no `line N` tail)
    cited_line: int | None      # line number written into the comment, if any


def scan_ported() -> list[PortedComment]:
    crates = RUST_ROOT / "crates"
    out: list[PortedComment] = []
    for path in sorted(crates.rglob("*.rs")):
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        for i, line in enumerate(lines, start=1):
            if "// Ported:" not in line:
                continue

            # 1. Strict same-line match.
            m = PORTED_RE.search(line)
            if m:
                ref, cited = _normalize_ref(m.group("ref"))
                out.append(PortedComment(path, i, m.group("desc"), ref, cited))
                continue

            # 2. Same line, but the description has unescaped inner quotes
            # or the ref is on this line via the loose pattern.
            m = PORTED_RE_LOOSE.search(line)
            if m:
                desc = m.group("q") + m.group("desc") + m.group("q")
                ref, cited = _normalize_ref(m.group("ref"))
                out.append(PortedComment(path, i, desc, ref, cited))
                continue

            # 3. Two-line form: this line has the description; the next line
            # has the `//  — <ref>` continuation.
            m = re.search(
                r'//\s*Ported:\s*(?P<q>["\'])(?P<desc>.*?)(?P=q)\s*$',
                line,
            )
            if m and i < len(lines):
                cont = PORTED_CONT_RE.match(lines[i])     # i is 1-based, lines is 0-based
                if cont:
                    desc = m.group("q") + m.group("desc") + m.group("q")
                    ref, cited = _normalize_ref(cont.group("ref"))
                    out.append(PortedComment(path, i, desc, ref, cited))
                    continue

            # 4. Could not parse at all.
            out.append(PortedComment(path, i, "", "", None))
    return out


# ---------------------------------------------------------------------------
# Spec-ref resolution
# ---------------------------------------------------------------------------

# The // Ported: comment refs are inconsistent in this codebase: some are
# relative to renovate/lib/, some to lib/modules/manager/, some are bare
# filenames. Resolve by trying every base prefix and finally a basename match.

BASE_PREFIXES = [
    "",                          # already lib-relative
    "modules/manager/",          # manager-relative (AGENTS.md convention)
    "modules/datasource/",
    "modules/platform/",
    "modules/versioning/",
    "modules/",                  # modules/foo/bar/x.spec.ts -> foo/bar/x.spec.ts
    "workers/",
    "util/",
    "config/",
]


def build_resolver(specs: list[SpecFile]):
    by_rel = {s.rel_path: s for s in specs}
    by_basename: dict[str, list[SpecFile]] = defaultdict(list)
    for s in specs:
        by_basename[Path(s.rel_path).name].append(s)

    def resolve(ref: str) -> SpecFile | None:
        if not ref:
            return None
        # Strip a leading "lib/" if present.
        ref = ref.removeprefix("lib/")
        # Drop trailing " line N" if it slipped through the regex.
        ref = re.sub(r"\s+line\s+\d+\s*$", "", ref)
        # 1. Direct match against any known base prefix.
        for prefix in BASE_PREFIXES:
            cand = f"{prefix}{ref}".lstrip("/")
            if cand in by_rel:
                return by_rel[cand]
        # 2. Bare filename — unique basename match counts.
        basename = Path(ref).name
        matches = by_basename.get(basename, [])
        if len(matches) == 1:
            return matches[0]
        return None

    return resolve


# ---------------------------------------------------------------------------
# Analysis
# ---------------------------------------------------------------------------

@dataclass
class ModuleStat:
    module_id: str
    group: str
    spec_files: list[SpecFile] = field(default_factory=list)
    # Distinct (spec_path, description) pairs covered by at least one Rust test.
    covered: set[tuple[str, str]] = field(default_factory=set)
    # Total number of // Ported: comments referencing this module (incl. dupes).
    comment_count: int = 0

    @property
    def total_it(self) -> int:
        return sum(s.it_count for s in self.spec_files)

    @property
    def ported_count(self) -> int:
        """Distinct upstream tests covered. Capped at total_it."""
        return min(len(self.covered), self.total_it)

    @property
    def duplicate_count(self) -> int:
        """Surplus // Ported: comments beyond the first one per upstream test."""
        return max(0, self.comment_count - len(self.covered))

    @property
    def coverage_pct(self) -> float:
        return (self.ported_count / self.total_it * 100) if self.total_it else 0.0


@dataclass
class Analysis:
    modules: dict[str, ModuleStat]
    orphan_ported: list[tuple[Path, int, str]]    # ref did not resolve
    malformed_ported: list[tuple[Path, int]]      # missing reference
    total_specs: int
    total_it: int
    total_ported: int                             # distinct, deduped
    total_comments: int                           # raw // Ported: count


def analyze() -> Analysis:
    specs = scan_specs()
    ported = scan_ported()
    resolve = build_resolver(specs)

    modules: dict[str, ModuleStat] = {}
    for s in specs:
        mod = modules.setdefault(s.module_id, ModuleStat(s.module_id, s.group))
        mod.group = s.group
        mod.spec_files.append(s)

    orphans: list[tuple[Path, int, str]] = []
    malformed: list[tuple[Path, int]] = []
    total_comments = 0
    for c in ported:
        if not c.spec_ref:
            malformed.append((c.rust_path, c.rust_line))
            continue
        spec = resolve(c.spec_ref)
        if spec is None:
            orphans.append((c.rust_path, c.rust_line, c.spec_ref))
            continue
        total_comments += 1
        mod = modules[spec.module_id]
        mod.comment_count += 1
        mod.covered.add((spec.rel_path, c.description))

    total_ported = sum(m.ported_count for m in modules.values())
    return Analysis(
        modules=modules,
        orphan_ported=orphans,
        malformed_ported=malformed,
        total_specs=len(specs),
        total_it=sum(s.it_count for s in specs),
        total_ported=total_ported,
        total_comments=total_comments,
    )


# ---------------------------------------------------------------------------
# Report formatters
# ---------------------------------------------------------------------------

def fmt_pct(n: int, total: int) -> str:
    if total == 0:
        return "—"
    return f"{n}/{total} ({n / total * 100:.0f}%)"


def report(a: Analysis) -> str:
    lines: list[str] = []
    dupes = sum(m.duplicate_count for m in a.modules.values())
    lines.append("Renovate parity coverage")
    lines.append("=" * 60)
    lines.append(f"Spec files:        {a.total_specs}")
    lines.append(f"Upstream it()s:    {a.total_it}")
    lines.append(f"Distinct ported:   {a.total_ported}  ({a.total_ported / a.total_it * 100:.1f}%)")
    lines.append(f"// Ported: total:  {a.total_comments}  ({dupes} duplicate, "
                 f"{len(a.orphan_ported)} orphan, "
                 f"{len(a.malformed_ported)} malformed)")
    lines.append("")
    by_group: dict[str, list[ModuleStat]] = defaultdict(list)
    for mod in a.modules.values():
        by_group[mod.group].append(mod)

    for group in sorted(by_group):
        mods = sorted(by_group[group], key=lambda m: m.module_id)
        ported = sum(m.ported_count for m in mods)
        total = sum(m.total_it for m in mods)
        lines.append(f"[{group}] {fmt_pct(ported, total)} across {len(mods)} modules")
        worst = sorted(
            (m for m in mods if m.total_it > 0),
            key=lambda m: m.coverage_pct,
        )[:5]
        for m in worst:
            lines.append(
                f"    {m.module_id:32s} {fmt_pct(m.ported_count, m.total_it)}"
            )
        lines.append("")
    return "\n".join(lines)


LEDGER_HEADER = """\
# Renovate Module Ledger

Auto-generated by `scripts/parity_coverage.py ledger`.
**Do not hand-edit Coverage or Total it() columns** — they are recomputed from
upstream `.spec.ts` files and `// Ported:` comments on each run.

Hand-edited columns:
- **Impl** — `none` · `partial` · `full`. Implementation agent owns.
- **Notes** — free-text gap summary. Implementation agent owns.

To regenerate:

```sh
python3 scripts/parity_coverage.py ledger > docs/parity/modules.md
```

To find untested upstream tests for a module:

```sh
python3 scripts/parity_coverage.py gaps manager/cargo
```

---

"""


def ledger(a: Analysis, prior_impl: dict[str, tuple[str, str]] | None = None) -> str:
    """Render the module ledger markdown.

    prior_impl maps module_id -> (impl_status, notes). When provided, the
    Impl/Notes columns carry the previous values forward; otherwise both
    default to `?` / `—`.
    """
    prior_impl = prior_impl or {}
    dupes = sum(m.duplicate_count for m in a.modules.values())
    out: list[str] = [LEDGER_HEADER]
    out.append(f"**Total:** {a.total_ported} / {a.total_it} distinct upstream "
               f"it() tests ported ({a.total_ported / a.total_it * 100:.1f}%) "
               f"across {len(a.modules)} modules.")
    out.append(f"**Quality:** {a.total_comments} `// Ported:` comments → "
               f"{dupes} duplicate, {len(a.orphan_ported)} orphan, "
               f"{len(a.malformed_ported)} malformed.\n")

    by_group: dict[str, list[ModuleStat]] = defaultdict(list)
    for mod in a.modules.values():
        by_group[mod.group].append(mod)

    group_order = ["managers", "datasources", "platforms", "versioning",
                   "workers", "config", "util", "infra", "cli", "other"]
    for group in group_order:
        if group not in by_group:
            continue
        mods = sorted(by_group[group], key=lambda m: m.module_id)
        g_ported = sum(m.ported_count for m in mods)
        g_total = sum(m.total_it for m in mods)
        out.append(f"## {group}  —  {fmt_pct(g_ported, g_total)}\n")
        out.append("| Module | Spec files | Impl | Coverage | Notes |")
        out.append("|---|---|---|---|---|")
        for m in mods:
            impl, notes = prior_impl.get(m.module_id, ("?", "—"))
            out.append(
                f"| `{m.module_id}` | {len(m.spec_files)} | {impl} | "
                f"{fmt_pct(m.ported_count, m.total_it)} | {notes} |"
            )
        out.append("")

    return "\n".join(out)


def gaps(a: Analysis, module_id: str) -> str:
    mod = a.modules.get(module_id)
    if mod is None:
        return f"unknown module: {module_id}\n"
    # Group covered descriptions by spec for quick lookup.
    covered_by_spec: dict[str, set[str]] = defaultdict(set)
    for spec_path, desc in mod.covered:
        covered_by_spec[spec_path].add(desc)

    out: list[str] = []
    out.append(f"# Untested upstream it()s for {module_id}")
    out.append("")
    out.append(f"Coverage: {fmt_pct(mod.ported_count, mod.total_it)}")
    out.append("")
    for spec in sorted(mod.spec_files, key=lambda s: s.rel_path):
        ported_descs = covered_by_spec.get(spec.rel_path, set())
        if len(ported_descs) >= spec.it_count:
            continue
        gap = spec.it_count - len(ported_descs)
        out.append(f"## {spec.rel_path}  —  {len(ported_descs)}/{spec.it_count} "
                   f"({gap} missing)")
        out.append(f"https://github.com/renovatebot/renovate/blob/main/lib/"
                   f"{spec.rel_path}")
        out.append("")
        path = RENOVATE_LIB / spec.rel_path
        try:
            with path.open(encoding="utf-8", errors="replace") as fh:
                for i, line in enumerate(fh, start=1):
                    if IT_CALL_RE.match(line) or XIT_RE.match(line):
                        out.append(f"  L{i:4d}  {line.strip()}")
        except FileNotFoundError:
            out.append(f"  (spec file missing: {path})")
        out.append("")
    return "\n".join(out)


def orphans_report(a: Analysis) -> str:
    out: list[str] = [f"# {len(a.orphan_ported)} orphan // Ported: comments\n"]
    out.append("These references in Rust code did not resolve to any upstream "
               "spec file. Either the spec was deleted upstream or the "
               "reference is malformed.\n")
    for path, line_no, ref in a.orphan_ported:
        rel = path.relative_to(RUST_ROOT)
        out.append(f"  {rel}:{line_no}  →  {ref}")
    return "\n".join(out) + "\n"


# ---------------------------------------------------------------------------
# Verify: spec-line/description match
# ---------------------------------------------------------------------------

# Capture the first quoted argument of an it(...) / test(...) / it.each(...)(...)
# / it.skip(...) / xit(...) call. Handles single/double/template quotes.
IT_DESC_RE = re.compile(
    r"""
    \b
    (?:it|test|xit|xtest)
    (?:\.(?:each|skip|only|failing|concurrent|todo))?
    (?:\.(?:each|skip|only|failing|concurrent|todo))?
    \s*
    (?:
        \(\s*\[[^\]]*\]\s*\)\s*   # it.each([...]) followed by (...)
    )?
    [\(\`]?                        # opening ( or backtick (tagged each)
    \s*
    (?P<q>['"`])                   # opening quote
    (?P<desc>.*?)                  # description body (non-greedy)
    (?P=q)                         # matching closing quote
    """,
    re.VERBOSE,
)


EACH_OPENER_RE = re.compile(
    r"^[ \t]+(?:it|test|xit|xtest)\.each\s*[\(\`]"
)


def _spec_it_at(spec_path: Path, line_no: int) -> str | None:
    r"""Return the it()/test() description for the test call site whose
    *opener* is at line_no. For single-line ``it('desc', ...)`` calls this
    is the same as reading line_no. For multi-line ``it.each`...`('desc', ...)``
    or ``it.each([...])('desc', ...)`` calls, this scans forward until it
    finds the description argument.

    Returns None if line_no is not a recognizable test opener.
    """
    lines = _read_spec_lines(spec_path)
    if lines is None or line_no < 1 or line_no > len(lines):
        return None
    line = lines[line_no - 1]

    # Fast path: description on the same line.
    m = IT_DESC_RE.search(line)
    if m:
        return m.group("desc")

    # Multi-line form: it.each`...`(desc, fn) or it.each([...])(desc, fn).
    if not EACH_OPENER_RE.match(line):
        return None

    # Read up to 80 lines forward as one chunk and scan for the description.
    end = min(line_no + 80, len(lines))
    chunk = "\n".join(lines[line_no - 1:end])

    # Backtick form: find the closing backtick that precedes `(`.
    m2 = re.search(
        r"`\s*\(\s*(?P<q>['\"`])(?P<desc>(?:\\.|(?!(?P=q)).)*)(?P=q)",
        chunk, re.DOTALL,
    )
    if m2:
        return m2.group("desc")
    # Array form: find `)(quote ... quote)` after the it.each(.
    m2 = re.search(
        r"\)\s*\(\s*(?P<q>['\"`])(?P<desc>(?:\\.|(?!(?P=q)).)*)(?P=q)",
        chunk, re.DOTALL,
    )
    if m2:
        return m2.group("desc")
    return None


def _spec_all_its(spec_path: Path) -> list[tuple[int, str]]:
    """Return [(line_no, description)] for every it()/test() call in the
    file. Includes single-line and multi-line `it.each` openers."""
    lines = _read_spec_lines(spec_path)
    if lines is None:
        return []
    out: list[tuple[int, str]] = []
    for i, line in enumerate(lines, start=1):
        if IT_CALL_RE.match(line) or XIT_RE.match(line):
            desc = _spec_it_at(spec_path, i)
            if desc is not None:
                out.append((i, desc))
    return out


_SPEC_LINE_CACHE: dict[Path, list[str] | None] = {}


def _read_spec_lines(spec_path: Path) -> list[str] | None:
    if spec_path in _SPEC_LINE_CACHE:
        return _SPEC_LINE_CACHE[spec_path]
    try:
        lines = spec_path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        lines = None
    _SPEC_LINE_CACHE[spec_path] = lines
    return lines


def _normalize_desc(s: str) -> str:
    """Lowercase, collapse whitespace, undo source-level escaping (so
    Rust's `\\"` and TS's `"` compare equal). Does NOT strip surrounding
    quotes; callers must pass an already-unquoted string."""
    s = s.strip()
    # Source-level escape unification.
    s = s.replace(r"\\", "\\")        # double-backslash → single
    s = s.replace(r'\"', '"')          # escaped double quote
    s = s.replace(r"\'", "'")          # escaped single quote
    s = s.replace(r"\`", "`")          # escaped backtick
    s = re.sub(r"\s+", " ", s)
    return s.lower()


@dataclass
class VerifyIssue:
    severity: str          # "error" or "warn"
    rust_path: Path
    rust_line: int
    kind: str              # short tag (missing-line, wrong-line, wrong-desc, ...)
    detail: str


def verify(a: Analysis | None = None) -> list[VerifyIssue]:
    """Walk every // Ported: comment and check its line and description
    against the upstream spec. Returns issues, severity ordered."""
    specs = scan_specs()
    resolve = build_resolver(specs)
    ported = scan_ported()
    issues: list[VerifyIssue] = []

    for c in ported:
        if not c.spec_ref:
            issues.append(VerifyIssue("error", c.rust_path, c.rust_line,
                                      "malformed",
                                      "comment is missing the spec reference"))
            continue
        spec = resolve(c.spec_ref)
        if spec is None:
            issues.append(VerifyIssue("error", c.rust_path, c.rust_line,
                                      "orphan",
                                      f"spec ref does not resolve: {c.spec_ref}"))
            continue
        if c.cited_line is None:
            issues.append(VerifyIssue("warn", c.rust_path, c.rust_line,
                                      "no-line",
                                      f"comment has no `line N` suffix "
                                      f"({spec.rel_path})"))
            continue

        spec_full = RENOVATE_LIB / spec.rel_path
        cited_desc = _spec_it_at(spec_full, c.cited_line)
        commented = _normalize_desc(_unquote(c.description))

        if cited_desc is not None and _normalize_desc(cited_desc) == commented:
            continue                       # exact line + description match — OK

        # Walk the whole spec file for the commented description.
        spec_descs = _spec_all_its(spec_full)
        matches = [ln for ln, d in spec_descs if _normalize_desc(d) == commented]

        if matches:
            # Description exists upstream — pick the line closest to the cited.
            nearest = min(matches, key=lambda ln: abs(ln - c.cited_line))
            issues.append(VerifyIssue("warn", c.rust_path, c.rust_line,
                                      "off-by-line",
                                      f"{spec.rel_path}: cited line "
                                      f"{c.cited_line}, actual it() at line "
                                      f"{nearest}"))
        elif cited_desc is None:
            issues.append(VerifyIssue("error", c.rust_path, c.rust_line,
                                      "missing-line",
                                      f"{spec.rel_path}:{c.cited_line} is "
                                      f"not an it()/test() call site and the "
                                      f"description does not match any it() "
                                      f"in the file"))
        else:
            issues.append(VerifyIssue("error", c.rust_path, c.rust_line,
                                      "wrong-desc",
                                      f"{spec.rel_path}:{c.cited_line} says "
                                      f"`{cited_desc}` but comment says "
                                      f"{c.description}; no it() in the file "
                                      f"matches either"))

    issues.sort(key=lambda x: (0 if x.severity == "error" else 1,
                               str(x.rust_path), x.rust_line))
    return issues


def verify_report(issues: list[VerifyIssue]) -> str:
    errors = [i for i in issues if i.severity == "error"]
    warns = [i for i in issues if i.severity == "warn"]
    by_kind: dict[str, int] = defaultdict(int)
    for i in issues:
        by_kind[i.kind] += 1
    out: list[str] = []
    out.append("# // Ported: comment verification\n")
    out.append(f"Errors:   {len(errors)}")
    out.append(f"Warnings: {len(warns)}")
    out.append("")
    out.append("By kind:")
    for k in sorted(by_kind, key=lambda k: -by_kind[k]):
        out.append(f"  {k:14s} {by_kind[k]}")
    out.append("")
    for i in issues:
        rel = i.rust_path.relative_to(RUST_ROOT)
        out.append(f"  [{i.severity:5s}] {rel}:{i.rust_line}  {i.kind}: {i.detail}")
    return "\n".join(out) + "\n"


# ---------------------------------------------------------------------------
# Prior-impl carry-forward parser
# ---------------------------------------------------------------------------

LEDGER_ROW_RE = re.compile(
    r"^\|\s*`(?P<mod>[^`]+)`\s*\|\s*\d+\s*\|\s*(?P<impl>[^|]+?)\s*\|"
    r"\s*[^|]*\|\s*(?P<notes>.*?)\s*\|\s*$"
)


def load_prior_impl(path: Path) -> dict[str, tuple[str, str]]:
    if not path.is_file():
        return {}
    out: dict[str, tuple[str, str]] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        m = LEDGER_ROW_RE.match(line)
        if m:
            out[m.group("mod")] = (m.group("impl").strip(), m.group("notes").strip())
    return out


# ---------------------------------------------------------------------------
# CLI entry point
# ---------------------------------------------------------------------------

def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="cmd")
    sub.add_parser("report", help="ledger summary (default)")
    p_ledger = sub.add_parser("ledger", help="regenerate docs/parity/modules.md")
    p_ledger.add_argument("--stdout", action="store_true",
                          help="print to stdout instead of writing the file")
    p_gaps = sub.add_parser("gaps", help="list missing upstream tests for a module")
    p_gaps.add_argument("module")
    sub.add_parser("orphans", help="list unresolved // Ported: comments")
    p_verify = sub.add_parser("verify", help="check every // Ported: against upstream")
    p_verify.add_argument("--errors-only", action="store_true",
                          help="suppress warnings")
    sub.add_parser("json", help="emit raw analysis as JSON")
    args = parser.parse_args(argv)

    a = analyze()
    cmd = args.cmd or "report"

    if cmd == "report":
        print(report(a))
    elif cmd == "ledger":
        out_path = RUST_ROOT / "docs" / "parity" / "modules.md"
        prior = load_prior_impl(out_path)
        text = ledger(a, prior)
        if getattr(args, "stdout", False):
            print(text)
        else:
            out_path.write_text(text, encoding="utf-8")
            print(f"wrote {out_path.relative_to(RUST_ROOT)} "
                  f"({a.total_ported}/{a.total_it} = "
                  f"{a.total_ported / a.total_it * 100:.1f}%)")
    elif cmd == "gaps":
        print(gaps(a, args.module))
    elif cmd == "orphans":
        print(orphans_report(a))
    elif cmd == "verify":
        issues = verify(a)
        if getattr(args, "errors_only", False):
            issues = [i for i in issues if i.severity == "error"]
        print(verify_report(issues))
        if any(i.severity == "error" for i in issues):
            return 1
    elif cmd == "json":
        payload = {
            "total_specs": a.total_specs,
            "total_it": a.total_it,
            "total_ported": a.total_ported,
            "total_comments": a.total_comments,
            "orphans": len(a.orphan_ported),
            "malformed": len(a.malformed_ported),
            "modules": [
                {
                    "module_id": m.module_id,
                    "group": m.group,
                    "spec_count": len(m.spec_files),
                    "total_it": m.total_it,
                    "ported": m.ported_count,
                    "duplicates": m.duplicate_count,
                    "coverage_pct": round(m.coverage_pct, 1),
                }
                for m in sorted(a.modules.values(), key=lambda x: x.module_id)
            ],
        }
        print(json.dumps(payload, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
