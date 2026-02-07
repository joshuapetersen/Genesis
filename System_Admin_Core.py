import wmi
import os
import sys
import subprocess
import ctypes
import json
from datetime import datetime

# Windows Specific: Suppress console windows
CREATE_NO_WINDOW = 0x08000000

class SystemAdminCore:
    """
    The Hand of the Sovereign.
    Provides deep integration with Windows Management Instrumentation (WMI)
    to monitor, control, and update hardware and system processes.
    REQUIRES: Administrator Privileges.
    """
    def __init__(self, monitor=None):
        self.monitor = monitor
        self.wmi = wmi.WMI()
        self.is_admin = self._check_admin()
        
        if not self.is_admin:
            print("[ADMIN CORE]: WARNING -> Insufficient Privileges. Read-Only Mode.")
            if self.monitor:
                self.monitor.capture("ADMIN", "PRIVILEGE_CHECK", {"status": "FAILED", "message": "Run as Admin required"})

    def _check_admin(self):
        try:
            return ctypes.windll.shell32.IsUserAnAdmin()
        except:
            return False

    def get_hardware_telemetry(self):
        """
        Scans all critical hardware components with defensive safety.
        Includes NVIDIA GPU and Lenovo System data if available.
        """
        telemetry = {
            'cpu': 'N/A', 
            'ram': 'N/A', 
            'gpu': 'N/A',
            'system_info': 'N/A',
            'timestamp': datetime.now().isoformat()
        }
        
        try:
            # CPU
            processors = self.wmi.Win32_Processor()
            if processors:
                cpu = processors[0]
                telemetry['cpu'] = {
                    'name': getattr(cpu, 'Name', 'Unknown'),
                    'load': getattr(cpu, 'LoadPercentage', 0) or 0,
                    'cores': getattr(cpu, 'NumberOfCores', 1)
                }
        except Exception as e:
            print(f"CPU Telemetry Failed: {e}")
            
        try:
            # RAM
            os_info = self.wmi.Win32_OperatingSystem()
            if os_info:
                mem = os_info[0]
                total = int(getattr(mem, 'TotalVisibleMemorySize', 1)) / 1024
                free = int(getattr(mem, 'FreePhysicalMemory', 0)) / 1024
                telemetry['ram'] = {
                    'total_mb': round(total, 2),
                    'free_mb': round(free, 2),
                    'usage_percent': round(((total - free) / total) * 100, 2)
                }
        except Exception as e:
            print(f"RAM Telemetry Failed: {e}")
            
        # NVIDIA TELEMETRY (via nvidia-smi)
        try:
            # Querying: name, temp, usage, memory
            cmd = "nvidia-smi --query-gpu=name,temperature.gpu,utilization.gpu,memory.used,memory.total --format=csv,noheader,nounits"
            result = subprocess.check_output(cmd, shell=True, text=True, creationflags=CREATE_NO_WINDOW).strip()
            if result:
                parts = [p.strip() for p in result.split(',')]
                telemetry['gpu'] = {
                    'name': parts[0],
                    'temp': parts[1],
                    'load_percent': parts[2],
                    'mem_used_mb': parts[3],
                    'mem_total_mb': parts[4]
                }
        except Exception as e:
            # Fallback to WMI if nvidia-smi fails
            try:
                gpus = []
                for gpu in self.wmi.Win32_VideoController():
                    gpus.append({'name': getattr(gpu, 'Name', 'Generic GPU'), 'status': getattr(gpu, 'Status', 'Active')})
                if gpus:
                    telemetry['gpu'] = gpus
            except:
                telemetry['gpu'] = "Not Detected"

        # LENOVO / SYSTEM IDENTITY
        try:
            bios = self.wmi.Win32_BIOS()[0]
            comp = self.wmi.Win32_ComputerSystem()[0]
            telemetry['system_info'] = {
                'manufacturer': getattr(comp, 'Manufacturer', 'PC'),
                'model': getattr(comp, 'Model', 'Substrate'),
                'serial': getattr(bios, 'SerialNumber', 'UNKNOWN'),
                'is_lenovo': "lenovo" in getattr(comp, 'Manufacturer', '').lower()
            }
        except Exception as e:
            telemetry['system_info'] = "PC_SUBSTRATE"

        if self.monitor:
            self.monitor.capture("ADMIN", "HARDWARE_SCAN", telemetry)
            
        return telemetry

    def list_processes(self):
        """
        Returns a list of running processes.
        """
        procs = []
        for process in self.wmi.Win32_Process():
            procs.append({
                'id': process.ProcessId,
                'name': process.Name,
                'memory': process.WorkingSetSize
            })
        return procs

    def kill_process(self, process_name):
        """
        Terminates a process by name.
        """
        if not self.is_admin:
            return False, "PERMISSION_DENIED"
            
        count = 0
        for process in self.wmi.Win32_Process(Name=process_name):
            try:
                process.Terminate()
                count += 1
            except:
                pass
        
        if self.monitor:
            self.monitor.capture("ADMIN", "KILL_PROCESS", {"target": process_name, "count": count})
            
        return True, f"Terminated {count} instances of {process_name}"

    def check_system_updates(self):
        """
        Checks for Windows Updates via PowerShell.
        """
        if not self.is_admin:
            return "PERMISSION_DENIED"
            
        print("[ADMIN CORE]: Scanning for Windows Updates...")
        # Uses the PSWindowsUpdate module logic or standard USOClient
        try:
            # Simple check command
            cmd = "usoclient StartScan"
            subprocess.run(cmd, shell=True, creationflags=CREATE_NO_WINDOW)
            return "Update Scan Initiated (Background)"
        except Exception as e:
            return f"Update Check Failed: {e}"

    def scan_new_hardware(self):
        """
        Forces a Plug and Play device scan.
        """
        if not self.is_admin:
            return "PERMISSION_DENIED"
            
        subprocess.run("pnputil /scan-devices", shell=True, creationflags=CREATE_NO_WINDOW)
        return "Hardware Scan Complete"

    def get_wifi_networks(self):
        """
        Scans for available WiFi networks using netsh.
        """
        try:
            result = subprocess.check_output(["netsh", "wlan", "show", "networks"], text=True, stderr=subprocess.STDOUT, creationflags=CREATE_NO_WINDOW)
            networks = []
            for line in result.split("\n"):
                if "SSID" in line and ":" in line:
                    ssid = line.split(":")[1].strip()
                    if ssid:
                        networks.append({"ssid": ssid, "secure": True})
            return networks
        except Exception as e:
            return [{"ssid": f"ERROR: {str(e)}", "secure": False}]

    def get_bluetooth_devices(self):
        """
        Scans for paired Bluetooth devices using powershell.
        """
        try:
            # Simple PS command to list paired BT devices
            cmd = "Get-PnpDevice -Class Bluetooth | Select-Object FriendlyName, Status"
            result = subprocess.check_output(["powershell", "-Command", cmd], text=True, creationflags=CREATE_NO_WINDOW)
            devices = []
            lines = result.strip().split("\n")
            if len(lines) > 2:
                for line in lines[2:]:
                    parts = line.split()
                    if len(parts) >= 2:
                        name = " ".join(parts[:-1])
                        status = parts[-1]
                        devices.append({"name": name, "connected": status == "OK"})
            return devices
        except Exception as e:
            return [{"name": f"BT_ERROR: {str(e)}", "connected": False}]
