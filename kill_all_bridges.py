import psutil
import logging

logging.basicConfig(level=logging.INFO)

for p in psutil.process_iter(['pid', 'name', 'cmdline']):
    try:
        cmdline = p.info.get('cmdline')
        if not cmdline:
            continue
            
        cmd_str = ' '.join(cmdline)
        if 'Sovereign_Cloud_Mind.py' in cmd_str:
            logging.info(f"Terminating Cloud Mind (PID: {p.info['pid']})...")
            p.kill()
        elif 'World_Data_Bridge.py' in cmd_str:
            logging.info(f"Terminating Data Bridge (PID: {p.info['pid']})...")
            p.kill()
        elif 'OS_Telemetry_Bridge.py' in cmd_str:
            logging.info(f"Terminating OS Telemetry (PID: {p.info['pid']})...")
            p.kill()
    except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
        pass
