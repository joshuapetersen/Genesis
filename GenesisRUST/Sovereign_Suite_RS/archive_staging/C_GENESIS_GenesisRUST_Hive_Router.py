class HiveRouter:
    """
    [HIVE ROUTER]
    Directs 'Eco-Flow' tasks to the specialized Disposable Agent.
    - SmolLM (135M): The Pattern Matcher (Regex, Cleanup, Formats)
    - Qwen (0.5B): The Logic Auditor (Summary, Reasoning, Creative)
    """
    
    AGENT_SMOLLM = "smollm"
    AGENT_QWEN = "qwen"

    def __init__(self):
        # Pattern Matching keywords (Syntax focused)
        self.smollm_triggers = [
            "regex", "pattern", "clean", "format", "json", "list", 
            "structure", "extract", "syntax", "code block"
        ]
        
        # Logic/Creative keywords (Context focused)
        self.qwen_triggers = [
            "summarize", "explain", "why", "logic", "draft", "write", 
            "idea", "check", "audit", "reason", "plan"
        ]

    def select_agent(self, prompt):
        """
        Returns the optimal agent key for the prompt.
        """
        prompt_lower = prompt.lower()
        
        # 1. Qwen Priority (Reasoning)
        # If it requires understanding 'why' or 'summarizing', use the 0.5B brain.
        if any(k in prompt_lower for k in self.qwen_triggers):
            return self.AGENT_QWEN
            
        # 2. SmolLM Priority (Syntax)
        # If it's pure formatting or extraction, use the 135M speedster.
        if any(k in prompt_lower for k in self.smollm_triggers):
            return self.AGENT_SMOLLM
            
        # Default fallback for "Errands" -> Qwen is safer/smarter
        return self.AGENT_QWEN
