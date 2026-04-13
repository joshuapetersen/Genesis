"""
ATS v3 — SOVEREIGN FUNCTIONAL ANATOMY SCANNER
===============================================
Goes beyond topology (who imports who) to map the actual
BEHAVIOR of every engine:

  - WHAT does it do? (Classes, functions, their purposes)
  - HOW does it do it? (DB access, file I/O, network, subprocess, math, crypto)
  - WHAT resources does it touch? (specific DBs, URLs, file paths, hardware)
  - WHAT is its role? (Reader, Writer, Router, Worker, Monitor, etc.)

This is the difference between an X-ray (v2) and a full-body MRI (v3).

Usage: python ats_v3.py
"""

import ast
import os
import re
import json
import sys
import time
from collections import defaultdict

# ─── CONFIGURATION ─────────────────────────────────────────────────
SCAN_DIRS = [
    r"C:\SarahCore",
]

SKIP_DIRS = ['.venv', 'node_modules', '.git', '__pycache__', 'Genesis_Zero', 'vault',
             'SarahCore_Archive', 'Sovereign_Engine_Cpp']

OUTPUT_PATH = r"C:\SarahCore\vault\ats_v3_anatomy.json"
OUTPUT_MD = r"C:\SarahCore\vault\ats_v3_anatomy.md"


# ─── BEHAVIOR SIGNATURES ──────────────────────────────────────────
# These patterns detect WHAT an engine does based on the actual code.

DB_PATTERNS = {
    "sqlite3":       r"sqlite3\.connect\(['\"]([^'\"]+)['\"]\)",
    "lancedb":       r"lancedb\.connect\(['\"]([^'\"]+)['\"]\)",
    "supabase":      r"create_client\(",
    "firebase":      r"firebase_admin",
}

NETWORK_PATTERNS = {
    "http_request":  r"requests\.(get|post|put|delete)\(",
    "http_server":   r"(Flask|FastAPI|uvicorn|BaseHTTPRequestHandler|socketio)",
    "websocket":     r"(websocket|webview)",
    "tcp_socket":    r"socket\.(socket|AF_INET|SOCK_STREAM)",
    "url_fetch":     r"urllib\.request\.(urlopen|urlretrieve)\(",
}

FILE_IO_PATTERNS = {
    "file_read":     r"open\([^)]+['\"]r['\"]",
    "file_write":    r"open\([^)]+['\"]w['\"]",
    "file_append":   r"open\([^)]+['\"]a['\"]",
    "json_load":     r"json\.load\(",
    "json_dump":     r"json\.dump\(",
    "os_walk":       r"os\.walk\(",
    "shutil":        r"shutil\.(copy|move|rmtree)\(",
}

PROCESS_PATTERNS = {
    "subprocess":    r"subprocess\.(run|Popen|call)\(",
    "os_system":     r"os\.system\(",
    "psutil":        r"psutil\.",
    "process_kill":  r"\.(kill|terminate)\(",
}

CRYPTO_PATTERNS = {
    "hmac":          r"hmac\.(new|digest)\(",
    "hashlib":       r"hashlib\.(sha256|sha512|md5)\(",
    "crypto":        r"(Fernet|AES|RSA|ECDSA)",
    "worm":          r"WORM",
}

ML_PATTERNS = {
    "llama":         r"(Llama|llama_cpp)",
    "torch":         r"(torch\.|nn\.Module|tensor)",
    "transformers":  r"(transformers|AutoModel|AutoTokenizer)",
    "embeddings":    r"(SentenceTransformer|encode\()",
    "numpy":         r"numpy|np\.",
}

MATH_PATTERNS = {
    "sovereign_math": r"(SOVEREIGN_ANCHOR|1\.09277703703)",
    "geometric":     r"(fibonacci|golden_ratio|phi|euler|pi\b)",
    "matrix":        r"(matrix|determinant|eigenvalue)",
    "resonance":     r"(resonance|harmonic|frequency)",
}

HARDWARE_PATTERNS = {
    "gpu_nvidia":    r"(nvidia-smi|cuda|CUDA|gpu_offload|cublas)",
    "cpu_info":      r"(cpu_count|cpu_percent|cpu_freq)",
    "memory_info":   r"(virtual_memory|swap_memory|mem_info)",
    "disk_info":     r"(disk_usage|disk_io_counters)",
}


