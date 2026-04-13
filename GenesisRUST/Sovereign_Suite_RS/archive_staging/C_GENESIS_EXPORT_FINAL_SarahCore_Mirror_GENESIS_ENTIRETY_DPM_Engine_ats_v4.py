"""
ATS v4 — SOVEREIGN TOPOLOGY SCANNER
=====================================
Dependency-first architecture scanner. Builds the complete
dependency graph BEFORE classifying anything.

DESIGN PRINCIPLE: A file's importance is determined by
HOW MANY OTHER FILES NEED IT, not by its size or name.

Features:
  1. Full dependency graph (AST + string refs + subprocess + importlib)
  2. Transitive dependency scoring (cascade analysis)
  3. Critical path tracing from entry points
  4. Protected name enforcement (sarah/sovereign/genesis/ace/aeris)
  5. Dry-run archive simulation
  6. Resource mapping (DBs, URLs, hardware, file paths)
  7. Behavioral analysis (what each file actually does)

Usage:
  python ats_v4.py                   # Full scan + report
  python ats_v4.py --dry-run FILE    # Simulate archiving FILE
"""

import ast
import os
import re
import json
import sys
import time
from collections import defaultdict, deque

# ─── CONFIGURATION ─────────────────────────────────────────────────
SCAN_ROOT = r"C:\SarahCore"
SKIP_DIRS = ['.venv', '__pycache__', '.git', 'node_modules',
             'Genesis_Zero', 'vault', 'Sovereign_Engine_Cpp',
             'SarahCore_Archive']

OUTPUT_JSON = r"C:\SarahCore\vault\ats_v4_topology.json"
OUTPUT_MD   = r"C:\SarahCore\vault\ats_v4_report.md"

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
    """Walk the directory tree and read every .py file."""
    files = {}
    for root, dirs, filenames in os.walk(SCAN_ROOT):
        if any(s in root for s in SKIP_DIRS):
            continue
        for f in filenames:
            if not f.endswith('.py'):
                continue
            full = os.path.join(root, f)
            try:
                with open(full, 'r', encoding='utf-8', errors='ignore') as fh:
                    content = fh.read()
                files[f] = {
                    'path': full,
                    'content': content,
                    'lines': content.count('\n') + 1,
                    'basename': os.path.splitext(f)[0],
                }
            except:
                pass
    return files


# ═══════════════════════════════════════════════════════════════════
#  PHASE 2: BUILD DEPENDENCY GRAPH (THE CORE IMPROVEMENT)
# ═══════════════════════════════════════════════════════════════════
def build_dependency_graph(files):
    """
    Build a complete dependency graph using MULTIPLE detection methods.
    Returns:
      depends_on[file] = set of files this file depends on
      depended_by[file] = set of files that depend on this file
      edges = list of (source, target, type) tuples
    """
    basename_to_file = {}
    for f, info in files.items():
        basename_to_file[info['basename']] = f

    depends_on = defaultdict(set)    # file -> set of files it imports/references
    depended_by = defaultdict(set)   # file -> set of files that need it
    edges = []

    for f, info in files.items():
        content = info['content']
        basename = info['basename']

        # ── METHOD 1: AST Imports ──────────────────────────────
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

        # ── METHOD 2: String References (subprocess, importlib) ──
        for candidate_base, candidate_file in basename_to_file.items():
            if candidate_file == f:
                continue

            # subprocess.run(["python", "file.py"])
            if candidate_base + '.py' in content:
                if candidate_file not in depends_on[f]:
                    depends_on[f].add(candidate_file)
                    depended_by[candidate_file].add(f)
                    edges.append((f, candidate_file, 'STRING_REF'))

            # importlib.import_module("module_name")
            elif re.search(r'import_module\(["\']' + re.escape(candidate_base) + r'["\']\)', content):
                if candidate_file not in depends_on[f]:
                    depends_on[f].add(candidate_file)
                    depended_by[candidate_file].add(f)
                    edges.append((f, candidate_file, 'DYNAMIC_IMPORT'))

            # Quoted references: "module_name" or 'module_name'
            elif re.search(r'["\']' + re.escape(candidate_base) + r'["\']', content):
                if candidate_file not in depends_on[f]:
                    depends_on[f].add(candidate_file)
                    depended_by[candidate_file].add(f)
                    edges.append((f, candidate_file, 'QUOTED_REF'))

    return depends_on, depended_by, edges


