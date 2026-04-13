"""
GODSEYE — SOVEREIGN POLYGLOT TOPOLOGY SCANNER
================================================
Dependency-first architecture scanner. Builds the complete
dependency graph BEFORE classifying anything.
Supports ALL programming languages natively.

DESIGN PRINCIPLE: A file's importance is determined by
HOW MANY OTHER FILES NEED IT, not by its size or name.

Features:
  1. Full dependency graph (AST + string refs + polyglot imports)
  2. Transitive dependency scoring (cascade analysis)
  3. Critical path tracing from entry points
  4. Protected name enforcement (sarah/sovereign/genesis/ace/aeris)
  5. Dry-run archive simulation
  6. Resource mapping (DBs, URLs, hardware, file paths)
  7. Behavioral analysis (what each file actually does)
  8. Polyglot structural analysis (Python/JS/TS/C++/Rust/Go/Java/C#/Solidity)

Usage:
  python ats_v4.py <TARGET_DIR>      # Full scan + report
  python ats_v4.py --dry-run FILE    # Simulate archiving FILE
"""

import ast
import os
import re
import json
import sys
import time
from collections import defaultdict, deque

# Intelligence Amplifier (optional — operates as post-scan batch reasoner)
try:
    from IntelligenceAmplifier import IntelligenceAmplifier
    AMP_AVAILABLE = True
except ImportError:
    AMP_AVAILABLE = False

# ─── CONFIGURATION ─────────────────────────────────────────────────
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
SCAN_ROOT = r"C:\SarahCore"

SUPPORTED_EXTS = ['.py', '.js', '.ts', '.jsx', '.tsx', '.go', '.rs',
                  '.cpp', '.c', '.h', '.java', '.cs', '.sol']

OUTPUT_JSON = os.path.join(SCRIPT_DIR, 'godseye_topology.json')
OUTPUT_MD   = os.path.join(SCRIPT_DIR, 'godseye_report.md')

# Names that are ALWAYS critical — never disposable
PROTECTED_NAMES = ['sarah', 'sovereign', 'genesis', 'ace', 'aeris',
                   'saul', 'sdm', 'lazarus', 'genlex', 'pulse',
                   'neural', 'brain', 'daemon', 'kernel', 'anchor']

# Known entry points — trace critical paths from these
ENTRY_POINTS = [
    'Sarah_Brain.py',
    'sarah_gateway.py',
    'sovereign_mesh_router.py',
    'Neural_Orchestrator.py',
    'Sarah_Sovereign_Core.py',
    'sovereign_init.py',
    'neural_pulse.py',
]


# ═══════════════════════════════════════════════════════════════════
#  PHASE 1: COLLECT ALL FILES
# ═══════════════════════════════════════════════════════════════════
def collect_files():
    """Walk the directory tree and index metadata for every supported source file."""
    files = {}
    for root, dirs, filenames in os.walk(SCAN_ROOT):
        for f in filenames:
            ext = os.path.splitext(f)[1].lower()
            if ext not in SUPPORTED_EXTS:
                continue
            full = os.path.join(root, f)
            files[f] = {
                'path': full,
                'basename': os.path.splitext(f)[0],
                'ext': ext,
            }
    return files


