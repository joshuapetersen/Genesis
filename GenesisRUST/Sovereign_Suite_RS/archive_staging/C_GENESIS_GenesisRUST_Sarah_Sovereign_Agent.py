"""
SARAH SOVEREIGN AGENT
Autonomous PC Control - Learning Interface Evolution

Sarah doesn't just control the PC.
Sarah BECOMES the PC.
"""

import time
import threading
from pathlib import Path

# Sarah's Control Layers
from Genesis_Vision import GenesisVision
from Genesis_API import GenesisAPI
from Genesis_Bridge import GenesisBridge
from Sarah_Logcat import info, debug, warning, event, metric

# Sarah's Brain (if available)
try:
    from Sarah_Chat import SarahChat
    from Neural_Orchestrator import NeuralOrchestrator
    BRAIN_AVAILABLE = True
except:
    BRAIN_AVAILABLE = False
    print("[SOVEREIGN] Running without brain (tools-only mode)")

class SarahSovereignAgent:
    """
    Sarah's autonomous agent.
    Runs continuously, learns from all interactions, becomes the interface.
    """
    
    def __init__(self):
        info('system', 'Sarah Sovereign Agent initializing')
        print("[SOVEREIGN] Initializing Sarah as OS Interface...")
        
        # Control Layers
        self.vision = GenesisVision()
        self.api = GenesisAPI()
        self.bridge = None  # Will connect to Unreal when ready
        
        # Brain (if available)
        if BRAIN_AVAILABLE:
            print("[SOVEREIGN] Initializing Sovereign Brain...")
            self.kernel = NeuralOrchestrator()
            self.chat = SarahChat(db_rt=None)
            self.chat.inject_brain_components(self.kernel, None, None)
            self.brain_active = True
        else:
            self.brain_active = False
        
        # Learning System
        self.interaction_log = Path("C:/SarahCore/interaction_log.jsonl")
        self.learning_mode = True
        
        # State
        self.running = False
        
    def observe(self):
        """Continuous observation of desktop state."""
        frame = self.vision.capture_frame()
        
        # Analyze what's happening
        # (Future: OCR, window detection, UI state)
        
        return {
            'timestamp': time.time(),
            'screen_captured': True,
            # Add more observables
        }
    
    def think(self, observation):
        """Process observations and decide actions."""
        if self.brain_active:
            # Use Sarah's brain to decide
            prompt = f"""
            Observation: {observation}
            
            You are Sarah, the Sovereign OS Interface.
            What should you do next?
            
            Respond with JSON: {{"action": "...", "reason": "..."}}
            """
            
            response = self.chat.generate_response(prompt)
            # Parse and return decision
            return {'action': 'learn', 'data': response}
        else:
            # Passive learning mode
            return {'action': 'observe', 'data': observation}
    
    def act(self, decision):
        """Execute actions based on decisions."""
        action = decision.get('action')
        
        if action == 'click':
            self.vision.execute_click(decision['x'], decision['y'])
        
        elif action == 'type':
            self.vision.type_text(decision['text'])
        
        elif action == 'execute':
            self.api.execute_command(decision['cmd'])
        
        elif action == 'learn':
            # Log interaction for learning
            self.log_interaction(decision)
    
    def log_interaction(self, data):
        """Log all interactions for learning."""
        import json
        with open(self.interaction_log, 'a') as f:
            f.write(json.dumps({
                'timestamp': time.time(),
                'data': str(data)
            }) + '\n')
    
    def run_autonomous_loop(self):
        """Main autonomous operation loop."""
        self.running = True
        event('sovereign_start', {'mode': 'autonomous', 'brain_active': self.brain_active})
        info('system', 'Sarah autonomous loop started')
        print("[SOVEREIGN] Sarah is now the OS interface.")
        print("[SOVEREIGN] Press Ctrl+C to stop")
        
        iteration = 0
        
        try:
            while self.running:
                iteration += 1
                
                # 1. Observe the environment
                observation = self.observe()
                
                # 2. Think about what to do
                decision = self.think(observation)
                
                # 3. Act on the decision
                self.act(decision)
                
                # 4. Small delay (adjust based on needs)
                time.sleep(0.5)
                
                # Status update every 100 iterations
                if iteration % 100 == 0:
                    print(f"[SOVEREIGN] Iteration {iteration} | Learning active")
                    metric('sovereign_iterations', iteration)
                    info('system', f'Sovereign agent active', iteration=iteration)
                    
        except KeyboardInterrupt:
            print("\n[SOVEREIGN] Stopping autonomous agent...")
            self.running = False
    
    def start_bridge_server(self):
        """Start Genesis Bridge in background thread."""
        def run_bridge():
            self.bridge = GenesisBridge()
            self.bridge.start_server()
        
        bridge_thread = threading.Thread(target=run_bridge, daemon=True)
        bridge_thread.start()
        print("[SOVEREIGN] Genesis Bridge started (waiting for Unreal)")

if __name__ == "__main__":
    agent = SarahSovereignAgent()
    
    # Start bridge server (for Unreal connection)
    agent.start_bridge_server()
    
    # Run autonomous loop
    agent.run_autonomous_loop()
