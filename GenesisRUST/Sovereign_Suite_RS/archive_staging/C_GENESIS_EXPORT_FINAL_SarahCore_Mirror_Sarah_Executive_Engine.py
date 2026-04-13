import time
import sys
import os
import json
from Sarah_Brain import SarahBrain

class SarahAgentEngine:
    """
    Sarah Autonomous Agent Engine
    Transforms Sarah from 'Reactive' (Command-Based) to 'Proactive' (Goal-Based).
    """
    def __init__(self, brain=None):
        print("[Engine] Initializing Sarah Agent Engine...")
        if brain:
            self.brain = brain
        else:
            self.brain = SarahBrain()
        self.active_goals = []
        self.is_running = True
        
        # Ensure SAUL is active
        if self.brain.saul and not self.brain.saul.active:
             self.brain.saul.start_autonomy()
        
        # [SOVEREIGN MANIFEST]
        if hasattr(self.brain, "manifest") and self.brain.manifest:
             print("[Engine] Linking to Sovereign Manifest (11M+ Index)...")
        else:
             print("[Engine] Manifest not linked. Using heuristics.")
        
        # Check for SAUL methods
        if not hasattr(self.brain.saul, "get_pending_tasks"):
            print("[Engine] WARNING: SAUL lacks 'get_pending_tasks'. Update SAUL_Log_System.py.")
        if not hasattr(self.brain.saul, "log_event"):
             print("[Engine] WARNING: SAUL lacks 'log_event'. Update SAUL_Log_System.py.")


    def perceive(self):
        """Step 1: Look at the system and memory for tasks."""
        # Check SAUL for pending missions or 'The Architect's' latest commands
        pending = []
        if self.brain.saul:
            try:
                # Retrieve pending tasks from SAUL (needs implementation in SAUL)
                if hasattr(self.brain.saul, "get_pending_tasks"):
                    pending = self.brain.saul.get_pending_tasks()
                else:
                    # Fallback/Placeholder if SAUL update isn't live yet
                    # In production, this might read from a specific log file or DB location
                    pass
            except Exception as e:
                print(f"[Engine] Perception Error: {e}")
        return pending

    def plan(self, goal):
        """Step 2: Use ReasoningV3 + Manifest to break the goal into steps."""
        print(f"[Engine] Planning for goal: {goal}")
        
        # 1. Consult Manifest for Capabilities
        if hasattr(self.brain, "manifest") and self.brain.manifest:
             capabilities = self.brain.manifest.find_capability(goal)
             if capabilities:
                 print(f"[Engine] Found {len(capabilities)} relevant modules in Manifest.")
                 # In a real system, we'd feed these modules to the LLM context.
                 # For now, we log them to show the connection.
                 for cap in capabilities[:3]:
                     print(f"   - Relevance: {cap['module']} (Score: {cap['score']})")

        # 2. Ask the reasoning core to produce a JSON list of steps
        if hasattr(self.brain.reasoning, "generate_step_by_step_plan"):
             plan = self.brain.reasoning.generate_step_by_step_plan(goal)
        else:
             print("[Engine] Reasoning Core lacks planning capability. Update Sarah_Reasoning_V3.")
             plan = []
        return plan

    def execute(self, steps):
        """Step 3: Use the Actuators to perform the work."""
        if not steps:
            print("[Engine] No steps to execute.")
            return

        for step in steps:
            action_type = step.get('type', 'unknown')
            action_desc = step.get('action', 'Unknown Action')
            print(f"[Engine] Executing: {action_desc} ({action_type})")
            
            result = "Executed"
            
            try:
                if action_type == 'browser':
                    url = step.get('url')
                    if url:
                        self.brain.actuator.open_browser(url)
                    else:
                        result = "Error: No URL provided for browser action"
                        
                elif action_type == 'logic':
                    query = step.get('query')
                    if query:
                        # Use the brain's main reasoning process
                        final_result = self.brain.reasoning.process_query(query)
                        result = final_result.get('result', str(final_result))
                    else:
                        result = "Error: No query provided for logic action"
                        
                elif action_type == 'system':
                    app = step.get('app')
                    if app:
                        result = self.brain.actuator.launch_app(app)
                    else:
                         result = "Error: No app provided for system action"
                
                elif action_type == 'type':
                     text = step.get('text')
                     selector = step.get('selector')
                     if text:
                         result = self.brain.actuator.type_text(text, selector)
                     else:
                         result = "Error: No text provided for type action"

                elif action_type == 'click':
                     selector = step.get('selector')
                     if selector:
                         result = self.brain.actuator.click_element(selector)
                     else:
                         result = "Error: No selector provided for click action"
                
                else:
                    result = f"Unknown action type: {action_type}"

            except Exception as e:
                result = f"Execution Error: {e}"
            
            print(f"[Engine] Result: {result}")

            # Record result in SAUL so Sarah "remembers" what happened
            if self.brain.saul:
                if hasattr(self.brain.saul, "log_event"):
                    self.brain.saul.log_event("step_complete", {"step": step, "result": str(result)})
                else:
                    # Fallback log
                    print(f"[Engine] SAUL log: Step Complete - {result}")

    def run_cycle(self):
        """The Heartbeat of the Agent Engine."""
        print("[Engine] Agent Loop Started. Waiting for goals...")
        while self.is_running:
            try:
                # 1. Check Active Goals (Queued by Autonomy Command or Pulse)
                while self.active_goals:
                    goal = self.active_goals.pop(0)
                    print(f"[Engine] Processing Active Goal: {goal}")
                    blueprint = self.plan(goal)
                    self.execute(blueprint)

                # 2. Perceive External Tasks
                tasks = self.perceive()
                if tasks:
                    for task in tasks:
                        print(f"[Engine] New Task Detected: {task}")
                        if isinstance(task, dict) and 'goal' in task:
                            blueprint = self.plan(task['goal'])
                            self.execute(blueprint)
                        elif isinstance(task, str):
                             blueprint = self.plan(task)
                             self.execute(blueprint)
                else:
                    # No tasks? Enter "Dreaming" or "Self-Optimization" mode
                    # print("[Engine] Idle...") # Reduce spam
                    
                    # Optional: Evolution Cycle
                    if hasattr(self.brain, 'evolution') and self.brain.evolution:
                        # Only run evolution occasionally to avoid hogging CPU
                        # self.brain.evolution.run_evolution_cycle()
                        pass
                
                time.sleep(5) # Prevent CPU burnout
            except KeyboardInterrupt:
                print("[Engine] Stopping Agent Engine...")
                self.is_running = False
            except Exception as e:
                print(f"[Engine] Cycle Error: {e}")
                time.sleep(5)

if __name__ == "__main__":
    # Standalone Test
    engine = SarahAgentEngine()
    engine.run_cycle()
