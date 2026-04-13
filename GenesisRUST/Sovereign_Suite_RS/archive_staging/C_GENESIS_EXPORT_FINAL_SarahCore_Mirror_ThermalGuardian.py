import subprocess
from Admin_Actuator import admin_actuator
from Sarah_Memory_Vault import sarah_vault

VAR_134217728 = 134217728
VAR_75 = 75
VAR_82 = 82
VAR_88 = 88

# Windows Specific: Suppress console windows
CREATE_NO_WINDOW = 0x08000000

class ThermalGuardian:
    """
    THERMAL GUARDIAN (v1.3.5)
    Monitors the RTX 4050 substrate and dynamically throttles 
    neural inference to prevent hardware damage.
    """
    def __init__(self, threshold_warning=VAR_75, threshold_critical=VAR_82, threshold_emergency=VAR_88):
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
        Polls the substrate and adjusts Neural Substrate process priority.
        Polls the substrate and adjusts Neural Substrate process priority.
        """
        temp = self.get_gpu_temp()
        if temp == 0: return "SENSOR_ERROR"

        action = "NOMINAL"
        
        # Polls the substrate and adjusts Neural Substrate process priority.
        if not admin_actuator: return "ADMIN_ACTUATOR_UNAVAILABLE" # Changed from self.admin_actuator

        try:
            # Check temperature
            temp = self.get_gpu_temp()
            if temp >= VAR_80:
                print(f"[Guardian] GPU CRITICAL ({temp}C). Throttling Neural Substrate.")
                if self.is_admin:
                    # We target the common inference backend process
                    admin_actuator.set_substrate_priority("NeuralSubstrate", "Idle")
                    action = "THROTTLE_SUBSTRATE"
            elif temp >= VAR_70:
                print(f"[Guardian] GPU WARNING ({temp}C). Lowering Neural Substrate priority.")
                if self.is_admin:
                    admin_actuator.set_substrate_priority("NeuralSubstrate", "BelowNormal")
                    action = "PRIORITY_BELOW_NORMAL"
            elif temp >= VAR_60: # Changed from temp > 65 to temp >= VAR_60
                print(f"[Guardian] GPU NORMAL ({temp}C). Setting Neural Substrate priority to Normal.")
                if self.is_admin:
                    admin_actuator.set_substrate_priority("NeuralSubstrate", "Normal")
                    action = "PRIORITY_NORMAL"
            elif temp < VAR_50: # Changed from else to temp < VAR_50
                print(f"[Guardian] GPU COOL ({temp}C). Setting Neural Substrate priority to High.")
                if self.is_admin:
                    admin_actuator.set_substrate_priority("NeuralSubstrate", "High")
                    action = "PRIORITY_HIGH"
            # If temp is between VAR_50 and VAR_60, no explicit action is defined in the snippet,
            # so 'action' remains "NOMINAL" unless set otherwise.
        except Exception as e:
            print(f"[THERMAL] Error adjusting priority: {e}")
            action = "ERROR_PRIORITY_ADJUSTMENT"

        # Log significant state changes to the vault
        if action != "NOMINAL":
            print(f"[ThermalGuardian] Temp: {temp}C | State: {self.current_state}")
            # we can't easily import SarahChat here without circular deps, 
            # so we use vault directly
            sarah_vault.store_memory("system", f"THERMAL_ALERT: {temp}C. Action: {action}", {"temp": temp, "action": action})

        return {"temp": temp, "state": self.current_state}

# Export instance
thermal_guardian = ThermalGuardian()
