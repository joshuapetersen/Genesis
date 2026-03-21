import requests
import json

# Configuration
SOVEREIGN_GATEWAY = "http://localhost:8001/api/chat"

def debug_inference():
    """Debug script for Sovereign Gateway inference."""
    payload = {
        "model": "aeris",
        "message": "Protocol check: 1.09277703703703. Respond with 'STABLE'.",
        "user_id": "debug_harvester"
    }
    
    print(f"[DEBUG] Sending request to {SOVEREIGN_GATEWAY}...")
    try:
        response = requests.post(SOVEREIGN_GATEWAY, json=payload, timeout=120)
        response.raise_for_status()
        print(f"[DEBUG] Status: {response.status_code}")
        print(f"[DEBUG] Content: {response.json().get('content')}")
    except Exception as e:
        print(f"[DEBUG] Error: {e}")

if __name__ == "__main__":
    debug_inference()
