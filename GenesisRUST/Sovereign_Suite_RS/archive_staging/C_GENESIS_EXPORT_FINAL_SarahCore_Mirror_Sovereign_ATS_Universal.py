"""
Sovereign Universal Mesh ATS (v5.0)
[Absolute Multi-Root Topology Auditor]
Maps Filename, Purpose, Logic-In, Logic-Out, Dead-Ends, Dependencies, and Resource Linkages across ALL roots.
Bridging SarahCore, DPM_Engine, and Genlex. 宣
"""

import os
import ast
import json
import sys
from datetime import datetime

class SovereignMeshATS:
    def __init__(self, roots):
        self.roots = [os.path.abspath(r) for r in roots]
        self.mesh = {
            "Manifest": {
                "Engine": "Sovereign_Universal_Mesh_ATS_v5.0",
                "Timestamp": datetime.now().isoformat(),
                "Roots": self.roots
            },
            "Global_Metrics": {
                "Total_Files": 0,
                "Total_Functions": 0,
                "Roots_Scanned": len(self.roots),
                "Mesh_Density": 0.0
            },
            "Neurons": {}
        }
        self.call_graph = {}

    def analyze_neuron(self, file_path, root_name):
        """Deep AST Audit of a single Python neuron."""
        neuron_id = f"{root_name}:{os.path.basename(file_path)}"
        neuron_map = {
            "Root": root_name,
            "Filename": os.path.basename(file_path),
            "Path": file_path,
            "Purpose": "Undefined",
            "Logic_In": [],
            "Logic_Out": [],
            "Dead_Ends": [],
            "Dependencies": [],
            "Resource_Linkages": [],
            "Complexity": 0
        }

        try:
            with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
                tree = ast.parse(f.read())

            doc = ast.get_docstring(tree)
            if doc: neuron_map["Purpose"] = doc.split('\n')[0]

            def_funcs = set()
            call_funcs = set()

            for node in ast.walk(tree):
                if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    def_funcs.add(node.name)
                    neuron_map["Logic_In"].extend([a.arg for a in node.args.args])
                    # Complexity calculation
                    neuron_map["Complexity"] += 1 + sum(1 for c in ast.walk(node) if isinstance(c, (ast.If, ast.For, ast.While, ast.And, ast.Or)))
                    
                    for sub in ast.walk(node):
                        if isinstance(sub, ast.Return) and sub.value:
                            neuron_map["Logic_Out"].append(type(sub.value).__name__)

                if isinstance(node, (ast.Import, ast.ImportFrom)):
                    if isinstance(node, ast.Import):
                        neuron_map["Dependencies"].extend([n.name for n in node.names])
                    else:
                        neuron_map["Dependencies"].append(node.module or "relative")

                if isinstance(node, ast.Call):
                    if isinstance(node.func, ast.Name): call_funcs.add(node.func.id)
                    elif isinstance(node.func, ast.Attribute): call_funcs.add(node.func.attr)

                if isinstance(node, ast.Constant) and isinstance(node.value, str):
                    if any(ext in node.value for ext in [".json", ".bin", ".dat", ".cpp", ".h", ".dll", ".py"]):
                        neuron_map["Resource_Linkages"].append(node.value)

            neuron_map["Dead_Ends"] = list(def_funcs - call_funcs)
            neuron_map["Logic_In"] = list(set(neuron_map["Logic_In"]))
            neuron_map["Logic_Out"] = list(set(neuron_map["Logic_Out"]))
            neuron_map["Dependencies"] = list(set(neuron_map["Dependencies"]))
            neuron_map["Resource_Linkages"] = list(set(neuron_map["Resource_Linkages"]))
            
            return neuron_map
        except:
            return None

    def ignite_mesh_audit(self):
        print(f"[IGNITION] Firing Universal Mesh Pulse across {len(self.roots)} roots...")
        for root in self.roots:
            root_id = os.path.basename(root)
            print(f"  [ROOT] Auditing: {root_id}")
            for r_dir, _, files in os.walk(root):
                for f in files:
                    if f.endswith(".py"):
                        f_path = os.path.join(r_dir, f)
                        neuron = self.analyze_neuron(f_path, root_id)
                        if neuron:
                            rel_key = os.path.relpath(f_path, root)
                            self.mesh["Neurons"][f"{root_id}/{rel_key}"] = neuron
                            self.mesh["Global_Metrics"]["Total_Files"] += 1
                        print(f"    [NEURON] Mapped: {f}", end="\r")

        # Sealing the Mesh Manifest
        output_file = "sovereign_mesh_topology.json"
        with open(output_file, "w") as f:
            json.dump(self.mesh, f, indent=4)
        
        self.generate_mesh_report()
        print(f"\n[SEALED] Universal Mesh Audit Complete. Report: sovereign_mesh_report.md")

    def generate_mesh_report(self):
        with open("sovereign_mesh_report.md", "w", encoding="utf-8") as f:
            f.write("# Sovereign Universal Mesh Report\n")
            f.write(f"Scanned: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
            f.write("## Mesh Coverage\n")
            for r in self.roots: f.write(f"- {r}\n")
            f.write(f"\n- **Total Neurons Mapped**: {self.mesh['Global_Metrics']['Total_Files']}\n\n")
            
            f.write("## Multi-Root Topology Table\n")
            f.write("| Root | Neuron | Logic-In | Resource Linkages | Dead Ends |\n")
            f.write("| :--- | :--- | :--- | :--- | :--- |\n")
            for key, data in self.mesh["Neurons"].items():
                res = ", ".join(data['Resource_Linkages'][:2])
                logic = ", ".join(data['Logic_In'][:2])
                f.write(f"| {data['Root']} | {data['Filename']} | {logic} | {res} | {len(data['Dead_Ends'])} |\n")

if __name__ == "__main__":
    # Scan Common Sovereign Roots
    sov_roots = ["C:\GENESIS\GenesisRUST\Sovereign_Suite_RS", "C:\\DPM_Engine", "D:\\"]
    active_roots = [r for r in sov_roots if os.path.exists(r)]
    auditor = SovereignMeshATS(active_roots)
    auditor.ignite_mesh_audit()
