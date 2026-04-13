"""
ATS v3.1 — DEEP PURPOSE AUDIT
==============================
For every file in SarahCore, this scanner:

1. Reads the actual code (not just patterns)
2. Determines its ORIGINAL PURPOSE from docstrings/comments
3. Classifies it: CORE | SCAFFOLDING | ONE-OFF | STUB | REPURPOSABLE
4. Measures its "weight" (real logic vs boilerplate)
5. Suggests repurposing opportunities

This is a surgical audit — no file gets removed without a verdict.
"""

import ast
import os
import re
import json
import time
from collections import defaultdict

SOVEREIGN_ROOT = r"C:\SarahCore"
SKIP = ['.venv', '__pycache__', '.git', 'node_modules', 'Genesis_Zero', 'vault',
        'Sovereign_Engine_Cpp', 'SarahCore_Archive']
OUTPUT = r"C:\SarahCore\vault\deep_purpose_audit.json"


def get_file_purpose(filepath, content, filename):
    """Deep analysis of a single file to determine its purpose."""
    lines = content.split('\n')
    line_count = len(lines)
    
    # ── 1. EXTRACT INTENT ──────────────────────────────────────
    # From docstrings, comments, class/function names
    
    intent = ""
    # Module docstring
    try:
        tree = ast.parse(content)
        doc = ast.get_docstring(tree) or ""
        intent = doc[:300]
    except:
        pass
    
    # If no docstring, grab first comment block
    if not intent:
        comment_lines = []
        for l in lines[:20]:
            stripped = l.strip()
            if stripped.startswith('#'):
                comment_lines.append(stripped.lstrip('#').strip())
            elif stripped.startswith('"""') or stripped.startswith("'''"):
                comment_lines.append(stripped.strip('"\'').strip())
        intent = ' '.join(comment_lines)[:300]
    
    # If STILL nothing, use the first few meaningful lines
    if not intent:
        meaningful = [l.strip() for l in lines[:10] if l.strip() and not l.strip().startswith(('import ', 'from ', '#'))]
        intent = ' '.join(meaningful)[:300]
    
    # ── 2. STRUCTURAL ANALYSIS ─────────────────────────────────
    classes = []
    functions = []
    try:
        tree = ast.parse(content)
        for node in ast.walk(tree):
            if isinstance(node, ast.ClassDef):
                methods = [n.name for n in node.body if isinstance(n, ast.FunctionDef)]
                classes.append({"name": node.name, "methods": methods, "line": node.lineno})
            elif isinstance(node, ast.FunctionDef):
                # Only top-level
                functions.append({"name": node.name, "line": node.lineno,
                                  "args": [a.arg for a in node.args.args]})
    except:
        pass
    
    # ── 3. WEIGHT ANALYSIS ─────────────────────────────────────
    # How much "real logic" vs boilerplate?
    
    code_lines = 0
    comment_lines = 0
    blank_lines = 0
    import_lines = 0
    
    for l in lines:
        stripped = l.strip()
        if not stripped:
            blank_lines += 1
        elif stripped.startswith('#'):
            comment_lines += 1
        elif stripped.startswith(('import ', 'from ')):
            import_lines += 1
        else:
            code_lines += 1
    
    logic_density = code_lines / max(line_count, 1)
    
    # ── 4. ONE-OFF DETECTION ───────────────────────────────────
    # Signs of a one-off script:
    one_off_signals = 0
    
    # Has a print-heavy main block
    if content.count('print(') > 5 and line_count < 100:
        one_off_signals += 1
    
    # Has hardcoded paths that look like test runs
    if re.search(r'http://127\.0\.0\.1|http://localhost', content):
        one_off_signals += 1
    
    # Starts with "test_" or "verify_" or "check_" or "debug_"
    basename = os.path.splitext(filename)[0]
    if basename.startswith(('test_', 'verify_', 'check_', 'debug_', 'find_', 'audit_')):
        one_off_signals += 1
    
    # No classes, no reusable functions, just imperative code
    if not classes and len(functions) <= 1 and line_count < 80:
        one_off_signals += 1
    
    # Has API keys or tokens hardcoded
    if re.search(r'ya29\.|sk-|api_key\s*=\s*["\']', content):
        one_off_signals += 1
    
    # ── 5. STUB DETECTION ──────────────────────────────────────
    is_stub = line_count <= 10 or (code_lines <= 5 and not classes)
    
    # ── 6. SCAFFOLDING DETECTION ───────────────────────────────
    # Ran once to set up something (DB tables, downloads, etc)
    scaffolding_signals = 0
    
    if re.search(r'CREATE TABLE|ALTER TABLE|ADD COLUMN', content):
        scaffolding_signals += 1
    if re.search(r'pip install|npm install', content):
        scaffolding_signals += 1
    if re.search(r'os\.makedirs|mkdir', content) and not classes:
        scaffolding_signals += 1
    if re.search(r'download|fetch|pull|clone', content.lower()) and not classes:
        scaffolding_signals += 1
    if basename.startswith(('setup_', 'install_', 'init_', 'create_', 'build_', 'seed_')):
        scaffolding_signals += 1
    
    # ── 7. REPURPOSE POTENTIAL ─────────────────────────────────
    repurpose = []
    
    # Has useful classes that could be imported
    for c in classes:
        if len(c["methods"]) >= 3:
            repurpose.append(f"Class '{c['name']}' has {len(c['methods'])} methods — candidate for bus registration")
    
    # Has standalone utility functions
    useful_fns = [f for f in functions if not f["name"].startswith('_') and f["name"] != "main"]
    if useful_fns and len(useful_fns) >= 2:
        fn_names = [f["name"] for f in useful_fns[:5]]
        repurpose.append(f"Has reusable functions: {', '.join(fn_names)}")
    
    # Has algorithm implementations
    if re.search(r'(fibonacci|sort|search|hash|encrypt|compress|parse|tokenize)', content.lower()):
        repurpose.append("Contains algorithm implementations")
    
    # Has data structures
    if re.search(r'class\s+\w+.*\(.*Enum\)|@dataclass|namedtuple', content):
        repurpose.append("Contains data structures/enums")
    
    # ── 8. CLASSIFICATION ──────────────────────────────────────
    if is_stub:
        classification = "STUB"
        verdict = "Empty or near-empty file. Safe to archive."
    elif scaffolding_signals >= 2:
        classification = "SCAFFOLDING"
        verdict = "Setup/installation script. Ran once. Archive unless needed again."
    elif one_off_signals >= 3:
        classification = "ONE-OFF"
        verdict = "Single-use script. Archive."
    elif one_off_signals >= 2 and not classes:
        classification = "ONE-OFF"
        verdict = "Likely a single-use diagnostic or query script."
    elif classes and any(len(c["methods"]) >= 5 for c in classes):
        classification = "CORE"
        verdict = "Substantial engine with real logic. KEEP."
    elif classes and any(len(c["methods"]) >= 2 for c in classes):
        classification = "REPURPOSABLE"
        verdict = "Has useful classes but isn't wired in. Consider integrating."
    elif len(useful_fns) >= 3 and line_count >= 50:
        classification = "REPURPOSABLE"
        verdict = "Has reusable utility functions. Consider integrating."
    elif line_count >= 100 and logic_density > 0.4:
        classification = "CORE"
        verdict = "Substantial logic. Needs review for bus registration."
    elif line_count >= 50 and logic_density > 0.3:
        classification = "REPURPOSABLE"
        verdict = "Moderate logic. Could be merged into a parent engine."
    else:
        classification = "ONE-OFF"
        verdict = "Small script with limited reuse potential."
    
    return {
        "file": filename,
        "path": filepath,
        "lines": line_count,
        "code_lines": code_lines,
        "logic_density": round(logic_density, 2),
        "intent": intent[:250],
        "classification": classification,
        "verdict": verdict,
        "classes": [{"name": c["name"], "methods": len(c["methods"])} for c in classes],
        "functions": [f["name"] for f in useful_fns[:10]],
        "repurpose_potential": repurpose,
        "signals": {
            "one_off": one_off_signals,
            "scaffolding": scaffolding_signals,
            "is_stub": is_stub,
        }
    }


