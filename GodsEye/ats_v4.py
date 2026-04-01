import os
import re
import sys
import time
import threading
from concurrent.futures import ThreadPoolExecutor

# ─── CONFIGURATION ─────────────────────────────────────────────────
SCAN_ROOT = r"C:\\"
LOG_FILE = r"C:\SarahCore\GodsEye\GodsEye_Audit_Log.txt"
SUPPORTED_EXTS = ['.py', '.js', '.ts', '.jsx', '.tsx', '.go', '.rs',
                  '.cpp', '.c', '.h', '.java', '.cs', '.sol']
SIZE_CAP = 512 * 1024 # 512KB per Mandate
NUM_WORKERS = 4      # RAM-Safe Constraint (2.1GB)

# Secret Patterns
SECRET_PATTERNS = {
    'BIP39_Seed_Phrase': re.compile(r'\b(?:[a-z]{3,8}\s){11}[a-z]{3,8}\b'),
    'Hardcoded_API_Key': re.compile(r'(?i)(?:api_key|secret|token|password)[\s\=\:]+[\'"]([a-zA-Z0-9_\-]{16,})[\'"]')
}

# Thread-safe reporting
report_lock = threading.Lock()
bounty_count = 0

def audit_neuron(f_path, log_handle):
    """Synchronous file-worker with gated reporting."""
    global bounty_count
    try:
        with open(f_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            local_bounties = False
            for s_type, pattern in SECRET_PATTERNS.items():
                if pattern.search(content):
                    local_bounties = True
                    with report_lock:
                        bounty_count += 1
                        msg = f"\n[!!] CRITICAL: {s_type} in {f_path}!\n"
                        sys.stdout.write(msg)
                        sys.stdout.flush()
                        log_handle.write(msg)
            return local_bounties
    except:
        return False

def audit_substrate(target):
    """Parallel-Stream Audit for 300GB Substrates."""
    files_indexed = 0
    start_time = time.time()
    
    print(f"[*] Igniting GodsEye Parallel-Stream on {target}...")
    print(f"[*] Calibration: 4-Workers | 512KB Shield | IDE-Safe Pulse\n")

    with open(LOG_FILE, 'w', encoding='utf-8') as log_handle:
        log_handle.write(f"=== GODSEYE PARALLEL-STREAM LOG: {time.ctime()} ===\n")
        
        with ThreadPoolExecutor(max_workers=NUM_WORKERS) as executor:
            stack = [target]
            while stack:
                current_dir = stack.pop()
                try:
                    with os.scandir(current_dir) as it:
                        for entry in it:
                            if entry.is_dir(follow_symlinks=False):
                                stack.append(entry.path)
                            elif entry.is_file(follow_symlinks=False):
                                ext = os.path.splitext(entry.name)[1].lower()
                                if ext in SUPPORTED_EXTS:
                                    files_indexed += 1
                                    
                                    # Stable Pulse Update
                                    if files_indexed % 100 == 0:
                                        elapsed = time.time() - start_time
                                        ips = files_indexed / elapsed if elapsed > 0 else 0
                                        with report_lock:
                                            sys.stdout.write(f"\r[*] PULSE: {files_indexed} Neurons | {ips:.1f} i/s | Loc: {entry.path[-50:]}   ")
                                            sys.stdout.flush()

                                    # Gated Parallel Audit
                                    if entry.stat().st_size < SIZE_CAP:
                                        executor.submit(audit_neuron, entry.path, log_handle)
                except (PermissionError, OSError):
                    continue
    
    elapsed = time.time() - start_time
    print(f"\n\n======================================================================")
    print(f" [ PARALLEL-STREAM COMPLETE ]")
    print(f" [ TOTAL NEURONS SCANNED: {files_indexed} ]")
    print(f" [ TOTAL CRITICAL BOUNTIES: {bounty_count} ]")
    print(f" [ PEAK VELOCITY: {files_indexed / elapsed:.1f} neurons/sec ]")
    print(f" [ ELAPSED TIME: {elapsed:.1f}s ]")
    print(f"======================================================================\n")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else SCAN_ROOT
    audit_substrate(target)
