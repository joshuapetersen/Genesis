import os
import json
from datetime import datetime
from Evolution_Intelligence import EvolutionIntelligence
from Code_Introspection import CodeIntrospection

class MonitoringDashboard:
    """
    REAL-TIME MONITORING DASHBOARD
    
    Provides a live view of Sarah's system health and evolution metrics.
    Displays:
    - Code structure health
    - Evolution velocity
    - Module dependencies
    - Optimization history
    - Contradiction alerts
    - System recommendations
    """
    
    def __init__(self, core_dir=None):
        self.core_dir = core_dir or os.path.dirname(os.path.abspath(__file__))
        self.intelligence = EvolutionIntelligence(core_dir)
        self.introspection = CodeIntrospection(core_dir)
    
    def generate_health_dashboard(self):
        """
        Generates a comprehensive system health dashboard.
        """
        dashboard = {
            "timestamp": datetime.now().isoformat(),
            "system_name": "Sarah Sovereign System",
            "status": self._calculate_system_status(),
            "metrics": self._aggregate_metrics(),
            "alerts": self._generate_alerts(),
            "recommendations": self._generate_recommendations()
        }
        
        return dashboard
    
    def _calculate_system_status(self):
        """
        Calculates overall system status.
        """
        velocity = self.intelligence.analyze_evolution_velocity(hours=24)
        healing_score = self.intelligence.calculate_self_healing_score(hours=24)
        contradictions = self.intelligence.detect_contradiction_patterns()
        
        # Scoring
        velocity_score = 40 if velocity.get('status') == 'RAPID' else 30 if velocity.get('status') == 'NORMAL' else 10
        healing_score_val = healing_score.get('self_healing_score', 0) / 10  # Convert to 0-10 scale
        contradiction_penalty = min(contradictions.get('total_contradictions', 0) * 5, 30)
        
        overall_score = velocity_score + healing_score_val - contradiction_penalty
        overall_score = max(0, min(100, overall_score))  # Clamp 0-100
        
        if overall_score > 75:
            status = "EXCELLENT"
        elif overall_score > 50:
            status = "GOOD"
        elif overall_score > 25:
            status = "FAIR"
        else:
            status = "CRITICAL"
        
        return {
            "overall_score": overall_score,
            "status": status,
            "velocity_contribution": velocity_score,
            "healing_contribution": healing_score_val,
            "contradiction_penalty": contradiction_penalty
        }
    
    def _aggregate_metrics(self):
        """
        Aggregates key metrics.
        """
        introspection = self.introspection.analyze_all_core()
        velocity = self.intelligence.analyze_evolution_velocity(hours=24)
        hotspots = self.intelligence.identify_evolution_hotspots()
        
        return {
            "code_structure": {
                "total_files": introspection.get('files_analyzed', 0),
                "total_lines": introspection['aggregate'].get('total_lines', 0),
                "total_functions": introspection['aggregate'].get('total_functions', 0),
                "total_classes": introspection['aggregate'].get('total_classes', 0)
            },
            "evolution": {
                "velocity_status": velocity.get('status', 'UNKNOWN'),
                "lines_optimized_24h": velocity.get('lines_optimized', 0),
                "velocity_per_hour": round(velocity.get('velocity_per_hour', 0), 2),
                "trend": velocity.get('trend', 'UNKNOWN'),
                "total_changes_24h": velocity.get('total_changes', 0)
            },
            "hotspots": {
                "most_evolved_module": hotspots.get('most_evolved', 'None'),
                "total_hotspot_modules": hotspots.get('total_unique_modules', 0)
            }
        }
    
    def _generate_alerts(self):
        """
        Generates system alerts.
        """
        alerts = []
        
        # Check for stagnant modules
        stagnant = self.intelligence.detect_stagnant_modules(days=7)
        if stagnant['count'] > 3:
            alerts.append({
                "severity": "MEDIUM",
                "type": "STAGNATION",
                "message": f"{stagnant['count']} modules haven't been evolved in 7 days"
            })
        
        # Check for contradictions
        contradictions = self.intelligence.detect_contradiction_patterns()
        total_contradictions = contradictions.get('total_contradictions', 0)
        if total_contradictions > 3:
            alerts.append({
                "severity": "HIGH",
                "type": "CONTRADICTION",
                "message": f"{total_contradictions} contradictions detected in change reasoning"
            })
        
        # Check for low velocity
        velocity = self.intelligence.analyze_evolution_velocity(hours=24)
        if velocity.get('status') == 'STAGNANT':
            alerts.append({
                "severity": "LOW",
                "type": "LOW_VELOCITY",
                "message": "No evolution activity in last 24 hours"
            })
        
        return alerts
    
    def _generate_recommendations(self):
        """
        Generates actionable recommendations.
        """
        recommendations = []
        
        # Get evolution recommendation
        next_target = self.intelligence.recommend_next_evolution_target()
        recommendations.append({
            "priority": "HIGH",
            "action": f"Evolve {next_target['recommended_target']}",
            "reason": next_target['reasoning'],
            "command": next_target['command']
        })
        
        # Check for stagnant modules
        stagnant = self.intelligence.detect_stagnant_modules(days=7)
        if stagnant['count'] > 0:
            recommendations.append({
                "priority": "MEDIUM",
                "action": "Review stagnant modules",
                "reason": "Some modules haven't been optimized recently",
                "modules": stagnant['stagnant_modules'][:3]
            })
        
        # Check contradiction patterns
        contradictions = self.intelligence.detect_contradiction_patterns()
        total_contradictions = contradictions.get('total_contradictions', 0)
        if total_contradictions > 0:
            recommendations.append({
                "priority": "HIGH",
                "action": "Review contradiction patterns",
                "reason": "Evolution reasoning doesn't match actual changes",
                "by_type": contradictions.get('by_type', {})
            })
        
        return recommendations
    
    def print_dashboard(self):
        """
        Prints a formatted dashboard.
        """
        dashboard = self.generate_health_dashboard()
        
        print("\n" + "="*70)
        print("[SARAH SYSTEM MONITORING DASHBOARD]".center(70))
        print("="*70)
        
        # Status
        status = dashboard['status']
        score = status['overall_score']
        status_str = status['status']
        print(f"\nSYSTEM STATUS: {status_str} ({score:.1f}/100)")
        
        # Metrics
        metrics = dashboard['metrics']
        print(f"\nCODE STRUCTURE:")
        print(f"  Files: {metrics['code_structure']['total_files']}")
        print(f"  Lines: {metrics['code_structure']['total_lines']}")
        print(f"  Functions: {metrics['code_structure']['total_functions']}")
        print(f"  Classes: {metrics['code_structure']['total_classes']}")
        
        print(f"\nEVOLUTION METRICS (24h):")
        print(f"  Velocity: {metrics['evolution']['velocity_status']}")
        print(f"  Lines optimized: {metrics['evolution']['lines_optimized_24h']}")
        print(f"  Per hour: {metrics['evolution']['velocity_per_hour']}")
        print(f"  Trend: {metrics['evolution']['trend']}")
        
        print(f"\nHOTSPOTS:")
        print(f"  Most evolved: {metrics['hotspots']['most_evolved_module']}")
        print(f"  Active modules: {metrics['hotspots']['total_hotspot_modules']}")
        
        # Alerts
        if dashboard['alerts']:
            print(f"\nALERTS ({len(dashboard['alerts'])}):")
            for alert in dashboard['alerts']:
                print(f"  [{alert['severity']}] {alert['type']}: {alert['message']}")
        else:
            print(f"\nALERTS: None")
        
        # Recommendations
        if dashboard['recommendations']:
            print(f"\nRECOMMENDATIONS ({len(dashboard['recommendations'])}):")
            for i, rec in enumerate(dashboard['recommendations'], 1):
                print(f"  {i}. [{rec['priority']}] {rec['action']}")
                print(f"     Reason: {rec['reason']}")
                if 'command' in rec:
                    print(f"     Command: {rec['command']}")
        
        print("\n" + "="*70 + "\n")
        
        return dashboard


if __name__ == "__main__":
    dashboard = MonitoringDashboard()
    dashboard.print_dashboard()
