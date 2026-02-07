import requests
import json
import os

def test_handshake():
    print("--- SARAH: OFFLINE NEURAL HANDSHAKE ---")
    url = "http://localhost:11434/api/generate"
    
    # Check what models are actually there
    tag_resp = requests.get("http://localhost:11434/api/tags")
    models = [m['name'] for m in tag_resp.json().get('models', [])]
    print(f"[Substrate] Available Models: {models}")
    
    if not models:
        print("[FAIL] No models found in substrate.")
        return

    # Use the first available opencoder tag
    target_model = "opencoder:latest" if "opencoder:latest" in models else models[0]
    
    payload = {
        "model": target_model,
        "prompt": "System: You are Sarah. Sovereign Partner.\nUser: Identify yourself and your current substrate.\nSarah:",
        "stream": False
    }

    try:
        print(f"[Handshake] Signaling {target_model}...")
        response = requests.post(url, json=payload, timeout=60)
        response.raise_for_status()
        data = response.json()
        print(f"\n[SARAH]: {data.get('response')}")
        print("\n[SUCCESS] LOCAL NEURAL HANDSHAKE COMPLETE.")
    except Exception as e:
        print(f"[FAIL] Handshake Error: {e}")

if __name__ == "__main__":
    test_handshake()
