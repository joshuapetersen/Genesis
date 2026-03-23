import os
import json
import firebase_admin
from firebase_admin import credentials, db


def backsync_manifests():
    """
    Backsync sovereign manifests to Firebase Realtime Database.
    Reads local manifests and pushes to /beta_node/manifests.
    """
    key_path = "serviceAccountKey.json"
    if not os.path.exists(key_path):
        print(f"[BACKSYNC] Error: {key_path} not found. Cannot initialize Firebase.")
        return

    try:
        cred = credentials.Certificate(key_path)
        firebase_admin.initialize_app(cred, {
            'databaseURL': 'https://sarah-john-genesis-default-rtdb.firebaseio.com'
        })
    except Exception as e:
        print(f"[BACKSYNC] Firebase init failed: {e}")
        return

    # Read local manifests
    manifests = {}
    try:
        with open("05_THE_CORE/sovereign_manifest.yaml", "r") as f:
            manifests["sovereign_manifest"] = f.read()
        print(f"[BACKSYNC] Loaded sovereign_manifest.yaml")
    except FileNotFoundError:
        print(f"[BACKSYNC] Warning: sovereign_manifest.yaml not found.")

    try:
        with open("audio_core/blueprint.yaml", "r") as f:
            manifests["audio_blueprint"] = f.read()
        print(f"[BACKSYNC] Loaded audio_core/blueprint.yaml")
    except FileNotFoundError:
        print(f"[BACKSYNC] Warning: audio_core/blueprint.yaml not found.")

    try:
        with open("audio_core/agents/agents.yaml", "r") as f:
            manifests["agents_manifest"] = f.read()
        print(f"[BACKSYNC] Loaded audio_core/agents/agents.yaml")
    except FileNotFoundError:
        print(f"[BACKSYNC] Warning: audio_core/agents/agents.yaml not found.")

    # Push to Firebase
    ref = db.reference("/beta_node/manifests")
    try:
        ref.set(manifests)
        print(f"[BACKSYNC] Successfully pushed manifests to /beta_node/manifests")
        print(f"[BACKSYNC] Project: sarah-john-genesis")
    except Exception as e:
        print(f"[BACKSYNC] Push failed: {e}")


if __name__ == "__main__":
    backsync_manifests()