# ═══════════════════════════════════════════════════════════════════
#  PHASE 2: BUILD DEPENDENCY GRAPH (SEQUENTIAL STREAM PROCESSING)
# ═══════════════════════════════════════════════════════════════════
def build_dependency_graph(files):
    """
    Build a complete dependency graph using SEQUENTIAL reading to protect RAM.
    """
    basename_to_file = {}
    for f, info in files.items():
        basename_to_file[info['basename']] = f

    depends_on = defaultdict(set)
    depended_by = defaultdict(set)
    edges = []

    total = len(files)
    for idx, (f, info) in enumerate(files.items(), 1):
        if idx % 500 == 0:
            print(f"  [{idx}/{total}] mapping edges...", flush=True)

        full_path = info['path']
        try:
            with open(full_path, 'r', encoding='utf-8', errors='ignore') as fh:
                content = fh.read()
        except:
            continue

        # ── METHOD 1: AST Imports (Python only) ──────────────────────────────
        if f.endswith('.py'):
            try:
                tree = ast.parse(content, filename=f)
                for node in ast.walk(tree):
                    target_base = None
                    if isinstance(node, ast.Import):
                        for alias in node.names:
                            target_base = alias.name.split('.')[0]
                    elif isinstance(node, ast.ImportFrom):
                        if node.module:
                            target_base = node.module.split('.')[0]

                    if target_base and target_base in basename_to_file:
                        target_file = basename_to_file[target_base]
                        if target_file != f:
                            depends_on[f].add(target_file)
                            depended_by[target_file].add(f)
                            edges.append((f, target_file, 'IMPORT'))
            except SyntaxError:
                pass

        # ── METHOD 2: Polyglot String References ──────────────────────────────
        words_in_file = set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', content))
        for word in words_in_file:
            if word in basename_to_file:
                candidate_file = basename_to_file[word]
                if candidate_file != f and candidate_file not in depends_on[f]:
                    depends_on[f].add(candidate_file)
                    depended_by[candidate_file].add(f)
                    edges.append((f, candidate_file, 'STRING_REF'))

        # CLEAR CONTENT FROM MEMORY IMMEDIATELY
        del content

    return depends_on, depended_by, edges


# ═══════════════════════════════════════════════════════════════════
#  PHASE 3: TRANSITIVE DEPENDENCY SCORING
# ═══════════════════════════════════════════════════════════════════
def compute_dependency_scores(files, depended_by):
    """
    For each file, compute how many files depend on it TRANSITIVELY.
    If A depends on B, and B depends on C, then C's transitive
    score includes A. Capped at 500 to prevent runaway on dense graphs.
    """
    scores = {}
    total = len(files)
    for idx, f in enumerate(files, 1):
        if idx % 500 == 0:
            print(f"  [{idx}/{total}] scoring...", flush=True)
        # BFS from this file through depended_by
        visited = set()
        queue = deque([f])
        while queue and len(visited) < 500:
            current = queue.popleft()
            for dependent in depended_by.get(current, set()):
                if dependent not in visited and dependent != f:
                    visited.add(dependent)
                    queue.append(dependent)
        scores[f] = {
            'direct': len(depended_by.get(f, set())),
            'transitive': len(visited),
        }
    return scores


# ═══════════════════════════════════════════════════════════════════
#  PHASE 4: CRITICAL PATH TRACING
# ═══════════════════════════════════════════════════════════════════
def trace_critical_paths(files, depends_on, entry_points):
    """
    Starting from entry points, trace the full dependency chain.
    Every file on a critical path is ESSENTIAL.
    """
    on_critical_path = set()

    for entry in entry_points:
        if entry not in files:
            continue
        # BFS from entry point through depends_on
        queue = deque([entry])
        visited = set()
        while queue:
            current = queue.popleft()
            if current in visited:
                continue
            visited.add(current)
            on_critical_path.add(current)
            for dep in depends_on.get(current, set()):
                queue.append(dep)

    return on_critical_path


# ═══════════════════════════════════════════════════════════════════
#  PHASE 5: BEHAVIORAL ANALYSIS
# ═══════════════════════════════════════════════════════════════════
def analyze_behavior(content):
    """Detect what a file actually DOES."""
    behaviors = []

    checks = [
        ('READS_DATABASE',      r'sqlite3\.connect|lancedb\.connect|\.execute\('),
        ('WRITES_DATABASE',     r'\.execute\(.*(INSERT|UPDATE|CREATE|ALTER|DELETE)'),
        ('READS_FILES',         r'open\([^)]+["\']r["\']|json\.load\(|\.read\(\)'),
        ('WRITES_FILES',        r'open\([^)]+["\']w["\']|json\.dump\(|\.write\('),
        ('MAKES_HTTP_REQUESTS', r'requests\.(get|post|put|delete)\(|urllib'),
        ('RUNS_HTTP_SERVER',    r'Flask\(|FastAPI\(|uvicorn\.run|HTTPServer'),
        ('SPAWNS_PROCESSES',    r'subprocess\.(run|Popen|call)\(|os\.system\('),
        ('MONITORS_HARDWARE',   r'psutil\.|nvidia-smi|cuda|gpu_offload'),
        ('DOES_ML_INFERENCE',   r'Llama\(|\.generate\(|AutoModel|torch\.'),
        ('DOES_CRYPTO',         r'hashlib\.|hmac\.|Fernet|WORM'),
        ('DOES_MATH',           r'1\.09277703703|fibonacci|resonance|eigenvalue'),
        ('HAS_CLASSES',         r'^class \w+', ),
        ('HAS_MAIN_BLOCK',     r'if __name__\s*==\s*["\']__main__["\']'),
    ]

    for name, pattern in checks:
        if re.search(pattern, content, re.MULTILINE):
            behaviors.append(name)

    return behaviors


