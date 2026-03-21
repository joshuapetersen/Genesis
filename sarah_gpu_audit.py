"""
Sarah GPU-Accelerated Audit Engine (V5)
Utilizes torch/CUDA for high-speed code scanning and quality score calculation.
Optimized for the Sovereign 3+1 Architecture.
"""

import os
import ast
import json
import re
import time
import torch
import numpy as np
from typing import List, Dict, Optional
from multiprocessing import Pool, cpu_count
from functools import partial

from Sovereign_Constants import (
    VAR_0_0001, VAR_10, VAR_20, VAR_3, VAR_4, VAR_5, VAR_30, VAR_60, VAR_99, VAR_100, SA_ROOT, SOVEREIGN_ANCHOR
)

# Constants
SKIP_DIRS = {
    'vault', '__pycache__', '.git', '.venv', 'node_modules', 
    'dist', 'build', '.vs', '.vscode', 'wim_mount', 'vault'
}

class SarahGPUAudit:
    """
    GPU-Accelerated self-audit engine for Sarah.
    Uses torch tensors for parallelized quality scoring.
    """
    
    def __init__(self, sarah_core_path=SA_ROOT):
        print("[GPU-Audit] Initializing Engine...", flush=True)
        self.sarah_core_path = os.path.abspath(sarah_core_path)
        self.device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
        print(f"[GPU-Audit] Target Device: {self.device}")
        
        self.cache_path = os.path.join(self.sarah_core_path, "audit_cache.json")
        self.report_path = os.path.join(self.sarah_core_path, "self_audit_report.json")
        
        # Load Coding Knowledge if available
        self.coding_knowledge = None
        try:
            from coding_knowledge import CodingKnowledge
            self.coding_knowledge = CodingKnowledge()
        except Exception:
            print("[GPU-Audit] Warning: Encyclopedia unavailable.")

    def scan_files(self) -> List[str]:
        """Scan directory for Python files."""
        python_files = []
        for root, dirs, files in os.walk(self.sarah_core_path):
            dirs[:] = [d for d in dirs if d not in SKIP_DIRS and not d.startswith('.')]
            for file in files:
                if file.endswith('.py'):
                    python_files.append(os.path.join(root, file))
        return python_files

    def calculate_score_tensor(self, issues_counts: List[int]) -> float:
        """
        Uses GPU to calculate mass average of scores.
        Score = max(0, 100 - (issues * 5))
        """
        if not issues_counts: return 0.0
        
        t_counts = torch.tensor(issues_counts, dtype=torch.float32, device=self.device)
        t_scores = torch.clamp(VAR_100 - (t_counts * VAR_5), min=0.0)
        
        avg_score = torch.mean(t_scores).item()
        return round(avg_score, 2)

    def analyze_codebase(self) -> Dict:
        """Full scan and report generation."""
        start_time = time.time()
        files = self.scan_files()
        print(f"[GPU-Audit] Found {len(files)} files pulse-checking...")
        
        # We leverage parallel CPU scanning for AST analysis (hard to do on GPU)
        # But we use GPU for the aggregate analytics
        workers = cpu_count()
        from sarah_self_audit import analyze_file_standalone
        
        analyze_func = partial(analyze_file_standalone, SOVEREIGN_ANCHOR)
        
        with Pool(processes=workers) as pool:
            file_results = pool.map(analyze_func, files)
            
        # Collect issue counts for GPU acceleration
        issues_counts = [len(res.get('issues', [])) for res in file_results]
        avg_score = self.calculate_score_tensor(issues_counts)
        
        # Extract top 10 high-priority issues
        all_issues = []
        for res in file_results:
            for issue in res.get('issues', []):
                all_issues.append({
                    'file': os.path.basename(res['file']),
                    'full_path': res['file'],
                    **issue
                })
        
        # Priority mapping
        priority = {'error_handling': 1, 'complexity': 2, 'documentation': VAR_3, 'naming': VAR_4, 'magic_number': VAR_5}
        all_issues.sort(key=lambda x: priority.get(x['type'], VAR_99))
        top_issues = all_issues[:VAR_10]
        
        report = {
            'timestamp': time.time(),
            'total_files': len(files),
            'total_lines': sum(res.get('lines', 0) for res in file_results),
            'total_issues': sum(issues_counts),
            'average_score': avg_score,
            'top_issues': top_issues,
            'scan_time_seconds': round(time.time() - start_time, 2),
            'gpu_accelerated': torch.cuda.is_available(),
            'file_results': file_results # Required for extractor
        }
        
        with open(self.report_path, 'w', encoding='utf-8') as f:
            json.dump(report, f, indent=2)
            
        print(f"[GPU-Audit] Audit Complete in {report['scan_time_seconds']}s. Score: {avg_score}")
        return report

if __name__ == "__main__":
    auditor = SarahGPUAudit()
    auditor.analyze_codebase()
