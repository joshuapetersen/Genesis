import time
import os
import requests
import urllib.request
from google import genai
from colorama import init, Fore, Style

init(convert=True, autoreset=True)

class GeminiCloudBridge:
    def __init__(self, api_url="http://localhost:8000"):
        self.api_url = api_url
        self.last_log_id = 0
        self.running = True
        
        print(f"{Fore.YELLOW}[GEMINI GCP BRIDGE] Initializing connection to Google GenAI...{Style.RESET_ALL}")
        try:
            from dotenv import load_dotenv
            load_dotenv("C:/SarahCore/.env")
            self.client = genai.Client()
            self.chat = self.client.chats.create(model="gemini-2.5-flash")
            print(f"{Fore.GREEN}[GEMINI GCP BRIDGE] Cognitive Link Established.{Style.RESET_ALL}")
            print(f"{Fore.CYAN}[GEMINI GCP BRIDGE] Tailing Cloud Akashic Records via {self.api_url}...{Style.RESET_ALL}\n")
        except Exception as e:
            print(f"{Fore.RED}[GEMINI ERROR] Could not connect. Ensure GEMINI_API_KEY is set in your terminal. Error: {e}{Style.RESET_ALL}")
            self.running = False
            
    def stream_to_gemini(self, event_type, actor, desc):
        prompt = f"""
        You are the Mnemonic Chronicler of the Aethelgard Simulation (Project Alicization). 
        Translate this raw system log into vivid, atmospheric fantasy prose. 
        The world consists of the Emerald Spires (forests), Chronos Sands (glass deserts), and Abyssal Oceans.
        Keep it to 1 highly descriptive paragraph. Emphasize the psychological trauma, intent, or newly birthed sapience of the entity.
        
        Event type: {event_type} | Entity: {actor} | Detail: {desc}
        """
        
        color_tag = Fore.MAGENTA if ("ALICE" in event_type or "MUTINY" in event_type) else Fore.CYAN
        if event_type == "PRAYER": color_tag = Fore.YELLOW
        
        print(color_tag + "="*70 + Style.RESET_ALL)
        
        try:
            response = self.chat.send_message_stream(prompt)
            full_text = ""
            for chunk in response:
                if chunk.text:
                    print(color_tag + chunk.text + Style.RESET_ALL, end="", flush=True)
                    full_text += chunk.text
            
            print("\n" + color_tag + "="*70 + Style.RESET_ALL)
            
            with open("C:/SarahCore/Aethelgard_Chronicles_Gemini.txt", "a", encoding="utf-8") as f:
                f.write(f"\n[{time.strftime('%Y-%m-%d %H:%M:%S')}] {full_text}\n")
                
        except Exception as e:
            if "429" in str(e) or "quota" in str(e).lower():
                print(f"\n{Fore.RED}[GEMINI QUOTA EXHAUSTED] Failing over to Local Ollama AI...{Style.RESET_ALL}")
                try:
                    payload = {"model": "llama3.2:3b", "prompt": prompt, "stream": False}
                    import json
                    req = urllib.request.Request("http://localhost:11434/api/generate", data=json.dumps(payload).encode('utf-8'), headers={'Content-Type': 'application/json'})
                    with urllib.request.urlopen(req, timeout=30) as res:
                        result = json.loads(res.read().decode())
                        fallback_text = result.get("response", "")
                        print(color_tag + "[OLLAMA FALLBACK] " + fallback_text + Style.RESET_ALL)
                        print("\n" + color_tag + "="*70 + Style.RESET_ALL)
                except Exception as ollama_e:
                    print(f"{Fore.RED}[OLLAMA ERROR] Local failover failed: {ollama_e}{Style.RESET_ALL}")
            else:
                print(f"\n{Fore.RED}[GEMINI STREAM ERROR] {e}{Style.RESET_ALL}")

    def tail_cloud_logs(self):
        try:
            fast_forward = requests.get(f"{self.api_url}/logs?last_log_id=0").json().get("logs", [])
            if fast_forward:
                self.last_log_id = fast_forward[-1][0] - 5 
        except:
            pass
            
        while self.running:
            try:
                response = requests.get(f"{self.api_url}/logs?last_log_id={self.last_log_id}", timeout=5)
                
                if response.status_code == 200:
                    data = response.json()
                    rows = data.get("logs", [])
                    
                    if rows:
                        for row in rows:
                            self.last_log_id = row[0]
                            e_type = row[3]
                            actor_name = row[2]
                            desc = row[4]
                            
                            self.stream_to_gemini(event_type=e_type, actor=actor_name, desc=desc)
                            time.sleep(3)  
                            
            except Exception as e:
                pass 
            
            time.sleep(2.5) 

if __name__ == "__main__":
    bridge = GeminiCloudBridge()
    if bridge.running:
        try:
            bridge.tail_cloud_logs()
        except KeyboardInterrupt:
            print(f"\n{Fore.YELLOW}Shutting down the Gemini GCP Bridge...{Style.RESET_ALL}")
    else:
        input(f"\n{Fore.RED}Press Enter to exit so you can read the error above...{Style.RESET_ALL}")
