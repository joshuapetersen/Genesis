import os
import sys

# Add directory to path so we can import local files
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from GodsEye_Accelerator import JetEngineAccelerator
from GodsEye_2_1_Kernel import hunt_vulnerabilities, SUPPORTED_EXTS

def mock_baseline_parser(content, file_path):
    """A lightweight mock of the Phase 2 import parser."""
    return set()

def live_intake_fan(target_dir):
    """The Intake. Yields files continuously as the OS finds them."""
    for root, dirs, files in os.walk(target_dir):
        # Prevent PermissionErrors from crashing the walk
        try:
            for f in files:
                ext = os.path.splitext(f)[1].lower()
                if ext in SUPPORTED_EXTS:
                    yield os.path.join(root, f)
        except PermissionError:
            continue

def main():
    target = sys.argv[1] if len(sys.argv) > 1 else r"C:\Users\drago\AppData\Local\Programs\Antigravity"
    print(f"\n[*] IGNITING JET ENGINE TEST ON: {target}")
    print("[*] Architecture: Threads maxed. Streaming Exhaust.\n")
    
    engine = JetEngineAccelerator()
    
    # Connect Intake Fan to the Engine
    thrust_stream = engine.stream_ignition(
        live_intake_fan(target), 
        ast_parser=mock_baseline_parser, 
        vuln_hunter=hunt_vulnerabilities
    )
    
    files_combusted = 0
    anomalies = 0
    
    # The output streams INSTANTLY. No blocking arrays.
    for file_path, intel in thrust_stream:
        files_combusted += 1
        
        # We only print the high-priority thrust right now to keep console clean
        if intel['vulns'] or intel['is_malicious']:
            anomalies += 1
            filename = os.path.basename(file_path)
            
            if intel['is_malicious']:
                print(f"  [!! EXHAUST WARN] {filename:25s} | MALICIOUS PAYLOAD DETECTED!")
            else:
                vulns = ", ".join([v['type'] for v in intel['vulns']])
                print(f"  [!! EXHAUST WARN] {filename:25s} | VULNS: {vulns[:50]}")
                
        if files_combusted % 2000 == 0:
            print(f"  ... [COMPRESSOR] {files_combusted} files pulled through chamber ...")

    print("\n" + "="*50)
    print(" ENGINE SPINDOWN - FLIGHT LOG")
    print("="*50)
    
    brief = engine.generate_intelligence_brief()
    print(f" Total Combusted: {brief['total_files_combusted']} files")
    print(f" Truth Density:   {brief['truth_density_score']}")
    print(f" Matrix Anomalies:{brief['critical_anomalies']}")

if __name__ == '__main__':
    main()
