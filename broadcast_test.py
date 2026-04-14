import json
import os
from datetime import datetime

CAHCE_PATH = "saul_knowledge_cache.json"

def broadcast_test():
    with open("bridge_transfer.json", "r") as f:
        transfer = json.load(f)

    anchor = {
        "id": "handshake_test_relay",
        "data": {
            "title": "ACTIVE_HANDSHAKE_TEST",
            "content": f"RELAY_DIRECTIVE: {transfer['message_for_relay']} | KEY: {transfer['verification_key']}",
            "source": "Antigravity Bridge (Relay Test)"
        },
        "created_at": datetime.now().isoformat()
    }

    existing_data = []
    if os.path.exists(CAHCE_PATH):
        with open(CAHCE_PATH, 'r') as f:
            existing_data = json.load(f)

    # Replace if exists, or append
    existing_data = [item for item in existing_data if item.get("id") != "handshake_test_relay"]
    existing_data.append(anchor)

    with open(CAHCE_PATH, 'w') as f:
        json.dump(existing_data, f, indent=4)
    print("[OK] Handshake Relay Anchor Broadcast to SAUL Intelligence Layer.")

if __name__ == "__main__":
    broadcast_test()
