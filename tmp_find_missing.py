#!/usr/bin/env python3
import re
from pathlib import Path

IT_CALL_RE = re.compile(r'^[ \t]+(?:it|test)(?:\.(?:each|skip|only|failing|concurrent|todo))?(?:\.(?:each|skip|only|failing|concurrent|todo))?\s*[\(\`]', re.VERBOSE)
XIT_RE = re.compile(r'^[ \t]+(?:xit|xtest)\s*\(')

PORTED_RE = re.compile(
    r'//\s*Ported:\s*(?P<desc>"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\')\s*[—–-]\s*(?P<ref>[^\s][^\n]*)',
    re.VERBOSE,
)

# Parse all it() descriptions from index.spec.ts
spec_path = Path('../renovate/lib/modules/platform/github/index.spec.ts')
spec_descs = set()
spec_lines = spec_path.read_text().splitlines()

IT_DESC_RE = re.compile(
    r'''
    \b
    (?:it|test|xit|xtest)
    (?:\.(?:each|skip|only|failing|concurrent|todo))?
    (?:\.(?:each|skip|only|failing|concurrent|todo))?
    \s*
    (?:\(\s*\[[^\]]*\]\s*\)\s*)?
    [\(\`]?
    \s*
    (?P<q>['"`])
    (?P<desc>.*?)
    (?P=q)
    ''',
    re.VERBOSE,
)

for i, line in enumerate(spec_lines, start=1):
    if IT_CALL_RE.match(line) or XIT_RE.match(line):
        m = IT_DESC_RE.search(line)
        if m:
            spec_descs.add(m.group('desc'))

# Parse all ported comments mentioning index.spec.ts
covered = set()
for path in sorted(Path('crates').rglob('*.rs')):
    text = path.read_text()
    for m in PORTED_RE.finditer(text):
        ref = m.group('ref')
        if 'index.spec.ts' in ref and 'platform/github' in ref:
            desc = m.group('desc').strip('"\'')
            covered.add(desc)

# Normalize
def norm(s):
    s = s.strip()
    s = s.replace(r'\\', '\\')
    s = s.replace(r'\"', '"')
    s = s.replace(r"\'", "'")
    s = re.sub(r'\s+', ' ', s)
    return s.lower()

norm_covered = {norm(d) for d in covered}
missing = [d for d in sorted(spec_descs) if norm(d) not in norm_covered]

print(f'Total it() descriptions: {len(spec_descs)}')
print(f'Covered: {len(norm_covered)}')
print(f'Missing: {len(missing)}')
print()
for d in missing:
    print(f'  {d}')
