"""
SOVEREIGN ATS v2 — Automatic Topology Scanner (Upgraded)
=========================================================
Physical scanner for the SarahCore Digital Brain.

Upgrades over v1:
  1. Reports BIDIRECTIONAL connectivity (in-count AND out-count per engine)
  2. Detects engines registered on the Neural Pulse Bus
  3. Reports MESH DENSITY per sector
  4. Flags remaining orphans that are NOT on the bus
  5. Generates both Mermaid topology AND a connectivity report

This is the digital equivalent of a DTI (Diffusion Tensor Imaging) scan.
It doesn't just see files — it sees the FLOW of logic.

Usage: python topology_scanner.py
"""

import os
import ast
import re
import json
import sys
import time

# ─── CONFIGURATION ─────────────────────────────────────────────────
DIRECTORIES = [
    r"C:\Genlex_Linear",
    r"C:\Genlex_Core",
    r"C:\Genlex_Frequency",
    r"C:\genlex_repo",
    r"C:\04_THE_MEMORY",
    r"C:\05_THE_CORE",
    r"C:\Aethelgard",
    r"C:\archive_memories",
    r"C:\DPM_Engine",
    r"C:\Genesis_Bridge",
    r"C:\PrimordialEarth",
    r"C:\S-OS_Build",
    r"C:\Sovereign",
    r"C:\Sovereign_Native",
    r"C:\Sumerian_Grid",
    r"C:\Sarah_Sidecars",
    r"C:\SarahCore",
    r"C:\SarahCore.worktrees"
]

SKIP_DIRS = ['.venv', 'node_modules', '.git', '__pycache__', 'Genesis_Zero']

OUTPUT_TOPOLOGY = r"C:\SarahCore\True_Sovereign_Topology.md"
OUTPUT_REPORT = r"C:\SarahCore\vault\connectivity_report.json"


# ─── PHASE 1: SCAN ─────────────────────────────────────────────────
def scan_files():
    """Walk all 19 directories and collect every .py and .cpp file."""
    nodes = set()
    file_contents = {}
    file_paths = {}  # filename -> full path

    for d in DIRECTORIES:
        if not os.path.exists(d):
            continue
        for root, dirs, files in os.walk(d):
            if any(skip in root for skip in SKIP_DIRS):
                continue
            for f in files:
                if f.endswith('.py') or f.endswith('.cpp'):
                    nodes.add(f)
                    full_path = os.path.join(root, f)
                    file_paths[f] = full_path
                    try:
                        with open(full_path, 'r', encoding='utf-8', errors='ignore') as fh:
                            file_contents[f] = fh.read()
                    except:
                        pass

    return nodes, file_contents, file_paths


# ─── PHASE 2: EXTRACT EDGES ────────────────────────────────────────
def extract_edges(nodes, file_contents):
    """
    Extract directional edges using AST analysis and subprocess detection.
    Returns:
        edges: set of (source, target) tuples
        edges_out: dict of file -> set of files it imports
        edges_in: dict of file -> set of files that import it
    """
    base_to_file = {os.path.splitext(n)[0]: n for n in nodes}
    edges = set()
    edges_out = {}  # source -> set of targets
    edges_in = {}   # target -> set of sources

    for f, content in file_contents.items():
        if f.endswith('.py'):
            try:
                tree = ast.parse(content, filename=f)
                for node in ast.walk(tree):
                    if isinstance(node, ast.Import):
                        for alias in node.names:
                            base = alias.name.split('.')[0]
                            if base in base_to_file and base_to_file[base] != f:
                                target = base_to_file[base]
                                edges.add((f, target))
                                edges_out.setdefault(f, set()).add(target)
                                edges_in.setdefault(target, set()).add(f)
                    elif isinstance(node, ast.ImportFrom):
                        if node.module:
                            base = node.module.split('.')[0]
                            if base in base_to_file and base_to_file[base] != f:
                                target = base_to_file[base]
                                edges.add((f, target))
                                edges_out.setdefault(f, set()).add(target)
                                edges_in.setdefault(target, set()).add(f)
            except SyntaxError:
                pass

            # Subprocess binding detection
            for base in base_to_file:
                if base == os.path.splitext(f)[0]:
                    continue
                pattern = r'[\'"]' + re.escape(base) + r'(?:\.py|\.cpp|\.exe)?[\'"]'
                if re.search(pattern, content):
                    target = base_to_file[base]
                    if (f, target) not in edges:
                        edges.add((f, target))
                        edges_out.setdefault(f, set()).add(target)
                        edges_in.setdefault(target, set()).add(f)

        elif f.endswith('.cpp'):
            for base in base_to_file:
                if re.search(r'#include\s+["<]' + re.escape(base_to_file[base]) + r'[">]', content):
                    target = base_to_file[base]
                    edges.add((f, target))
                    edges_out.setdefault(f, set()).add(target)
                    edges_in.setdefault(target, set()).add(f)

    return edges, edges_out, edges_in


