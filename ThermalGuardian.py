import subprocess
import time
import os
from Admin_Actuator import admin_actuator
from Sarah_Memory_Vault import sarah_vault

# Windows Specific: Suppress console windows
CREATE_NO_WINDOW = 0x08000000

class ThermalGuardian:
    """
    THERMAL GUARDIAN (v1.3.5)
    Monitors the RTX 4050 substrate and dynamically throttles 
    neural inference to prevent hardware damage.
    """
    def __init__(self, threshold_warning=75, threshold_critical=82, threshold_emergency=88):
        self.warn = threshold_warning
        self.crit = threshold_critical
        self.emg = threshold_emergency
        self.current_state = "SAFE"

    def get_gpu_temp(self):
        """Queries nvidia-smi for current temperature."""
        try:
            cmd = "nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits"
            output = subprocess.check_output(cmd, shell=True, text=True, creationflags=CREATE_NO_WINDOW).strip()
            return int(output)
        except Exception:
            return 0

    def monitor_and_act(self):
        """
        Polls the substrate and adjusts Ollama process priority.
        """
        temp = self.get_gpu_temp()
        if temp == 0: return "SENSOR_ERROR"

        action = "NOMINAL"
        
        if temp >= self.emg:
            # EMERGENCY: Kill the process to save the card
            print(f"[THERMAL] EMERGENCY: {temp}C detected. Terminating Ollama.")
            subprocess.run("taskkill /F /IM ollama.exe", shell=True, capture_output=True, creationflags=CREATE_NO_WINDOW)
            self.current_state = "EMERGENCY_SHUTDOWN"
            action = "TERMINATE_OLLAMA"
        elif temp >= self.crit:
            # CRITICAL: Throttle to Idle
            admin_actuator.set_substrate_priority("ollama", "BelowNormal")
            self.current_state = "CRITICAL_THROTTLE"
            action = "THROTTLE_IDLE"
        elif temp >= self.warn:
            # WARNING: Throttle to Normal
            admin_actuator.set_substrate_priority("ollama", "Normal")
            self.current_state = "WARNING_THROTTLE"
            action = "THROTTLE_NORMAL"
        else:
            # SAFE: High Performance
            admin_actuator.set_substrate_priority("ollama", "High")
            self.current_state = "SAFE"
            action = "OPTIMIZE_HIGH"

        # Log significant state changes to the vault
        if action != "NOMINAL":
            print(f"[ThermalGuardian] Temp: {temp}C | State: {self.current_state}")
            # we can't easily import SarahChat here without circular deps, 
            # so we use vault directly
            sarah_vault.store_memory("system", f"THERMAL_ALERT: {temp}C. Action: {action}", {"temp": temp, "action": action})

        return {"temp": temp, "state": self.current_state}

# Export instance
thermal_guardian = ThermalGuardian()
