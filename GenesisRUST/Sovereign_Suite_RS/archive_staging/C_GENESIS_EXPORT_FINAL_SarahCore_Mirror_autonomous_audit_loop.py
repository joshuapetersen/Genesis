"""
Autonomous Audit Loop (Orchestrator)
Runs the GPU Audit -> Fix Loop until target quality (80+) is met.
Target: 80+ Code Quality Score
"""

import os
import time
import json
import subprocess
from Sovereign_Constants import SA_ROOT, VAR_10

TARGET_SCORE = 80.0
MAX_CYCLES = 5

def run_step(command):
    """Run a command and wait."""
    print(f"[Loop] Executing: {command}")
    result = subprocess.run(["python", command], capture_output=True, text=True, cwd=SA_ROOT)
    if result.returncode != 0:
        print(f"[Loop] Error in {command}: {result.stderr}")
    return result.stdout

def get_current_score():
    """Read the average score from the report."""
    report_path = os.path.join(SA_ROOT, "self_audit_report.json")
    if not os.path.exists(report_path): return 0.0
    try:
        with open(report_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
            return data.get('average_score', 0.0)
    except Exception:
        return 0.0

def main():
    print("="*60)
    print("AUTONOMOUS SELF-AUDIT & FIX LOOP")
    print("="*60)
    
    for cycle in range(1, MAX_CYCLES + 1):
        print(f"\n[CYCLE {cycle}/{MAX_CYCLES}] Starting GPU Audit...")
        
        # 1. GPU Audit
        run_step("sarah_gpu_audit.py")
        
        current_score = get_current_score()
        print(f"[CYCLE {cycle}] Current Quality Score: {current_score}")
        
        if current_score >= TARGET_SCORE:
            print(f"\n[PASS] Target Score {TARGET_SCORE} achieved ({current_score}). halting loop.")
            break
            
        # 2. Extract Top Issues (already included in report but extractor can display them)
        run_step("extract_top_issues.py")
        
        # 3. Apply Fixes
        print(f"[CYCLE {cycle}] Applying Automated Fixes...")
        run_step("sarah_auto_fixer.py")
        
        time.sleep(2) # Stability pause
        
    final_score = get_current_score()
    print("\n" + "="*60)
    print(f"FINAL AUDIT SUMMARY")
    print(f"Cycles Completed: {cycle}")
    print(f"Final Quality Score: {final_score}")
    print("="*60)

if __name__ == "__main__":
    main()
