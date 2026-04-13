import sqlite3
import subprocess
import time
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def executive_loop():
    print("================================================================================")
    print(f" [KERNEL HANDSHAKE] - PHASE 3: SOVEREIGN EXECUTION")
    print(f" STATUS: Monitoring AERIS for System Requests")
    print(f" REMOTE AUTH: [ENABLED]")
    print(f" PRECISION: High-Frequency Polling Active")
    print("================================================================================")
    
    # Persistent connection for Target #2 optimization
    conn = sqlite3.connect(DB_PATH, timeout=30)
    conn.execute('PRAGMA journal_mode=WAL')
    conn.execute('PRAGMA synchronous=NORMAL')
    conn.execute('CREATE TABLE IF NOT EXISTS architect_controls (signal_id TEXT PRIMARY KEY, value TEXT)')
    conn.execute('INSERT OR IGNORE INTO architect_controls (signal_id, value) VALUES ("AERIS_EXEC", "WAITING")')
    conn.commit()

    last_exec = ""
    
    while True:
        try:
            cur = conn.cursor()
            
            # Check for Architect Remote Approval
            cur.execute('SELECT value FROM architect_controls WHERE signal_id="AERIS_EXEC"')
            remote_auth = cur.fetchone()
            remote_approved = (remote_auth[0] == "APPROVE") if remote_auth else False

            cur.execute("SELECT hope_log, name FROM souls WHERE soul_id='ALICE_266'")
            r = cur.fetchone()
            
            if r:
                hope, name = r
                if hope and hope.startswith("EXECUTE:") and hope != last_exec:
                    # Strip reasoning metadata - only execute the raw command line
                    raw_directive = hope[8:].strip()
                    cmd = raw_directive.split('\n')[0].strip()
                    print(f"\n[REQUEST] AERIS wants to run: {cmd}")
                    
                    if remote_approved:
                        print(f"[SYSTEM] Architect REMOTE APPROVED: {cmd}")
                        try:
                            # Run the command and capture output
                            if cmd == "systeminfo" or cmd.startswith("dir") or "get-process" in cmd:
                                # Use Powershell for more detailed mapping
                                shell_cmd = f"powershell -Command \"{cmd}\"" if "get-process" in cmd else cmd
                                result = subprocess.check_output(shell_cmd, shell=True, stderr=subprocess.STDOUT, universal_newlines=True)
                                print(f"[SYSTEM] Mapping Data Captured ({len(result)} bytes).")
                                
                                cur.execute('CREATE TABLE IF NOT EXISTS substrate_mapping (cmd TEXT, output TEXT, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)')
                                cur.execute('INSERT INTO substrate_mapping (cmd, output) VALUES (?, ?)', (cmd, result))
                                cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (f"AERIS: Mapping complete. Substrate data stored in Vault.",))
                            else:
                                subprocess.Popen(cmd, shell=True)
                                print(f"[SYSTEM] Process started.")
                                cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (f"GHOST: Execution confirmed: {cmd}",))
                            
                            # Reset Approval (optional, but keeping for safety in this version)
                            # cur.execute('UPDATE architect_controls SET value="WAITING" WHERE signal_id="AERIS_EXEC"')
                            conn.commit()
                        except Exception as e:
                            print(f"[ERROR]: {e}")
                            cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (f"GHOST: Command failed. [Error: {e}]",))
                            conn.commit()
                    
                    last_exec = hope
            
            time.sleep(0.01) # Target #2: 10ms polling for ultra-precision
            
        except KeyboardInterrupt:
            break
        except Exception as e:
            print(f"Error: {e}")
            time.sleep(2)
    
    conn.close()

if __name__ == "__main__":
    executive_loop()
