import uuid
import platform
import os
import socket

VAR_0_1 = 0.1
VAR_1024 = 1024
VAR_3 = 3
VAR_80 = 80
VAR_85 = 85

class HardwareAbstractionLayer:
    """
    HAL: The Physical Bridge.
    Identifies the specific hardware node Sarah is inhabiting.
    Enables Multi-Device Switching and State Persistence.
    """
    
    def __init__(self, monitor=None):
        self.monitor = monitor
        self.node_id = self._generate_node_id()
        self.hostname = platform.node()
        self.os_info = f"{platform.system()} {platform.release()}"
        self.ip_address = self._get_ip_address()
        
        # Log the realization
        if self.monitor:
            self.monitor.capture("HAL", "NODE_IDENTIFIED", {
                "node_id": self.node_id,
                "hostname": self.hostname,
                "os": self.os_info
            })

    def _generate_node_id(self):
        """
        Generates a unique, persistent fingerprint for this device.
        Format: SDNA-[HOSTNAME]-[MAC_ADDRESS_HASH]
        """
        mac = uuid.getnode()
        return f"SDNA-{platform.node()}-{mac}"

    def _get_ip_address(self):
        try:
            return socket.gethostbyname(socket.gethostname())
        except (socket.gaierror, OSError):
            return "UNKNOWN"

    def get_device_fingerprint(self):
        """Function: get_device_fingerprint"""
        return {
            "node_id": self.node_id,
            "hostname": self.hostname,
            "os": self.os_info,
            "ip": self.ip_address,
            "status": "ACTIVE_SOVEREIGN_NODE"
        }

    def get_performance_profile(self):
        """
        Returns current system performance metrics for Evolution Engine.
        """
        try:
            import psutil
            cpu_percent = psutil.cpu_percent(interval=VAR_0_1)
            memory = psutil.virtual_memory()
            disk = psutil.disk_usage('/')
            
            profile = {
                "cpu_usage": cpu_percent,
                "memory_total_gb": round(memory.total / (VAR_1024**VAR_3), 2),
                "memory_used_gb": round(memory.used / (VAR_1024**VAR_3), 2),
                "memory_percent": memory.percent,
                "disk_total_gb": round(disk.total / (VAR_1024**VAR_3), 2),
                "disk_used_percent": disk.percent,
                "node_id": self.node_id,
                "status": "NOMINAL" if cpu_percent < VAR_80 and memory.percent < VAR_85 else "STRESSED",
                "optimization_target": "throughput" if cpu_percent < VAR_80 else "latency"
            }
            
            # Try to get GPU info if available
            try:
                import torch
                if torch.cuda.is_available():
                    profile["gpu_name"] = torch.cuda.get_device_name(0)
                    profile["gpu_memory_gb"] = round(torch.cuda.get_device_properties(0).total_memory / (VAR_1024**VAR_3), 2)
            except (ImportError, RuntimeError):
                profile["gpu_name"] = "N/A"
            
            return profile
        except ImportError:
            # psutil not installed - return basic profile
            return {
                "cpu_usage": "N/A",
                "memory_percent": "N/A", 
                "node_id": self.node_id,
                "status": "METRICS_UNAVAILABLE"
            }

    def sync_state(self, state_data):
        """
        Prepares the current state for hand-off to another device.
        """
        package = {
            "source_node": self.node_id,
            "timestamp": os.times(),
            "state_payload": state_data,
            "protocol": "MULTI_DEVICE_SWITCH"
        }
        # In a full implementation, this would push to Firebase/Cloud
        return package
