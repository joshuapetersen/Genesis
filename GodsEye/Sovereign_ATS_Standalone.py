"""
Sovereign Automated Topology Scanner (ATS) Standalone v2.0
[200+ Line Refined Kernel]
Identifies Filename, Purpose, Logic-In, Logic-Out, Dead-Ends, Dependencies, and Resource Linkages.
Designed for 1.66M line ecosystem auditing. 宣
"""

import os
import ast
import json
import sys
import datetime
import math

class SovereignATS:
    def __init__(self, target_dir):
        self.target_dir = os.path.abspath(target_dir)
        self.topology = {
            "Manifest": {
                "Engine": "Sovereign_ATS_Standalone_v2.0",
                "Timestamp": datetime.datetime.now().isoformat(),
                "Target": self.target_dir
            },
            "Global_Metrics": {
                "Total_Files": 0,
                "Total_Functions": 0,
                "Total_Classes": 0,
                "Dead_Ends": 0,
                "Complexity_Avg": 0.0
            },
            "Neurons": {}
        }
        self.all_complexities = []

    def calculate_complexity(self, node):
        """Calculates cyclomatic complexity of a given AST node (Functions/Methods)."""
        complexity = 1
        for child in ast.walk(node):
            if isinstance(child, (ast.If, ast.While, ast.For, ast.And, ast.Or, ast.ExceptHandler)):
                complexity += 1
        return complexity

    def audit_neuron(self, file_path):
        """Audits a single Python file for deep topology metadata."""
        relative_path = os.path.relpath(file_path, self.target_dir)
        neuron_map = {
            "Filename": os.path.basename(file_path),
            "Purpose": "Undefined",
            "Logic_In": [],   # Arguments/Inputs
            "Logic_Out": [],  # Return Types/Objects
            "Dead_Ends": [],  # Uncalled internal functions
            "Dependencies": [],# Imports
            "Resource_Linkages": [], # Strings matching file patterns
            "Complexity_Score": 0,
            "Internal_Bridge": False # Checks for C++ linkages
        }

        try:
            with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
                content = f.read()
                tree = ast.parse(content)

            # Extract Purpose from Module Docstring
            docstring = ast.get_docstring(tree)
            if docstring:
                neuron_map["Purpose"] = docstring.split('\n')[0]

            defined_funcs = set()
            called_funcs = set()
            
            for node in ast.walk(tree):
                # 1. Logic In / Logic Out / Complexity
                if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    defined_funcs.add(node.name)
                    # Logic In: Arguments
                    args = [a.arg for a in node.args.args]
                    neuron_map["Logic_In"].extend(args)
                    
                    # Logic Out: Return Analysis
                    for sub_node in ast.walk(node):
                        if isinstance(sub_node, ast.Return) and sub_node.value:
                            neuron_map["Logic_Out"].append(type(sub_node.value).__name__)
                    
                    # Complexity
                    c = self.calculate_complexity(node)
                    neuron_map["Complexity_Score"] += c
                    self.all_complexities.append(c)

                # 2. Dependencies
                if isinstance(node, ast.Import):
                    for alias in node.names:
                        neuron_map["Dependencies"].append(alias.name)
                elif isinstance(node, ast.ImportFrom):
                    neuron_map["Dependencies"].append(node.module or "relative")

                # 3. Identify function calls for Dead-End detection
                if isinstance(node, ast.Call):
                    if isinstance(node.func, ast.Name):
                        called_funcs.add(node.func.id)
                    elif isinstance(node.func, ast.Attribute):
                        called_funcs.add(node.func.attr)

                # 4. Resource Linkages & C++ Bridges
                if isinstance(node, ast.Constant) and isinstance(node.value, str):
                    if any(ext in node.value for ext in [".json", ".bin", ".dat", ".txt", ".png", ".jpg"]):
                        neuron_map["Resource_Linkages"].append(node.value)
                    if any(ext in node.value for ext in [".cpp", ".h", ".dll", ".so"]):
                        neuron_map["Internal_Bridge"] = True
                        neuron_map["Resource_Linkages"].append(node.value)

            # Identity Dead Ends: Defined but never called in the same module
            neuron_map["Dead_Ends"] = list(defined_funcs - called_funcs)
            
            # Final Parameter Formatting
            neuron_map["Logic_In"] = list(set(neuron_map["Logic_In"]))
            neuron_map["Logic_Out"] = list(set(neuron_map["Logic_Out"]))
            neuron_map["Dependencies"] = list(set(neuron_map["Dependencies"]))
            neuron_map["Resource_Linkages"] = list(set(neuron_map["Resource_Linkages"]))
            
            return neuron_map

        except Exception as e:
            return {"Error": f"Audit Failed: {str(e)}"}

    def scan_ecosystem(self):
        """Recursively scans the target directory and maps the total topology."""
        print(f"[IGNITION] Firing ATS Pulse: {self.target_dir}")
        for root, _, files in os.walk(self.target_dir):
            for file in files:
                if file.endswith(".py"):
                    full_path = os.path.join(root, file)
                    rel_path = os.path.relpath(full_path, self.target_dir)
                    
                    self.topology["Global_Metrics"]["Total_Files"] += 1
                    neuron_map = self.audit_neuron(full_path)
                    
                    if "Error" not in neuron_map:
                        self.topology["Neurons"][rel_path] = neuron_map
                        self.topology["Global_Metrics"]["Total_Functions"] += len(neuron_map.get("Logic_In", []))
                        self.topology["Global_Metrics"]["Dead_Ends"] += len(neuron_map.get("Dead_Ends", []))
                    
                    print(f"  [PULSE] Mapping Neuron: {rel_path}")

        if self.all_complexities:
            self.topology["Global_Metrics"]["Complexity_Avg"] = sum(self.all_complexities) / len(self.all_complexities)

        # Generate Final Report
        report_path = os.path.join(self.target_dir, "ats_topology_report_v2.md")
        self.generate_report(report_path)
        
        # Save JSON Manifest
        json_path = os.path.join(self.target_dir, "ats_manifest_v2.json")
        with open(json_path, "w") as f:
            json.dump(self.topology, f, indent=4)
        
        print(f"\n[SEALED] Audit Complete. Report: {report_path}")

    def generate_report(self, report_path):
        """Constructs the feature-complete Markdown report."""
        with open(report_path, "w", encoding="utf-8") as f:
            f.write(f"# Sovereign ATS Topology Report (v2.0)\n")
            f.write(f"Generated: {self.topology['Manifest']['Timestamp']}\n")
            f.write(f"Target: {self.topology['Manifest']['Target']}\n\n")
            
            f.write(f"## Global Metrics\n")
            f.write(f"- **Total Python Neurons**: {self.topology['Global_Metrics']['Total_Files']}\n")
            f.write(f"- **Total Dead Ends**: {self.topology['Global_Metrics']['Dead_Ends']}\n")
            f.write(f"- **Average Complexity**: {self.topology['Global_Metrics']['Complexity_Avg']:.2f}\n\n")
            
            f.write(f"## Neuron Topology Table\n")
            f.write(f"| Filename | Purpose | Logic-In | Dead-Ends | Bridge |\n")
            f.write(f"| :--- | :--- | :--- | :--- | :--- |\n")
            
            for rel_path, data in self.topology["Neurons"].items():
                bridge = " [YES]" if data.get("Internal_Bridge") else " [NO]"
                logic_in = ", ".join(data.get("Logic_In", []))[:30] + "..." if len(data.get("Logic_In", [])) > 0 else "None"
                dead = len(data.get("Dead_Ends", []))
                f.write(f"| {data['Filename']} | {data['Purpose'][:40]} | {logic_in} | {dead} | {bridge} |\n")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "."
    scanner = SovereignATS(target)
    scanner.scan_ecosystem()
