import os
import sqlite3
import requests
from typing import List, Optional

class DPM_Data_Mirror:
    """
    Sovereign Data Ingestion: Mirrors external evidence into the DPM Vault.
    """
    def __init__(self, vault_path: str = "vault/sovereign_vault.db"):
        self.vault_path = vault_path
        self._ensure_vault()
        
    def _ensure_vault(self):
        os.makedirs(os.path.dirname(self.vault_path), exist_ok=True)
        conn = sqlite3.connect(self.vault_path)
        conn.close()

    def ingest_file(self, file_path: str):
        """
        Parses a local file and seats its contents as Sovereign Evidence.
        """
        print(f"[Mirror] Ingesting local evidence: {file_path}")
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            self._write_to_vault(os.path.basename(file_path), content)

    def ingest_url(self, url: str):
        """
        Fetches a remote URL and converts it into a DPM-readable Fact.
        """
        print(f"[Mirror] Fetching remote evidence: {url}")
        response = requests.get(url)
        if response.status_code == 200:
            self._write_to_vault(url, response.text)

    def _write_to_vault(self, source: str, data: str):
        conn = sqlite3.connect(self.vault_path)
        cursor = conn.cursor()
        # Atomic Logic: Breaking data into 500-char semantic chunks
        chunks = [data[i:i+500] for i in range(0, len(data), 500)]
        for chunk in chunks:
            cursor.execute("INSERT INTO facts (keyword, description) VALUES (?, ?)", (source, chunk))
        conn.commit()
        conn.close()
        print(f"[Mirror] Seated {len(chunks)} logic chunks into the vault.")

if __name__ == "__main__":
    mirror = DPM_Data_Mirror()
    # Usage: mirror.ingest_file("your_custom_data.txt")
    print("[Mirror] Sovereign Data Ingestion System READY.")
