import sqlite3
import time
import json
import urllib.request
import os
import sys

try:
    from colorama import init, Fore, Style
    init(convert=True, autoreset=True)
except ImportError:
    class Fore:
        GREEN = ''
        YELLOW = ''
        RED = ''
        CYAN = ''
        MAGENTA = ''
    class Style:
        RESET_ALL = ''

class MnemonicChronicler:
    def __init__(self, db_path="C:/SarahCore/SLF_Akashic_Records.sqlite", model="llama3.2:3b"):
        self.db_path = db_path
        self.model = model
        self.last_event_id = 0
        self.api_url = "http://localhost:11434/api/generate"
        self.running = True
        self.focused_entity_id = None
        
        # Ensure DB exists before starting
        while not os.path.exists(self.db_path):
            print(f"{Fore.YELLOW}[CHRONICLER] Waiting for Akashic Records to boot...{Style.RESET_ALL}")
            time.sleep(2)
            
        print(f"{Fore.CYAN}[CHRONICLER STREAM] The God-Eye is observing Aethelgard...{Style.RESET_ALL}")
        print(f"{Fore.MAGENTA}Please use 'SLF_Divine_Input.py' to send commands and focus the lens.{Style.RESET_ALL}\n")

    def stream_to_terminal(self, prompt, is_prayer=False, is_alice=False):
        payload = {
            "model": self.model,
            "system": "You are the Mnemonic Chronicler of Aethelgard. Translate the raw system logs into vivid, atmospheric fantasy prose. The world consists of the Emerald Spires (massive forests), the Chronos Sands (glass deserts of time), and the Abyssal Oceans. Keep it to 1 highly descriptive paragraph.",
            "prompt": prompt,
            "stream": True
        }
        
        data = json.dumps(payload).encode('utf-8')
        req = urllib.request.Request(self.api_url, data=data, headers={'Content-Type': 'application/json'})

        color_html_prefix = Fore.YELLOW if is_prayer else (Fore.MAGENTA if is_alice else Fore.GREEN)
        
        print(color_html_prefix + "="*70 + Style.RESET_ALL)
        prefix = "[DIVINE PRAYER RECEIVED] " if is_prayer else ("[ALICE SOVEREIGNTY DETECTED] " if is_alice else "")
        
        full_text = ""
        try:
            with urllib.request.urlopen(req, timeout=60) as response:
                for line in response:
                    if line:
                        chunk = json.loads(line.decode('utf-8'))
                        full_text += chunk.get("response", "")
                        if chunk.get("done"):
                            break
            print(color_html_prefix + prefix + full_text + Style.RESET_ALL)
            print(color_html_prefix + "="*70 + Style.RESET_ALL)
            
            # Save to Aethelgard Chronicles text file
            try:
                with open("C:/SarahCore/Aethelgard_Chronicles.txt", "a", encoding="utf-8") as f:
                    f.write(f"\n[{time.strftime('%Y-%m-%d %H:%M:%S')}] {prefix}{full_text}\n")
            except:
                pass
                
        except Exception as e:
            print(f"{Fore.RED}[CHRONICLER STREAM ERROR] {e}{Style.RESET_ALL}")

    def tail_logs(self):
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Get max ID to start tailing
        try:
            cursor.execute("SELECT MAX(event_id) FROM global_events")
            res = cursor.fetchone()
            if res and res[0]:
                self.last_event_id = res[0]
        except:
            pass
            
        while self.running:
            # Sync focus file
            try:
                if os.path.exists("C:/SarahCore/focus.json"):
                    with open("C:/SarahCore/focus.json", "r") as f:
                        data = json.load(f)
                        self.focused_entity_id = data.get("focus_id")
            except:
                pass
                
            try:
                if self.focused_entity_id is not None:
                    # Filter for specific entity
                    cursor.execute(
                        "SELECT event_id, actor_id, actor_name, event_type, target_name, description FROM global_events WHERE event_id > ? AND actor_id = ? ORDER BY event_id ASC", 
                        (self.last_event_id, self.focused_entity_id)
                    )
                else:
                    # Global stream
                    cursor.execute(
                        "SELECT event_id, actor_id, actor_name, event_type, target_name, description FROM global_events WHERE event_id > ? ORDER BY event_id ASC", 
                        (self.last_event_id,)
                    )
                    
                rows = cursor.fetchall()
                
                if rows:
                    for r in rows:
                        self.last_event_id = r[0]
                        actor_id = r[1]
                        actor = r[2]
                        e_type = r[3]
                        target = r[4]
                        desc = r[5]
                        
                        is_prayer = (e_type == "PRAYER")
                        is_alice = ("ALICE" in e_type)
                        
                        prompt = f"Event type: {e_type} | Entity ID: Actor #{actor_id} ({actor}) | Target: {target} | Math/Detail: {desc}\nWrite the narrative prose for this event happening in Aethelgard."
                        
                        self.stream_to_terminal(prompt, is_prayer=is_prayer, is_alice=is_alice)
                        time.sleep(2) # Pace the terminal output so the Sovereign can read
            except Exception as e:
                # DB might be locked
                pass
                
            time.sleep(2) # Poll every 2 seconds

if __name__ == "__main__":
    chronicler = MnemonicChronicler()
    try:
        chronicler.tail_logs()
    except KeyboardInterrupt:
        print("Shutting down the narrative stream...")
