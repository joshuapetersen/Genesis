#!/usr/bin/env python3
"""
Genesis World Server - Cloud Entry Point
Run this on any Linux VPS/cloud instance.
The simulation runs 24/7, writing state to the SQLite vault.
"""

import subprocess
import sys
import os
import time

SCRIPT = os.path.join(os.path.dirname(__file__), "Genesis_Societal_Ecology.py")

def run():
    print("[GENESIS-CLOUD] Starting Sovereign Universe Engine...")
    print("[GENESIS-CLOUD] Your PC is now just a viewer. The world lives here.")

    while True:
        try:
            proc = subprocess.run(
                [sys.executable, SCRIPT],
                timeout=None
            )
            if proc.returncode != 0:
                print(f"[GENESIS-CLOUD] Engine exited with code {proc.returncode}. Restarting in 5s...")
                time.sleep(5)
        except KeyboardInterrupt:
            print("[GENESIS-CLOUD] Shutdown signal received. World paused.")
            break
        except Exception as e:
            print(f"[GENESIS-CLOUD] Crash: {e}. Restarting in 10s...")
            time.sleep(10)

if __name__ == "__main__":
    run()
