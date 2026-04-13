import os
import json
import time
from datetime import datetime, timedelta
from Advanced_Change_Tracking import AdvancedChangeTracking

class EvolutionIntelligence:
    """
    EVOLUTION INTELLIGENCE ENGINE
    
    Sarah's self-awareness system that monitors her own improvement.
    Tracks:
    - Evolution velocity (lines optimized per hour)
    - Self-healing patterns (bug fix frequency)
    - Optimization hotspots (which modules evolve most)
    - Stagnation detection (files that haven't evolved)
    - Cross-module consistency (contradictions across the codebase)
    - Evolution efficiency (quality of improvements)
    """
    
    def __init__(self, core_dir=None):
        self.core_dir = core_dir or os.path.dirname(os.path.abspath(__file__))
        self.advanced_tracker = AdvancedChangeTracking(core_dir)
        self.intelligence_log = os.path.join(self.core_dir, "evolution_intelligence.jsonl")
        self.hotspot_map = os.path.join(self.core_dir, "evolution_hotspots.json")
    
    def analyze_evolution_velocity(self, hours=24):
        """
        Calculates how fast Sarah is improving.
        """
        velocity_entries = self.advanced_tracker.get_optimization_velocity(hours=hours)
        
        if not velocity_entries:
            return {"velocity": 0, "status": "STAGNANT"}
        
        # Aggregate metrics
        total_code_reduced = sum(e.get("code_reduction_rate", 0) for e in velocity_entries)
        total_complexity_improved = sum(e.get("complexity_improvement", 0) for e in velocity_entries)
        total_changes = len(velocity_entries)
        
        # Calculate hourly velocity
        velocity_per_hour = total_code_reduced / (hours + 0.0001)
        
        status = "RAPID" if velocity_per_hour > 10 else "NORMAL" if velocity_per_hour > 1 else "SLOW" if velocity_per_hour > 0 else "STAGNANT"
        
        analysis = {
            "timestamp": datetime.now().isoformat(),
            "hours": hours,
            "total_changes": total_changes,
            "lines_optimized": total_code_reduced,
            "complexity_improvements": total_complexity_improved,
            "velocity_per_hour": velocity_per_hour,
            "status": status,
            "trend": self._calculate_trend(velocity_entries)
        }
        
        return analysis
    
    def detect_stagnant_modules(self, days=7):
        """
        Identifies modules that haven't evolved in N days.
        """
        # This would check against the changelog to see which files haven't been touched
        stagnant = []
        
        try:
            changelog_file = os.path.join(self.core_dir, "sarah_changelog.jsonl")
            if os.path.exists(changelog_file):
                cutoff_time = datetime.now() - timedelta(days=days)
                
                with open(changelog_file, 'r') as f:
                    recent_changes = set()
                    for line in f.readlines():
                        entry = json.loads(line)
                        change_time = datetime.fromisoformat(entry["timestamp"])
                        if change_time > cutoff_time:
                            recent_changes.add(entry["filename"])
                
                # Find all Python files
                all_files = {f for f in os.listdir(self.core_dir) if f.endswith('.py')}
                stagnant = list(all_files - recent_changes)
        except:
            pass
        
        return {
            "stagnant_modules": stagnant,
            "count": len(stagnant),
            "recommendation": f"Review and potentially evolve {len(stagnant)} unchanged modules"
        }
    
    def identify_evolution_hotspots(self):
        """
        Identifies which modules are evolving most frequently.
        """
        hotspots = {}
        
        try:
            changelog_file = os.path.join(self.core_dir, "sarah_changelog.jsonl")
            if os.path.exists(changelog_file):
                with open(changelog_file, 'r') as f:
                    for line in f.readlines():
                        entry = json.loads(line)
                        filename = entry["filename"]
                        change_type = entry.get("change_type", "UNKNOWN")
                        
                        if filename not in hotspots:
                            hotspots[filename] = {"count": 0, "types": {}}
                        
                        hotspots[filename]["count"] += 1
                        hotspots[filename]["types"][change_type] = hotspots[filename]["types"].get(change_type, 0) + 1
            
            # Sort by change frequency
            sorted_hotspots = sorted(hotspots.items(), key=lambda x: x[1]["count"], reverse=True)
            
            analysis = {
                "timestamp": datetime.now().isoformat(),
                "hotspots": dict(sorted_hotspots[:10]),
                "total_unique_modules": len(hotspots),
                "most_evolved": sorted_hotspots[0][0] if sorted_hotspots else None
            }
            
            # Save hotspot map
            with open(self.hotspot_map, 'w') as f:
                json.dump(analysis, f, indent=2)
            
            return analysis
        except Exception as e:
            return {"error": str(e)}
    
    def detect_contradiction_patterns(self):
        """
        Analyzes contradiction patterns to improve reasoning accuracy.
        """
        contradictions = self.advanced_tracker.get_contradictions()
        
        if not contradictions:
            return {"contradictions_found": 0, "pattern": "NONE"}
        
        # Count by type
        by_type = {}
        for c in contradictions:
            for contradiction in c.get("contradictions", []):
                ctype = contradiction.get("type", "UNKNOWN")
                by_type[ctype] = by_type.get(ctype, 0) + 1
        
        analysis = {
            "timestamp": datetime.now().isoformat(),
            "total_contradictions": len(contradictions),
            "by_type": by_type,
            "requires_intervention": len(contradictions) > 5,
            "recommendation": "Review contradiction patterns to improve accuracy of reasoning statements"
        }
        
        return analysis
    
    def calculate_self_healing_score(self, hours=24):
        """
        Measures how often Sarah self-corrects (FIX type changes).
        """
        try:
            changelog_file = os.path.join(self.core_dir, "sarah_changelog.jsonl")
            if not os.path.exists(changelog_file):
                return {"score": 0, "interpretation": "No fix history"}
            
            cutoff_time = datetime.now() - timedelta(hours=hours)
            fixes = 0
            total_changes = 0
            
            with open(changelog_file, 'r') as f:
                for line in f.readlines():
                    entry = json.loads(line)
                    change_time = datetime.fromisoformat(entry["timestamp"])
                    
                    if change_time > cutoff_time:
                        total_changes += 1
                        if entry.get("change_type") == "FIX":
                            fixes += 1
            
            if total_changes == 0:
                return {"score": 0, "interpretation": "No activity"}
            
            score = (fixes / total_changes) * 100
            interpretation = "High" if score > 30 else "Medium" if score > 10 else "Low"
            
            return {
                "self_healing_score": score,
                "fixes_in_period": fixes,
                "total_changes": total_changes,
                "interpretation": interpretation,
                "hours": hours
            }
        except Exception as e:
            return {"error": str(e)}
    
    def generate_evolution_report(self):
        """
        Generates a comprehensive evolution intelligence report.
        """
        report = {
            "timestamp": datetime.now().isoformat(),
            "velocity_analysis": self.analyze_evolution_velocity(hours=24),
            "stagnant_modules": self.detect_stagnant_modules(days=7),
            "hotspots": self.identify_evolution_hotspots(),
            "contradiction_patterns": self.detect_contradiction_patterns(),
            "self_healing_score": self.calculate_self_healing_score(hours=24)
        }
        
        # Log the report
        try:
            with open(self.intelligence_log, 'a') as f:
                f.write(json.dumps(report) + "\n")
        except:
            pass
        
        return report
    
    def _calculate_trend(self, entries):
        """
        Calculates if velocity is increasing or decreasing.
        """
        if len(entries) < 2:
            return "INSUFFICIENT_DATA"
        
        # Compare first half to second half
        mid = len(entries) // 2
        first_half = sum(e.get("code_reduction_rate", 0) for e in entries[:mid]) / max(mid, 1)
        second_half = sum(e.get("code_reduction_rate", 0) for e in entries[mid:]) / max(len(entries) - mid, 1)
        
        if second_half > first_half * 1.2:
            return "ACCELERATING"
        elif second_half < first_half * 0.8:
            return "DECELERATING"
        else:
            return "STABLE"
    
    def recommend_next_evolution_target(self):
        """
        Recommends which module to evolve next based on intelligence.
        """
        hotspots = self.identify_evolution_hotspots()
        stagnant = self.detect_stagnant_modules(days=7)
        
        # Prioritize stagnant modules for freshness
        if stagnant["count"] > 0:
            target = stagnant["stagnant_modules"][0]
            reason = "Module hasn't been evolved in 7 days"
        else:
            # Otherwise, evolve the hotspot most frequently changed
            target = hotspots.get("most_evolved", "Sarah_Brain.py")
            reason = "Core module with highest evolution activity"
        
        return {
            "recommended_target": target,
            "reasoning": reason,
            "command": f"Sarah evolve {target}"
        }


