"""
Sarah's Automated Code Fixer - Stabilized V2
Systematically fixes code quality issues with high-precision line handling.
"""
import os
import ast
import re
import multiprocessing as mp
from typing import List, Dict, Optional
from Sovereign_Constants import SA_ROOT, VAR_4

class SarahAutoFixer:
    """Class: SarahAutoFixer"""
    def __init__(self, root_dir=SA_ROOT):
        self.root_dir = root_dir
        self.stats = {
            "files_processed": 0,
            "bare_except_fixed": 0,
            "docstrings_added": 0,
            "total_fixes": 0
        }
        self.skip_dirs = {'vault', '__pycache__', '.git', '.venv', 'node_modules', 'dist', 'build', '.vs', '.vscode', 'wim_mount'}

    def fix_bare_except(self, file_path: str, lines: List[str], tree: ast.AST) -> int:
        """Function: fix_bare_except"""
        fixes = 0
        for node in ast.walk(tree):
            if isinstance(node, ast.Try):
                for handler in node.handlers:
                    if handler.type is None:
                        idx = handler.lineno - 1
                        if idx < len(lines) and "except:" in lines[idx]:
                            indent = len(lines[idx]) - len(lines[idx].lstrip())
                            lines[idx] = " " * indent + "except Exception:  # Auto-fixed bare except"
                            fixes += 1
        return fixes

    def add_docstrings(self, file_path: str, lines: List[str], tree: ast.AST) -> int:
        """Function: add_docstrings"""
        fixes_made = 0
        nodes_to_fix = []
        
        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.ClassDef, ast.AsyncFunctionDef)):
                if not node.name.startswith('_'): # Skip private/magic
                    has_docstring = (
                        node.body and
                        isinstance(node.body[0], ast.Expr) and
                        isinstance(node.body[0].value, (ast.Constant, ast.Str))
                    )
                    if not has_docstring:
                        nodes_to_fix.append(node)
        
        # Sort descending by line number to avoid shifting
        nodes_to_fix.sort(key=lambda n: n.lineno, reverse=True)
        
        for node in nodes_to_fix:
            # We need to find the ':' that ends the def/class header
            # node.lineno might point to a decorator. We scan forward.
            start_search = node.lineno - 1
            found_header = False
            for i in range(start_search, len(lines)):
                if ":" in lines[i]:
                    # Verify it's not a dictionary or something else by checking keywords
                    # This is a heuristic but safe for most Python headers
                    if any(kw in lines[i] for kw in ["def ", "class ", "async def "]):
                        # The docstring goes on the next line
                        indent = len(lines[i]) - len(lines[i].lstrip()) + VAR_4
                        msg = f'"""Function: {node.name}"""' if not isinstance(node, ast.ClassDef) else f'"""Class: {node.name}"""'
                        lines.insert(i + 1, " " * indent + msg)
                        fixes_made += 1
                        found_header = True
                        break
        return fixes_made

    def fix_mutable_defaults(self, file_path: str, lines: List[str], tree: ast.AST) -> int:
        """Function: fix_mutable_defaults"""
        fixes_made = 0
        nodes_to_fix = []
        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                for i, default in enumerate(node.args.defaults):
                    if isinstance(default, (ast.List, ast.Dict, ast.Set)):
                        nodes_to_fix.append((node, i, default))
        
        nodes_to_fix.sort(key=lambda x: x[2].lineno, reverse=True)

        for node, arg_idx, default in nodes_to_fix:
            line_idx = default.lineno - 1
            if line_idx < len(lines):
                target = ""
                if isinstance(default, ast.List) and not default.elts: target = "[]"
                elif isinstance(default, ast.Dict) and not default.keys: target = "{}"
                if target and target in lines[line_idx]:
                    lines[line_idx] = lines[line_idx].replace(target, "None", 1)
                    
                    for i in range(node.lineno, len(lines)):
                        if ":" in lines[i-1]:
                            indent = 0
                            for j in range(i, len(lines)):
                                if lines[j].strip():
                                    indent = len(lines[j]) - len(lines[j].lstrip())
                                    break
                            if indent == 0: indent = (len(lines[i-1]) - len(lines[i-1].lstrip())) + VAR_4
                            
                            arg_name = node.args.args[len(node.args.args) - len(node.args.defaults) + arg_idx].arg
                            init_type = "[]" if isinstance(default, ast.List) else "{}"
                            init_logic = f"if {arg_name} is None: {arg_name} = {init_type}"
                            
                            insert_at = i
                            if i < len(lines) and '"""' in lines[i]:
                                for k in range(i, len(lines)):
                                    if '"""' in lines[k] and k > i:
                                        insert_at = k + 1
                                        break
                            
                            lines.insert(insert_at, " " * indent + init_logic)
                            fixes_made += 1
                            break
        return fixes_made

    def fix_file(self, file_path: str) -> Dict[str, int]:
        """Function: fix_file"""
        fixes = {"bare_except": 0, "docstrings": 0, "mutable_defaults": 0}
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            lines = content.splitlines()
            
            # Pass 1: Mutable Defaults (Shifts lines)
            tree = ast.parse('\n'.join(lines))
            fixes["mutable_defaults"] = self.fix_mutable_defaults(file_path, lines, tree)
            
            # Pass 2: Docstrings (Shifts lines)
            tree = ast.parse('\n'.join(lines))
            fixes["docstrings"] = self.add_docstrings(file_path, lines, tree)
            
            # Pass 3: Bare Excepts (In-place replacement)
            tree = ast.parse('\n'.join(lines))
            fixes["bare_except"] = self.fix_bare_except(file_path, lines, tree)
            
            if sum(fixes.values()) > 0:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write('\n'.join(lines) + '\n')
            return fixes
        except Exception:
            return fixes

    def fix_all(self):
        """Function: fix_all"""
        print(f"[Auto-Fixer] Scanning {self.root_dir}...")
        python_files = []
        for root, dirs, files in os.walk(self.root_dir):
            dirs[:] = [d for d in dirs if d not in self.skip_dirs and not d.startswith('.')]
            for file in files:
                if file.endswith('.py'):
                    python_files.append(os.path.join(root, file))
        
        print(f"[Auto-Fixer] Found {len(python_files)} files. Processing...")
        for f in python_files:
            res = self.fix_file(f)
            self.stats["bare_except_fixed"] += res.get("bare_except", 0)
            self.stats["docstrings_added"] += res.get("docstrings", 0)
            self.stats["mutable_defaults_fixed"] = self.stats.get("mutable_defaults_fixed", 0) + res.get("mutable_defaults", 0)
            self.stats["files_processed"] += 1
        
        self.stats["total_fixes"] = self.stats.get("bare_except_fixed", 0) + self.stats.get("docstrings_added", 0) + self.stats.get("mutable_defaults_fixed", 0)
        print(f"[Auto-Fixer] Complete. Fixed {self.stats['total_fixes']} issues across {self.stats['files_processed']} files.")
        return self.stats

if __name__ == "__main__":
    fixer = SarahAutoFixer()
    fixer.fix_all()
