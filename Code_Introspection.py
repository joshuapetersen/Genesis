import os
import json
import hashlib
from datetime import datetime

class CodeIntrospection:
    """
    Sarah's Self-Awareness Module.
    Allows the system to analyze its own code structure, count lines, and report status.
    """
    
    def __init__(self, core_dir=None):
        self.core_dir = core_dir or os.path.dirname(os.path.abspath(__file__))
        self.introspection_log = os.path.join(self.core_dir, "introspection_log.jsonl")
        
    def analyze_file(self, filename):
        """
        Analyzes a Python file and returns detailed metrics.
        """
        filepath = os.path.join(self.core_dir, filename)
        
        if not os.path.exists(filepath):
            return {"error": f"File not found: {filename}"}
        
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            # Calculate metrics
            total_lines = len(lines)
            code_lines = sum(1 for line in lines if line.strip() and not line.strip().startswith('#'))
            comment_lines = sum(1 for line in lines if line.strip().startswith('#'))
            blank_lines = total_lines - code_lines - comment_lines
            
            # Calculate file hash for change detection
            file_hash = self._calculate_hash(filepath)
            
            # Count functions and classes
            functions = sum(1 for line in lines if line.strip().startswith('def '))
            classes = sum(1 for line in lines if line.strip().startswith('class '))
            
            analysis = {
                "filename": filename,
                "timestamp": datetime.now().isoformat(),
                "filepath": filepath,
                "metrics": {
                    "total_lines": total_lines,
                    "code_lines": code_lines,
                    "comment_lines": comment_lines,
                    "blank_lines": blank_lines,
                    "functions": functions,
                    "classes": classes,
                    "code_ratio": round(code_lines / total_lines * 100, 2) if total_lines > 0 else 0
                },
                "file_hash": file_hash,
                "size_bytes": os.path.getsize(filepath)
            }
            
            # Log introspection
            self._log_introspection(analysis)
            
            return analysis
            
        except Exception as e:
            return {"error": str(e)}
    
    def analyze_all_core(self):
        """
        Analyzes all Python files in the core directory.
        """
        py_files = [f for f in os.listdir(self.core_dir) if f.endswith('.py')]
        
        results = []
        total_stats = {
            "total_lines": 0,
            "total_code_lines": 0,
            "total_functions": 0,
            "total_classes": 0
        }
        
        for filename in sorted(py_files):
            analysis = self.analyze_file(filename)
            if "error" not in analysis:
                results.append(analysis)
                total_stats["total_lines"] += analysis["metrics"]["total_lines"]
                total_stats["total_code_lines"] += analysis["metrics"]["code_lines"]
                total_stats["total_functions"] += analysis["metrics"]["functions"]
                total_stats["total_classes"] += analysis["metrics"]["classes"]
        
        summary = {
            "timestamp": datetime.now().isoformat(),
            "files_analyzed": len(results),
            "aggregate": total_stats,
            "modules": results
        }
        
        return summary
    
    def get_code_signature(self, filename):
        """
        Returns a signature (hash) of a file's content.
        Used to detect if code has changed.
        """
        filepath = os.path.join(self.core_dir, filename)
        if os.path.exists(filepath):
            return self._calculate_hash(filepath)
        return None
    
    def compare_versions(self, filename, old_hash):
        """
        Compares current file hash against a previous version.
        Returns True if changed, False if identical.
        """
        current_hash = self.get_code_signature(filename)
        return current_hash != old_hash if current_hash else False
    
    def _calculate_hash(self, filepath):
        """
        Calculates SHA256 hash of a file.
        """
        sha256_hash = hashlib.sha256()
        try:
            with open(filepath, "rb") as f:
                for byte_block in iter(lambda: f.read(4096), b""):
                    sha256_hash.update(byte_block)
            return sha256_hash.hexdigest()
        except:
            return None
    
    def _log_introspection(self, analysis):
        """
        Logs introspection results to JSONL file.
        """
        try:
            with open(self.introspection_log, 'a') as f:
                f.write(json.dumps(analysis) + "\n")
        except Exception as e:
            print(f"[Introspection] Failed to log: {e}")
    
    def get_introspection_history(self, filename=None, limit=10):
        """
        Retrieves historical introspection data.
        """
        if not os.path.exists(self.introspection_log):
            return []
        
        history = []
        try:
            with open(self.introspection_log, 'r') as f:
                for line in f.readlines()[-limit:]:
                    entry = json.loads(line)
                    if filename is None or entry.get("filename") == filename:
                        history.append(entry)
        except Exception as e:
            print(f"[Introspection] Failed to read history: {e}")
        
        return history


if __name__ == "__main__":
    introspect = CodeIntrospection()
    
    # Analyze all core modules
    summary = introspect.analyze_all_core()
    
    print("[SARAH CODE INTROSPECTION REPORT]")
    print(f"Files Analyzed: {summary['files_analyzed']}")
    print(f"Total Lines: {summary['aggregate']['total_lines']}")
    print(f"Total Code Lines: {summary['aggregate']['total_code_lines']}")
    print(f"Total Functions: {summary['aggregate']['total_functions']}")
    print(f"Total Classes: {summary['aggregate']['total_classes']}")
    print("\n[MODULE BREAKDOWN]")
    for module in summary['modules']:
        m = module['metrics']
        print(f"  {module['filename']:30s} | {m['total_lines']:5d} lines | {m['functions']:3d} func | {m['classes']:3d} class | {m['code_ratio']:5.1f}% code")