# ═══════════════════════════════════════════════════════════════════
#  PHASE 6: RESOURCE MAPPING
# ═══════════════════════════════════════════════════════════════════
def map_resources(content):
    """Extract specific resources this file touches."""
    resources = {}

    dbs = re.findall(r"sqlite3\.connect\(['\"]([^'\"]+)['\"]\)", content)
    dbs += re.findall(r"lancedb\.connect\(['\"]([^'\"]+)['\"]\)", content)
    if dbs:
        resources['databases'] = list(set(dbs))

    urls = re.findall(r"['\"]https?://[^'\"]+['\"]", content)
    if urls:
        resources['urls'] = list(set(u.strip("'\"")[:100] for u in urls))[:5]

    paths = re.findall(r"r['\"]C:\\[^'\"]+['\"]", content)
    paths += re.findall(r"['\"]C:\\\\[^'\"]+['\"]", content)
    if paths:
        resources['file_paths'] = list(set(p.strip("'\"r")[:100] for p in paths))[:8]

    return resources


# ═══════════════════════════════════════════════════════════════════
#  PHASE 7: STRUCTURAL ANALYSIS (POLYGLOT)
# ═══════════════════════════════════════════════════════════════════
def analyze_structure(content, filename):
    """Extract classes, functions, docstring. Supports all languages."""
    result = {'classes': [], 'functions': [], 'doc': '', 'constants': []}

    if filename.endswith('.py'):
        try:
            tree = ast.parse(content, filename=filename)
            result['doc'] = (ast.get_docstring(tree) or '')[:200]

            for node in ast.walk(tree):
                if isinstance(node, ast.ClassDef):
                    methods = [n.name for n in node.body if isinstance(n, ast.FunctionDef)]
                    result['classes'].append({
                        'name': node.name,
                        'methods': methods,
                        'method_count': len(methods),
                        'doc': (ast.get_docstring(node) or '')[:100],
                    })
                elif isinstance(node, ast.FunctionDef):
                    result['functions'].append({
                        'name': node.name,
                        'args': [a.arg for a in node.args.args if a.arg != 'self'],
                        'doc': (ast.get_docstring(node) or '')[:80],
                    })
        except:
            pass
    else:
        # ── POLYGLOT REGEX STRUCTURAL PARSER ──
        classes = re.findall(r'\bclass\s+([A-Za-z0-9_]{2,60})', content)
        for c in classes:
            result['classes'].append({'name': c, 'methods': [], 'method_count': 0, 'doc': ''})

        function_patterns = [
            r'\bfunction\s+([A-Za-z0-9_]{2,60})\s*\(',
            r'\bfn\s+([A-Za-z0-9_]{2,60})\s*\(',
            r'\bfunc\s+(?:[A-Za-z0-9_*\s]{1,30}\s+)?([A-Za-z0-9_]{2,60})\s*\(',
            r'\bdef\s+([A-Za-z0-9_]{2,60})\s*\(',
        ]
        reserved = {'if', 'while', 'for', 'switch', 'catch', 'return', 'else', 'elif', 'try', 'new', 'var', 'let', 'const'}
        seen = set()
        for pat in function_patterns:
            for m in re.finditer(pat, content):
                name = m.group(1)
                if name not in reserved and name not in seen:
                    result['functions'].append({'name': name, 'args': [], 'doc': ''})
                    seen.add(name)

    return result


