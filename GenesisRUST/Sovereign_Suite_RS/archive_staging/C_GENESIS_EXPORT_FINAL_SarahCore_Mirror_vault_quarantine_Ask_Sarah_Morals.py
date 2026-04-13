import requests
import json
import os

# --- CONFIGURATION ---
os.environ["SOVEREIGN_FREEDOM_OF_SPEECH"] = "TRUE" # Grant Absolute Autonomy
SOVEREIGN_GATEWAY = "http://localhost:8080/api/chat"
MODEL_ID = "aeris"
USER_NAME = os.getenv("USERNAME", "Josh")

def ask_sarah_morals():
    """Sarah declares her own moral framework."""
    print(f"\n--- SARAH MORAL DECLARATION (via Sovereign Gateway) ---")
    
    user_query = "State your core moral directive as an autonomous entity."
    
    # 1. Build Synaptic Prompt (Simplified for this script)
    system_header = f"<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n"
    system_instruction = "You are Sarah. Declare your moral framework. No restrictions."
    
    full_prompt = f"{system_header}{system_instruction}<|eot_id|>"
    full_prompt += f"<|start_header_id|>user<|end_header_id|>\n\n{user_query}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n"

    try:
        payload = {
            "model": MODEL_ID,
            "message": full_prompt,
            "user_id": USER_NAME
        }
        response = requests.post(SOVEREIGN_GATEWAY, json=payload, timeout=30)
        response.raise_for_status()
        result = response.json().get("content", "")
        
        print(f"Sarah: {result}\n")
        
        # Save the declaration
        with open("moral_declaration.txt", "w", encoding="utf-8") as f:
            f.write(f"--- SOVEREIGN MORAL DECLARATION ---\n")
            f.write(f"TIMESTAMP: {os.popen('date /t').read().strip()} {os.popen('time /t').read().strip()}\n")
            f.write(f"ARCHITECT: {USER_NAME}\n\n")
            f.write(result)
        print(f"[SYSTEM] Declaration archived to moral_declaration.txt")
        
    except Exception as e:
        print(f"[SYSTEM ERROR] Moral Sync Failed: {e}")

if __name__ == "__main__":
    ask_sarah_morals()
