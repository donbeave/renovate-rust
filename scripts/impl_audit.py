#!/usr/bin/env python3
"""Implementation audit: check whether upstream modules have Rust counterparts.

For each module tracked by parity_coverage.py, this script:
1. Maps the upstream module to expected Rust source paths
2. Checks if Rust files exist and how many lines they contain
3. Scans for stub indicators (todo!(), unimplemented!(), empty bodies)
4. Produces a report categorizing each module as:
   - implemented    : >100 lines and no heavy stubbing
   - partial        : some files exist but clearly incomplete
   - stub           : files exist but are mostly stubs (<50 real lines)
   - missing        : no corresponding Rust files found

Usage:
    python3 scripts/impl_audit.py report          # summary
    python3 scripts/impl_audit.py json             # JSON dump
    python3 scripts/impl_audit.py detail <module>  # per-file breakdown
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

RUST_ROOT = Path(__file__).resolve().parent.parent
REPO_ROOT = RUST_ROOT.parent
RENOVATE_ROOT = REPO_ROOT / "renovate"
RENOVATE_LIB = RENOVATE_ROOT / "lib"

# Mapping from module_id prefix to expected Rust crate subdirectory.
MODULE_PREFIX_TO_RS_DIR: dict[str, list[str]] = {
    "manager/":     ["crates/renovate-core/src/manager",
                       "crates/renovate-core/src/extractors"],
    "datasource/":  ["crates/renovate-core/src/datasource",
                       "crates/renovate-core/src/datasources"],
    "platform/":    ["crates/renovate-core/src/platform"],
    "versioning/":  ["crates/renovate-core/src/versioning"],
    "worker/":      ["crates/renovate-core/src/workers"],
    "config/":      ["crates/renovate-core/src/config"],
    "util/":        ["crates/renovate-core/src/util"],
    "cli/":         ["crates/renovate-cli/src"],
    "logger":       ["crates/renovate-core/src/logger"],
    "instrumentation": ["crates/renovate-core/src/instrumentation"],
    "constants":    ["crates/renovate-core/src/constants"],
    "data":         ["crates/renovate-core/src/data"],
    "types":        ["crates/renovate-core/src/types"],
}

# Explicit overrides for modules whose Rust files don't follow naming conventions.
MODULE_OVERRIDES: dict[str, list[str]] = {
    "util/http": ["crates/renovate-core/src/http.rs"],
    "util/fs": ["crates/renovate-core/src/fs.rs"],
    "util/exec": ["crates/renovate-core/src/exec.rs"],
    "util/json-writer": ["crates/renovate-core/src/json_writer.rs"],
    "util/schema-utils": ["crates/renovate-core/src/schema.rs"],
    "util/vulnerability": ["crates/renovate-core/src/vulnerability.rs"],
    "util/github": ["crates/renovate-core/src/github.rs"],
    "util/merge-confidence": ["crates/renovate-core/src/merge_confidence.rs"],
    "config/migrations": ["crates/renovate-core/src/config/migration.rs",
                           "crates/renovate-core/src/config/migration"],
    "config/options": ["crates/renovate-cli/src/config_env.rs",
                        "crates/renovate-cli/src/config_builder.rs"],
    "config/_root": ["crates/renovate-core/src/config.rs"],
    "config/presets": ["crates/renovate-core/src/config/presets.rs",
                        "crates/renovate-core/src/config/presets"],
    "config/decrypt": ["crates/renovate-core/src/config/decrypt.rs",
                        "crates/renovate-core/src/config/decrypt"],
    "config/validation-helpers": ["crates/renovate-core/src/config/validation_helpers.rs",
                                   "crates/renovate-core/src/config/validation_helpers"],
    "versioning/semver": ["crates/renovate-core/src/versioning/semver_generic.rs",
                           "crates/renovate-core/src/versioning/semver_node.rs",
                           "crates/renovate-core/src/versioning/semver_coerced.rs",
                           "crates/renovate-core/src/versioning/semver_partial.rs"],
    "versioning/regex": ["crates/renovate-core/src/versioning/regex.rs",
                          "crates/renovate-core/src/versioning/regex_versioning.rs"],
    "datasource/crate": ["crates/renovate-core/src/datasources/crates_io.rs"],
    "datasource/docker": ["crates/renovate-core/src/datasources/docker_hub.rs",
                           "crates/renovate-core/src/datasources/docker_ecr.rs",
                           "crates/renovate-core/src/datasources/docker_google.rs"],
    "datasource/go": ["crates/renovate-core/src/datasources/gomod.rs",
                       "crates/renovate-core/src/datasources/go_releases_direct.rs",
                       "crates/renovate-core/src/datasources/go_goproxy_parser.rs"],
    "datasource/pod": ["crates/renovate-core/src/datasources/pod.rs"],
    "datasource/terraform-module": ["crates/renovate-core/src/datasources/terraform_module.rs"],
    "datasource/terraform-provider": ["crates/renovate-core/src/datasources/terraform_provider.rs"],
    "datasource/endoflife-date": ["crates/renovate-core/src/datasources/endoflife_date.rs"],
    "datasource/azure-bicep-resource": ["crates/renovate-core/src/datasources/azure_bicep_resource.rs"],
    "cli/_root": ["crates/renovate-cli/src/main.rs",
                   "crates/renovate-cli/src/cli.rs"],
    "constants": ["crates/renovate-core/src/constants.rs"],
    "data": ["crates/renovate-core/src/data.rs"],
    "logger": ["crates/renovate-core/src/logger.rs"],
    "instrumentation": ["crates/renovate-core/src/instrumentation.rs"],
    "types": ["crates/renovate-core/src/types.rs"],
    "platform/utils": ["crates/renovate-core/src/platform/util.rs"],
    "platform/_common": ["crates/renovate-core/src/platform.rs"],
    "manager/_common": ["crates/renovate-core/src/managers.rs"],
}

STUB_RE = re.compile(r"\btodo!\(|\bunimplemented!\(|\bpanic!\(")
FN_RE = re.compile(r"\bfn\s+\w+")
STRUCT_RE = re.compile(r"\b(struct|enum|trait|impl)\b")
COMMENT_BLANK_RE = re.compile(r"^\s*(//|/\*|\*|\s*)$")


def rs_dirs_for_module(module_id: str) -> list[Path]:
    """Return list of candidate Rust directories for a module_id."""
    candidates: list[Path] = []
    for prefix, dirs in MODULE_PREFIX_TO_RS_DIR.items():
        if module_id.startswith(prefix):
            suffix = module_id[len(prefix):]
            for d in dirs:
                candidates.append(RUST_ROOT / d / suffix.replace("_", "-"))
                candidates.append(RUST_ROOT / d / suffix)
                candidates.append(RUST_ROOT / d / suffix.replace("-", "_"))
            break
    else:
        # Fallback: try direct match under crates/renovate-core/src
        candidates.append(RUST_ROOT / "crates/renovate-core/src" / module_id.replace("_", "-"))
        candidates.append(RUST_ROOT / "crates/renovate-core/src" / module_id)
    return candidates


def find_rs_files(module_id: str) -> list[Path]:
    """Find all .rs files that might implement this module."""
    files: list[Path] = []
    # Check explicit overrides first.
    if module_id in MODULE_OVERRIDES:
        for p_str in MODULE_OVERRIDES[module_id]:
            p = RUST_ROOT / p_str
            if p.is_file() and p.suffix == ".rs":
                files.append(p)
            elif p.is_dir():
                for child in sorted(p.rglob("*.rs")):
                    files.append(child)
        if files:
            seen: set[Path] = set()
            out: list[Path] = []
            for p in files:
                rp = p.resolve()
                if rp not in seen:
                    seen.add(rp)
                    out.append(p)
            return out

    for cand_dir in rs_dirs_for_module(module_id):
        if cand_dir.is_file() and cand_dir.suffix == ".rs":
            files.append(cand_dir)
            continue
        if cand_dir.is_dir():
            for p in sorted(cand_dir.rglob("*.rs")):
                files.append(p)
        # Also check for a direct .rs file with the same name as the dir
        direct_rs = Path(str(cand_dir) + ".rs")
        if direct_rs.is_file():
            files.append(direct_rs)
    # Deduplicate while preserving order
    seen: set[Path] = set()
    out: list[Path] = []
    for p in files:
        rp = p.resolve()
        if rp not in seen:
            seen.add(rp)
            out.append(p)
    return out


def file_stats(path: Path) -> dict[str, int]:
    """Return line counts: total, code (non-comment/blank), stubs, fns, structs."""
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return {"total": 0, "code": 0, "stubs": 0, "fns": 0, "structs": 0}
    lines = text.splitlines()
    total = len(lines)
    code = 0
    stubs = 0
    fns = 0
    structs = 0
    for line in lines:
        if COMMENT_BLANK_RE.match(line):
            continue
        code += 1
        if STUB_RE.search(line):
            stubs += 1
        if FN_RE.search(line):
            fns += 1
        if STRUCT_RE.search(line):
            structs += 1
    return {"total": total, "code": code, "stubs": stubs, "fns": fns, "structs": structs}


def classify_module(files: list[Path]) -> tuple[str, int, int, int]:
    """Return (status, total_lines, code_lines, stub_count) for a module."""
    if not files:
        return "missing", 0, 0, 0
    totals = [file_stats(f) for f in files]
    total_lines = sum(t["total"] for t in totals)
    code_lines = sum(t["code"] for t in totals)
    stub_count = sum(t["stubs"] for t in totals)
    fn_count = sum(t["fns"] for t in totals)
    struct_count = sum(t["structs"] for t in totals)

    if code_lines < 30:
        return "stub", total_lines, code_lines, stub_count
    if stub_count > fn_count * 0.5 and fn_count > 0:
        return "partial", total_lines, code_lines, stub_count
    if code_lines < 100 and fn_count < 5:
        return "stub", total_lines, code_lines, stub_count
    if code_lines < 200:
        return "partial", total_lines, code_lines, stub_count
    return "implemented", total_lines, code_lines, stub_count


def load_modules_from_coverage() -> list[tuple[str, str]]:
    """Run parity_coverage.py in json mode to get the module list."""
    import subprocess
    result = subprocess.run(
        [sys.executable, str(RUST_ROOT / "scripts/parity_coverage.py"), "json"],
        capture_output=True, text=True, cwd=str(RUST_ROOT)
    )
    if result.returncode != 0:
        sys.exit(f"parity_coverage.py failed: {result.stderr}")
    data = json.loads(result.stdout)
    return [(m["module_id"], m["group"]) for m in data["modules"]]


def report() -> str:
    modules = load_modules_from_coverage()
    results: list[dict] = []
    for mod_id, group in modules:
        files = find_rs_files(mod_id)
        status, total, code, stubs = classify_module(files)
        results.append({
            "module_id": mod_id,
            "group": group,
            "status": status,
            "files": len(files),
            "total_lines": total,
            "code_lines": code,
            "stubs": stubs,
            "rust_paths": [str(f.relative_to(RUST_ROOT)) for f in files],
        })

    lines: list[str] = []
    lines.append("Implementation Audit Report")
    lines.append("=" * 60)

    by_group: dict[str, list[dict]] = defaultdict(list)
    for r in results:
        by_group[r["group"]].append(r)

    for group in sorted(by_group):
        items = by_group[group]
        counts: dict[str, int] = defaultdict(int)
        total_code = 0
        for item in items:
            counts[item["status"]] += 1
            total_code += item["code_lines"]
        lines.append(f"\n[{group}]  {len(items)} modules, ~{total_code} code lines")
        for status in ["implemented", "partial", "stub", "missing"]:
            if counts[status]:
                lines.append(f"  {status:12s} {counts[status]}")

    lines.append("\n--- Modules with status != implemented ---\n")
    for group in sorted(by_group):
        for item in by_group[group]:
            if item["status"] != "implemented":
                lines.append(
                    f"  {item['module_id']:40s} {item['status']:10s} "
                    f"{item['files']:2d} files  {item['code_lines']:4d} code lines  "
                    f"{item['stubs']:3d} stubs"
                )
    return "\n".join(lines)


def detail(module_id: str) -> str:
    files = find_rs_files(module_id)
    lines: list[str] = [f"# {module_id}\n"]
    if not files:
        lines.append("No Rust files found.")
        return "\n".join(lines)
    for f in files:
        st = file_stats(f)
        rel = f.relative_to(RUST_ROOT)
        lines.append(f"{rel}  —  {st['total']} total / {st['code']} code / {st['stubs']} stubs / {st['fns']} fns / {st['structs']} types")
    status, total, code, stubs = classify_module(files)
    lines.append(f"\nOverall: {status}  ({total} total lines, {code} code lines, {stubs} stubs)")
    return "\n".join(lines)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="cmd")
    sub.add_parser("report", help="summary report (default)")
    sub.add_parser("json", help="JSON dump")
    p_detail = sub.add_parser("detail", help="per-file breakdown for a module")
    p_detail.add_argument("module")
    args = parser.parse_args(argv)
    cmd = args.cmd or "report"

    if cmd == "report":
        print(report())
    elif cmd == "json":
        modules = load_modules_from_coverage()
        out = []
        for mod_id, group in modules:
            files = find_rs_files(mod_id)
            status, total, code, stubs = classify_module(files)
            out.append({
                "module_id": mod_id,
                "group": group,
                "status": status,
                "files": len(files),
                "total_lines": total,
                "code_lines": code,
                "stubs": stubs,
                "rust_paths": [str(f.relative_to(RUST_ROOT)) for f in files],
            })
        print(json.dumps(out, indent=2))
    elif cmd == "detail":
        print(detail(args.module))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
