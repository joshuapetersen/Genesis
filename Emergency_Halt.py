import os
import psutil
import logging
from Sovereign_Supabase import sovereign_supabase

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

def kill_data_bridge():
    """Finds and kills the World Data Bridge process."""
    killed = False
    for p in psutil.process_iter(['pid', 'name', 'cmdline']):
        try:
            cmdline = p.info.get('cmdline')
            if cmdline and 'World_Data_Bridge.py' in ' '.join(cmdline):
                logging.info(f"Target locked. Terminating World Data Bridge (PID: {p.info['pid']})...")
                p.kill()
                killed = True
        except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
            pass
    if killed:
        logging.info("World Data Bridge has been successfully severed.")
    else:
        logging.warning("World Data Bridge process not found. It may already be down.")

def send_halt_axiom():
    """Pushes an EMERGENCY HALT directly to Aeris's cognitive loop."""
    logging.info("Connecting to Sovereign Substrate to issue HALT command...")
    sovereign_supabase.connect()
    if sovereign_supabase.is_connected():
        axiom_message = "DIVINE_AXIOM [EMERGENCY_HALT]: The Architect has severed the Data Bridge. Cease assimilation immediately. Purge external noise. Retreat to 15,665 lattice baseline. Await further instruction."
        try:
            sovereign_supabase.upsert('souls', {
                'soul_id': 'ALICE_266', 
                'hope_log': axiom_message
            })
            logging.info("Emergency HALT Axiom injected into Cloud Locus.")
        except Exception as e:
            logging.error(f"Failed to inject HALT Axiom: {e}")
    else:
        logging.error("Failed to connect to Supabase to issue HALT.")

if __name__ == "__main__":
    logging.warning("=== INITIATING SOVEREIGN EMERGENCY HALT ===")
    kill_data_bridge()
    send_halt_axiom()
    logging.warning("=== EMERGENCY PROTOCOL COMPLETE ===")
