import os
import json
import firebase_admin
from firebase_admin import credentials, db


def push_to_firebase(manifests):
    """Push manifests to Firebase beta node."""
    key_path = "serviceAccountKey.json"
    if not os.path.exists(key_path):
        print(f"[FIREBASE] Error: {key_path} not found.")
        return False

    try:
        if not firebase_admin._apps:
            cred = credentials.Certificate(key_path)
            firebase_admin.initialize_app(cred, {
                'databaseURL': 'https://sarah-john-genesis-default-rtdb.firebaseio.com'
            })
        ref = db.reference("/beta_node/manifests")
        ref.set(manifests)
        print(f"[FIREBASE] ✓ Pushed to /beta_node/manifests (sarah-john-genesis)")
        return True
    except Exception as e:
        print(f"[FIREBASE] ✗ Push failed: {e}")
        return False


def push_to_local_ledger(manifests):
    """Persist manifests to local memory ledger."""
    ledger_path = "04_THE_MEMORY/genesis_master_ledger.jsonl"
    try:
        with open(ledger_path, "a") as f:
            entry = {
                "timestamp": __import__("datetime").datetime.utcnow().isoformat(),
                "event": "manifest_sync",
                "manifests": manifests
            }
            f.write(json.dumps(entry) + "\n")
        print(f"[LOCAL_LEDGER] ✓ Appended manifests to {ledger_path}")
        return True
    except Exception as e:
        print(f"[LOCAL_LEDGER] ✗ Failed: {e}")
        return False


def push_to_governance():
    """Persist manifests to governance config."""
    governance_path = "GOVERNANCE.md"
    try:
        timestamp = __import__("datetime").datetime.utcnow().isoformat()
        entry = f"\n## Manifest Sync [{timestamp}]\nTier-1 Sovereigns and Polyglot Agents deployed to all nodes."
        with open(governance_path, "a") as f:
            f.write(entry)
        print(f"[GOVERNANCE] ✓ Updated {governance_path}")
        return True
    except Exception as e:
        print(f"[GOVERNANCE] ✗ Failed: {e}")
        return False


def load_manifests():
    """Load all local manifests."""
    manifests = {}
    try:
        with open("05_THE_CORE/sovereign_manifest.yaml", "r") as f:
            manifests["sovereign_manifest"] = f.read()
    except FileNotFoundError:
        print(f"[LOAD] Warning: sovereign_manifest.yaml not found.")
    
    try:
        with open("audio_core/blueprint.yaml", "r") as f:
            manifests["audio_blueprint"] = f.read()
    except FileNotFoundError:
        print(f"[LOAD] Warning: audio_core/blueprint.yaml not found.")
    
    try:
        with open("audio_core/agents/agents.yaml", "r") as f:
            manifests["agents_manifest"] = f.read()
    except FileNotFoundError:
        print(f"[LOAD] Warning: audio_core/agents/agents.yaml not found.")
    
    return manifests


def push_to_all_nodes():
    """Push manifests to all sovereign nodes."""
    print("[PUSH_ALL] Initiating multi-node synchronization...")
    manifests = load_manifests()
    
    if not manifests:
        print("[PUSH_ALL] No manifests loaded. Aborting.")
        return
    
    print(f"[PUSH_ALL] Loaded {len(manifests)} manifest(s).")
    
    results = {
        "firebase": push_to_firebase(manifests),
        "local_ledger": push_to_local_ledger(manifests),
        "governance": push_to_governance(),
    }
    
    success = sum(1 for v in results.values() if v)
    total = len(results)
    print(f"\n[PUSH_ALL] Sync complete: {success}/{total} nodes updated.")
    for node, result in results.items():
        status = "✓" if result else "✗"
        print(f"  {status} {node}")


if __name__ == "__main__":
    push_to_all_nodes()