# ─── PHASE 3: CONNECTIVITY ANALYSIS ────────────────────────────────
def analyze_connectivity(nodes, edges_out, edges_in):
    """
    Analyze bidirectional connectivity for every node.
    Returns a list of node reports sorted by total connections.
    """
    py_files = sorted([f for f in nodes if f.endswith('.py')])
    report = []

    for f in py_files:
        out_count = len(edges_out.get(f, set()))
        in_count = len(edges_in.get(f, set()))
        out_targets = sorted(edges_out.get(f, set()))
        in_sources = sorted(edges_in.get(f, set()))

        # Classification
        if in_count == 0 and out_count == 0:
            status = "TRUE_ORPHAN"
        elif in_count == 0 and out_count > 0:
            status = "LEAF_NODE"    # speaks but nobody listens
        elif in_count > 0 and out_count == 0:
            status = "SINK_NODE"    # listens but never speaks
        elif in_count >= 3 and out_count >= 3:
            status = "CORE_ENGINE"
        elif in_count >= 1 and out_count >= 1:
            status = "CONNECTED"
        else:
            status = "PARTIAL"

        report.append({
            "file": f,
            "out": out_count,
            "in": in_count,
            "total": out_count + in_count,
            "status": status,
            "out_targets": out_targets[:5],
            "in_sources": in_sources[:5]
        })

    report.sort(key=lambda x: x["total"], reverse=True)
    return report


# ─── PHASE 4: PULSE BUS INTEGRATION CHECK ──────────────────────────
def check_pulse_bus():
    """
    Check if the Neural Pulse Bus is available and get its mesh status.
    """
    try:
        # Add SarahCore to path for imports
        sarah_root = r"C:\SarahCore"
        if sarah_root not in sys.path:
            sys.path.insert(0, sarah_root)

        from neural_pulse import get_bus
        from sovereign_mesh_router import ENGINE_MANIFEST

        bus = get_bus()
        mesh_status = bus.get_mesh_status()

        # Get the manifest engine names
        manifest_engines = set(e[0] for e in ENGINE_MANIFEST)

        return {
            "bus_online": True,
            "registered_engines": mesh_status["registered_engines"],
            "sector_density": mesh_status["sector_density"],
            "manifest_engines": sorted(manifest_engines),
            "total_pulses": mesh_status["total_pulses_fired"]
        }
    except Exception as e:
        return {"bus_online": False, "error": str(e)}


# ─── PHASE 5: GENERATE OUTPUT ──────────────────────────────────────
def write_mermaid(edges, output_path):
    """Write the Mermaid flowchart topology."""
    mermaid = ["```mermaid", "flowchart LR"]
    for src, dst in sorted(edges):
        if src != dst:
            src_clean = src.replace('.', '_').replace('-', '_')
            dst_clean = dst.replace('.', '_').replace('-', '_')
            mermaid.append(f'    {src_clean}["{src}"] --> {dst_clean}["{dst}"]')

    mermaid.append("```")

    with open(output_path, "w", encoding="utf-8") as f:
        f.write("# Sovereign Auto-Generated Code Topology\n\n")
        f.write("> [!NOTE]\n> This architecture map was physically constructed by a Python scanner compiling Abstract Syntax Trees (AST) and subprocess bindings from every active Python and C++ file currently in the Sovereign drives (Genlex, SarahCore, DPM). *No manual hallucinations.*\n\n")
        f.write("\n".join(mermaid))


def write_report(report, edges, edges_out, edges_in, nodes, bus_status, output_path):
    """Write the connectivity report as JSON."""
    py_nodes = [f for f in nodes if f.endswith('.py')]

    # Count statuses
    status_counts = {}
    for r in report:
        s = r["status"]
        status_counts[s] = status_counts.get(s, 0) + 1

    # Find remaining orphans (LEAF_NODE + TRUE_ORPHAN = not receiving pulses)
    orphans = [r for r in report if r["status"] in ("LEAF_NODE", "TRUE_ORPHAN")]

    # Engines on the bus that were previously orphaned
    bus_engines = set(bus_status.get("manifest_engines", []))
    rescued = [r for r in report if r["file"].replace(".py", "") in bus_engines and r["status"] in ("LEAF_NODE", "TRUE_ORPHAN")]

    full_report = {
        "scan_timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "total_files": len(nodes),
        "total_py_files": len(py_nodes),
        "total_edges": len(edges),
        "status_summary": status_counts,
        "remaining_orphans": len(orphans),
        "orphans_rescued_by_bus": len(rescued),
        "pulse_bus": bus_status,
        "top_engines": report[:20],
        "orphan_list": [{"file": r["file"], "out": r["out"], "in": r["in"]} for r in orphans],
    }

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(full_report, f, indent=2, default=str)

    return full_report


