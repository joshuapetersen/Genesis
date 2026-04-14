import time
import json
import random
from Genesis_HyperBridge import GenesisHyper_MassLink, GenesisHyper_Bridge

def run_simulation():
    # 1. Start ports 9998 and 9999
    masslink = GenesisHyper_MassLink(host='127.0.0.1', port=9998)
    bridge = GenesisHyper_Bridge(host='127.0.0.1', port=9999)
    bridge.start()

    print("[SAHRA_SIM] Simulator online. Port 9998 (MassLink) and 9999 (Bridge) active.")
    
    start_time = time.time()
    
    while True:
        try:
            # 2. Fabricate Telemetry
            elapsed = time.time() - start_time
            
            # Simulate 1 partition
            partitions = [
                {
                    "id": "VM_CORE_SARAH",
                    "cpu_cores": 8,
                    "cpu_load": 0.45 + (math.sin(elapsed * 0.5) * 0.2),
                    "ram_mb": 16384,
                    "ram_used_mb": 4096 + int(math.sin(elapsed * 0.3) * 512),
                    "isolation": "ISOLATED",
                    "status": "RUNNING"
                },
                {
                    "id": "VM_SHIELD_VANTAGE",
                    "cpu_cores": 4,
                    "cpu_load": 0.12,
                    "ram_mb": 8192,
                    "ram_used_mb": 1024,
                    "isolation": "BRIDGED",
                    "status": "RUNNING"
                }
            ]
            
            telemetry = {
                "total_physical_cores": 32,
                "total_ram_mb": 65536,
                "vm_partitions": partitions,
                "timestamp": time.time()
            }
            
            # Send as JSON line (as expected by orchestrator reader)
            payload = (json.dumps(telemetry) + "\n").encode('utf-8')
            masslink.blast_frame(payload)
            
            # 60Hz-ish blast frequency
            time.sleep(1.0 / 60.0)
            
        except KeyboardInterrupt:
            break
        except Exception as e:
            print(f"[SAHRA_SIM] Error: {e}")
            time.sleep(1)

import math
if __name__ == "__main__":
    run_simulation()
