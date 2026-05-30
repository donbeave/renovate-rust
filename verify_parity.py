#!/usr/bin/env python3
"""Verify ported test parity between detail files and Rust code."""

import os
import re
import glob
from collections import defaultdict

def find_ported_rows():
    """Scan all detail files for 'ported' rows with Rust references."""
    detail_files = glob.glob('docs/parity/**/*.md', recursive=True)
    ported = []
    for f in detail_files:
        with open(f) as fh:
            for line in fh:
                m = re.match(
                    r'\|\s*([^|]+)\|\s*(\d+)\s*\|\s*ported\s*\|\s*([^|]+)\|\s*([^|]+)\|\s*([^|]*)\|',
                    line.strip()
                )
                if m:
                    test_name = m.group(1).strip()
                    line_no = m.group(2).strip()
                    rust_file = m.group(3).strip().strip('`')
                    rust_test = m.group(4).strip()
                    reason = m.group(5).strip()
                    if rust_file and rust_file != '—' and rust_test and rust_test != '—':
                        ported.append({
                            'detail_file': f,
                            'ts_test': test_name,
                            'ts_line': line_no,
                            'rust_file': rust_file,
                            'rust_test': rust_test,
                            'reason': reason,
                        })
    return ported


def find_rust_tests():
    """Find all #[test] and #[tokio::test] functions in Rust source."""
    rust_files = glob.glob('crates/**/*.rs', recursive=True)
    tests = defaultdict(dict)  # file -> {test_name: line_no}
    for f in rust_files:
        with open(f) as fh:
            lines = fh.readlines()
        rel = os.path.relpath(f, '.')
        if rel not in tests:
            tests[rel] = {}
        i = 0
        while i < len(lines):
            line = lines[i].strip()
            if line.startswith('#[test]') or line.startswith('#[tokio::test]'):
                for j in range(i + 1, min(i + 5, len(lines))):
                    fn_match = re.match(r'\s*(?:async\s+)?fn\s+(\w+)', lines[j])
                    if fn_match:
                        tests[rel][fn_match.group(1)] = j + 1
                        break
            i += 1
    return tests


def extract_test_names(raw):
    """Extract individual test names from a rust_test cell.
    
    Handles:
    - `test_name` -> ['test_name']
    - `test_name` (+ other_name, another) -> ['test_name']
    - `test1`, `test2` -> ['test1', 'test2']
    - `test1, test2, test3` -> ['test1', 'test2', 'test3']
    - test_name_* (wildcards) -> ['test_name_*']  (can't verify)
    - descriptions without backticks -> []  (covered by description)
    """
    # Find all backtick-wrapped names
    names = re.findall(r'`([^`]+)`', raw)
    if names:
        # Some entries have `name1, name2, name3` as a single backtick block
        result = []
        for name in names:
            # Check if it's a comma-separated list of identifiers
            parts = [p.strip() for p in name.split(',')]
            if all(re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', p) for p in parts):
                result.extend(parts)
            else:
                result.append(name)
        return result
    # If no backticks, check if it looks like an identifier
    if re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', raw):
        return [raw]
    # Description-only, no identifiable test name
    return []


def verify():
    ported = find_ported_rows()
    rust_tests = find_rust_tests()

    genuinely_missing = []
    covered_by_description = []
    multi_test_unclear = []
    found = []

    for p in ported:
        test_names = extract_test_names(p['rust_test'])
        
        if not test_names:
            covered_by_description.append(p)
            continue
            
        # Check if any test name exists
        matched_any = False
        for actual_path, tests in rust_tests.items():
            if p['rust_file'] in actual_path or actual_path.endswith(p['rust_file']):
                for name in test_names:
                    if name in tests:
                        matched_any = True
                        found.append({**p, 'actual_path': actual_path, 'actual_line': tests[name], 'matched_name': name})
                        break
                if matched_any:
                    break
        
        if not matched_any:
            # Check if it's a wildcard pattern
            if any('*' in n for n in test_names):
                multi_test_unclear.append(p)
            else:
                genuinely_missing.append(p)

    print(f"=" * 70)
    print(f"VERIFICATION REPORT")
    print(f"=" * 70)
    print(f"Total ported rows with Rust refs: {len(ported)}")
    print(f"Found in code: {len(found)}")
    print(f"Covered by description (no specific test name): {len(covered_by_description)}")
    print(f"Multi-test / wildcard (need manual check): {len(multi_test_unclear)}")
    print(f"Genuinely missing: {len(genuinely_missing)}")
    print()

    if covered_by_description:
        print("-" * 70)
        print("COVERED BY DESCRIPTION (behavior tested but no specific test named):")
        print("-" * 70)
        by_detail = defaultdict(list)
        for p in covered_by_description:
            by_detail[p['detail_file']].append(p)
        for detail_file, items in sorted(by_detail.items()):
            print(f"\n  {detail_file}")
            for item in items:
                print(f"    - {item['ts_test']} (line {item['ts_line']})")
                print(f"      Desc: {item['rust_test']}")
        print()

    if multi_test_unclear:
        print("-" * 70)
        print("MULTI-TEST / WILDCARD (at least one test may exist, verify manually):")
        print("-" * 70)
        by_detail = defaultdict(list)
        for p in multi_test_unclear:
            by_detail[p['detail_file']].append(p)
        for detail_file, items in sorted(by_detail.items()):
            print(f"\n  {detail_file}")
            for item in items:
                print(f"    - {item['ts_test']} (line {item['ts_line']})")
                print(f"      Expected: {item['rust_file']} :: {item['rust_test']}")
        print()

    if genuinely_missing:
        print("-" * 70)
        print("GENUINELY MISSING (documented as ported but no Rust test found):")
        print("-" * 70)
        by_detail = defaultdict(list)
        for p in genuinely_missing:
            by_detail[p['detail_file']].append(p)
        for detail_file, items in sorted(by_detail.items()):
            print(f"\n  {detail_file}")
            for item in items:
                print(f"    - {item['ts_test']} (line {item['ts_line']})")
                print(f"      Expected: {item['rust_file']} :: {item['rust_test']}")
        print()

    # Summary
    print("-" * 70)
    print("VERIFICATION BY DETAIL FILE:")
    print("-" * 70)
    by_detail_all = defaultdict(lambda: {'found': 0, 'missing': 0, 'desc': 0, 'multi': 0})
    for p in found:
        by_detail_all[p['detail_file']]['found'] += 1
    for p in genuinely_missing:
        by_detail_all[p['detail_file']]['missing'] += 1
    for p in covered_by_description:
        by_detail_all[p['detail_file']]['desc'] += 1
    for p in multi_test_unclear:
        by_detail_all[p['detail_file']]['multi'] += 1
        
    for detail_file in sorted(set(p['detail_file'] for p in ported)):
        stats = by_detail_all[detail_file]
        total = stats['found'] + stats['missing'] + stats['desc'] + stats['multi']
        missing = stats['missing']
        status = "OK" if missing == 0 else f"{missing} MISSING"
        print(f"  {status:12} {detail_file}")

    print()
    print(f"=" * 70)
    print(f"Verification: {len(found)} found, {len(covered_by_description)} by description, {len(multi_test_unclear)} wildcard/multi, {len(genuinely_missing)} genuinely missing")
    print(f"=" * 70)


if __name__ == '__main__':
    verify()
