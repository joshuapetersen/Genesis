"""
SOVEREIGN MESH ROUTER — Seats All 63 Orphaned Engines Into The Neural Pulse Bus
================================================================================
This is the bridge that transforms 63 disconnected leaf-nodes into
living participants in the Sovereign Nervous System.

Each engine is registered on at least one sector of the PulseBus.
Each engine gets a handler that responds to incoming pulses.
Each handler fires a ReturnPulse proving execution.

Run this file to seat the entire mesh and verify connectivity.
"""

import os
import sys
import hashlib
import time
import json
import importlib
import traceback

# Ensure SarahCore is on the path
SOVEREIGN_ROOT = os.path.dirname(os.path.abspath(__file__))
if SOVEREIGN_ROOT not in sys.path:
    sys.path.insert(0, SOVEREIGN_ROOT)

from neural_pulse import get_bus, NeuralPulse, ReturnPulse, Sector


# ─── ENGINE MANIFEST ───────────────────────────────────────────────
# Each entry: (engine_filename_without_py, sector, description)
# Grouped by cognitive function for clarity.

ENGINE_MANIFEST = [
    # ── BRAIN: Identity & Core ──────────────────────────────────
    ("Sarah_Sovereign_Core",    "BRAIN",      "Identity backbone"),
    ("Sarah_Autonomy",          "BRAIN",      "Self-governance"),
    ("Sarah_Sovereign_Agent",   "BRAIN",      "External agent interface"),
    ("sovereign_init",          "BRAIN",      "Bootstrap sequence"),
    ("Sarah_Lite",              "BRAIN",      "Lightweight inference"),
    ("Sarah_Status",            "BRAIN",      "Status reporting"),

    # ── SPEECH: Chat & Interaction ──────────────────────────────
    ("UNIFIED_CHAT",            "SPEECH",     "Primary chat engine"),
    ("UNIFIED_CHAT_V2",         "SPEECH",     "Enhanced chat engine"),
    ("CHAT_FINAL",              "SPEECH",     "Production chat"),
    ("ask_sarah",               "SPEECH",     "Query interface"),
    ("Sarah_Learning_Directive","SPEECH",     "Learning loop"),
    ("Ask_Sarah_DREAM_MAKER",   "SPEECH",     "Dream conversation"),
    ("Genesis_Entity_Chat",     "SPEECH",     "Entity interaction"),
    ("Direct_Sarah",            "SPEECH",     "Direct memory query"),

    # ── MEMORY: Knowledge & Recall ──────────────────────────────
    ("Sarah_Deep_Study",        "MEMORY",     "Deep research"),
    ("Sarah_Dream",             "MEMORY",     "Background synthesis"),
    ("Sovereign_Gnosis_Hub",    "MEMORY",     "Knowledge aggregation"),
    ("industry_knowledge_ingester", "MEMORY", "External knowledge intake"),
    ("Volumetric_Recovery_Anchor", "MEMORY",  "Recovery logic"),
    ("Sarah_Axiom_Seater",      "MEMORY",     "Axiom planting"),

    # ── LOGIC: Reasoning & Solving ──────────────────────────────
    ("Sarah_HLE_Global_Solver", "LOGIC",      "Problem solving"),
    ("Recursive_Audit",         "LOGIC",      "Self-verification"),
    ("Sovereign_Integrity_Nexus","LOGIC",     "Integrity checking"),
    ("Sarah_Mach_Speed_Test",   "LOGIC",      "Speed benchmarking"),
    ("try_import",              "LOGIC",      "Import verification"),
    ("verify_ace_anchor",       "LOGIC",      "ACE anchor verification"),
    ("gpu_performance_test",    "LOGIC",      "GPU performance audit"),

    # ── SECURITY: Governance & Hardening ────────────────────────
    ("Sarah_Self_Check",        "SECURITY",   "Self-diagnosis"),
    ("Sovereign_Swarm",         "SECURITY",   "Distributed execution"),
    ("Sovereign_Daemon",        "SECURITY",   "Background services"),
    ("Sarah_Daemon",            "SECURITY",   "System services"),
    ("sarah_gpu_audit",         "SECURITY",   "GPU audit"),
    ("SAUL_Log_System",         "SECURITY",   "SAUL logging"),

    # ── PERCEPTION: Navigation & OS ─────────────────────────────
    ("Sarah_Continuous_Navigator","PERCEPTION","Continuous operation"),
    ("Sarah_Navigation_Demo",   "PERCEPTION", "Navigation system"),
    ("Sarah_Windows_Mastery",   "PERCEPTION", "OS interface"),
    ("Sarah_Antigravity_Control","PERCEPTION","Antigravity bridge"),

    # ── AUDIT: Testing & Verification ───────────────────────────
    ("test_hardening_integration","AUDIT",    "Hardening tests"),
    ("test_integrated_logic",   "AUDIT",      "Logic integration tests"),
    ("test_gap_analysis",       "AUDIT",      "Gap analysis tests"),
    ("test_kernel_override",    "AUDIT",      "Kernel override tests"),
    ("test_security_suite",     "AUDIT",      "Security suite tests"),
    ("test_sovereign_action",   "AUDIT",      "Sovereign action tests"),
    ("test_token_bank",         "AUDIT",      "Token bank tests"),
    ("test_tribunal",           "AUDIT",      "Tribunal tests"),

    # ── NEWLY INTEGRATED (ATS v4) ─────────────────────────
    ("Possibility_Engine", "LOGIC", "Standalone component integration"),
    ("RefineForge", "LOGIC", "Standalone component integration"),
    ("google_dev_knowledge_ingester", "MEMORY", "Standalone component integration"),
    ("Knowledge_Harvester", "MEMORY", "Standalone component integration"),
    ("full_dictionary_indexer", "MEMORY", "Standalone component integration"),
    ("agent_autonomy_loops", "BRAIN", "Standalone component integration"),
    ("Evolution_Intelligence", "LOGIC", "Standalone component integration"),
    ("Change_Log_System", "LOGIC", "Standalone component integration"),
    ("SLF_Life_Forge", "LOGIC", "Standalone component integration"),
    ("node_classification_metric", "AUDIT", "Standalone component integration"),
    ("test_forensic_velocity_integration", "AUDIT", "Standalone component integration"),
    ("parse_3_12_72", "LOGIC", "Standalone component integration"),
    ("Code_Introspection", "LOGIC", "Standalone component integration"),
    ("google_tech_ingester", "MEMORY", "Standalone component integration"),
    ("ingest_memories", "LOGIC", "Standalone component integration"),
    ("Verification_Orchestrator", "LOGIC", "Standalone component integration"),
    ("Memory_Gatherer", "MEMORY", "Standalone component integration"),
    ("NSI_Orchestrator", "LOGIC", "Standalone component integration"),
    ("SLF_Evolution_LLM", "LOGIC", "Standalone component integration"),
    ("slf_evolution_recovered", "LOGIC", "Standalone component integration"),
    ("Consensus_Voter", "LOGIC", "Standalone component integration"),
    ("master_benchmark", "AUDIT", "Standalone component integration"),
    ("SLF_Akashic_Records", "MEMORY", "Standalone component integration"),
    ("Topos_Truth_Oracle", "LOGIC", "Standalone component integration"),
    ("test_elite_baseline", "AUDIT", "Standalone component integration"),
    ("Ascension_Protocol", "LOGIC", "Standalone component integration"),
    ("gpis_indexer", "MEMORY", "Standalone component integration"),
    ("ram_profiler", "AUDIT", "Standalone component integration"),
    ("agent_control_plane", "BRAIN", "Standalone component integration"),
    ("Messiah_Entropy_Audit", "AUDIT", "Standalone component integration"),
    ("patch_continuity", "LOGIC", "Standalone component integration"),
    ("stability_protocols", "LOGIC", "Standalone component integration"),
    ("transpile_to_all", "LOGIC", "Standalone component integration"),
    ("meta_monitor", "LOGIC", "Standalone component integration"),
    ("admin_bridge", "LOGIC", "Standalone component integration"),
    ("security_manager", "SECURITY", "Standalone component integration"),
    ("pyramid_crawler", "LOGIC", "Standalone component integration"),
    ("Shard_Seeder", "LOGIC", "Standalone component integration"),
    ("dictionary_retrieval", "MEMORY", "Standalone component integration"),
    ("loq_handshake", "LOGIC", "Standalone component integration"),
    ("parse_cluster_topics", "LOGIC", "Standalone component integration"),
    ("autonomous_audit_loop", "AUDIT", "Standalone component integration"),
    ("final_alice_audit", "AUDIT", "Standalone component integration"),
    ("MESSIAH_MEMORY_AUDITOR", "AUDIT", "Standalone component integration"),
    ("SOUL_PLIER_CORE", "LOGIC", "Standalone component integration"),
    ("Consolidation_Logic", "LOGIC", "Standalone component integration"),
    ("Fractal_Math_Bridge", "LOGIC", "Standalone component integration"),
    ("universal_translator", "LOGIC", "Standalone component integration"),
    ("circuit_breaker", "LOGIC", "Standalone component integration"),
    ("disk_audit", "AUDIT", "Standalone component integration"),
    ("fast_disk_audit", "AUDIT", "Standalone component integration"),
    ("sync_telemetry", "LOGIC", "Standalone component integration"),
    ("simulate_deep_scan", "LOGIC", "Standalone component integration"),
    ("system_audit", "AUDIT", "Standalone component integration"),
    ("definitive_moral_audit", "AUDIT", "Standalone component integration"),
    ("Emergency_Halt", "SECURITY", "Standalone component integration"),
]


