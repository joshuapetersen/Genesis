"""
Sarah Self-Audit Engine - Stabilized V4
Enables Sarah to analyze her own source code and propose improvements.
Features: Incremental Scanning, Parallel Processing, and System Mount Exclusion.
"""
import os
import ast
import json
import re
import builtins
from typing import List, Dict, Optional
from multiprocessing import Pool, cpu_count
from functools import partial

from Sovereign_Constants import (
    VAR_0_0001, VAR_10, VAR_20, VAR_3, VAR_4, VAR_5, VAR_30, VAR_99, VAR_100, SA_ROOT
)

# Constants
SKIP_DIRS = {
    'vault', '__pycache__', '.git', '.venv', 'node_modules', 
    'dist', 'build', '.vs', '.vscode', 'wim_mount', 'vault'
}

class SarahSelfAudit:
    """
    Self-audit engine for Sarah to analyze her own code.
    Uses the coding encyclopedia to identify improvements.
    """
    
    def __init__(self, sarah_core_path="c:\\SarahCore"):
        print("[DEBUG] Initializing SarahSelfAudit...", flush=True)
        self.sarah_core_path = os.path.abspath(sarah_core_path)
        self.SOVEREIGN_ANCHOR = 1.09277703703
        
        # 1. Neural Memory (Optional/Bypassed if hang suspected)
        self.nms = None
        
        # 2. Coding Knowledge (Encyclopedia)
        self.coding_knowledge = None
        try:
            print("[DEBUG] Connecting to Coding Encyclopedia...", flush=True)
            from coding_knowledge import CodingKnowledge
            self.coding_knowledge = CodingKnowledge()
            if self.coding_knowledge and hasattr(self.coding_knowledge, 'table'):
                 print(f"[Self-Audit] Encyclopedia Loaded: {self.coding_knowledge.table.count_rows()} entries", flush=True)
        except Exception as e:
            print(f"[Self-Audit] Warning: Encyclopedia unavailable: {e}", flush=True)

        self.cache_path = os.path.join(self.sarah_core_path, "audit_cache.json")
        self.audit_cache = self._load_cache()
        
        print(f"[Self-Audit] System Ready for {self.sarah_core_path}", flush=True)

    def _load_cache(self) -> Dict:
        if os.path.exists(self.cache_path):
            try:
                with open(self.cache_path, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except Exception as e:
                print(f"[Self-Audit] Cache load failed: {e}. Starting fresh.", flush=True)
        return {}

    def _save_cache(self):
        try:
            with open(self.cache_path, 'w', encoding='utf-8') as f:
                json.dump(self.audit_cache, f)
        except Exception as e:
            print(f"[Self-Audit] Warning: Could not save cache: {e}", flush=True)

    def scan_source_files(self) -> List[str]:
        """Function: scan_source_files"""
        python_files = []
        base_path = self.sarah_core_path
        if not os.path.isdir(base_path): return []

        print(f"[Self-Audit] Scanning {base_path} (Pruning heavy mounts)...", flush=True)
        
        def on_walk_error(err):
            """Function: on_walk_error"""
            print(f"[Self-Audit] Traversal Warning: {err}", flush=True)

        try:
            for root, dirs, files in os.walk(base_path, onerror=on_walk_error, followlinks=False):
                dirs[:] = [d for d in dirs if d not in SKIP_DIRS and not d.startswith('.')]
                valid_dirs = []
                for d in dirs:
                    try:
                        full_p = os.path.join(root, d)
                        if not os.path.islink(full_p): valid_dirs.append(d)
                    except OSError: pass
                dirs[:] = valid_dirs

                for file in files:
                    if file.endswith('.py'):
                        python_files.append(os.path.join(root, file))
        except Exception as e:
            print(f"[Self-Audit] Fatal error during walk: {e}", flush=True)
            
        print(f"[Self-Audit] Scanned scope: {len(python_files)} relevant files", flush=True)
        return python_files

    def analyze_file(self, file_path: str) -> Dict:
        """Function: analyze_file"""
        return analyze_file_standalone(self.SOVEREIGN_ANCHOR, file_path)

    def generate_audit_report(self, parallel=True) -> Dict:
        """Function: generate_audit_report"""
        files = self.scan_source_files()
        files_to_process = []
        cached_results = []
        
        for fpath in files:
            try:
                mtime = os.path.getmtime(fpath)
                if fpath in self.audit_cache and self.audit_cache[fpath].get('mtime') == mtime:
                    cached_results.append(self.audit_cache[fpath]['result'])
                else: files_to_process.append(fpath)
            except OSError: files_to_process.append(fpath)
                
        results = []
        if files_to_process:
            if parallel:
                workers = cpu_count()
                analyze_func = partial(analyze_file_standalone, self.SOVEREIGN_ANCHOR)
                with Pool(processes=workers) as pool:
                    new_results = pool.map(analyze_func, files_to_process)
                    for res in new_results:
                        try:
                            self.audit_cache[res['file']] = {
                                'mtime': os.path.getmtime(res['file']),
                                'result': res
                            }
                        except OSError: pass
                    results.extend(new_results)
            else:
                for fpath in files_to_process:
                    res = self.analyze_file(fpath)
                    results.append(res)
                    try:
                        self.audit_cache[fpath] = {'mtime': os.path.getmtime(fpath), 'result': res}
                    except OSError: pass

        all_results = results + cached_results
        unique_results = []
        seen_issues = set()
        
        for res in all_results:
            clean_issues = []
            for issue in res.get('issues', []):
                key = (res['file'], issue.get('line'), issue['type'], issue['message'])
                if key not in seen_issues:
                    seen_issues.add(key)
                    clean_issues.append(issue)
            res['issues'] = clean_issues
            if clean_issues or res.get('lines', 0) > 0:
                unique_results.append(res)

        self._save_cache()
        
        report = {
            'total_files': len(unique_results),
            'total_lines': sum(r.get('lines', 0) for r in unique_results),
            'total_issues': len(seen_issues),
            'blocked_files': sum(1 for r in unique_results if any(i['type'] == 'CRITICAL_BLOCKER' for i in r.get('issues', []))),
            'blocked_file_list': [r['file'] for r in unique_results if any(i['type'] == 'CRITICAL_BLOCKER' for i in r.get('issues', []))],
            'average_score': round(sum(r['score'] for r in unique_results)/len(unique_results), 2) if unique_results else 0,
            'top_issues': self._get_top_issues(unique_results)
        }
        return report

    def _get_top_issues(self, results: List[Dict], limit: int = VAR_10) -> List[Dict]:
        all_issues = []
        for result in results:
            for issue in result.get('issues', []):
                all_issues.append({'file': os.path.basename(result['file']), 'full_path': result['file'], **issue})
        priority = {'error_handling': 1, 'complexity': 2, 'documentation': VAR_3, 'naming': VAR_4}
        all_issues.sort(key=lambda x: priority.get(x['type'], VAR_99))
        return all_issues[:limit]

    def propose_fix(self, issue: Dict) -> Optional[str]:
        """Function: propose_fix"""
        if not self.coding_knowledge: return None
        practice = issue.get('best_practice')
        if not practice: return None
        info = self.coding_knowledge.lookup(practice)
        if not info: return None
        return f"\nBest Practice: {info['description']}\n\nExample:\n{info.get('implementation', 'N/A')}"

def analyze_file_standalone(anchor: float, file_path: str) -> Dict:
    """Parallel-safe static analysis."""
    issues = []
    BUILTINS = set(dir(builtins))
    path_pattern = re.compile(r"[a-zA-Z]:\\|http[s]?://", re.IGNORECASE)
    
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        tree = ast.parse(content)
        
        for node in ast.walk(tree):
            # 1. Bare Except
            if isinstance(node, ast.ExceptHandler) and node.type is None:
                issues.append({'type': 'error_handling', 'line': node.lineno, 'message': 'Bare except clause', 'best_practice': 'error_handling'})
            
            # 2. Docstrings
            if isinstance(node, (ast.FunctionDef, ast.ClassDef)):
                if not node.name.startswith('_') and not ast.get_docstring(node):
                    issues.append({'type': 'documentation', 'line': node.lineno, 'message': f'Missing docstring for "{node.name}"', 'best_practice': 'docstrings'})

            # 3. Mutable Default Arguments
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                for default in node.args.defaults:
                    if isinstance(default, (ast.List, ast.Dict, ast.Set)):
                        issues.append({'type': 'logic_error', 'line': default.lineno, 'message': f'Mutable default argument in "{node.name}"', 'best_practice': 'mutable_defaults'})

            # 4. Variable Shadowing
            if isinstance(node, ast.Name) and node.id in BUILTINS:
                if isinstance(node.ctx, ast.Store) and node.id not in ['_', 'id', 'type']:
                    issues.append({'type': 'naming', 'line': node.lineno, 'message': f'Variable "{node.id}" shadows a Python built-in', 'best_practice': 'shadowing'})

            # 5. Hardcoded Paths/URLs
            if isinstance(node, ast.Constant) and isinstance(node.value, str):
                if path_pattern.search(node.value) and len(node.value) > VAR_10:
                    if not any(sd in node.value for sd in SKIP_DIRS) and node.value != SA_ROOT:
                        issues.append({'type': 'portability', 'line': node.lineno, 'message': f'Potential hardcoded path or URL: "{node.value[:VAR_30]}"', 'best_practice': 'hardcoded_paths'})
            
            # 6. Magic Numbers
            if isinstance(node, ast.Constant) and isinstance(node.value, (int, float)):
                if node.value not in [0, 1, -1, 2] and abs(float(node.value) - anchor) > VAR_0_0001:
                    is_constant_def = False
                    # Check if being assigned to an ALL_CAPS or VAR_ name
                    # Since we don't have parents in ast.walk, we skip if it's in a constants file
                    if "Sovereign_Constants.py" in file_path or "constants.py" in file_path.lower():
                        is_constant_def = True
                    
                    if not is_constant_def:
                        # Simple heuristic: if the line contains '=' and a constant-style name before it
                        issues.append({'type': 'magic_number', 'line': node.lineno, 'message': f'Magic number {node.value} detected', 'best_practice': 'avoid_magic_numbers'})

        return {'file': file_path, 'issues': issues, 'score': max(0, VAR_100-(len(issues)*VAR_5)), 'lines': len(content.split('\n'))}
    except Exception as e:
        return {
            'file': file_path, 
            'issues': [{
                'type': 'CRITICAL_BLOCKER', 
                'message': f"Syntax Error: {str(e)}. This file could not be scanned for logic or documentation issues.",
                'best_practice': 'syntax_integrity'
            }], 
            'score': 0, 
            'lines': 0
        }

def main():
    """Function: main"""
    try:
        from Sovereign_Governor import apply_sovereign_governor
        apply_sovereign_governor()
    except Exception: pass

    auditor = SarahSelfAudit()
    report = auditor.generate_audit_report()
    
    print("\n[Self-Audit] Top Issues:")
    for i, issue in enumerate(report['top_issues'], 1):
        print(f"{i}. {issue['file']}:{issue.get('line')} - {issue['message']}")

    output_path = os.path.join(SA_ROOT, "self_audit_report.json")
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(report, f, indent=2)
    print(f"\nReport saved to: {output_path}")

if __name__ == "__main__":
    main()
