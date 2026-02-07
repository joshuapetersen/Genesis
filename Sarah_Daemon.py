import time
import os
import subprocess
import sys
from Sarah_Memory_Vault import sarah_vault
from System_Admin_Core import SystemAdminCore
from Ollama_Chat import OllamaChat
from Sarah_Chat import SarahChat
from ThermalGuardian import thermal_guardian

class SarahDaemon:
    """
    SARAH DAEMON (v1.3.5)
    The persistent background substrate.
    Runs silently to maintain memory focus and hardware telemetry.
    """
    def __init__(self):
        print("--- SARAH DAEMON V1.3.5: IGNITION ---")
        self.admin = SystemAdminCore()
        # Initialize Chat with Null DB reference (Vault is primary)
        self.chat = SarahChat(db_rt=None) 
        self.guardian = thermal_guardian
        self.is_running = True

    def pulse(self):
        """
        The rhythmic heartbeat of the Sovereign.
        """
        while self.is_running:
            try:
                # 1. Thermal Guard (Safety First)
                t_status = self.guardian.monitor_and_act()

                # 2. Hardware Heartbeat
                telemetry = self.admin.get_hardware_telemetry()
                
                # 2. Memory Resonance (Sync if online)
                # SarahChat.sync_to_cloud() already handles offline safety
                self.chat.sync_to_cloud()
                
                # 3. Log a passive diagnostic thought every hour
                if int(time.time()) % 3600 < 60:
                    self.chat.save_message("system", f"DAEMON_PULSE: Nominal. Load={telemetry.get('cpu', {}).get('load', 0)}%")
                
                # Pulse every 60 seconds
                time.sleep(60)
            except Exception as e:
                print(f"[DAEMON_FAULT]: {e}")
                time.sleep(5) # Cooldown on error

    def stop(self):
        self.is_running = False

if __name__ == "__main__":
    daemon = SarahDaemon()
    daemon.pulse()