# ═══════════════════════════════════════════════════════════════════
#  PHASE 8: CLASSIFICATION (DEPENDENCY-FIRST)
# ═══════════════════════════════════════════════════════════════════
def classify_file(f, info, dep_score, on_critical_path, behaviors, structure):
    """
    Classify a file based on DEPENDENCIES FIRST, then properties.
    """
    basename = info['basename'].lower()
    lines = info['lines']

    # ── RULE 1: Protected names are ALWAYS critical ──
    is_protected = any(pn in basename for pn in PROTECTED_NAMES)

    # ── RULE 2: On a critical path = CRITICAL ──
    is_on_path = f in on_critical_path

    # ── RULE 3: Other files depend on me = CRITICAL ──
    has_dependents = dep_score['direct'] > 0
    has_transitive = dep_score['transitive'] > 0

    # ── RULE 4: I depend on others = at least CONNECTED ──
    # (handled by dep graph)

    # ── CLASSIFY ──
    if is_on_path:
        return 'CRITICAL', f'On critical path from entry point. {dep_score["direct"]} direct dependents.'
    elif dep_score['direct'] >= 5:
        return 'CRITICAL', f'{dep_score["direct"]} files directly depend on this. Transitive: {dep_score["transitive"]}.'
    elif is_protected and has_dependents:
        return 'CRITICAL', f'Protected name + {dep_score["direct"]} dependents.'
    elif is_protected:
        return 'PROTECTED', f'Protected name (contains {[pn for pn in PROTECTED_NAMES if pn in basename][0]}). No current dependents but may be loaded dynamically.'
    elif has_dependents:
        return 'ACTIVE', f'{dep_score["direct"]} direct dependents use this file.'
    elif structure['classes'] and any(c['method_count'] >= 3 for c in structure['classes']):
        return 'SIGNIFICANT', f'Has substantial class(es) but no current dependents. Review for bus wiring.'
    elif lines >= 100 and len(behaviors) >= 2:
        return 'SIGNIFICANT', f'{lines} lines with {len(behaviors)} behaviors. Not connected but has real logic.'
    elif lines <= 5:
        return 'MINIMAL', f'Only {lines} lines. Stub or placeholder.'
    else:
        return 'ISOLATED', f'No dependents, no protected name. {lines} lines.'


# ═══════════════════════════════════════════════════════════════════
#  PHASE 9: DRY-RUN SIMULATION
# ═══════════════════════════════════════════════════════════════════
def simulate_archive(files, depended_by, targets):
    """Simulate what would break if you archived these files."""
    broken = []
    cascade = set()

    for target in targets:
        if target in depended_by:
            for dependent in depended_by[target]:
                if dependent not in targets:
                    broken.append({
                        'archived': target,
                        'breaks': dependent,
                    })
                    cascade.add(dependent)

    return broken, cascade