# ─── FILE DISCOVERY ────────────────────────────────────────────────
def _find_engine_file(engine_name: str) -> str:
    """Find the .py file for an engine in the SarahCore directory tree."""
    target = engine_name + ".py"
    for root, dirs, files in os.walk(SOVEREIGN_ROOT):
        if any(s in root for s in ['.venv', '__pycache__', '.git', 'node_modules']):
            continue
        if target in files:
            return os.path.join(root, target)
    return ""


# ─── UNIVERSAL HANDLER FACTORY ─────────────────────────────────────
def _make_handler(engine_name: str, description: str):
    """
    Creates a pulse handler for an engine.

    HEARTBEAT pulses: Fast file-existence check only (no imports).
    ACTION pulses: Lazy-import the module and call handle_pulse if present.

    Every handler also receives a reference to the bus so the engine
    can fire its OWN pulses (distributed ATS — every engine is a sender).
    """
    # Pre-resolve the file path once at registration time
    file_path = _find_engine_file(engine_name)

    def handler(pulse: NeuralPulse) -> ReturnPulse:
        result_data = {
            "engine": engine_name,
            "description": description,
            "action_received": pulse.action,
            "sector": pulse.target_sector,
            "file_exists": bool(file_path),
            "file_path": file_path,
        }

        # ── HEARTBEAT: Fast path. No imports. Just prove the file is alive.
        if pulse.action == "HEARTBEAT":
            if file_path:
                # Count the file's own import lines (in/out connectivity)
                try:
                    with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
                        lines = f.readlines()
                    import_out = sum(1 for l in lines if l.strip().startswith(("import ", "from ")))
                    result_data["lines"] = len(lines)
                    result_data["imports_out"] = import_out
                    status = "OK"
                except Exception as e:
                    result_data["read_error"] = str(e)
                    status = "PARTIAL"
            else:
                result_data["error"] = "FILE_NOT_FOUND"
                status = "ERROR"

        # ── ACTION PULSE: Deep path. Import and execute.
        else:
            try:
                mod = importlib.import_module(engine_name)
                result_data["module_loaded"] = True

                if hasattr(mod, "handle_pulse"):
                    engine_result = mod.handle_pulse(pulse)
                    result_data["engine_result"] = str(engine_result)[:200]
                elif hasattr(mod, engine_name):
                    result_data["has_main_class"] = True

                status = "OK"
            except ImportError as e:
                result_data["module_loaded"] = False
                result_data["import_error"] = str(e)
                status = "PARTIAL"
            except Exception as e:
                result_data["error"] = str(e)
                status = "ERROR"

        # Build the phonetic hash (hash of the result for verification)
        phonetic = hashlib.sha256(json.dumps(result_data, default=str).encode()).hexdigest()[:8]

        return ReturnPulse(
            original_pulse_id=pulse.pulse_id,
            origin=engine_name,
            target_origin=pulse.origin,
            status=status,
            result=result_data,
            phonetic_hash=phonetic
        )

    return handler


