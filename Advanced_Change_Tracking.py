import os
import json
import difflib
import hashlib
from datetime import datetime
from pathlib import Path

class AdvancedChangeTracking:
    """
    EVOLVED CHANGE TRACKING SYSTEM
    
    Beyond simple logging. This tracks:
    - Line-by-line diffs
    - Performance metrics (before/after)
    - Dependency chains (which files depend on changed code)
    - Contradiction detection (reasoning vs actual changes)
    - Change propagation (impact analysis)
    - Optimization velocity (improvement trajectory)
    """
    
    def __init__(self, core_dir=None):
        self.core_dir = core_dir or os.path.dirname(os.path.abspath(__file__))
        self.diff_store = os.path.join(self.core_dir, "diffs_store")
        self.metrics_log = os.path.join(self.core_dir, "performance_metrics.jsonl")
        self.contradiction_log = os.path.join(self.core_dir, "contradiction_warnings.jsonl")
        self.impact_graph = os.path.join(self.core_dir, "impact_graph.json")
        self.velocity_log = os.path.join(self.core_dir, "optimization_velocity.jsonl")
        
        # Create directories
        os.makedirs(self.diff_store, exist_ok=True)
        
        # Load dependency graph
        self.dependencies = self._load_dependency_graph()
    
    def track_change_with_diff(self, filename, old_content, new_content, reason, trigger):
        """
        Tracks a change with full diff and stores it for retrieval.
        """
        filepath = os.path.join(self.core_dir, filename)
        
        # Generate diff
        diff = self._generate_diff(old_content, new_content)
        lines_changed = self._count_changed_lines(diff)
        
        # Store diff
        diff_id = self._store_diff(filename, diff, reason, trigger)
        
        # Detect contradictions
        contradictions = self._detect_contradictions(reason, diff, new_content)
        if contradictions:
            self._log_contradiction(filename, diff_id, reason, contradictions)
        
        # Calculate metrics
        old_metrics = self._analyze_code_metrics(old_content)
        new_metrics = self._analyze_code_metrics(new_content)
        improvement = self._calculate_improvement(old_metrics, new_metrics)
        
        # Log metrics
        self._log_performance_metrics(filename, old_metrics, new_metrics, improvement, trigger)
        
        # Update optimization velocity
        self._update_velocity(filename, improvement)
        
        # Track impact (which files will be affected)
        self._analyze_change_impact(filename, diff)
        
        return {
            "diff_id": diff_id,
            "lines_changed": lines_changed,
            "improvement": improvement,
            "contradictions": contradictions
        }
    
    def _generate_diff(self, old_content, new_content):
        """
        Generates a unified diff between old and new content.
        """
        old_lines = old_content.splitlines(keepends=True)
        new_lines = new_content.splitlines(keepends=True)
        
        diff = list(difflib.unified_diff(
            old_lines,
            new_lines,
            fromfile="original",
            tofile="evolved",
            lineterm=''
        ))
        
        return diff
    
    def _count_changed_lines(self, diff):
        """
        Counts additions, deletions, and modifications.
        """
        additions = sum(1 for line in diff if line.startswith('+') and not line.startswith('+++'))
        deletions = sum(1 for line in diff if line.startswith('-') and not line.startswith('---'))
        
        return {
            "additions": additions,
            "deletions": deletions,
            "total_changes": additions + deletions
        }
    
    def _store_diff(self, filename, diff, reason, trigger):
        """
        Stores diff in structured format for retrieval.
        """
        timestamp = datetime.now().isoformat()
        diff_id = f"{filename.replace('.py', '')}_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
        
        diff_file = os.path.join(self.diff_store, f"{diff_id}.patch")
        
        entry = {
            "timestamp": timestamp,
            "diff_id": diff_id,
            "filename": filename,
            "reason": reason,
            "trigger": trigger,
            "diff_lines": len(diff),
            "patch_file": diff_file
        }
        
        # Write patch file
        try:
            with open(diff_file, 'w') as f:
                f.writelines(diff)
        except Exception as e:
            print(f"[AdvancedTracking] Failed to write patch: {e}")
        
        # Log entry
        try:
            log_file = os.path.join(self.core_dir, "change_diffs.jsonl")
            with open(log_file, 'a') as f:
                f.write(json.dumps(entry) + "\n")
        except Exception as e:
            print(f"[AdvancedTracking] Failed to log diff entry: {e}")
        
        return diff_id
    
    def _analyze_code_metrics(self, content):
        """
        Analyzes code quality metrics.
        """
        lines = content.splitlines()
        
        metrics = {
            "total_lines": len(lines),
            "code_lines": sum(1 for l in lines if l.strip() and not l.strip().startswith('#')),
            "comment_lines": sum(1 for l in lines if l.strip().startswith('#')),
            "avg_line_length": sum(len(l) for l in lines) / len(lines) if lines else 0,
            "function_count": sum(1 for l in lines if l.strip().startswith('def ')),
            "class_count": sum(1 for l in lines if l.strip().startswith('class ')),
            "import_count": sum(1 for l in lines if l.strip().startswith('import ') or l.strip().startswith('from ')),
            "code_density": (sum(1 for l in lines if l.strip() and not l.strip().startswith('#')) / len(lines) * 100) if lines else 0
        }
        
        return metrics
    
    def _calculate_improvement(self, old_metrics, new_metrics):
        """
        Calculates improvement vectors.
        """
        improvement = {
            "code_lines_reduced": old_metrics["code_lines"] - new_metrics["code_lines"],
            "code_density_improved": new_metrics["code_density"] - old_metrics["code_density"],
            "avg_line_length_reduced": old_metrics["avg_line_length"] - new_metrics["avg_line_length"],
            "functions_added": new_metrics["function_count"] - old_metrics["function_count"],
            "complexity_score": self._calculate_complexity_delta(old_metrics, new_metrics)
        }
        
        return improvement
    
    def _calculate_complexity_delta(self, old_metrics, new_metrics):
        """
        Rough complexity metric (can be expanded).
        Higher code density + fewer lines = better efficiency.
        """
        old_complexity = old_metrics["code_lines"] / max(old_metrics["function_count"], 1)
        new_complexity = new_metrics["code_lines"] / max(new_metrics["function_count"], 1)
        
        return old_complexity - new_complexity
    
    def _detect_contradictions(self, reason, diff, new_content):
        """
        Detects if the stated reason contradicts the actual changes.
        """
        contradictions = []
        reason_lower = reason.lower()
        
        # If reason says "optimized" but code grew
        lines_added = sum(1 for line in diff if line.startswith('+') and not line.startswith('+++'))
        lines_deleted = sum(1 for line in diff if line.startswith('-') and not line.startswith('---'))
        net_change = lines_added - lines_deleted
        
        if "optimized" in reason_lower and net_change > 5:
            contradictions.append({
                "type": "OPTIMIZATION_CONTRADICTION",
                "message": f"Claimed optimization added {net_change} net lines"
            })
        
        # If reason says "removed redundancy" but imports didn't change
        if "redundant" in reason_lower or "removed" in reason_lower:
            if lines_deleted < lines_added:
                contradictions.append({
                    "type": "REMOVAL_CONTRADICTION",
                    "message": f"Claimed removal but more lines added ({lines_added}) than deleted ({lines_deleted})"
                })
        
        return contradictions
    
    def _log_contradiction(self, filename, diff_id, reason, contradictions):
        """
        Logs potential contradictions for review.
        """
        entry = {
            "timestamp": datetime.now().isoformat(),
            "filename": filename,
            "diff_id": diff_id,
            "stated_reason": reason,
            "contradictions": contradictions,
            "severity": "MEDIUM",
            "requires_review": True
        }
        
        try:
            with open(self.contradiction_log, 'a') as f:
                f.write(json.dumps(entry) + "\n")
        except Exception as e:
            print(f"[AdvancedTracking] Failed to log contradiction: {e}")
    
    def _log_performance_metrics(self, filename, old_metrics, new_metrics, improvement, trigger):
        """
        Logs performance metrics for each change.
        """
        entry = {
            "timestamp": datetime.now().isoformat(),
            "filename": filename,
            "trigger": trigger,
            "old_metrics": old_metrics,
            "new_metrics": new_metrics,
            "improvement": improvement
        }
        
        try:
            with open(self.metrics_log, 'a') as f:
                f.write(json.dumps(entry) + "\n")
        except Exception as e:
            print(f"[AdvancedTracking] Failed to log metrics: {e}")
    
    def _update_velocity(self, filename, improvement):
        """
        Tracks optimization velocity (rate of improvement).
        """
        velocity_entry = {
            "timestamp": datetime.now().isoformat(),
            "filename": filename,
            "code_reduction_rate": improvement["code_lines_reduced"],
            "complexity_improvement": improvement["complexity_score"],
            "density_improvement": improvement["code_density_improved"]
        }
        
        try:
            with open(self.velocity_log, 'a') as f:
                f.write(json.dumps(velocity_entry) + "\n")
        except Exception as e:
            print(f"[AdvancedTracking] Failed to log velocity: {e}")
    
    def _analyze_change_impact(self, changed_file, diff):
        """
        Analyzes which other files might be affected by this change.
        """
        impacted_files = self.dependencies.get(changed_file, [])
        
        impact_entry = {
            "timestamp": datetime.now().isoformat(),
            "changed_file": changed_file,
            "potentially_affected": impacted_files,
            "impact_count": len(impacted_files),
            "propagation_risk": "HIGH" if len(impacted_files) > 3 else "MEDIUM" if len(impacted_files) > 0 else "LOW"
        }
        
        return impact_entry
    
    def _load_dependency_graph(self):
        """
        Loads or builds the module dependency graph.
        """
        graph_file = self.impact_graph
        
        if os.path.exists(graph_file):
            try:
                with open(graph_file, 'r') as f:
                    return json.load(f)
            except:
                return {}
        
        # Build dependency graph from imports
        graph = self._build_dependency_graph()
        
        try:
            with open(graph_file, 'w') as f:
                json.dump(graph, f, indent=2)
        except:
            pass
        
        return graph
    
    def _build_dependency_graph(self):
        """
        Scans all Python files to build import dependency graph.
        """
        graph = {}
        
        for filename in os.listdir(self.core_dir):
            if not filename.endswith('.py'):
                continue
            
            filepath = os.path.join(self.core_dir, filename)
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Extract imports
                imports = []
                for line in content.splitlines():
                    if line.strip().startswith('from ') or line.strip().startswith('import '):
                        # Extract module name
                        if 'from' in line:
                            module = line.split('from')[1].split('import')[0].strip()
                        else:
                            module = line.split('import')[1].split('as')[0].strip().split(',')[0].strip()
                        
                        # Check if it's a local import
                        if module and not module.startswith('.'):
                            module_file = f"{module}.py"
                            if os.path.exists(os.path.join(self.core_dir, module_file)):
                                imports.append(module_file)
                
                graph[filename] = imports
            except:
                pass
        
        return graph
    
    def get_diff(self, diff_id):
        """
        Retrieves a stored diff.
        """
        diff_file = os.path.join(self.diff_store, f"{diff_id}.patch")
        
        if os.path.exists(diff_file):
            try:
                with open(diff_file, 'r') as f:
                    return f.read()
            except:
                return None
        
        return None
    
    def get_optimization_velocity(self, filename=None, hours=24):
        """
        Returns optimization velocity metrics.
        """
        if not os.path.exists(self.velocity_log):
            return []
        
        from datetime import timedelta
        cutoff_time = datetime.now() - timedelta(hours=hours)
        
        results = []
        try:
            with open(self.velocity_log, 'r') as f:
                for line in f.readlines():
                    entry = json.loads(line)
                    entry_time = datetime.fromisoformat(entry["timestamp"])
                    
                    if entry_time > cutoff_time:
                        if filename is None or entry["filename"] == filename:
                            results.append(entry)
        except:
            pass
        
        return results
    
    def get_contradictions(self, severity=None):
        """
        Returns all logged contradictions.
        """
        if not os.path.exists(self.contradiction_log):
            return []
        
        results = []
        try:
            with open(self.contradiction_log, 'r') as f:
                for line in f.readlines():
                    entry = json.loads(line)
                    if severity is None or entry.get("severity") == severity:
                        results.append(entry)
        except:
            pass
        
        return results
    
    def generate_impact_report(self, filename):
        """
        Generates an impact analysis report for a changed file.
        """
        impacted = self.dependencies.get(filename, [])
        
        report = {
            "changed_file": filename,
            "directly_affects": impacted,
            "indirect_impact_depth": self._calculate_impact_depth(filename),
            "recommendation": self._generate_recommendation(impacted)
        }
        
        return report
    
    def _calculate_impact_depth(self, filename, visited=None):
        """
        Calculates transitive dependency depth.
        """
        if visited is None:
            visited = set()
        
        if filename in visited:
            return 0
        
        visited.add(filename)
        max_depth = 0
        
        for dependent in self.dependencies.get(filename, []):
            depth = 1 + self._calculate_impact_depth(dependent, visited)
            max_depth = max(max_depth, depth)
        
        return max_depth
    
    def _generate_recommendation(self, impacted_files):
        """
        Generates recommendations based on impact.
        """
        if len(impacted_files) > 5:
            return "CRITICAL: This change affects many modules. Run full test suite."
        elif len(impacted_files) > 2:
            return "WARNING: Multiple modules affected. Test " + ", ".join(impacted_files[:3])
        else:
            return "LOW RISK: Minimal propagation. Targeted testing sufficient."


if __name__ == "__main__":
    tracker = AdvancedChangeTracking()
    print("[ADVANCED CHANGE TRACKING] System initialized.")
    print(f"Tracked metrics: Diffs, Performance, Contradictions, Impact, Velocity")