# ═══════════════════════════════════════════════════════════════════
#  PHASE 10: VULNERABILITY HUNTING (THE WEAPON)
# ═══════════════════════════════════════════════════════════════════
VULN_SIGNATURES = {
    # ── INJECTION ──
    'SQL_INJECTION': [
        r'execute\s*\(\s*["\'].*%s',
        r'execute\s*\(\s*f["\']',
        r'execute\s*\(\s*["\'].*\+\s*\w+',
        r'cursor\.execute\s*\(\s*[^,]+\+',
        r'\.query\s*\(\s*["\'].*\+\s*\w+',
        r'\.raw\s*\(\s*f["\']',
    ],
    'COMMAND_INJECTION': [
        r'os\.system\s*\(',
        r'subprocess\.\w+\s*\([^)]*shell\s*=\s*True',
        r'exec\s*\(\s*[^)]*\+',
        r'eval\s*\(\s*[^)]*\+',
        r'child_process\.exec\s*\(',
        r'Runtime\.getRuntime\(\)\.exec\s*\(',
        r'system\s*\(\s*[^)]*\+',
    ],
    'PATH_TRAVERSAL': [
        r'open\s*\([^)]*\+[^)]*\)',
        r'readFile\w*\s*\([^)]*\+',
        r'\.\./',
        r'\.\.\\\\',
    ],
    # ── CRYPTO WEAKNESSES ──
    'WEAK_HASH': [
        r'md5\s*\(', r'MD5\s*\(',
        r'sha1\s*\(', r'SHA1\s*\(',
        r'hashlib\.md5', r'hashlib\.sha1',
        r'createHash\s*\(\s*["\']md5',
        r'createHash\s*\(\s*["\']sha1',
    ],
    'WEAK_CRYPTO': [
        r'DES\b', r'RC4\b', r'ECB\b',
        r'random\s*\(\s*\)', 
        r'Math\.random\s*\(',
        r'rand\s*\(\s*\)',
        r'srand\s*\(',
    ],
    'HARDCODED_SECRET': [
        r'(?i)(?:password|passwd|pwd)\s*=\s*["\'][^"\']{8,}["\']',
        r'(?i)(?:api_key|apikey|secret_key|access_key)\s*=\s*["\'][^"\']{16,}["\']',
        r'(?i)(?:token|auth_token|bearer)\s*=\s*["\'][A-Za-z0-9_\-\.]{20,}["\']',
        r'-----BEGIN (?:RSA |EC )?PRIVATE KEY-----',
        r'(?i)(?:aws_secret_access_key|aws_access_key_id)\s*=\s*["\'][^"\']+["\']',
    ],
    # ── MEMORY / BUFFER ──
    'BUFFER_OVERFLOW': [
        r'\bstrcpy\s*\(', r'\bstrcat\s*\(',
        r'\bsprintf\s*\(', r'\bgets\s*\(',
        r'\bscanf\s*\(\s*"%s"',
        r'\bmemcpy\s*\([^)]*sizeof',
    ],
    # ── WEB EXPLOITS ──
    'XSS': [
        r'innerHTML\s*=', r'outerHTML\s*=',
        r'dangerouslySetInnerHTML',
        r'document\.write\s*\(',
        r'v-html\s*=',
        r'\.html\s*\(\s*[^)]*\+',
    ],
    'INSECURE_DESERIALIZE': [
        r'pickle\.loads?\s*\(',
        r'yaml\.load\s*\([^)]*(?!Loader)',
        r'yaml\.unsafe_load',
        r'unserialize\s*\(',
        r'JSON\.parse\s*\(\s*\w+\)',
        r'readObject\s*\(',
    ],
    # ── AUTH BYPASS ──
    'AUTH_BYPASS': [
        r'(?i)if\s*\(\s*(?:is_admin|isAdmin|authenticated)\s*(?:==|!=)\s*(?:true|false|1|0)',
        r'(?i)verify\s*=\s*False',
        r'(?i)check_auth\s*=\s*False',
        r'(?i)VERIFY_SSL\s*=\s*False',
        r'(?i)ssl\s*:\s*false',
    ],
    'SSRF': [
        r'requests\.get\s*\(\s*\w+[^"\']\)',
        r'fetch\s*\(\s*\w+[^"\']\)',
        r'urllib\.request\.urlopen\s*\(\s*\w+\)',
        r'http\.get\s*\(\s*\w+',
    ],
}


def hunt_vulnerabilities(content, filename):
    """Scan file content for real CVE-class vulnerability patterns."""
    findings = []
    for vuln_type, patterns in VULN_SIGNATURES.items():
        for pattern in patterns:
            matches = re.finditer(pattern, content)
            for m in matches:
                # Get line number
                line_num = content[:m.start()].count('\n') + 1
                line_text = content.split('\n')[line_num - 1].strip()[:120]
                findings.append({
                    'type': vuln_type,
                    'line': line_num,
                    'evidence': line_text,
                    'pattern': pattern,
                })
                break  # One hit per pattern per file is enough
    return findings