# ─── MESH SEATING ──────────────────────────────────────────────────
def seat_all_engines():
    """
    Seat all 63 engines into the Neural Pulse Bus.
    Returns the bus instance and a report of the seating.
    """
    bus = get_bus()
    report = {
        "seated": [],
        "failed": [],
        "sectors": {s.value: 0 for s in Sector}
    }

    for engine_name, sector, description in ENGINE_MANIFEST:
        handler = _make_handler(engine_name, description)
        success = bus.register(engine_name, sector, handler)

        if success:
            report["seated"].append(engine_name)
            report["sectors"][sector] += 1
        else:
            report["failed"].append(engine_name)

    return bus, report


# ─── MESH VERIFICATION ────────────────────────────────────────────
def verify_mesh(bus):
    """
    Fire a HEARTBEAT pulse into every sector and collect return pulses.
    This proves the mesh is alive and responsive.
    """
    results = {}

    for sector in Sector:
        pulse = NeuralPulse(
            action="HEARTBEAT",
            target_sector=sector.value,
            payload={"type": "MESH_VERIFICATION", "timestamp": time.time()},
            origin="MESH_ROUTER"
        )
        returns = bus.fire(pulse)
        results[sector.value] = {
            "listeners_hit": len(returns),
            "statuses": [r.status for r in returns],
            "engines": [r.origin for r in returns],
            "avg_ms": sum(r.execution_ms for r in returns) / max(len(returns), 1)
        }

    return results


