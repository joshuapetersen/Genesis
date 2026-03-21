
VAR_2000 = 2000
class SovereignRouter:
    """
    [COGNITIVE LOAD SENSOR]
    Determines the optimal Neural Gear for a given prompt.
    Modes:
    - ECO_FLOW: Simple tasks, 1B Model (Node Beta)
    - SPECULATIVE: Standard chat, 1B->8B (Node Alpha + Beta)
    - SOVEREIGN_DEEP: Complex logic, 8B Precision Lock (Node Alpha)
    """
    
    ECO_FLOW = "ECO_FLOW"
    SPECULATIVE = "SPECULATIVE"
    SOVEREIGN_DEEP = "SOVEREIGN_DEEP"

    def __init__(self):
        self.errand_keywords = [
            "format this", "summarize this", "list files", "cleanup", "organize",
            "search", "find", "check calendar", "remind me"
        ]
        self.deep_keywords = [
            "0.0903", "architect", "evolution", "system core", "annihilation", 
            "protocol", "sovereign", "define", "explain", "why", "logic", 
            "analyze", "synthesize", "bridge"
        ]

    def evaluate_complexity(self, prompt):
        """
        Scans input for triggers and returns the target Mode.
        """
        prompt_lower = prompt.lower()
        
        # 1. DEEP SCAN (Priority Override)
        # If the user invokes Sovereign keywords, we lock precision immediately.
        if any(k in prompt_lower for k in self.deep_keywords):
            return self.SOVEREIGN_DEEP
            
        # 2. ERRAND SCAN (Eco-Flow)
        # If it's a simple task and NOT deep, we save 8B resources.
        if any(k in prompt_lower for k in self.errand_keywords):
            # Length check: Long prompts might need 8B context even for summarizing
            if len(prompt) < VAR_2000: 
                return self.ECO_FLOW
                
        # 3. DEFAULT (Speculative Drive)
        # Standard conversation uses the accelerator.
        return self.SPECULATIVE