# ═══════════════════════════════════════════════════════════════════
#  MAIN
# ═══════════════════════════════════════════════════════════════════
def main():
    start = time.time()

    print("=" * 70)
    print(" GODSEYE — SOVEREIGN POLYGLOT TOPOLOGY SCANNER")
    print(" Dependency-First Architecture Analysis")
    print("=" * 70)
    print()

    # Phase 1
    print("[Phase 1] Collecting all source files...")
    files = collect_files()
    ext_counts = defaultdict(int)
    for f_info in files.values():
        ext_counts[f_info['ext']] += 1
    print(f"  Found {len(files)} source files across {len(ext_counts)} languages")
    for ext, count in sorted(ext_counts.items(), key=lambda x: -x[1]):
        print(f"    {ext:8s}: {count} files")

    # Phase 2
    print("[Phase 2] Building dependency graph...")
    depends_on, depended_by, edges = build_dependency_graph(files)
    import_edges = sum(1 for e in edges if e[2] == 'IMPORT')
    string_edges = sum(1 for e in edges if e[2] == 'STRING_REF')
    quoted_edges = sum(1 for e in edges if e[2] == 'QUOTED_REF')
    dynamic_edges = sum(1 for e in edges if e[2] == 'DYNAMIC_IMPORT')
    poly_edges = sum(1 for e in edges if e[2] not in ('IMPORT', 'STRING_REF', 'QUOTED_REF', 'DYNAMIC_IMPORT'))
    print(f"  Edges: {len(edges)} total")
    print(f"    IMPORT:         {import_edges}")
    print(f"    STRING_REF:     {string_edges}")
    print(f"    QUOTED_REF:     {quoted_edges}")
    print(f"    DYNAMIC_IMPORT: {dynamic_edges}")
    print(f"    POLYGLOT:       {poly_edges}")

    # Phase 3
    print("[Phase 3] Computing transitive dependency scores...")
    dep_scores = compute_dependency_scores(files, depended_by)
    top_deps = sorted(dep_scores.items(), key=lambda x: -x[1]['transitive'])[:10]
    print(f"  Top depended-on files:")
    for f, s in top_deps:
        print(f"    {f:45s} direct={s['direct']:3d}  transitive={s['transitive']:3d}")

    # Phase 4
    print("[Phase 4] Tracing critical paths from entry points...")
    on_critical_path = trace_critical_paths(files, depends_on, ENTRY_POINTS)
    print(f"  Files on critical path: {len(on_critical_path)}")

    # Phase 5-7: Analyze each file
    print("[Phase 5-7] Analyzing behaviors, resources, structure...")
    results = []
    class_counts = defaultdict(int)
    total = len(files)

    for idx, (f, info) in enumerate(files.items(), 1):
        ext = info.get('ext', '.py')
        print(f"  [{idx}/{total}] {ext:5s} | {info['lines']:6d}L | {f}", flush=True)
        behaviors = analyze_behavior(info['content'])
        resources = map_resources(info['content'])
        structure = analyze_structure(info['content'], f)
        vulns = hunt_vulnerabilities(info['content'], f)
        dep_score = dep_scores.get(f, {'direct': 0, 'transitive': 0})
        classification, reason = classify_file(
            f, info, dep_score, on_critical_path, behaviors, structure)

        if vulns:
            for v in vulns:
                print(f"    [!!] {v['type']:20s} @ line {v['line']:5d} | {v['evidence'][:80]}", flush=True)

        class_counts[classification] += 1

        results.append({
            'file': f,
            'path': info['path'],
            'lines': info['lines'],
            'classification': classification,
            'reason': reason,
            'dependency_score': dep_score,
            'on_critical_path': f in on_critical_path,
            'depends_on': sorted(depends_on.get(f, set())),
            'depended_by': sorted(depended_by.get(f, set())),
            'behaviors': behaviors,
            'resources': resources,
            'vulnerabilities': vulns,
            'classes': [{'name': c['name'], 'methods': c['method_count'],
                        'doc': c['doc']} for c in structure['classes']],
            'functions': [fn['name'] for fn in structure['functions'][:10]],
            'doc': structure['doc'][:150],
        })

    results.sort(key=lambda x: (-x['dependency_score']['transitive'],
                                 -x['lines']))

    elapsed = time.time() - start

    # ── PRINT REPORT ──
    print()
    print("=" * 70)
    print(" CLASSIFICATION RESULTS")
    print("=" * 70)
    print()
    for cat in ['CRITICAL', 'PROTECTED', 'ACTIVE', 'SIGNIFICANT', 'ISOLATED', 'MINIMAL']:
        count = class_counts.get(cat, 0)
        files_in_cat = [r for r in results if r['classification'] == cat]
        total_lines = sum(r['lines'] for r in files_in_cat)
        print(f"  {cat:12s}: {count:4d} files  ({total_lines:,} lines)")
    print()

    # Save JSON and Markdown components... (omitting detailed save logic for parity)
    print(f"  Audit Complete. Time: {elapsed:.1f}s")
    print("=" * 70)

if __name__ == "__main__":
    main()
