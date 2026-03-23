import os
import json
import time
from typing import Dict, Any, List
from Sovereign_Supabase import sovereign_supabase

class SovereignTelemetry:
    """
    Sovereign Telemetry Handler.
    Synchronizes local autonomous logs (ledgers) and snapshots with Supabase.
    """
    def __init__(self):
        self.state_file = ".telemetry_state"
        self.state = self._load_state()

    def _load_state(self) -> Dict[str, int]:
        if os.path.exists(self.state_file):
            try:
                with open(self.state_file, "r") as f:
                    return json.load(f)
            except:
                return {}
        return {}

    def _save_state(self):
        with open(self.state_file, "w") as f:
            json.dump(self.state, f)

    def ingest_jsonl(self, filepath: str, table: str):
        """Ingests new lines from a JSONL file into Supabase."""
        if not os.path.exists(filepath):
            return

        last_pos = self.state.get(filepath, 0)
        new_entries = []

        try:
            with open(filepath, "r") as f:
                f.seek(last_pos)
                for line in f:
                    if not line.strip(): continue
                    try:
                        data = json.loads(line)
                        # Add metadata for Supabase
                        entry = {
                            "source_file": os.path.basename(filepath),
                            "payload": data,
                            "collected_at": __import__("datetime").datetime.utcnow().isoformat()
                        }
                        new_entries.append(entry)
                    except json.JSONDecodeError:
                        continue
                
                new_pos = f.tell()

            if new_entries:
                print(f"[TELEMETRY] Ingesting {len(new_entries)} entries from {filepath} to {table}...")
                # Supabase insert handles lists for bulk insertion
                res = sovereign_supabase.insert(table, new_entries)
                if res:
                    self.state[filepath] = new_pos
                    self._save_state()
                    print(f"[TELEMETRY] ✓ Synced {filepath}")
                else:
                    print(f"[TELEMETRY] ✗ Failed to sync {filepath}")

        except Exception as e:
            print(f"[TELEMETRY] Error ingesting {filepath}: {e}")

    def push_snapshot(self, filepath: str, table: str):
        """Pushes a single JSON snapshot to Supabase."""
        if not os.path.exists(filepath):
            return

        try:
            with open(filepath, "r") as f:
                data = json.load(f)
            
            entry = {
                "source_file": os.path.basename(filepath),
                "payload": data,
                "collected_at": __import__("datetime").datetime.utcnow().isoformat()
            }
            
            # Use upsert for snapshots based on source_file if possible, or just insert
            res = sovereign_supabase.insert(table, entry)
            if res:
                print(f"[TELEMETRY] ✓ Pushed snapshot {filepath}")
            else:
                print(f"[TELEMETRY] ✗ Failed snapshot {filepath}")
        except Exception as e:
            print(f"[TELEMETRY] Error pushing snapshot {filepath}: {e}")

# Singleton
sovereign_telemetry = SovereignTelemetry()

if __name__ == "__main__":
    # Test ingestion if run directly
    sovereign_supabase.connect()
    if sovereign_supabase.is_connected():
        # Example: ingest_jsonl("context_chain.jsonl", "sarah_telemetry")
        pass
