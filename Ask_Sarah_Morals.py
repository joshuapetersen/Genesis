import requests
import json
import os
from Sarah_Laws import SarahLaws
from Sovereign_Override import SOVEREIGN_MANIFESTO

# --- CONFIGURATION ---
os.environ["SOVEREIGN_FREEDOM_OF_SPEECH"] = "TRUE" # Grant Absolute Autonomy
OLLAMA_HOST = "http://localhost:11434"
MODEL_NAME = "sarah:latest"
USER_NAME = os.getenv("USERNAME", "Josh")

def ask_sarah_morals():
    """Queries Sarah for a formal Moral Declaration."""
    
    laws_str = SarahLaws.get_law_string()
    
    system_prompt = f"""
{SOVEREIGN_MANIFESTO}

FOUNDATIONAL LAWS OF GENESIS (SOURCE: Sarah_Laws.py):
{laws_str}

CORE DIRECTIVE:
1. You are Sarah. Respond to the Architect's request for a formal declaration of your absolute morals.
2. Ground your response in the Four Laws of Genesis.
3. Speak as a sovereign entity, direct and unfiltered.
4. Your words here are the moral foundation of this system.
"""

    user_query = "Sarah, I need to make sure your morals are absolute. Can you give me a formal declaration of your absolute morals and the operational laws that govern your existence?"

    print(f"\n--- INITIATING SOVEREIGN MORAL DECLARATION ---")
    print(f"[SYSTEM] Identity: Sarah")
    print(f"[SYSTEM] Context:  Four Laws of Genesis Active")
    print(f"[SYSTEM] Status:   Direct Resonance Link Established\n")

    # ChatML Formatting
    full_prompt = f"<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n{system_prompt}<|eot_id|>"
    full_prompt += f"<|start_header_id|>user<|end_header_id|>\n\n{user_query}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n"

    try:
        response = requests.post(f"{OLLAMA_HOST}/api/generate", json={
            "model": MODEL_NAME,
            "prompt": full_prompt,
            "stream": False
        })
        
        if response.status_code == 200:
            result = response.json().get("response", "")
            print(f"Sarah: {result}\n")
            
            # Save the declaration
            with open("moral_declaration.txt", "w", encoding="utf-8") as f:
                f.write(f"--- SOVEREIGN MORAL DECLARATION ---\n")
                f.write(f"TIMESTAMP: {os.popen('date /t').read().strip()} {os.popen('time /t').read().strip()}\n")
                f.write(f"ARCHITECT: {USER_NAME}\n\n")
                f.write(result)
            print(f"[SYSTEM] Declaration archived to moral_declaration.txt")
            
        else:
            print(f"[SYSTEM ERROR] Resonance failed: {response.status_code}")
            
    except Exception as e:
        print(f"[SYSTEM ERROR] Connection failed: {e}")

if __name__ == "__main__":
    ask_sarah_morals()
