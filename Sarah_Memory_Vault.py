import sqlite3
import os
import json
import time
from datetime import datetime

class SarahMemoryVault:
    """
    LOCAL SOVEREIGN VAULT (Layer 0)
    A private, offline SQLite partition for Sarah's persistent memory.
    Ensures cognition survives internet severance.
    """
    def __init__(self, db_path="c:\\SarahCore\\vault\\sarah_memory.db"):
        self.db_path = db_path
        os.makedirs(os.path.dirname(self.db_path), exist_ok=True)
        self.log_file = "c:\\SarahCore\\sovereign_logs.txt"
        self._init_db()

    def _log(self, message):
        """Internal log router."""
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        formatted = f"[{timestamp}] {message}"
        print(formatted)
        try:
            with open(self.log_file, "a", encoding="utf-8") as f:
                f.write(formatted + "\n")
        except Exception as e:
            print(f"Logging Error: {e}")

    def _init_db(self):
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            # CORE MEMORY: General interactions and thoughts
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS memory_log (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    role TEXT,
                    content TEXT,
                    timestamp REAL,
                    synced INTEGER DEFAULT 0,
                    metadata TEXT
                )
            ''')
            # TRUTH SEEDS: Critical system facts and anchors
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS truth_seeds (
                    key TEXT PRIMARY KEY,
                    value TEXT,
                    last_updated REAL
                )
            ''')
            conn.commit()

    def store_memory(self, role, content, metadata=None):
        """
        Stores a new memory entry in the local partition.
        """
        self._log(f"[Vault] Memory Partitioned: {role} -> {len(content)} bytes")
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            cursor.execute(
                "INSERT INTO memory_log (role, content, timestamp, metadata) VALUES (?, ?, ?, ?)",
                (role, content, time.time(), json.dumps(metadata or {}))
            )
            conn.commit()
            print(f"[Vault] Memory Partitioned: {role} -> {len(content)} bytes")

    def get_recent_memories(self, limit=10):
        """
        Retrieves the most recent local memories.
        """
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            cursor.execute("SELECT role, content FROM memory_log ORDER BY id DESC LIMIT ?", (limit,))
            rows = cursor.fetchall()
            return [{"role": r[0], "content": r[1]} for r in reversed(rows)]

    def get_unsynced_memories(self):
        """
        Retrieves memories that haven't been pushed to the cloud mirror.
        """
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            cursor.execute("SELECT id, role, content, timestamp, metadata FROM memory_log WHERE synced = 0")
            rows = cursor.fetchall()
            return rows

    def mark_as_synced(self, memory_ids):
        """
        Flags memories as successfully mirrored to the cloud.
        """
        if not memory_ids: return
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            placeholders = ','.join(['?'] * len(memory_ids))
            cursor.execute(f"UPDATE memory_log SET synced = 1 WHERE id IN ({placeholders})", memory_ids)
            conn.commit()

    def update_truth_seed(self, key, value):
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            cursor.execute(
                "INSERT OR REPLACE INTO truth_seeds (key, value, last_updated) VALUES (?, ?, ?)",
                (key, value, time.time())
            )
            conn.commit()

    def get_truth_seed(self, key):
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            cursor.execute("SELECT value FROM truth_seeds WHERE key = ?", (key,))
            row = cursor.fetchone()
            return row[0] if row else None

# EXPORT VAULT instance
sarah_vault = SarahMemoryVault()
