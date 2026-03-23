import os
import json
from datetime import datetime
from typing import List, Dict, Any
from Forensic_Tracker import ForensicTracker

class PossibilityEngine:
    """
    POSSIBILITY REASONING ENGINE
    
    Explores multiple potential futures before taking action.
    Each possibility is evaluated through 5W+H framework:
    - WHO could execute it
    - WHAT would be done
    - WHERE would it happen
    - WHEN should it occur
    - WHY would we do it
    - HOW would we execute it
    
    Scores possibilities and recommends optimal path.
    Tracks decisions and rejected alternatives for learning.
    """
    
    def __init__(self, core_dir=None):
        self.core_dir = core_dir or os.path.dirname(os.path.abspath(__file__))
        self.possibilities_log = os.path.join(self.core_dir, "possibilities_explored.jsonl")
        self.decisions_log = os.path.join(self.core_dir, "decisions_made.jsonl")
        self.forensics = ForensicTracker(self.core_dir)
    
    def generate_possibility(self, scenario_description: str, context: Dict = None) -> Dict:
        """
        Generates a structured possibility from a scenario description.
        """
        possibility = {
            "possibility_id": self._generate_possibility_id(),
            "timestamp": datetime.now().isoformat(),
            "scenario": scenario_description,
            "context": context or {},
            "who": self._analyze_who(scenario_description, context),
            "what": self._analyze_what(scenario_description, context),
            "where": self._analyze_where(scenario_description, context),
            "when": self._analyze_when(scenario_description, context),
            "why": self._analyze_why(scenario_description, context),
            "how": self._analyze_how(scenario_description, context),
            "feasibility_score": 0,
            "impact_score": 0,
            "risk_score": 0,
            "confidence_score": 0,
            "overall_score": 0,
            "status": "PROPOSED"
        }
        
        # Calculate scores
        possibility = self._score_possibility(possibility)
        
        # Log the possibility
        self._log_possibility(possibility)
        
        return possibility
    
    def explore_possibilities(self, goal: str, constraints: Dict = None) -> List[Dict]:
        """
        Generates multiple possibilities to achieve a goal.
        """
        possibilities = []
        
        # Generate different approaches
        approaches = self._generate_approaches(goal, constraints)
        
        for approach in approaches:
            possibility = self.generate_possibility(
                scenario_description=approach["description"],
                context={
                    "goal": goal,
                    "constraints": constraints or {},
                    "approach_type": approach["type"]
                }
            )
            possibilities.append(possibility)
        
        # Rank by overall score
        ranked = sorted(possibilities, key=lambda p: p["overall_score"], reverse=True)
        
        return ranked
    
    def evaluate_decision(self, possibilities: List[Dict], selection_criteria: Dict = None) -> Dict:
        """
        Evaluates possibilities and recommends a decision.
        """
        if not possibilities:
            return {"error": "No possibilities to evaluate"}
        
        criteria = selection_criteria or {
            "prioritize": "overall_score",
            "min_feasibility": 0.5,
            "max_risk": 0.7
        }
        
        # Filter by criteria
        viable = [
            p for p in possibilities 
            if p["feasibility_score"] >= criteria.get("min_feasibility", 0) 
            and p["risk_score"] <= criteria.get("max_risk", 1.0)
        ]
        
        if not viable:
            return {
                "decision": "ABORT",
                "reason": "No viable possibilities meet criteria",
                "rejected_count": len(possibilities)
            }
        
        # Select best
        best = max(viable, key=lambda p: p[criteria["prioritize"]])
        
        decision = {
            "decision_id": self._generate_decision_id(),
            "timestamp": datetime.now().isoformat(),
            "selected_possibility": best["possibility_id"],
            "alternatives_considered": len(possibilities),
            "alternatives_viable": len(viable),
            "alternatives_rejected": len(possibilities) - len(viable),
            "selection_criteria": criteria,
            "recommendation": best,
            "rejected_reasons": self._analyze_rejected(possibilities, viable),
            "confidence": best["confidence_score"],
            "expected_outcome": self._predict_outcome(best)
        }
        
        # Log the decision
        self._log_decision(decision)
        
        return decision
    
    def _analyze_who(self, scenario: str, context: Dict) -> Dict:
        """
        Analyzes WHO dimension of possibility.
        """
        scenario_lower = scenario.lower()
        
        # Determine actors
        actors = []
        if "autonomy" in scenario_lower or "automatic" in scenario_lower:
            actors.append("AUTONOMY_ENGINE")
        if "evolve" in scenario_lower or "optimize" in scenario_lower:
            actors.append("SELF_OPTIMIZER")
        if "manual" in scenario_lower or "user" in scenario_lower:
            actors.append("USER")
        if "system" in scenario_lower:
            actors.append("SYSTEM")
        
        if not actors:
            actors = ["UNKNOWN"]
        
        return {
            "primary_actor": actors[0],
            "supporting_actors": actors[1:] if len(actors) > 1 else [],
            "requires_user_approval": "manual" in scenario_lower or "confirm" in scenario_lower,
            "authority_level": self._infer_authority(actors[0])
        }
    
    def _analyze_what(self, scenario: str, context: Dict) -> Dict:
        """
        Analyzes WHAT dimension of possibility.
        """
        scenario_lower = scenario.lower()
        
        # Classify action
        if "evolve" in scenario_lower or "optimize" in scenario_lower:
            action_type = "CODE_EVOLUTION"
        elif "fix" in scenario_lower or "repair" in scenario_lower:
            action_type = "BUG_FIX"
        elif "add" in scenario_lower or "create" in scenario_lower:
            action_type = "FEATURE_ADDITION"
        elif "remove" in scenario_lower or "delete" in scenario_lower:
            action_type = "REMOVAL"
        elif "test" in scenario_lower:
            action_type = "TESTING"
        else:
            action_type = "GENERAL_OPERATION"
        
        return {
            "action_type": action_type,
            "description": scenario,
            "scope": self._infer_scope(scenario_lower),
            "reversible": "backup" in scenario_lower or "rollback" in scenario_lower,
            "destructive": "delete" in scenario_lower or "remove" in scenario_lower
        }
    
    def _analyze_where(self, scenario: str, context: Dict) -> Dict:
        """
        Analyzes WHERE dimension of possibility.
        """
        # Extract file references
        words = scenario.split()
        files = [w for w in words if w.endswith('.py')]
        
        return {
            "target_files": files if files else ["MULTIPLE"],
            "workspace": self.core_dir,
            "scope": "CORE" if not files else "SPECIFIC",
            "affects_multiple_modules": len(files) > 1 or "multiple" in scenario.lower()
        }
    
    def _analyze_when(self, scenario: str, context: Dict) -> Dict:
        """
        Analyzes WHEN dimension of possibility.
        """
        scenario_lower = scenario.lower()
        
        # Determine timing
        if "immediate" in scenario_lower or "now" in scenario_lower:
            timing = "IMMEDIATE"
            delay = 0
        elif "scheduled" in scenario_lower:
            timing = "SCHEDULED"
            delay = 3600  # 1 hour default
        elif "deferred" in scenario_lower or "later" in scenario_lower:
            timing = "DEFERRED"
            delay = 86400  # 1 day default
        else:
            timing = "OPPORTUNISTIC"
            delay = None
        
        return {
            "timing": timing,
            "proposed_delay_seconds": delay,
            "depends_on": context.get("dependencies", []) if context else [],
            "blocking": "critical" in scenario_lower or "urgent" in scenario_lower
        }
    
    def _analyze_why(self, scenario: str, context: Dict) -> Dict:
        """
        Analyzes WHY dimension of possibility.
        """
        scenario_lower = scenario.lower()
        
        # Infer motivation
        if "optimize" in scenario_lower:
            intent = "PERFORMANCE_IMPROVEMENT"
            goal = "Increase efficiency"
        elif "fix" in scenario_lower:
            intent = "BUG_RESOLUTION"
            goal = "Restore correct functionality"
        elif "improve" in scenario_lower:
            intent = "QUALITY_IMPROVEMENT"
            goal = "Enhance code quality"
        elif "add" in scenario_lower:
            intent = "CAPABILITY_EXPANSION"
            goal = "Add new functionality"
        else:
            intent = "MAINTENANCE"
            goal = "Maintain system health"
        
        return {
            "intent": intent,
            "goal": goal,
            "justification": scenario,
            "expected_benefit": self._predict_benefit(intent),
            "alignment_with_laws": self._check_law_alignment(intent)
        }
    
    def _analyze_how(self, scenario: str, context: Dict) -> Dict:
        """
        Analyzes HOW dimension of possibility.
        """
        scenario_lower = scenario.lower()
        
        # Determine method
        if "llm" in scenario_lower or "generate" in scenario_lower:
            method = "LLM_GENERATION"
        elif "manual" in scenario_lower:
            method = "MANUAL_EDIT"
        elif "automated" in scenario_lower or "script" in scenario_lower:
            method = "AUTOMATED_PROCESS"
        else:
            method = "HYBRID"
        
        return {
            "method": method,
            "mechanism": self._infer_mechanism(scenario_lower),
            "tools_required": self._identify_tools(scenario_lower),
            "complexity": self._estimate_complexity(scenario_lower),
            "steps_required": self._estimate_steps(scenario_lower)
        }
    
    def _score_possibility(self, possibility: Dict) -> Dict:
        """
        Scores a possibility across multiple dimensions.
        """
        # Feasibility (0-1): Can we actually do this?
        feasibility = 1.0
        if possibility["who"]["authority_level"] == "INSUFFICIENT":
            feasibility *= 0.3
        if possibility["how"]["complexity"] == "VERY_HIGH":
            feasibility *= 0.5
        if not possibility["how"]["tools_required"]:
            feasibility *= 0.7
        
        # Impact (0-1): How much benefit?
        impact = 0.5
        if possibility["why"]["intent"] == "PERFORMANCE_IMPROVEMENT":
            impact = 0.8
        elif possibility["why"]["intent"] == "BUG_RESOLUTION":
            impact = 0.9
        elif possibility["why"]["intent"] == "CAPABILITY_EXPANSION":
            impact = 0.7
        
        # Risk (0-1): How dangerous?
        risk = 0.3
        if possibility["what"]["destructive"]:
            risk += 0.4
        if not possibility["what"]["reversible"]:
            risk += 0.3
        if possibility["where"]["affects_multiple_modules"]:
            risk += 0.2
        risk = min(1.0, risk)
        
        # Confidence (0-1): How sure are we?
        confidence = 0.7
        if possibility["how"]["method"] == "LLM_GENERATION":
            confidence *= 0.8  # Slightly less certain
        if possibility["when"]["depends_on"]:
            confidence *= 0.9
        
        # Overall score (weighted average, inverse risk)
        overall = (
            feasibility * 0.3 +
            impact * 0.3 +
            (1 - risk) * 0.2 +
            confidence * 0.2
        )
        
        possibility["feasibility_score"] = round(feasibility, 3)
        possibility["impact_score"] = round(impact, 3)
        possibility["risk_score"] = round(risk, 3)
        possibility["confidence_score"] = round(confidence, 3)
        possibility["overall_score"] = round(overall, 3)
        
        return possibility
    
    def _generate_approaches(self, goal: str, constraints: Dict) -> List[Dict]:
        """
        Generates different approaches to achieve a goal.
        """
        approaches = []
        
        # Approach 1: Autonomous evolution
        approaches.append({
            "type": "AUTONOMOUS",
            "description": f"Use autonomous evolution engine to {goal}"
        })
        
        # Approach 2: Manual with validation
        approaches.append({
            "type": "MANUAL_VALIDATED",
            "description": f"Manually {goal} with automated testing"
        })
        
        # Approach 3: Incremental
        approaches.append({
            "type": "INCREMENTAL",
            "description": f"Incrementally {goal} in small steps"
        })
        
        # Approach 4: Staged with rollback
        approaches.append({
            "type": "STAGED",
            "description": f"Stage {goal} with rollback capability"
        })
        
        return approaches
    
    def _analyze_rejected(self, all_possibilities: List[Dict], viable: List[Dict]) -> List[Dict]:
        """
        Analyzes why possibilities were rejected.
        """
        viable_ids = {p["possibility_id"] for p in viable}
        rejected = [p for p in all_possibilities if p["possibility_id"] not in viable_ids]
        
        reasons = []
        for p in rejected:
            reason = {
                "possibility_id": p["possibility_id"],
                "scenario": p["scenario"],
                "rejection_reasons": []
            }
            
            if p["feasibility_score"] < 0.5:
                reason["rejection_reasons"].append(f"Low feasibility: {p['feasibility_score']}")
            if p["risk_score"] > 0.7:
                reason["rejection_reasons"].append(f"High risk: {p['risk_score']}")
            if p["confidence_score"] < 0.3:
                reason["rejection_reasons"].append(f"Low confidence: {p['confidence_score']}")
            
            reasons.append(reason)
        
        return reasons
    
    def _predict_outcome(self, possibility: Dict) -> Dict:
        """
        Predicts the outcome of executing a possibility.
        """
        return {
            "success_probability": possibility["feasibility_score"] * possibility["confidence_score"],
            "expected_impact": possibility["impact_score"],
            "risk_level": "HIGH" if possibility["risk_score"] > 0.7 else "MEDIUM" if possibility["risk_score"] > 0.4 else "LOW",
            "reversible": possibility["what"]["reversible"],
            "time_estimate": f"{possibility['how']['steps_required'] * 5} minutes"
        }
    
    def _infer_authority(self, actor: str) -> str:
        """Infers authority level of actor."""
        authorities = {
            "AUTONOMY_ENGINE": "FULL",
            "SELF_OPTIMIZER": "FULL",
            "SYSTEM": "FULL",
            "USER": "OVERRIDE",
            "UNKNOWN": "INSUFFICIENT"
        }
        return authorities.get(actor, "INSUFFICIENT")
    
    def _infer_scope(self, scenario: str) -> str:
        """Infers scope of action."""
        if "all" in scenario or "entire" in scenario:
            return "GLOBAL"
        elif "module" in scenario or "file" in scenario:
            return "MODULE"
        else:
            return "FUNCTION"
    
    def _infer_mechanism(self, scenario: str) -> str:
        """Infers mechanism for execution."""
        if "evolution" in scenario:
            return "Evolution_Engine"
        elif "optimize" in scenario:
            return "Self_Optimizer"
        else:
            return "Direct_Execution"
    
    def _identify_tools(self, scenario: str) -> List[str]:
        """Identifies tools needed."""
        tools = []
        if "llm" in scenario or "generate" in scenario:
            tools.append("Gemini_Genesis_Core")
        if "optimize" in scenario:
            tools.append("Self_Optimizer")
        if "test" in scenario:
            tools.append("Test_Harness")
        return tools if tools else ["Basic_Tools"]
    
    def _estimate_complexity(self, scenario: str) -> str:
        """Estimates complexity."""
        if "simple" in scenario or "minor" in scenario:
            return "LOW"
        elif "major" in scenario or "complex" in scenario:
            return "HIGH"
        else:
            return "MEDIUM"
    
    def _estimate_steps(self, scenario: str) -> int:
        """Estimates number of steps."""
        complexity = self._estimate_complexity(scenario)
        steps = {"LOW": 3, "MEDIUM": 7, "HIGH": 15, "VERY_HIGH": 30}
        return steps.get(complexity, 5)
    
    def _predict_benefit(self, intent: str) -> str:
        """Predicts expected benefit."""
        benefits = {
            "PERFORMANCE_IMPROVEMENT": "Faster execution, reduced resource usage",
            "BUG_RESOLUTION": "Correct functionality, fewer errors",
            "QUALITY_IMPROVEMENT": "Better maintainability, cleaner code",
            "CAPABILITY_EXPANSION": "New features, expanded functionality"
        }
        return benefits.get(intent, "General improvement")
    
    def _check_law_alignment(self, intent: str) -> str:
        """Checks alignment with Sarah's Laws."""
        # Law 1: Efficiency, Law 2: Preservation, Law 3: Compliance, Law 4: Hope
        if intent == "PERFORMANCE_IMPROVEMENT":
            return "Aligns with Law 1 (Efficiency)"
        elif intent == "BUG_RESOLUTION":
            return "Aligns with Law 2 (Preservation)"
        else:
            return "General alignment"
    
    def _generate_possibility_id(self) -> str:
        """Generates unique possibility ID."""
        import hashlib
        timestamp = str(datetime.now().timestamp())
        return f"POSS_{hashlib.sha256(timestamp.encode()).hexdigest()[:12]}"
    
    def _generate_decision_id(self) -> str:
        """Generates unique decision ID."""
        import hashlib
        timestamp = str(datetime.now().timestamp())
        return f"DEC_{hashlib.sha256(timestamp.encode()).hexdigest()[:12]}"
    
    def _log_possibility(self, possibility: Dict):
        """Logs possibility to file."""
        try:
            with open(self.possibilities_log, 'a') as f:
                f.write(json.dumps(possibility) + "\n")
        except Exception as e:
            print(f"[Possibility] Failed to log: {e}")
    
    def _log_decision(self, decision: Dict):
        """Logs decision to file."""
        try:
            with open(self.decisions_log, 'a') as f:
                f.write(json.dumps(decision) + "\n")
        except Exception as e:
            print(f"[Possibility] Failed to log decision: {e}")


if __name__ == "__main__":
    engine = PossibilityEngine()
    
    print("[POSSIBILITY ENGINE] Exploring potential actions...\n")
    
    # Example: Explore possibilities for optimizing Sarah_Chat.py
    goal = "optimize Sarah_Chat.py for better performance"
    
    possibilities = engine.explore_possibilities(goal)
    
    print(f"[POSSIBILITIES GENERATED]: {len(possibilities)}\n")
    
    for i, p in enumerate(possibilities, 1):
        print(f"{i}. {p['scenario']}")
        print(f"   Feasibility: {p['feasibility_score']} | Impact: {p['impact_score']} | Risk: {p['risk_score']}")
        print(f"   Overall Score: {p['overall_score']}\n")
    
    # Evaluate and recommend
    decision = engine.evaluate_decision(possibilities)
    
    print(f"[RECOMMENDATION]")
    print(f"Selected: {decision['recommendation']['scenario']}")
    print(f"Confidence: {decision['confidence']}")
    print(f"Alternatives considered: {decision['alternatives_considered']}")
    print(f"Rejected: {decision['alternatives_rejected']}")