# ═══════════════════════════════════════════════════════════════════
#  PHASE 3: TRANSITIVE DEPENDENCY SCORING
# ═══════════════════════════════════════════════════════════════════
def compute_dependency_scores(files, depended_by):
    """
    For each file, compute how many files depend on it TRANSITIVELY.
    If A depends on B, and B depends on C, then C's transitive
    score includes A.
    """
    scores = {}
    for f in files:
        # BFS from this file through depended_by
        visited = set()
        queue = deque([f])
        while queue:
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
#  PHASE 7: STRUCTURAL ANALYSIS
# ═══════════════════════════════════════════════════════════════════
def analyze_structure(content, filename):
    """Extract classes, functions, docstring."""
    result = {'classes': [], 'functions': [], 'doc': '', 'constants': []}

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
                # Top-level only (rough check)
                result['functions'].append({
                    'name': node.name,
                    'args': [a.arg for a in node.args.args if a.arg != 'self'],
                    'doc': (ast.get_docstring(node) or '')[:80],
                })
    except:
        pass

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
#  MAIN
# ═══════════════════════════════════════════════════════════════════
def main():
    start = time.time()

    print("=" * 70)
    print(" ATS v4 — SOVEREIGN TOPOLOGY SCANNER")
    print(" Dependency-First Architecture Analysis")
    print("=" * 70)
    print()

    # Phase 1
    print("[Phase 1] Collecting files...")
    files = collect_files()
    print(f"  Found {len(files)} .py files")

    # Phase 2
    print("[Phase 2] Building dependency graph...")
    depends_on, depended_by, edges = build_dependency_graph(files)
    import_edges = sum(1 for e in edges if e[2] == 'IMPORT')
    string_edges = sum(1 for e in edges if e[2] == 'STRING_REF')
    quoted_edges = sum(1 for e in edges if e[2] == 'QUOTED_REF')
    dynamic_edges = sum(1 for e in edges if e[2] == 'DYNAMIC_IMPORT')
    print(f"  Edges: {len(edges)} total")
    print(f"    IMPORT:        {import_edges}")
    print(f"    STRING_REF:    {string_edges}")
    print(f"    QUOTED_REF:    {quoted_edges}")
    print(f"    DYNAMIC_IMPORT:{dynamic_edges}")

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

    for f, info in files.items():
        behaviors = analyze_behavior(info['content'])
        resources = map_resources(info['content'])
        structure = analyze_structure(info['content'], f)
        dep_score = dep_scores.get(f, {'direct': 0, 'transitive': 0})
        classification, reason = classify_file(
            f, info, dep_score, on_critical_path, behaviors, structure)

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

    # CRITICAL
    critical = [r for r in results if r['classification'] == 'CRITICAL']
    print(f"  CRITICAL ({len(critical)} files) — DO NOT TOUCH:")
    for r in critical[:25]:
        ds = r['dependency_score']
        cp = "CP" if r['on_critical_path'] else "  "
        print(f"    [{cp}] {r['file']:45s} {r['lines']:5d}L  deps={ds['direct']:2d}/{ds['transitive']:3d}  {r['reason'][:50]}")
    if len(critical) > 25:
        print(f"    ... and {len(critical)-25} more")
    print()

    # PROTECTED
    protected = [r for r in results if r['classification'] == 'PROTECTED']
    print(f"  PROTECTED ({len(protected)} files) — Has critical name:")
    for r in protected:
        print(f"    {r['file']:45s} {r['lines']:5d}L  {r['reason'][:60]}")
    print()

    # ACTIVE
    active = [r for r in results if r['classification'] == 'ACTIVE']
    print(f"  ACTIVE ({len(active)} files) — Has dependents:")
    for r in active:
        ds = r['dependency_score']
        print(f"    {r['file']:45s} {r['lines']:5d}L  deps={ds['direct']:2d}/{ds['transitive']:3d}")
    print()

    # SIGNIFICANT
    significant = [r for r in results if r['classification'] == 'SIGNIFICANT']
    print(f"  SIGNIFICANT ({len(significant)} files) — Real logic, not connected:")
    for r in significant[:15]:
        print(f"    {r['file']:45s} {r['lines']:5d}L  {r['reason'][:55]}")
    if len(significant) > 15:
        print(f"    ... and {len(significant)-15} more")
    print()

    # ISOLATED
    isolated = [r for r in results if r['classification'] == 'ISOLATED']
    print(f"  ISOLATED ({len(isolated)} files) — No deps, no protected name:")
    for r in isolated[:10]:
        print(f"    {r['file']:45s} {r['lines']:5d}L")
    if len(isolated) > 10:
        print(f"    ... and {len(isolated)-10} more")
    print()

    # MINIMAL
    minimal = [r for r in results if r['classification'] == 'MINIMAL']
    print(f"  MINIMAL ({len(minimal)} files) — Stubs:")
    for r in minimal:
        print(f"    {r['file']:45s} {r['lines']:5d}L")
    print()

    # Save
    report = {
        'scanner': 'ATS v4 — Sovereign Topology Scanner',
        'timestamp': time.strftime('%Y-%m-%dT%H:%M:%S'),
        'scan_time_seconds': round(elapsed, 1),
        'total_files': len(results),
        'total_lines': sum(r['lines'] for r in results),
        'total_edges': len(edges),
        'edge_types': {
            'IMPORT': import_edges,
            'STRING_REF': string_edges,
            'QUOTED_REF': quoted_edges,
            'DYNAMIC_IMPORT': dynamic_edges,
        },
        'critical_path_files': len(on_critical_path),
        'classification_counts': dict(class_counts),
        'entry_points': ENTRY_POINTS,
        'engines': results,
    }

    os.makedirs(os.path.dirname(OUTPUT_JSON), exist_ok=True)
    with open(OUTPUT_JSON, 'w', encoding='utf-8') as f:
        json.dump(report, f, indent=2, default=str)

    # Generate markdown
    md_lines = []
    md_lines.append("# ATS v4 — Sovereign Topology Report\n")
    md_lines.append(f"> Scanned: {report['timestamp']} | {elapsed:.1f}s\n")
    md_lines.append(f"| Metric | Value |")
    md_lines.append(f"|---|---|")
    md_lines.append(f"| Files | {report['total_files']} |")
    md_lines.append(f"| Lines | {report['total_lines']:,} |")
    md_lines.append(f"| Edges | {report['total_edges']} |")
    md_lines.append(f"| Critical Path | {report['critical_path_files']} files |")
    md_lines.append("")
    md_lines.append("## Classification\n")
    md_lines.append("| Category | Files | Lines | Meaning |")
    md_lines.append("|---|---|---|---|")
    descs = {
        'CRITICAL': 'On critical path or heavily depended on',
        'PROTECTED': 'Has sovereign name, may be dynamically loaded',
        'ACTIVE': 'Other files depend on this',
        'SIGNIFICANT': 'Real logic but no current dependents',
        'ISOLATED': 'No dependencies in or out',
        'MINIMAL': 'Stub or placeholder (≤5 lines)',
    }
    for cat in ['CRITICAL', 'PROTECTED', 'ACTIVE', 'SIGNIFICANT', 'ISOLATED', 'MINIMAL']:
        fc = [r for r in results if r['classification'] == cat]
        md_lines.append(f"| {cat} | {len(fc)} | {sum(r['lines'] for r in fc):,} | {descs.get(cat, '')} |")
    md_lines.append("")

    for r in results[:60]:
        ds = r['dependency_score']
        md_lines.append(f"### {r['file']} — `{r['classification']}`\n")
        md_lines.append(f"**{r['lines']} lines** | Direct deps: {ds['direct']} | Transitive: {ds['transitive']} | Path: `{r['path']}`\n")
        if r['doc']:
            md_lines.append(f"> {r['doc'][:120]}\n")
        md_lines.append(f"**Verdict:** {r['reason']}\n")
        if r['depends_on']:
            md_lines.append(f"**Depends on:** {', '.join(r['depends_on'][:8])}")
        if r['depended_by']:
            md_lines.append(f"**Depended by:** {', '.join(r['depended_by'][:8])}")
        if r['behaviors']:
            md_lines.append(f"**Behaviors:** {', '.join(r['behaviors'])}")
        if r['classes']:
            cls = ', '.join(f"{c['name']}({c['methods']}m)" for c in r['classes'][:3])
            md_lines.append(f"**Classes:** {cls}")
        md_lines.append("\n---\n")

    with open(OUTPUT_MD, 'w', encoding='utf-8') as f:
        f.write('\n'.join(md_lines))

    print(f"  JSON:     {OUTPUT_JSON}")
    print(f"  Markdown: {OUTPUT_MD}")
    print(f"  Time:     {elapsed:.1f}s")
    print("=" * 70)


if __name__ == '__main__':
    main()