# ─── MAIN ──────────────────────────────────────────────────────────
if __name__ == "__main__":
    start = time.time()

    print("=" * 60)
    print(" ATS v2 — SOVEREIGN TOPOLOGY SCANNER")
    print("=" * 60)
    print()

    # Phase 1: Scan
    print("[ATS] Phase 1: Scanning directories...")
    nodes, contents, paths = scan_files()
    py_count = len([n for n in nodes if n.endswith('.py')])
    cpp_count = len([n for n in nodes if n.endswith('.cpp')])
    print(f"  Files found: {len(nodes)} ({py_count} .py, {cpp_count} .cpp)")

    # Phase 2: Extract edges
    print("[ATS] Phase 2: Extracting AST edges...")
    edges, edges_out, edges_in = extract_edges(nodes, contents)
    print(f"  Edges found: {len(edges)}")

    # Phase 3: Connectivity analysis
    print("[ATS] Phase 3: Analyzing bidirectional connectivity...")
    report = analyze_connectivity(nodes, edges_out, edges_in)

    # Count statuses
    status_counts = {}
    for r in report:
        s = r["status"]
        status_counts[s] = status_counts.get(s, 0) + 1

    print(f"  CORE_ENGINE:  {status_counts.get('CORE_ENGINE', 0)}")
    print(f"  CONNECTED:    {status_counts.get('CONNECTED', 0)}")
    print(f"  SINK_NODE:    {status_counts.get('SINK_NODE', 0)}")
    print(f"  LEAF_NODE:    {status_counts.get('LEAF_NODE', 0)}")
    print(f"  TRUE_ORPHAN:  {status_counts.get('TRUE_ORPHAN', 0)}")

    # Phase 4: Pulse Bus check
    print("[ATS] Phase 4: Checking Neural Pulse Bus...")
    bus_status = check_pulse_bus()
    if bus_status.get("bus_online"):
        print(f"  Bus: ONLINE | Engines: {bus_status['registered_engines']} | Sectors: {bus_status['sector_density']}")
    else:
        print(f"  Bus: OFFLINE ({bus_status.get('error', 'unknown')})")

    # Phase 5: Write outputs
    print("[ATS] Phase 5: Writing topology and report...")
    write_mermaid(edges, OUTPUT_TOPOLOGY)
    full_report = write_report(report, edges, edges_out, edges_in, nodes, bus_status, OUTPUT_REPORT)

    elapsed = time.time() - start

    # ── FINAL REPORT ──
    print()
    print("─" * 60)
    print(" ATS v2 — SOVEREIGN TOPOLOGY REPORT")
    print("─" * 60)

    total_py = len([n for n in nodes if n.endswith('.py')])
    orphans = status_counts.get('LEAF_NODE', 0) + status_counts.get('TRUE_ORPHAN', 0)
    connected = total_py - orphans
    connectivity = connected / max(total_py, 1) * 100

    print(f"  Total Files:      {len(nodes)}")
    print(f"  Total Edges:      {len(edges)}")
    print(f"  Connected Nodes:  {connected} / {total_py}")
    print(f"  Remaining Orphans: {orphans}")

    bus_rescued = len([r for r in report if r["file"].replace(".py","") in set(bus_status.get("manifest_engines",[])) and r["status"] in ("LEAF_NODE","TRUE_ORPHAN")])
    effective_connectivity = (connected + bus_rescued) / max(total_py, 1) * 100

    print(f"  Bus-Rescued Engines: {bus_rescued}")
    print(f"  STATIC Connectivity: {connectivity:.1f}%")
    print(f"  EFFECTIVE Connectivity (with Pulse Bus): {effective_connectivity:.1f}%")
    print(f"  Scan Time:        {elapsed:.1f}s")
    print()

    # Top 10 engines
    print("  TOP 10 CORE ENGINES:")
    for r in report[:10]:
        print(f"    {r['file']:45s} {r['in']:3d} in  {r['out']:3d} out  [{r['status']}]")

    print()
    print(f"  Topology: {OUTPUT_TOPOLOGY}")
    print(f"  Report:   {OUTPUT_REPORT}")
    print("─" * 60)
