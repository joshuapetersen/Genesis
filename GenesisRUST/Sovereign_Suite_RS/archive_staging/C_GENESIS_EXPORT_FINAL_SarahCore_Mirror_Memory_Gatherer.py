import os
import json
import sqlite3
from datetime import datetime
class MemoryGatherer:
    """
    Collects data from multiple sources:
    - Google Drive (Cloud)
    - Antigravity Logs (Local)
    - Gemini Code Assist Checkpoints (Local)
    - VS Code History (Local)
    - SarahCore Archives (Local)
    """
    def __init__(self, output_file="unified_memory_stream.jsonl"):
        self.output_file = output_file
        self.memory_stream = []
        self.stats = {
            "antigravity": 0,
            "gemini_assist": 0,
            "vscode": 0,
            "local_archive": 0,
            "drive": 0
        }

    def _add_event(self, source: str, content: str, timestamp: str, metadata: Dict = None):
        event = {
            "source": source,
            "timestamp": timestamp,
            "content": content,
            "metadata": metadata or {}
        }
        self.memory_stream.append(event)
        self.stats[source] = self.stats.get(source, 0) + 1

    def crawl_antigravity(self, path=r"C:\Users\drago\.gemini\antigravity\brain"):
        """Function: crawl_antigravity"""
        print(f"[Gatherer] Crawling Antigravity: {path}")
        if not os.path.exists(path):
            print(f"  [WARN] Path not found: {path}")
            return

        for root, dirs, files in os.walk(path):
            for file in files:
                if file.endswith(('.txt', '.md', '.log', '.json')):
                    file_path = os.path.join(root, file)
                    if ".system_generated" in file_path: continue # Skip noise
                    
                    try:
                        mtime = os.path.getmtime(file_path)
                        ts = datetime.fromtimestamp(mtime).isoformat()
                        
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                            if content.strip():
                                self._add_event("antigravity", content, ts, {"file": file, "path": file_path})
                    except Exception as e:
                        print(f"  [ERROR] Failed to read {file}: {e}")

    def crawl_gemini_assist(self, path=r"C:\Users\drago\AppData\Roaming\Code\User\globalStorage\google.geminicodeassist\chat_checkpoint_files"):
        """Function: crawl_gemini_assist"""
        print(f"[Gatherer] Crawling Gemini Assist: {path}")
        if not os.path.exists(path):
            print(f"  [WARN] Path not found: {path}")
            return

        for root, dirs, files in os.walk(path):
            for file in files:
                # Checkpoints often have no extension or long hex names
                file_path = os.path.join(root, file)
                try:
                    mtime = os.path.getmtime(file_path)
                    ts = datetime.fromtimestamp(mtime).isoformat()
                    
                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                        if content.strip():
                            # Try to parse as JSON if it looks like it
                            metadata = {"file": file, "path": file_path}
                            if content.strip().startswith('{') or content.strip().startswith('['):
                                try:
                                    json_data = json.loads(content)
                                    metadata["json_payload"] = True
                                except json.JSONDecodeError:
                                    pass
                            
                            self._add_event("gemini_assist", content, ts, metadata)
                except Exception as e:
                    print(f"  [ERROR] Failed to read {file}: {e}")

    def crawl_vscode_sqlite(self, path=r"C:\Users\drago\AppData\Roaming\Code\User\workspaceStorage"):
        """Function: crawl_vscode_sqlite"""
        print(f"[Gatherer] Crawling VS Code SQLite: {path}")
        if not os.path.exists(path): return

        for root, dirs, files in os.walk(path):
            for file in files:
                if file == "state.vscdb":
                    file_path = os.path.join(root, file)
                    try:
                        conn = sqlite3.connect(file_path)
                        cursor = conn.cursor()
                        # VS Code stores state in key-value pairs (key, value)
                        cursor.execute("SELECT key, value FROM ItemTable")
                        rows = cursor.fetchall()
                        mtime = os.path.getmtime(file_path)
                        ts = datetime.fromtimestamp(mtime).isoformat()
                        
                        found_data = False
                        for key, value in rows:
                            # Search for chat-related keys
                            if "chat" in key.lower() or "history" in key.lower() or "conversations" in key.lower():
                                self._add_event("vscode", f"Key: {key}\nValue: {value}", ts, {"file": file, "path": file_path, "key": key})
                                found_data = True
                        
                        conn.close()
                    except Exception as e:
                        print(f"  [ERROR] Failed to read SQLite {file_path}: {e}")

    def crawl_local_archives(self, path=r"C:\SarahCore\archive_memories"):
        """Function: crawl_local_archives"""
        print(f"[Gatherer] Crawling Local Archives: {path}")
        if not os.path.exists(path): return

        for root, dirs, files in os.walk(path):
            for file in files:
                if file.endswith(('.txt', '.md', '.json')):
                    file_path = os.path.join(root, file)
                    try:
                        mtime = os.path.getmtime(file_path)
                        ts = datetime.fromtimestamp(mtime).isoformat()
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                            if content.strip():
                                self._add_event("local_archive", content, ts, {"file": file, "path": file_path})
                    except Exception as e:
                        print(f"  [ERROR] Failed to read {file}: {e}")

    def save_stream(self):
        """Function: save_stream"""
        print(f"[Gatherer] Saving {len(self.memory_stream)} events to {self.output_file}...")
        # Sort by timestamp
        self.memory_stream.sort(key=lambda x: x['timestamp'])
        
        with open(self.output_file, 'w', encoding='utf-8') as f:
            for event in self.memory_stream:
                f.write(json.dumps(event) + '\n')
        
        print(f"[Gatherer] Stats: {json.dumps(self.stats, indent=2)}")

if __name__ == "__main__":
    gatherer = MemoryGatherer()
    gatherer.crawl_antigravity()
    gatherer.crawl_gemini_assist()
    gatherer.crawl_vscode_sqlite()
    gatherer.crawl_local_archives()
    gatherer.save_stream()
