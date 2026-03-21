import sqlite3
import time
import queue
import threading
from datetime import datetime

class SLFAkashicRecords:
    """
    Shangri-La Frontier - The Akashic Records
    An asynchronous, high-throughput SQLite logger that permanently records
    every birth, death, spell cast, and mutation in the VRAM Matrix.
    """
    def __init__(self, db_path="SLF_Akashic_Records.sqlite"):
        self.db_path = db_path
        self.log_queue = queue.Queue()
        self.running = False
        self._worker_thread = None
        self._initialize_schema()
        
    def _initialize_schema(self):
        # Create schema on the main thread to ensure it exists before workers start
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # High performance PRAGMAs for massive insert throughput
        cursor.execute("PRAGMA journal_mode=WAL;")
        cursor.execute("PRAGMA synchronous=NORMAL;")
        
        cursor.executescript("""
            CREATE TABLE IF NOT EXISTS global_events (
                event_id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT,
                actor_id INTEGER,
                actor_name TEXT,
                event_type TEXT,
                target_id INTEGER,
                target_name TEXT,
                description TEXT
            );
            
            CREATE INDEX IF NOT EXISTS idx_timestamp ON global_events(timestamp);
            CREATE INDEX IF NOT EXISTS idx_actor ON global_events(actor_id);
            CREATE INDEX IF NOT EXISTS idx_event_type ON global_events(event_type);
        """)
        conn.commit()
        conn.close()

    def start(self):
        """Starts the async database writing thread."""
        if not self.running:
            self.running = True
            self._worker_thread = threading.Thread(target=self._db_writer_loop, daemon=True)
            self._worker_thread.start()
            print("[AKASHIC RECORDS] Global Event Logger Online.")

    def log_event(self, actor_id, actor_name, event_type, target_id, target_name, description):
        """
        Thread-safe method called by the Hypervisor.
        Throws the event tuple into memory instantly, so the GPU doesn't hitch.
        """
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S.%f")[:-3]
        event_tuple = (timestamp, actor_id, actor_name, event_type, target_id, target_name, description)
        self.log_queue.put(event_tuple)

    def _db_writer_loop(self):
        """
        Runs on its own thread. Extracts logs from memory and writes to SQLite in batches.
        """
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # We hold connection state in the sub-thread
        batch = []
        batch_size = 100
        
        while self.running or not self.log_queue.empty():
            try:
                # Block for up to 1 second waiting for an event
                event = self.log_queue.get(timeout=1.0)
                batch.append(event)
                self.log_queue.task_done()
            except queue.Empty:
                pass # Timeout triggered, try batch commit if needed
                
            if len(batch) >= batch_size or (len(batch) > 0 and self.log_queue.empty()):
                try:
                    cursor.executemany(
                        "INSERT INTO global_events (timestamp, actor_id, actor_name, event_type, target_id, target_name, description) VALUES (?, ?, ?, ?, ?, ?, ?)",
                        batch
                    )
                    conn.commit()
                    batch = []
                except sqlite3.Error as e:
                    print(f"[AKASHIC RECORDS ERROR] Failed to write batch: {e}")
                    # In a production environment, we'd log this locally or requeue
                    batch = [] 
                    
        conn.close()

    def stop(self):
        """Graceful shutdown. Prevents dataloss of remaining queue."""
        print("[AKASHIC RECORDS] Flushing remaining memories to disk...")
        self.running = False
        if self._worker_thread:
            self._worker_thread.join()
        print("[AKASHIC RECORDS] Closed offline.")

if __name__ == "__main__":
    # Quick Test
    akashic = SLFAkashicRecords()
    akashic.start()
    akashic.log_event(235, "The Arcane Terror-Carapace", "MUTATION", -1, "System", "Entity organically evolved.")
    akashic.log_event(405, "Predator_DireWolf_405", "COMBAT_KILL", 892, "Prey_SilverStag_892", "Slaughtered prey for 120 damage.")
    time.sleep(1) # Let the queue flush
    akashic.stop()
