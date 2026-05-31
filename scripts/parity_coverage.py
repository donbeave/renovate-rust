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
    (?P<desc>".*?"|'[^']*?')      # the original it() description (quoted)
    \s*[—–-]\s*                   # separator (em, en, or hyphen)
    (?P<ref>[^\s][^\n]*)          # the spec reference (greedy, normalized later)
    """,
    re.VERBOSE,
)


def _normalize_ref(ref: str) -> str:
    """Strip trailing ` line N`, ` lines N-M`, parenthetical commentary, etc."""
    ref = ref.strip()
    # Cut at first ` line ` / ` lines `.
    ref = re.split(r"\s+lines?\s+\d", ref, maxsplit=1)[0]
    # Cut at parenthetical commentary like ` (parse modules with phases)`.
    ref = re.split(r"\s+\(", ref, maxsplit=1)[0]
    return ref.strip().rstrip(",").rstrip()


def scan_ported() -> list[tuple[Path, int, str, str]]:
    """Return [(rust_file, line_no, description, normalized_ref)] for each
    `// Ported:` comment. `description` includes its quotes for display.
    """
    crates = RUST_ROOT / "crates"
    out: list[tuple[Path, int, str, str]] = []
    for path in sorted(crates.rglob("*.rs")):
        with path.open(encoding="utf-8", errors="replace") as fh:
            for i, line in enumerate(fh, start=1):
                if "// Ported:" not in line:
                    continue
                m = PORTED_RE.search(line)
                if not m:
                    out.append((path, i, "", ""))
                    continue
                out.append((path, i, m.group("desc"), _normalize_ref(m.group("ref"))))
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
    for rust_file, line_no, desc, ref in ported:
        if not ref:
            malformed.append((rust_file, line_no))
            continue
        spec = resolve(ref)
        if spec is None:
            orphans.append((rust_file, line_no, ref))
            continue
        total_comments += 1
        mod = modules[spec.module_id]
        mod.comment_count += 1
        mod.covered.add((spec.rel_path, desc))

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
