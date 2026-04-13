"""
GODSEYE 3.1 — SOVEREIGN FUSION
================================================================
The Final Evolution. 
Fuses the Jet Engine (v2.5) with the Neural Dissector (v3.0) 
under the Sovereign Neural HAL (v3.1).

"We CREATE, never rewrite."
"""

import os
import sys
import re
import time
import hashlib
import concurrent.futures
from collections import defaultdict, deque

# Import specialized Neural HAL
from Sovereign_Neural_HAL import SovereignNeuralHAL

# Configuration
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
SCAN_ROOT = r"C:\GENESIS"
MODEL_PATH = r"C:\GENESIS\.lmstudio\models\mradermacher\MobileLLM-125M-HF-GGUF\MobileLLM-125M-HF.Q8_0.gguf"
OUTPUT_MD = os.path.join(SCRIPT_DIR, 'godseye_v3_fusion_report.md')

# VULNERABILITY SIGNATURES [Manifested from v2.5]
VULN_SIGNATURES = {
    'SQL_INJECTION': [
        r'execute\s*\(\s*f["\']', 
        r'execute\s*\(\s*["\'][^"\']*%s', 
        r'cursor\.execute\s*\(\s*[^,)]+\+', 
        r'\.query\s*\(\s*["\'][^"\']*\+\s*\w+', 
        r'\.raw\s*\(\s*f["\']'
    ],
    'COMMAND_INJECTION': [r'os\.system\s*\(', r'subprocess\.\w+\s*\([^)]*shell\s*=\s*True', r'exec\s*\(\s*[^)]*\+', r'eval\s*\(\s*[^)]*\+', r'child_process\.exec\s*\(', r'Runtime\.getRuntime\(\)\.exec\s*\(', r'system\s*\(\s*[^)]*\+'],
    'PATH_TRAVERSAL': [r'open\s*\([^)]*\+[^)]*\)', r'readFile\w*\s*\([^)]*\+', r'\.\./', r'\.\.\\\\'],
    'HARDCODED_SECRET': [r'(?i)(?:password|passwd|pwd)\s*=\s*["\'][^"\']{8,}["\']', r'(?i)(?:api_key|apikey|secret_key|access_key)\s*=\s*["\'][^"\']{16,}["\']', r'(?i)(?:token|auth_token|bearer)\s*=\s*["\'][A-Za-z0-9_\-\.]{20,}["\']', r'-----BEGIN PRIVATE KEY-----'],
    'INSECURE_DESERIALIZE': [r'pickle\.loads?\s*\(', r'yaml\.load\s*\([^)]*(?!Loader)', r'unserialize\s*\(', r'JSON\.parse\s*\(\s*\w+\)']
}

def intake_filter(path):
    """Exclude sandboxed garbage from the scan."""
    skip = ['.venv', '.git', '__pycache__', '.lmstudio']
    for s in skip:
        if s in path: return True
    return False

def live_intake_fan(target_dir):
    """Yields repository files while filtering out sandbox pollution."""
    for root, dirs, files in os.walk(target_dir):
        if intake_filter(root): continue
        for f in files:
            yield os.path.join(root, f), f, os.path.splitext(f)[1].lower()