def main():
    start = time.time()
    
    print("=" * 60)
    print(" ATS v3.1 — DEEP PURPOSE AUDIT")
    print("=" * 60)
    print()
    
    results = []
    
    for root, dirs, files in os.walk(SOVEREIGN_ROOT):
        if any(s in root for s in SKIP):
            continue
        for f in files:
            if not f.endswith('.py'):
                continue
            full = os.path.join(root, f)
            try:
                with open(full, 'r', encoding='utf-8', errors='ignore') as fh:
                    content = fh.read()
            except:
                continue
            
            result = get_file_purpose(full, content, f)
            results.append(result)
    
    results.sort(key=lambda x: x["lines"], reverse=True)
    
    # Count classifications
    counts = defaultdict(int)
    line_totals = defaultdict(int)
    for r in results:
        counts[r["classification"]] += 1
        line_totals[r["classification"]] += r["lines"]
    
    elapsed = time.time() - start
    
    # ── PRINT SUMMARY ──
    print(f"  Files scanned: {len(results)}")
    print(f"  Scan time:     {elapsed:.1f}s")
    print()
    
    print("  CLASSIFICATION BREAKDOWN:")
    print(f"  {'Category':15s} {'Files':>6s} {'Lines':>8s} {'Avg':>6s}")
    print(f"  {'-'*37}")
    for cat in ["CORE", "REPURPOSABLE", "ONE-OFF", "SCAFFOLDING", "STUB"]:
        c = counts.get(cat, 0)
        l = line_totals.get(cat, 0)
        avg = l // max(c, 1)
        print(f"  {cat:15s} {c:6d} {l:8d} {avg:6d}")
    print()
    
    # CORE engines
    core = [r for r in results if r["classification"] == "CORE"]
    print(f"  CORE ENGINES ({len(core)}):")
    for r in core[:20]:
        cls_info = ", ".join(f"{c['name']}({c['methods']}m)" for c in r["classes"][:3])
        print(f"    {r['file']:45s} {r['lines']:5d} lines  {cls_info}")
    if len(core) > 20:
        print(f"    ... and {len(core)-20} more")
    print()
    
    # REPURPOSABLE
    repur = [r for r in results if r["classification"] == "REPURPOSABLE"]
    print(f"  REPURPOSABLE ({len(repur)}):")
    for r in repur[:15]:
        rp = r["repurpose_potential"][0] if r["repurpose_potential"] else r["verdict"]
        print(f"    {r['file']:45s} {r['lines']:5d} lines  {rp[:60]}")
    if len(repur) > 15:
        print(f"    ... and {len(repur)-15} more")
    print()
    
    # ONE-OFF
    oneoff = [r for r in results if r["classification"] == "ONE-OFF"]
    print(f"  ONE-OFF SCRIPTS ({len(oneoff)}):")
    for r in oneoff[:10]:
        print(f"    {r['file']:45s} {r['lines']:5d} lines  {r['intent'][:60]}")
    if len(oneoff) > 10:
        print(f"    ... and {len(oneoff)-10} more")
    print()
    
    # SCAFFOLDING
    scaff = [r for r in results if r["classification"] == "SCAFFOLDING"]
    print(f"  SCAFFOLDING ({len(scaff)}):")
    for r in scaff:
        print(f"    {r['file']:45s} {r['lines']:5d} lines  {r['intent'][:60]}")
    print()
    
    # STUBS
    stubs = [r for r in results if r["classification"] == "STUB"]
    print(f"  STUBS ({len(stubs)}):")
    for r in stubs:
        print(f"    {r['file']:45s} {r['lines']:5d} lines")
    print()
    
    # Save full report
    report = {
        "scan_timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "total_files": len(results),
        "classification_counts": dict(counts),
        "classification_lines": dict(line_totals),
        "engines": results,
    }
    
    os.makedirs(os.path.dirname(OUTPUT), exist_ok=True)
    with open(OUTPUT, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2, default=str)
    
    print(f"  Full report: {OUTPUT}")
    print("=" * 60)


if __name__ == "__main__":
    main()