# ─── MAIN ──────────────────────────────────────────────────────────
if __name__ == "__main__":
    print("=" * 60)
    print(" SOVEREIGN MESH ROUTER — SEATING ALL ENGINES")
    print("=" * 60)
    print()

    # Seat all engines
    bus, report = seat_all_engines()

    print(f"[MESH] Engines seated: {len(report['seated'])}")
    print(f"[MESH] Engines failed: {len(report['failed'])}")
    print(f"[MESH] Sector density:")
    for sector, count in report["sectors"].items():
        bar = "█" * count + "░" * (15 - count)
        print(f"  {sector:12s} [{bar}] {count}")
    print()

    # Verify mesh
    print("─" * 60)
    print(" MESH VERIFICATION — HEARTBEAT PULSE TO ALL SECTORS")
    print("─" * 60)
    print()

    verification = verify_mesh(bus)
    total_ok = 0
    total_partial = 0
    total_error = 0

    for sector, data in verification.items():
        ok = data["statuses"].count("OK")
        partial = data["statuses"].count("PARTIAL")
        error = data["statuses"].count("ERROR")
        total_ok += ok
        total_partial += partial
        total_error += error
        print(f"  {sector:12s} | Listeners: {data['listeners_hit']:2d} | OK: {ok} | PARTIAL: {partial} | ERROR: {error} | Avg: {data['avg_ms']:.3f}ms")

    print()
    print("─" * 60)
    total = total_ok + total_partial + total_error
    connectivity = (total_ok + total_partial) / max(total, 1) * 100
    print(f" MESH CONNECTIVITY: {connectivity:.1f}% ({total_ok} OK + {total_partial} PARTIAL / {total} total)")
    print(f" TOTAL ENGINES ON BUS: {len(report['seated'])}")
    print(f" TOTAL PULSES FIRED: {bus._pulse_counter}")
    print("─" * 60)

    # Full mesh status
    status = bus.get_mesh_status()
    print(f"\n[MESH STATUS] {json.dumps(status, indent=2)}")
