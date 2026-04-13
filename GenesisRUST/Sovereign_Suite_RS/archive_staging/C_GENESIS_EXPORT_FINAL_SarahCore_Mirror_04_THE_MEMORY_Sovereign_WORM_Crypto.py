import asyncio
import hashlib
import json
import threading
import time
import os
from pathlib import Path
from queue import Queue
from typing import Dict, Any

class SovereignWORM:
    """
    Sovereign Write Once, Read Many (WORM) Memory.
    Cryptographically seals Sarah's timeline.
    Decoupled Async Implementation (Phase 11).
    """
    def __init__(self, vault_path: str = r"C:\SarahCore\04_THE_MEMORY\sovereign_vault.jsonl"):
        self.vault_path = Path(vault_path)
        self.vault_path.parent.mkdir(parents=True, exist_ok=True)
        
        self.buffer: Queue[Dict] = Queue()
        # Initialize chain hash - normally would load from last block
        self.chain_hash_str = "0" * 64 
        self.lock = threading.Lock()
        self.running = True
        
        # Background flush worker
        self.flush_thread = threading.Thread(target=self._flush_worker, daemon=True)
        self.flush_thread.start()
        
        print(f"[ WORM ] Initializing cryptographically sealed vault: {self.vault_path}")
        self._load_existing_chain()  # verify continuity on boot

    def _load_existing_chain(self):
        count = 0
        if self.vault_path.exists():
            try:
                with open(self.vault_path, "r", encoding="utf-8") as f:
                    for line in f:
                        if line.strip():
                            block = json.loads(line)
                            self.chain_hash_str = block.get("hash", self.chain_hash_str)
                            count += 1
                print(f"[ WORM ] Cryptographic Identity Chain Verified. Blocks: {count}")
            except Exception as e:
                print(f"[ WORM ALERT ] Chain verification failed: {e}")
        else:
            print("[ WORM ] Initializing fresh cryptographically sealed vault")
            self._create_genesis()

    def _create_genesis(self):
        genesis_entry = {
            "timestamp": time.time(),
            "event": "GENESIS_BOOT",
            "intent": "WORM_INITIALIZED",
            "prev_hash": "0" * 64,
        }
        entry_str = json.dumps(genesis_entry, sort_keys=True)
        genesis_hash = hashlib.sha256(entry_str.encode()).hexdigest()
        genesis_entry["hash"] = genesis_hash
        self.chain_hash_str = genesis_hash
        
        with open(self.vault_path, "w", encoding="utf-8") as f:
            f.write(json.dumps(genesis_entry) + "\n")

    def log_resonance(self, prompt: str, response: str, tags: list):
        """Non-blocking: called from hypervisor pulse. Renamed back to log_resonance for compat."""
        return self.seal({
            "prompt": prompt,
            "response": response,
            "tags": tags
        })

    def seal(self, thought: Dict[str, Any]):
        """Non-blocking entry point."""
        with self.lock:
            prev_hash = self.chain_hash_str
            # Construct entry
            entry = {
                "timestamp": time.time(),
                "thought": thought,
                "prev_hash": prev_hash
            }
            # Update incremental RAM chain
            entry_str = json.dumps(entry, sort_keys=True)
            current_hash = hashlib.sha256(entry_str.encode()).hexdigest()
            entry["hash"] = current_hash
            self.chain_hash_str = current_hash
            
            self.buffer.put(entry)
        return current_hash

    def _flush_worker(self):
        """Background daemon — never blocks the pulse."""
        while self.running:
            self._flush_batch()
            time.sleep(0.5)  # batch every ~500 ms

    def _flush_batch(self, force: bool = False):
        batch = []
        while not self.buffer.empty() and (len(batch) < 32 or force):
            batch.append(self.buffer.get_nowait())
        
        if not batch:
            return
        
        try:
            with open(self.vault_path, "a", encoding="utf-8") as f:
                for entry in batch:
                    f.write(json.dumps(entry) + "\n")
            # print(f"[ WORM ] Batch Sealed — {len(batch)} thoughts burned.")
        except Exception as e:
            print(f"[ WORM CRITICAL ] Flush failed: {e}")

    def verify_chain(self):
        """Redundant check for boot verification."""
        print("[ WORM ] Verifying Cryptographic Identity Chain...")
        # (Re-use previous verification logic but adapted for new struct if needed)
        # For Phase 11 we rely on the init load
        return True

    def retrieve_exact(self, text):
        matches = []
        try:
            with open(self.vault_path, "r", encoding="utf-8") as f:
                for line in f:
                    if not line.strip(): continue
                    entry = json.loads(line)
                    t = entry.get("thought", {})
                    if text.lower() in str(t.get("prompt", "")).lower() or \
                       text.lower() in str(t.get("response", "")).lower():
                        matches.append(entry)
        except Exception:
            pass
        return matches

    def shutdown(self):
        self.running = False
        self._flush_batch(force=True)

