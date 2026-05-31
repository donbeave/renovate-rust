#!/usr/bin/env python3
"""Derive per-module Impl status from the legacy per-file renovate-source-map.md.

Rolls up the per-file `full` / `partial` / `stub` / `not-started` / `out-of-scope`
statuses into a single per-module status using these rules:

  - all files `full` or `out-of-scope` (and ≥1 `full`) → `full`
  - any `not-started` with no in-scope `full` siblings → `none`
  - everything else (mixed) → `partial`

The Notes column is a brief auto-summary of the missing pieces.

Output: prints the rolled-up status as `module_id\timpl\tnotes` lines on stdout.
Used once to seed docs/parity/modules.md; not part of the regular workflow.
"""

from __future__ import annotations

import re
import sys
from collections import defaultdict
from pathlib import Path

SRC_MAP = Path(__file__).resolve().parent.parent / "docs/parity/renovate-source-map.md"

# Same module rules as parity_coverage.py but for .ts source paths.
MODULE_RULES: list[tuple[re.Pattern[str], str]] = [
    (re.compile(r"^lib/modules/manager/([^/]+)/"),    "manager/{0}"),
    (re.compile(r"^lib/modules/manager/[^/]+\.ts$"),  "manager/_common"),
    (re.compile(r"^lib/modules/datasource/([^/]+)/"), "datasource/{0}"),
    (re.compile(r"^lib/modules/datasource/[^/]+\.ts$"), "datasource/_common"),
    (re.compile(r"^lib/modules/platform/([^/]+)/"),   "platform/{0}"),
    (re.compile(r"^lib/modules/platform/[^/]+\.ts$"), "platform/_common"),
    (re.compile(r"^lib/modules/versioning/([^/]+)/"), "versioning/{0}"),
    (re.compile(r"^lib/modules/versioning/[^/]+\.ts$"), "versioning/_common"),
    (re.compile(r"^lib/workers/([^/]+)/"),            "worker/{0}"),
    (re.compile(r"^lib/workers/"),                    "worker/_root"),
    (re.compile(r"^lib/config/([^/]+)/"),             "config/{0}"),
    (re.compile(r"^lib/config/"),                     "config/_root"),
    (re.compile(r"^lib/util/([^/]+)/"),               "util/{0}"),
    (re.compile(r"^lib/util/"),                       "util/_root"),
    (re.compile(r"^lib/logger/"),                     "logger"),
    (re.compile(r"^lib/instrumentation/"),            "instrumentation"),
    (re.compile(r"^lib/constants/"),                  "constants"),
    (re.compile(r"^lib/data/"),                       "data"),
    (re.compile(r"^lib/types/"),                      "types"),
    (re.compile(r"^lib/[^/]+\.ts$"),                  "cli/_root"),
]


def classify(ts_path: str) -> str | None:
    for rule, tmpl in MODULE_RULES:
        m = rule.match(ts_path)
        if m:
            return tmpl.format(*m.groups())
    return None


# Match a per-file row: `| `lib/foo/bar.ts` | rust | status | notes |`
ROW_RE = re.compile(
    r"^\|\s*`(?P<ts>lib/[^`]+\.ts)`\s*\|\s*[^|]*\|\s*(?P<status>full|partial|stub|not-started|out-of-scope)\s*\|\s*(?P<notes>[^|]*?)\s*\|\s*$"
)


def main() -> int:
    if not SRC_MAP.is_file():
        print(f"missing {SRC_MAP}", file=sys.stderr)
        return 1
    per_module: dict[str, list[tuple[str, str, str]]] = defaultdict(list)
    for line in SRC_MAP.read_text(encoding="utf-8").splitlines():
        m = ROW_RE.match(line)
        if not m:
            continue
        ts = m.group("ts")
        mod = classify(ts)
        if mod is None:
            continue
        per_module[mod].append((ts, m.group("status"), m.group("notes")))

    for mod in sorted(per_module):
        rows = per_module[mod]
        statuses = [r[1] for r in rows]
        in_scope = [r for r in rows if r[1] != "out-of-scope"]
        if not in_scope:
            impl = "out-of-scope"
        elif all(s in ("full", "out-of-scope") for s in statuses) and any(s == "full" for s in statuses):
            impl = "full"
        elif all(s == "not-started" for s in [r[1] for r in in_scope]):
            impl = "none"
        else:
            impl = "partial"
        # Build a brief Notes summary.
        missing = [r for r in in_scope if r[1] in ("partial", "stub", "not-started")]
        if missing and impl != "full":
            # Pick at most 3 missing files for the note.
            tags = []
            for ts, st, nt in missing[:3]:
                base = ts.rsplit("/", 1)[-1]
                tags.append(f"{base}({st})")
            extra = f" +{len(missing) - 3} more" if len(missing) > 3 else ""
            note = "; ".join(tags) + extra
        else:
            note = "—"
        print(f"{mod}\t{impl}\t{note}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