# ─── AST ANALYZER ─────────────────────────────────────────────────
class EngineAnalyzer(ast.NodeVisitor):
    """Extract structural information from a Python AST."""

    def __init__(self):
        self.classes = []
        self.functions = []
        self.imports = []
        self.constants = []
        self.decorators = []

    def visit_ClassDef(self, node):
        methods = []
        for item in node.body:
            if isinstance(item, ast.FunctionDef):
                doc = ast.get_docstring(item) or ""
                methods.append({
                    "name": item.name,
                    "args": [a.arg for a in item.args.args if a.arg != "self"],
                    "doc": doc[:150],
                    "line": item.lineno,
                })
        self.classes.append({
            "name": node.name,
            "doc": (ast.get_docstring(node) or "")[:200],
            "methods": methods,
            "line": node.lineno,
            "bases": [self._get_name(b) for b in node.bases],
        })
        self.generic_visit(node)

    def visit_FunctionDef(self, node):
        # Only top-level functions (not methods)
        if not any(isinstance(p, ast.ClassDef) for p in ast.walk(node)):
            doc = ast.get_docstring(node) or ""
            self.functions.append({
                "name": node.name,
                "args": [a.arg for a in node.args.args],
                "doc": doc[:150],
                "line": node.lineno,
                "decorators": [self._get_name(d) for d in node.decorator_list],
            })
        self.generic_visit(node)

    def visit_Import(self, node):
        for alias in node.names:
            self.imports.append(alias.name)

    def visit_ImportFrom(self, node):
        if node.module:
            self.imports.append(node.module)

    def visit_Assign(self, node):
        # Capture top-level constants
        if isinstance(node, ast.Assign) and len(node.targets) == 1:
            target = node.targets[0]
            if isinstance(target, ast.Name) and target.id.isupper():
                try:
                    val = ast.literal_eval(node.value)
                    self.constants.append({"name": target.id, "value": str(val)[:100]})
                except:
                    pass
        self.generic_visit(node)

    def _get_name(self, node):
        if isinstance(node, ast.Name):
            return node.id
        elif isinstance(node, ast.Attribute):
            return f"{self._get_name(node.value)}.{node.attr}"
        elif isinstance(node, ast.Call):
            return self._get_name(node.func)
        return "?"


# ─── BEHAVIOR DETECTOR ────────────────────────────────────────────
def detect_behaviors(content):
    """Scan raw source code for behavioral patterns."""
    behaviors = {}

    all_pattern_groups = {
        "database": DB_PATTERNS,
        "network": NETWORK_PATTERNS,
        "file_io": FILE_IO_PATTERNS,
        "process": PROCESS_PATTERNS,
        "crypto": CRYPTO_PATTERNS,
        "ml_inference": ML_PATTERNS,
        "math": MATH_PATTERNS,
        "hardware": HARDWARE_PATTERNS,
    }

    for category, patterns in all_pattern_groups.items():
        hits = {}
        for name, pattern in patterns.items():
            matches = re.findall(pattern, content)
            if matches:
                hits[name] = len(matches)
        if hits:
            behaviors[category] = hits

    # Extract specific resources
    resources = {}

    # DB paths
    db_paths = re.findall(r"sqlite3\.connect\(['\"]([^'\"]+)['\"]\)", content)
    db_paths += re.findall(r"lancedb\.connect\(['\"]([^'\"]+)['\"]\)", content)
    if db_paths:
        resources["databases"] = list(set(db_paths))

    # URLs
    urls = re.findall(r"['\"]https?://[^'\"]+['\"]", content)
    if urls:
        resources["urls"] = [u.strip("'\"")[:100] for u in set(urls)][:5]

    # File paths accessed
    file_paths = re.findall(r"['\"]C:\\\\[^'\"]+['\"]", content)
    file_paths += re.findall(r"['\"]C:/[^'\"]+['\"]", content)
    file_paths += re.findall(r"r['\"]C:\\[^'\"]+['\"]", content)
    if file_paths:
        resources["file_paths"] = [p.strip("'\"r")[:100] for p in set(file_paths)][:10]

    return behaviors, resources