class SovereignFusionEngine:
    def __init__(self, model_path):
        print(f"\n[+] IGNITING GODSEYE 3.1 FUSION CORE ...")
        self.hal = SovereignNeuralHAL(model_path)
        self.results = []
        self.vulns_found = 0

    def _neural_combustion(self, full_path, filename, ext, file_size):
        """Fusion Combustion: Combines classical Regex with Neural Stress mapping."""
        info = {
            'path': full_path, 'filename': filename, 'ext': ext, 'size': file_size,
            'neural_stress': 0.0, 'vulnerabilities': []
        }

        try:
            # 1. Binary payload detection
            with open(full_path, 'rb') as f_bin:
                header = f_bin.read(4)
                if header.startswith(b'MZ') or header.startswith(b'\x7fELF'):
                    info['vulnerabilities'].append({'type': 'BINARY_PAYLOAD', 'evidence': 'Native binary.'})
                    return info

            # 2. Text Analysis
            with open(full_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read(1048576) # 1MB limit

                # A. Classical Vulnerability Hunt [v2.5 Engine Legacy]
                for v_type, patterns in VULN_SIGNATURES.items():
                    for pat in patterns:
                        m = re.search(pat, content)
                        if m:
                            line = content[:m.start()].count('\n') + 1
                            evidence = content.split('\n')[line-1].strip()[:80]
                            info['vulnerabilities'].append({'type': v_type, 'line': line, 'evidence': evidence})
                            break

                # B. Neural Harmonic Stress [v3.0 Engine Evolution]
                h_vec = [int(c, 16) for c in hashlib.sha384(content[:4096].encode()).hexdigest()]
                stress = 0.0
                limit = min(self.hal.neural_heads, len(h_vec))
                for i in range(limit):
                    stress += (h_vec[i] / 15.0) * (self.hal.neural_layers / self.hal.neural_heads)
                info['neural_stress'] = round(stress / self.hal.neural_heads, 4)

        except Exception as e:
            info['vulnerabilities'].append({'type': 'ERROR', 'evidence': str(e)})
            
        return info

    def run_sweep(self, target_dir):
        start = time.time()
        print(f"\n[+] STREAMING SOVEREIGN FUSION ACROSS {target_dir}")
        print("="*70)
        
        combust_results = []
        with concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count() * 2) as executor:
            futures = set()
            idx = 0
            
            for fp, fn, ext in live_intake_fan(target_dir):
                futures.add(executor.submit(self._neural_combustion, fp, fn, ext, os.path.getsize(fp)))
                
                done = {f for f in futures if f.done()}
                for f in done:
                    try:
                        res = f.result()
                        combust_results.append(res)
                        idx += 1
                        if res['vulnerabilities'] and res['vulnerabilities'][0]['type'] != 'BINARY_PAYLOAD':
                            print(f"  [!! VULN ] {res['filename']:30s} | {res['vulnerabilities'][0]['type']}")
                        elif idx % 200 == 0:
                            print(f"  ... [JET STREAM] {idx} files fused ...")
                    except: pass
                futures.difference_update(done)
                
            for f in concurrent.futures.as_completed(futures):
                try: combust_results.append(f.result())
                except: pass

        elapsed = time.time() - start
        self.generate_report(combust_results, elapsed)

    def generate_report(self, results, elapsed):
        with open(OUTPUT_MD, "w", encoding="utf-8") as f:
            f.write("# GodsEye 3.1 Sovereign Fusion Report\n")
            f.write(f"> **Engine Runtime:** {elapsed:.2f}s | **Files Cleaned:** {len(results)}\n\n")
            
            f.write("## Critical Security Findings\n")
            f.write("| File | Vuln Type | Evidence | Neural Stress |\n")
            f.write("| :--- | :--- | :--- | :--- |\n")
            
            for r in sorted(results, key=lambda x: len(x['vulnerabilities']), reverse=True):
                if r['vulnerabilities']:
                    for v in r['vulnerabilities']:
                        line_info = f" (Line {v.get('line','?')})" if 'line' in v else ""
                        f.write(f"| `{r['filename']}` | **{v['type']}** | {v['evidence']}{line_info} | {r['neural_stress']} |\n")

        print(f"\n[SUCCESS] Final Sovereign Fusion Report Seated: {OUTPUT_MD}")

if __name__ == "__main__":
    if not os.path.exists(MODEL_PATH):
        print(f"[ERROR] Substrate missing at {MODEL_PATH}. Check your mirror.")
        sys.exit(1)
        
    engine = SovereignFusionEngine(MODEL_PATH)
    engine.run_sweep(SCAN_ROOT)
