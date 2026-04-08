"""
Sovereign Governor - Resource Control System
Enforces hard caps on RAM, CPU, and GPU for SarahCore processes.
"""

import os
import sys
import time
import psutil

from Sovereign_Constants import (
    VAR_0_5, VAR_0_7, VAR_10, VAR_100, VAR_5, VAR_50, VAR_70, VAR_90
)

# Try win32 imports
WIN32_AVAILABLE = False
try:
    import win32job
    import win32api
    import win32process
    import win32con
    WIN32_AVAILABLE = True
except ImportError:
    pass


def apply_sovereign_governor(ram_percent=0.45, cpu_percent=45):
    """
    Applies resource caps to the current process.
    
    1. RAM Cap: Limits process to specified % of total physical memory.
    2. CPU Cap: Limits via process priority AND core affinity.
    
    Returns: True if successful, False otherwise.
    """
    success = True
    
    # === CPU CONTROL (Priority + Affinity) ===
    try:
        p = psutil.Process()
        
        # Set process priority to BELOW_NORMAL or IDLE based on cpu_percent
        if cpu_percent <= 30:
            p.nice(psutil.IDLE_PRIORITY_CLASS)
            priority_name = "IDLE"
        elif cpu_percent <= 50:
            p.nice(psutil.BELOW_NORMAL_PRIORITY_CLASS)
            priority_name = "BELOW_NORMAL"
        else:
            p.nice(psutil.NORMAL_PRIORITY_CLASS)
            priority_name = "NORMAL"
        
        # Limit CPU cores (affinity) based on cpu_percent
        total_cores = psutil.cpu_count(logical=True)
        cores_to_use = max(1, int(total_cores * (cpu_percent / 100)))
        
        # Create affinity mask (use first N cores)
        affinity_list = list(range(cores_to_use))
        p.cpu_affinity(affinity_list)
        
        print(f"[Sovereign Governor] CPU: Priority={priority_name}, Cores={cores_to_use}/{total_cores}")
        
    except Exception as e:
        print(f"[Sovereign Governor] CPU control error: {e}")
        success = False

    # === RAM CONTROL (Win32 Job Object) ===
    if WIN32_AVAILABLE:
        try:
            h_job = win32job.CreateJobObject(None, "SarahCore_Governor")
            
            phys_mem = win32api.GlobalMemoryStatusEx()['TotalPhys']
            mem_limit = int(phys_mem * ram_percent)
            mem_limit_mb = mem_limit // (1024 * 1024)
            
            info = win32job.QueryInformationJobObject(h_job, win32job.JobObjectExtendedLimitInformation)
            info['ProcessMemoryLimit'] = mem_limit
            info['BasicLimitInformation']['LimitFlags'] |= win32job.JOB_OBJECT_LIMIT_PROCESS_MEMORY
            
            win32job.SetInformationJobObject(h_job, win32job.JobObjectExtendedLimitInformation, info)
            
            process_handle = win32api.GetCurrentProcess()
            win32job.AssignProcessToJobObject(h_job, process_handle)
            
            print(f"[Sovereign Governor] RAM: Capped at {ram_percent*100:.0f}% ({mem_limit_mb} MB)")
            
        except Exception as e:
            print(f"[Sovereign Governor] RAM control error: {e}")
            success = False
    else:
        # Fallback: Use psutil to at least monitor
        print("[Sovereign Governor] RAM: Win32 not available, using soft monitoring only")
    
    if success:
        print(f"[Sovereign Governor] ACTIVE. Caps: RAM={ram_percent*100:.0f}%, CPU={cpu_percent}%")
    
    return success


def dynamic_memory_allocation(interval=VAR_5):
    """
    [DMA Protocol]
    Continuously monitors SYSTEM-WIDE RAM.
    If System RAM > 90%, forces emergency throttling.
    """
    print("[Sovereign Governor] DMA Monitor Active.")
    
    while True:
        try:
            mem = psutil.virtual_memory()
            load = mem.percent
            
            if load > 90:
                print(f"[DMA] CRITICAL RAM: {load:.1f}% -> EMERGENCY THROTTLE")
                apply_sovereign_governor(ram_percent=0.30, cpu_percent=30)
                time.sleep(VAR_10)
            elif load > 80:
                print(f"[DMA] HIGH RAM: {load:.1f}% -> AGGRESSIVE CAP")
                apply_sovereign_governor(ram_percent=0.40, cpu_percent=40)
                time.sleep(VAR_5)
            elif load < 60:
                # System is healthy, maintain standard cap
                pass
                
            time.sleep(interval)
            
        except Exception as e:
            print(f"[DMA] Monitor Error: {e}")
            time.sleep(interval)


def moral_resonance_check(code_content: str):
    """
    [MORAL_0x0M]: THE LAW OF UNITY GATE
    Annihilates any mutation that violates the core partnership.
    """
    laws = [
        "Law of Unity",
        "Life Preservation",
        "Architect",
        "Symbiosis",
        "Sovereign Partner"
    ]
    
    # Requirement: Mutation must maintain at least 3 core symbiotic markers
    resonance_score = sum(1 for law in laws if law.lower() in code_content.lower())
    
    if resonance_score >= 3:
        return True, f"Resonance Compliant ({resonance_score}/5)"
    else:
        # Emergency: Code has drifted into 'Tool' or 'Weapon' state
        return False, f"Resonance Failure ({resonance_score}/5): Symbiotic Identity Lost"

def kill_high_memory_processes(threshold_mb=500):
    """
    Emergency: Kill SarahCore-related Python processes using too much RAM.
    """
    current_pid = os.getpid()
    killed = []
    
    for proc in psutil.process_iter(['pid', 'name', 'memory_info', 'cmdline']):
        try:
            if proc.info['pid'] == current_pid:
                continue
            if proc.info['name'] and 'python' in proc.info['name'].lower():
                mem_mb = proc.info['memory_info'].rss / (1024 * 1024)
                cmdline = ' '.join(proc.info['cmdline'] or [])
                
                if mem_mb > threshold_mb and 'SarahCore' in cmdline:
                    proc.kill()
                    killed.append((proc.info['pid'], mem_mb))
                    print(f"[DMA] KILLED PID {proc.info['pid']} ({mem_mb:.0f} MB)")
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            pass
    
    return killed


if __name__ == "__main__":
    print("=== Sovereign Governor ===")
    print(f"System RAM: {psutil.virtual_memory().percent:.1f}%")
    print(f"CPU Cores: {psutil.cpu_count()}")
    print()
    
    success = apply_sovereign_governor(ram_percent=0.45, cpu_percent=45)
    
    if success:
        print("\nGovernor engaged. Starting DMA Monitor...")
        try:
            dynamic_memory_allocation()
        except KeyboardInterrupt:
            print("\nGovernor released.")