# ─── ROLE CLASSIFIER ──────────────────────────────────────────────
def classify_role(behaviors, structure):
    """Assign a functional role based on detected behaviors."""
    roles = []

    b = behaviors
    if "database" in b:
        if any(k in b.get("file_io", {}) for k in ["file_write", "json_dump", "file_append"]):
            roles.append("VAULT_WRITER")
        else:
            roles.append("VAULT_READER")

    if "network" in b:
        if any(k in b["network"] for k in ["http_server", "tcp_socket"]):
            roles.append("SERVER")
        if "http_request" in b["network"]:
            roles.append("CLIENT")

    if "ml_inference" in b:
        roles.append("INFERENCE_ENGINE")

    if "crypto" in b:
        roles.append("CRYPTO_GUARDIAN")

    if "hardware" in b:
        roles.append("HARDWARE_MONITOR")

    if "process" in b:
        roles.append("PROCESS_MANAGER")

    if "math" in b:
        roles.append("MATH_ENGINE")

    if structure.get("classes") and not roles:
        roles.append("CLASS_LIBRARY")

    if structure.get("functions") and not roles:
        roles.append("UTILITY")

    if not roles:
        roles.append("UNKNOWN")

    return roles


# ─── MAIN SCAN ─────────────────────────────────────────────────────
def scan_all():
    """Scan every .py file and produce the full anatomy."""
    results = []
    total_classes = 0
    total_functions = 0
    total_lines = 0
    role_counts = defaultdict(int)

    for scan_dir in SCAN_DIRS:
        if not os.path.exists(scan_dir):
            continue
        for root, dirs, files in os.walk(scan_dir):
            if any(s in root for s in SKIP_DIRS):
                continue
            for f in files:
                if not f.endswith('.py'):
                    continue

                full_path = os.path.join(root, f)
                try:
                    with open(full_path, 'r', encoding='utf-8', errors='ignore') as fh:
                        content = fh.read()
                except:
                    continue

                lines = content.count('\n') + 1
                total_lines += lines

                # AST analysis
                structure = {"classes": [], "functions": [], "imports": [], "constants": []}
                try:
                    tree = ast.parse(content, filename=f)
                    analyzer = EngineAnalyzer()
                    analyzer.visit(tree)
                    structure = {
                        "classes": analyzer.classes,
                        "functions": analyzer.functions,
                        "imports": analyzer.imports,
                        "constants": analyzer.constants[:10],
                    }
                except SyntaxError:
                    pass

                total_classes += len(structure["classes"])
                total_functions += len(structure["functions"])

                # Behavior detection
                behaviors, resources = detect_behaviors(content)

                # Role classification
                roles = classify_role(behaviors, structure)
                for r in roles:
                    role_counts[r] += 1

                # Module docstring
                doc = ""
                try:
                    tree = ast.parse(content)
                    doc = (ast.get_docstring(tree) or "")[:200]
                except:
                    pass

                results.append({
                    "file": f,
                    "path": full_path,
                    "lines": lines,
                    "doc": doc,
                    "roles": roles,
                    "classes": [{"name": c["name"], "methods": len(c["methods"]), "doc": c["doc"][:80]} for c in structure["classes"]],
                    "functions": [{"name": fn["name"], "args": fn["args"], "doc": fn["doc"][:80]} for fn in structure["functions"][:15]],
                    "behaviors": behaviors,
                    "resources": resources,
                    "imports": structure["imports"][:20],
                    "constants": structure["constants"][:5],
                })

    results.sort(key=lambda x: x["lines"], reverse=True)

    report = {
        "scan_timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "scanner_version": "ATS v3 — Functional Anatomy",
        "total_files": len(results),
        "total_lines": total_lines,
        "total_classes": total_classes,
        "total_functions": total_functions,
        "role_distribution": dict(role_counts),
        "engines": results,
    }

    return report


