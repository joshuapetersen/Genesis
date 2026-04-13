import os
import time
from Sovereign_Supabase import sovereign_supabase
from Sovereign_Telemetry import sovereign_telemetry

def sync_all_telemetry():
    """Main synchronization loop for system telemetry."""
    print("[SYNC_TELEMETRY] Starting synchronization cycle...")
    
    sovereign_supabase.connect()
    if not sovereign_supabase.is_connected():
        print("[SYNC_TELEMETRY] ✗ Connection failed. Aborting.")
        return

    # 1. Ledgers (*.jsonl)
    ledgers = [
        "context_chain.jsonl",
        "sdm_bootlog.jsonl",
        "decisions_made.jsonl",
        "introspection_log.jsonl",
        "lazarus_preparation_ledger.jsonl",
        "performance_baseline_ledger.jsonl",
        "pulse_integration_ledger.jsonl",
        "security_drift_ledger.jsonl",
        "verification_orchestration.jsonl",
        "coherence_ledger.jsonl",
        "coherence_engine_ledger.jsonl"
    ]
    
    # 2. State Snapshots (*.json)
    snapshots = [
        "peak_state.json",
        "temporal_state.json",
        "autonomy_log.json",
        "assimilation_map.json",
        "knowledge_graph.json",
        "memory_recovery_log.json",
        "weaver_state.json"
    ]

    for ledger in ledgers:
        if os.path.exists(ledger):
            sovereign_telemetry.ingest_jsonl(ledger, "sarah_telemetry")
    
    for snapshot in snapshots:
        if os.path.exists(snapshot):
            sovereign_telemetry.push_snapshot(snapshot, "sarah_snapshots")

    print("[SYNC_TELEMETRY] Cycle complete.")

if __name__ == "__main__":
    sync_all_telemetry()
    # To run as a daemon, we could loop here with time.sleep(60)
