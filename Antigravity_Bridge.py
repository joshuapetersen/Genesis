"""
Antigravity Bridge
Integrates Agentic logic (tools, planning, execution) into SarahCore.
"""

import os
import sys
import json
from typing import Dict, Any, List, Optional

# Import Sovereign Constants
try:
    from Sovereign_Constants import (
        VAR_5, VAR_10, SOVEREIGN_ANCHOR, VAR_60
    )
except ImportError:
    # Fallback if specific constants aren't available
    VAR_5 = 5
    VAR_10 = 10
    SOVEREIGN_ANCHOR = 1.09277703703703 # Phase 18 fix for Gap 2: Float fallback

# Import Tools
try:
    from local_file_tool import LocalFileTool
    from awesome_skills_tool import AwesomeSkillsTool
    TOOLS_AVAILABLE = True
except ImportError:
    print("[Antigravity] WARNING: Tools not found. Agentic mode limited.")
    TOOLS_AVAILABLE = False


class AntigravityProtocol:
    """
    Antigravity Protocol: The Agentic Bridge.
    Allows Sarah to pause, plan, use tools, and verify results before responding.
    """
    
    def __init__(self):
        self.active = True
        self.agent_name = "Antigravity_Connect"
        self.execution_log = []
        
        # Initialize Tools
        if TOOLS_AVAILABLE:
            self.file_tool = LocalFileTool()
            # Assuming skills root is relative or defined in environment
            skills_root = os.environ.get("SKILLS_ROOT", os.path.join(os.getcwd(), "antigravity-awesome-skills"))
            self.skills_tool = AwesomeSkillsTool(skills_root)
        else:
            self.file_tool = None
            self.skills_tool = None
            
        print(f"[Antigravity] Protocol Initialized. Tools: {'ACTIVE' if TOOLS_AVAILABLE else 'INACTIVE'}")

    def should_intervene(self, query: str) -> bool:
        """
        Determines if the Antigravity Agent should handle this query.
        Triggers on keywords indicating a need for action, research, or complex planning.
        """
        if not self.active:
            return False
            
        import re
        # Phase 18 fix for Gap 3: Word-boundary triggers (No more substring 'run' collisions)
        triggers = [
            r"\baudit\b", r"\bcheck file\b", r"\bread file\b", r"\blist directory\b", 
            r"\blist files\b", r"\bscan\b", r"\bresearch\b", r"\bplan\b", 
            r"\bexecute\b", r"\brun\b", r"\bverify\b", r"\bdebug\b", 
            r"\bagent\b", r"\bantigravity\b"
        ]
        
        query_lower = query.lower()
        for pattern in triggers:
            if re.search(pattern, query_lower):
                return True
        
        return False

    def process_task(self, query: str, context: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """
        Main entry point for agentic task execution.
        """
        if context is None:
            context = {}
            
        print(f"[Antigravity] Intercepted Task: {query}")
        self.execution_log.append({"event": "TASK_START", "query": query})
        
        # 1. PLAN
        plan = self._create_plan(query)
        
        # 2. EXECUTE
        execution_results = self._execute_plan(plan)
        
        # 3. VERIFY / SYNTHESIZE
        final_answer = self._synthesize_results(query, execution_results)
        
        return {
            "processing_mode": "antigravity_agent",
            "agent_status": "SUCCESS",
            "result": final_answer,
            "execution_log": self.execution_log,
            "tools_used": list(execution_results.get("tool_outputs", {}).keys())
        }

    def inject_components(self, orchestrator):
        """Inject Neural Orchestrator for LLM planning."""
        self.orchestrator = orchestrator
        print(f"[Antigravity] Neural Orchestrator Injected.")

    def _create_plan(self, query: str) -> List[Dict[str, Any]]:
        """
        Creates a plan using LLM if available, otherwise heuristics.
        """
        # Try LLM Planning First
        if hasattr(self, 'orchestrator') and self.orchestrator:
            try:
                system_prompt = """You are the Antigravity Planner. Your goal is to break down the user's request into a sequence of tool calls.
Available Tools:
- local_file_tool (actions: list, read) - Use for file system exploration. arguments: target (path).
- awesome_skills_tool (actions: search, read) - Use to find how to do things. arguments: query (search) or skill_name (read).

Response Format: STRICT JSON ARRAY. No markdown, no explanations.
Example:
[
  {"tool": "local_file_tool", "action": "list", "target": "C:/SarahCore"},
  {"tool": "awesome_skills_tool", "action": "search", "query": "memory"}
]"""
                response = self.orchestrator.generate_response(
                    user_input=f"Plan this task: {query}",
                    system_instruction=system_prompt,
                    history=[]
                )
                
                # Clean JSON
                json_str = response
                if "```json" in json_str:
                    json_str = json_str.split("```json")[1].split("```")[0].strip()
                elif "```" in json_str:
                    json_str = json_str.split("```")[1].split("```")[0].strip()
                
                try:
                    steps = json.loads(json_str)
                except json.JSONDecodeError:
                    # Phase 18 fix for Gap 4: Robust JSON Parse (No more silent failure)
                    print("[Antigravity] JSON Decode Error. Falling back to heuristics.")
                    return self._create_heuristic_plan(query)

                if isinstance(steps, list) and len(steps) > 0:
                    self.execution_log.append({"event": "LLM_PLAN_CREATED", "steps": steps})
                    print(f"[Antigravity] LLM Plan Generated: {len(steps)} steps.")
                    return steps
            except Exception as e:
                print(f"[Antigravity] LLM Planning Failed: {e}. Falling back to heuristics.")

        return self._create_heuristic_plan(query)

    def _create_heuristic_plan(self, query: str) -> List[Dict[str, Any]]:
        """Phase 18: Isolated Heuristic Path."""
        q_lower = query.lower()
        
        # Heuristic 1: File Operations
        if "read" in q_lower or "check" in q_lower:
            # Extract path (simple heuristic)
            words = query.split()
            for word in words:
                if "." in word or "/" in word or "\\" in word:
                    steps.append({"tool": "local_file_tool", "action": "read", "target": word})
        
        elif "list" in q_lower or "dir" in q_lower:
             # Extract path
            words = query.split()
            target = os.getcwd() # Default
            for word in words:
                if "/" in word or "\\" in word:
                    target = word
            steps.append({"tool": "local_file_tool", "action": "list", "target": target})

        # Heuristic 2: Skills Search
        if "skill" in q_lower or "how to" in q_lower:
            steps.append({"tool": "awesome_skills_tool", "action": "search", "query": query})

        # Fallback if no specific tool identified but intercepted
        if not steps:
            steps.append({"tool": "thought", "action": "analyze", "content": "Complex query requiring detailed analysis."})
            
        self.execution_log.append({"event": "HEURISTIC_PLAN_CREATED", "steps": steps})
        return steps

    def _is_path_safe(self, target_path: str) -> bool:
        """Phase 18 fix for Gap 12: Path Sanitization (SA_ROOT Jail)."""
        from Sovereign_Constants import SA_ROOT
        try:
            abs_root = os.path.abspath(SA_ROOT)
            abs_target = os.path.abspath(target_path)
            return abs_target.startswith(abs_root)
        except:
            return False

    def _execute_plan(self, plan: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Executes the steps in the plan using available tools.
        """
        results = {"errors": [], "tool_outputs": {}} # Phase 18 fix for Gap 5: Multi-error tracking
        
        for index, step in enumerate(plan):
            tool_name = step.get("tool")
            action = step.get("action")
            
            try:
                if tool_name == "local_file_tool" and self.file_tool:
                    target = step.get("target")
                    
                    # Gap 12 Security Check
                    if not self._is_path_safe(target):
                        error_msg = f"SECURITY_VIOLATION: Path '{target}' is outside SA_ROOT."
                        print(f"[Antigravity] {error_msg}")
                        results["errors"].append(error_msg)
                        continue

                    print(f"[Antigravity] Executing File Tool: {action} on {target}")
                    output = self.file_tool.execute(action, target)
                    results["tool_outputs"][f"file_{action}_{target}"] = output
                    
                elif tool_name == "awesome_skills_tool" and self.skills_tool:
                    q = step.get("query")
                    print(f"[Antigravity] Executing Skills Tool: {action} for {q}")
                    output = self.skills_tool.execute(action, query=q)
                    results["tool_outputs"]["skills_search"] = output
                    
                elif tool_name == "thought":
                    results["tool_outputs"]["thought"] = step.get("content")
                    
                else:
                    results["errors"].append(f"Step {index}: Tool {tool_name} not available or unknown.")
                    
            except Exception as e:
                results["errors"].append(f"Step {index} Failed: {str(e)}")
                
        return results

    def _synthesize_results(self, query: str, results: Dict[str, Any]) -> str:
        """
        Synthesizes the execution results into a final answer.
        """
        # In a full system, this would be an LLM call.
        # Here we format the output.
        
        response_parts = [f"Antigravity Agent Report for: '{query}'\n"]
        
        for key, value in results.items():
            response_parts.append(f"--- Result: {key} ---")
            # Truncate long outputs for valid display
            val_str = str(value)
            if len(val_str) > 1000:
                response_parts.append(val_str[:1000] + "... [truncated]")
            else:
                response_parts.append(val_str)
            response_parts.append("\n")
            
        return "\n".join(response_parts)

if __name__ == "__main__":
    # Internal Test
    print("Testing Antigravity Bridge...")
    bridge = AntigravityProtocol()
    
    test_query = "Please list the files in c:\\SarahCore"
    if bridge.should_intervene(test_query):
        result = bridge.process_task(test_query)
        print(result["result"])
    else:
        print("Test Failed: Should have intervened.")