# ─── MARKDOWN GENERATOR ───────────────────────────────────────────
def generate_markdown(report):
    """Generate a human-readable anatomy report."""
    lines = []
    lines.append("# ATS v3 — Sovereign Functional Anatomy\n")
    lines.append(f"> Scanned: {report['scan_timestamp']}\n")
    lines.append(f"| Metric | Count |")
    lines.append(f"|---|---|")
    lines.append(f"| Files | {report['total_files']} |")
    lines.append(f"| Lines | {report['total_lines']:,} |")
    lines.append(f"| Classes | {report['total_classes']} |")
    lines.append(f"| Functions | {report['total_functions']} |")
    lines.append("")

    # Role distribution
    lines.append("## Role Distribution\n")
    lines.append("| Role | Count |")
    lines.append("|---|---|")
    for role, count in sorted(report["role_distribution"].items(), key=lambda x: -x[1]):
        lines.append(f"| {role} | {count} |")
    lines.append("")

    # Top 50 engines with full anatomy
    lines.append("## Engine Anatomy (Top 50 by size)\n")
    for eng in report["engines"][:50]:
        roles_str = ", ".join(eng["roles"])
        lines.append(f"### {eng['file']} ({eng['lines']} lines) — `{roles_str}`\n")
        if eng["doc"]:
            lines.append(f"> {eng['doc'][:150]}\n")
        lines.append(f"**Path:** `{eng['path']}`\n")

        # Classes
        if eng["classes"]:
            lines.append("**Classes:**")
            for c in eng["classes"]:
                lines.append(f"- `{c['name']}` ({c['methods']} methods) — {c['doc'][:60]}")
            lines.append("")

        # Functions
        if eng["functions"]:
            lines.append("**Functions:**")
            for fn in eng["functions"][:8]:
                args = ", ".join(fn["args"][:4])
                lines.append(f"- `{fn['name']}({args})` — {fn['doc'][:60]}")
            lines.append("")

        # Behaviors
        if eng["behaviors"]:
            lines.append("**Behaviors:**")
            for category, hits in eng["behaviors"].items():
                hit_str = ", ".join(f"{k}({v})" for k, v in hits.items())
                lines.append(f"- **{category}**: {hit_str}")
            lines.append("")

        # Resources
        if eng["resources"]:
            if eng["resources"].get("databases"):
                lines.append(f"**Databases:** {', '.join(eng['resources']['databases'][:3])}")
            if eng["resources"].get("urls"):
                lines.append(f"**URLs:** {', '.join(eng['resources']['urls'][:2])}")
            lines.append("")

        lines.append("---\n")

    return "\n".join(lines)


# ─── MAIN ──────────────────────────────────────────────────────────
if __name__ == "__main__":
    start = time.time()

    print("=" * 60)
    print(" ATS v3 — SOVEREIGN FUNCTIONAL ANATOMY SCANNER")
    print("=" * 60)
    print()

    print("[ATS v3] Scanning...")
    report = scan_all()
    elapsed = time.time() - start

    # Save JSON
    os.makedirs(os.path.dirname(OUTPUT_PATH), exist_ok=True)
    with open(OUTPUT_PATH, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2, default=str)

    # Save Markdown
    md = generate_markdown(report)
    with open(OUTPUT_MD, "w", encoding="utf-8") as f:
        f.write(md)

    # Print summary
    print(f"  Files:     {report['total_files']}")
    print(f"  Lines:     {report['total_lines']:,}")
    print(f"  Classes:   {report['total_classes']}")
    print(f"  Functions: {report['total_functions']}")
    print(f"  Scan Time: {elapsed:.1f}s")
    print()

    print("  ROLE DISTRIBUTION:")
    for role, count in sorted(report["role_distribution"].items(), key=lambda x: -x[1]):
        bar = "█" * min(count, 40)
        print(f"    {role:20s} {bar} {count}")
    print()

    # Top 15 engines
    print("  TOP 15 ENGINES (by size):")
    for eng in report["engines"][:15]:
        roles = ", ".join(eng["roles"])
        cls_count = len(eng["classes"])
        fn_count = len(eng["functions"])
        beh_count = len(eng["behaviors"])
        print(f"    {eng['file']:45s} {eng['lines']:5d} lines | {cls_count:2d} cls | {fn_count:2d} fn | {beh_count} behaviors | [{roles}]")
    print()

    # Behavior summary
    behavior_totals = defaultdict(int)
    for eng in report["engines"]:
        for cat in eng["behaviors"]:
            behavior_totals[cat] += 1

    print("  BEHAVIOR COVERAGE:")
    for cat, count in sorted(behavior_totals.items(), key=lambda x: -x[1]):
        pct = count / max(report["total_files"], 1) * 100
        print(f"    {cat:20s} {count:4d} engines ({pct:.0f}%)")
    print()

    # Resource summary
    all_dbs = set()
    for eng in report["engines"]:
        for db in eng.get("resources", {}).get("databases", []):
            all_dbs.add(db)

    if all_dbs:
        print("  DATABASES TOUCHED:")
        for db in sorted(all_dbs):
            print(f"    {db}")
    print()

    print(f"  JSON:     {OUTPUT_PATH}")
    print(f"  Markdown: {OUTPUT_MD}")
    print("=" * 60)