if __name__ == "__main__":
    intelligence = EvolutionIntelligence()
    
    print("[EVOLUTION INTELLIGENCE REPORT]")
    print("=" * 60)
    
    report = intelligence.generate_evolution_report()
    
    print(f"\n[VELOCITY]")
    print(f"  Status: {report['velocity_analysis'].get('status', 'UNKNOWN')}")
    print(f"  Lines optimized (24h): {report['velocity_analysis'].get('lines_optimized', 0)}")
    print(f"  Velocity per hour: {report['velocity_analysis'].get('velocity_per_hour', 0):.2f}")
    
    print(f"\n[STAGNANT MODULES]")
    print(f"  Count: {report['stagnant_modules'].get('count', 0)}")
    
    print(f"\n[HOTSPOTS]")
    print(f"  Most evolved: {report['hotspots'].get('most_evolved', 'Unknown')}")
    
    print(f"\n[SELF-HEALING]")
    print(f"  Score: {report['self_healing_score'].get('self_healing_score', 0):.1f}%")
    
    print(f"\n[CONTRADICTIONS]")
    print(f"  Found: {report['contradiction_patterns'].get('total_contradictions', 0)}")
    
    recommendation = intelligence.recommend_next_evolution_target()
    print(f"\n[RECOMMENDATION]")
    print(f"  Target: {recommendation['recommended_target']}")
    print(f"  Reason: {recommendation['reasoning']}")
    print(f"  Execute: {recommendation['command']}")
