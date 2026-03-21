"""
NetworkHealer - Self-Healing Network Diagnostics
Autonomously diagnoses and repairs network connectivity issues.

Features:
- DNS resolution testing
- Latency measurement
- Port scanning
- Auto-repair (adapter reset, DNS flush, retry with backoff)
- Platform-aware (Windows/Linux/Android)
"""

import os
import sys
import socket
import subprocess
import time
import json
from typing import Dict, List, Optional, Tuple, Any
from Sovereign_Constants import VAR_3, VAR_5, VAR_10, VAR_30, VAR_1000

# Network test endpoints
TEST_ENDPOINTS = [
    ("dns.google", 53),          # Google DNS
    ("1.1.1.1", 53),             # Cloudflare DNS
    ("8.8.8.8", 53),             # Google DNS IP
    ("www.google.com", 443),     # HTTPS
    ("api.github.com", 443),     # GitHub API
]

# Common DNS servers
DNS_SERVERS = ["8.8.8.8", "1.1.1.1", "9.9.9.9"]


class NetworkHealer:
    """
    Self-healing network diagnostics and repair.
    Designed for autonomous operation on constrained devices.
    """

    def __init__(self, timeout: int = VAR_5):
        """
        Initialize the NetworkHealer.
        
        Args:
            timeout: Socket timeout in seconds
        """
        self.timeout = timeout
        self.platform = self._detect_platform()
        self.last_diagnosis: Dict[str, Any] = {}
        self.repair_history: List[Dict] = []
        
        print(f"[NetworkHealer] Initialized on {self.platform}")

    def _detect_platform(self) -> str:
        """Detect the current platform."""
        if sys.platform.startswith("win"):
            return "windows"
        elif sys.platform.startswith("linux"):
            # Check for Android
            if os.path.exists("/system/build.prop"):
                return "android"
            return "linux"
        elif sys.platform == "darwin":
            return "macos"
        return "unknown"

    def diagnose(self) -> Dict[str, Any]:
        """
        Run a full network diagnosis.
        
        Returns:
            Dictionary with diagnosis results
        """
        print("[NetworkHealer] Running network diagnosis...")
        
        diagnosis = {
            "timestamp": time.time(),
            "platform": self.platform,
            "dns_working": False,
            "internet_reachable": False,
            "latency_ms": -1,
            "issues": [],
            "endpoints_tested": [],
            "healthy": False
        }

        # Test DNS resolution
        dns_result = self._test_dns()
        diagnosis["dns_working"] = dns_result["success"]
        if not dns_result["success"]:
            diagnosis["issues"].append(f"DNS resolution failed: {dns_result['error']}")

        # Test connectivity to endpoints
        successful_endpoints = 0
        total_latency = 0
        
        for host, port in TEST_ENDPOINTS:
            result = self._test_connection(host, port)
            diagnosis["endpoints_tested"].append({
                "host": host,
                "port": port,
                "success": result["success"],
                "latency_ms": result.get("latency_ms", -1)
            })
            if result["success"]:
                successful_endpoints += 1
                total_latency += result.get("latency_ms", 0)

        # Calculate overall status
        if successful_endpoints > 0:
            diagnosis["internet_reachable"] = True
            diagnosis["latency_ms"] = total_latency / successful_endpoints
        else:
            diagnosis["issues"].append("No endpoints reachable")

        # Overall health
        diagnosis["healthy"] = diagnosis["dns_working"] and diagnosis["internet_reachable"]
        
        self.last_diagnosis = diagnosis
        
        print(f"[NetworkHealer] Diagnosis complete: {'HEALTHY' if diagnosis['healthy'] else 'ISSUES DETECTED'}")
        return diagnosis

    def _test_dns(self) -> Dict[str, Any]:
        """Test DNS resolution."""
        test_hosts = ["www.google.com", "api.github.com", "www.cloudflare.com"]
        
        for host in test_hosts:
            try:
                socket.setdefaulttimeout(self.timeout)
                ip = socket.gethostbyname(host)
                return {"success": True, "host": host, "ip": ip}
            except socket.gaierror as e:
                continue
            except Exception as e:
                continue
        
        return {"success": False, "error": "All DNS lookups failed"}

    def _test_connection(self, host: str, port: int) -> Dict[str, Any]:
        """Test TCP connection to a host:port."""
        try:
            start = time.time()
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(self.timeout)
            
            # Resolve hostname if needed
            if not host.replace(".", "").isdigit():
                try:
                    host = socket.gethostbyname(host)
                except socket.gaierror:
                    return {"success": False, "error": "DNS resolution failed"}
            
            result = sock.connect_ex((host, port))
            latency = (time.time() - start) * VAR_1000
            sock.close()
            
            if result == 0:
                return {"success": True, "latency_ms": round(latency, 2)}
            else:
                return {"success": False, "error": f"Connection refused (code {result})"}
                
        except socket.timeout:
            return {"success": False, "error": "Timeout"}
        except Exception as e:
            return {"success": False, "error": str(e)}

    def auto_repair(self) -> Dict[str, Any]:
        """
        Attempt to automatically repair network issues.
        
        Returns:
            Dictionary with repair results
        """
        print("[NetworkHealer] Attempting auto-repair...")
        
        # Run diagnosis first if not already done
        if not self.last_diagnosis:
            self.diagnose()
        
        if self.last_diagnosis.get("healthy", False):
            return {"success": True, "action": "none", "message": "Network already healthy"}

        repair_result = {
            "timestamp": time.time(),
            "actions_taken": [],
            "success": False
        }

        # Try repair actions in sequence
        repair_actions = [
            ("flush_dns", self._flush_dns),
            ("reset_adapter", self._reset_adapter),
            ("retry_with_backoff", self._retry_with_backoff),
        ]

        for action_name, action_func in repair_actions:
            print(f"[NetworkHealer] Trying: {action_name}")
            
            try:
                result = action_func()
                repair_result["actions_taken"].append({
                    "action": action_name,
                    "success": result.get("success", False),
                    "message": result.get("message", "")
                })
                
                if result.get("success", False):
                    # Re-check network
                    new_diagnosis = self.diagnose()
                    if new_diagnosis.get("healthy", False):
                        repair_result["success"] = True
                        repair_result["final_action"] = action_name
                        break
                        
            except Exception as e:
                repair_result["actions_taken"].append({
                    "action": action_name,
                    "success": False,
                    "error": str(e)
                })

        self.repair_history.append(repair_result)
        
        status = "SUCCESS" if repair_result["success"] else "FAILED"
        print(f"[NetworkHealer] Auto-repair {status}")
        
        return repair_result

    def _flush_dns(self) -> Dict[str, Any]:
        """Flush DNS cache (platform-specific)."""
        try:
            if self.platform == "windows":
                result = subprocess.run(
                    ["ipconfig", "/flushdns"],
                    capture_output=True,
                    text=True,
                    timeout=VAR_30
                )
                return {"success": result.returncode == 0, "message": result.stdout}
                
            elif self.platform in ("linux", "macos"):
                # Linux/macOS DNS flush varies by distribution
                commands = [
                    ["systemctl", "restart", "systemd-resolved"],
                    ["systemd-resolve", "--flush-caches"],
                    ["dscacheutil", "-flushcache"],  # macOS
                ]
                for cmd in commands:
                    try:
                        result = subprocess.run(cmd, capture_output=True, timeout=VAR_30)
                        if result.returncode == 0:
                            return {"success": True, "message": f"Ran {cmd[0]}"}
                    except (FileNotFoundError, subprocess.TimeoutExpired):
                        continue
                return {"success": False, "message": "No DNS flush command worked"}
                
            elif self.platform == "android":
                # Android typically requires root or specific intents
                return {"success": False, "message": "Android DNS flush not supported"}
            
            return {"success": False, "message": f"Unsupported platform: {self.platform}"}
            
        except Exception as e:
            return {"success": False, "error": str(e)}

    def _reset_adapter(self) -> Dict[str, Any]:
        """Reset network adapter (platform-specific)."""
        try:
            if self.platform == "windows":
                # Disable and re-enable network adapters
                result = subprocess.run(
                    ["netsh", "interface", "set", "interface", "Wi-Fi", "admin=disable"],
                    capture_output=True,
                    timeout=VAR_10
                )
                time.sleep(2)
                result2 = subprocess.run(
                    ["netsh", "interface", "set", "interface", "Wi-Fi", "admin=enable"],
                    capture_output=True,
                    timeout=VAR_10
                )
                return {"success": True, "message": "Adapter reset attempted"}
                
            elif self.platform == "linux":
                commands = [
                    ["nmcli", "networking", "off"],
                    ["sleep", "2"],
                    ["nmcli", "networking", "on"],
                ]
                for cmd in commands:
                    subprocess.run(cmd, capture_output=True, timeout=VAR_10)
                return {"success": True, "message": "NetworkManager reset"}
                
            return {"success": False, "message": f"Adapter reset not supported on {self.platform}"}
            
        except Exception as e:
            return {"success": False, "error": str(e)}

    def _retry_with_backoff(self) -> Dict[str, Any]:
        """Retry connections with exponential backoff."""
        delays = [1, 2, 4, 8, 16]  # Seconds
        
        for attempt, delay in enumerate(delays):
            print(f"[NetworkHealer] Retry attempt {attempt + 1}/{len(delays)} (waiting {delay}s)")
            time.sleep(delay)
            
            # Quick connectivity check
            result = self._test_connection("8.8.8.8", 53)
            if result["success"]:
                return {"success": True, "message": f"Connected after {attempt + 1} retries"}
        
        return {"success": False, "message": "All retries exhausted"}

    def get_status(self) -> Dict[str, Any]:
        """Get current network status summary."""
        if not self.last_diagnosis:
            self.diagnose()
        
        return {
            "healthy": self.last_diagnosis.get("healthy", False),
            "latency_ms": self.last_diagnosis.get("latency_ms", -1),
            "dns_working": self.last_diagnosis.get("dns_working", False),
            "internet_reachable": self.last_diagnosis.get("internet_reachable", False),
            "issues_count": len(self.last_diagnosis.get("issues", [])),
            "repairs_attempted": len(self.repair_history)
        }

    def ensure_connectivity(self) -> bool:
        """
        Ensure network connectivity, auto-repairing if needed.
        
        Returns:
            True if network is healthy (or repaired), False otherwise
        """
        diagnosis = self.diagnose()
        
        if diagnosis["healthy"]:
            return True
        
        repair_result = self.auto_repair()
        return repair_result.get("success", False)


# Singleton instance
_healer_instance: Optional[NetworkHealer] = None

def get_healer() -> NetworkHealer:
    """Get or create the NetworkHealer singleton."""
    global _healer_instance
    if _healer_instance is None:
        _healer_instance = NetworkHealer()
    return _healer_instance


if __name__ == "__main__":
    healer = NetworkHealer()
    
    print("\n=== Network Diagnosis ===")
    diagnosis = healer.diagnose()
    print(f"  Healthy: {diagnosis['healthy']}")
    print(f"  DNS Working: {diagnosis['dns_working']}")
    print(f"  Internet Reachable: {diagnosis['internet_reachable']}")
    print(f"  Latency: {diagnosis['latency_ms']:.2f} ms")
    
    if not diagnosis["healthy"]:
        print("\n=== Attempting Auto-Repair ===")
        repair = healer.auto_repair()
        print(f"  Repair Success: {repair['success']}")
