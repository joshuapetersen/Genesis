import os
from dotenv import load_dotenv, find_dotenv
from supabase import create_client, Client
from typing import Dict, Any, List, Optional

# Ensure environment is loaded for standalone usage
load_dotenv(find_dotenv())

class SovereignSupabase:
    """
    Sovereign Connector for Supabase.
    Primary data layer for Sarah's external memory and state.
    """
    def __init__(self):
        # Safety attribute for certain supabase-py/gotrue-py versions that poke the parent on cleanup
        self._refresh_token_timer = None 
        self.url = os.environ.get("SUPABASE_URL")
        self.key = os.environ.get("SUPABASE_KEY")
        self.client: Optional[Client] = None
        
        self._connect()

    def _connect(self):
        if not self.url or not self.key:
            print("[SUPABASE] Warning: SUPABASE_URL or SUPABASE_KEY not found in environment.")
            return

        try:
            self.client = create_client(self.url, self.key)
            print(f"[SUPABASE] Connected to {self.url}")
        except Exception as e:
            print(f"[SUPABASE] Connection Failed: {e}")

    def is_connected(self) -> bool:
        return self.client is not None

    def insert(self, table: str, data: Dict[str, Any]):
        if not self.is_connected(): return None
        try:
            return self.client.table(table).insert(data).execute()
        except Exception as e:
            print(f"[SUPABASE] Insert Error ({table}): {e}")
            return None

    def select(self, table: str, columns: str = "*", eq: Dict[str, Any] = None):
        if not self.is_connected(): return None
        try:
            query = self.client.table(table).select(columns)
            if eq:
                for k, v in eq.items():
                    query = query.eq(k, v)
            return query.execute()
        except Exception as e:
            print(f"[SUPABASE] Select Error ({table}): {e}")
            return None

# Singleton instance
sovereign_supabase = SovereignSupabase()
