import os
import time
import gc

VAR_0_3 = 0.3
VAR_0_4 = 0.4
VAR_1024 = 1024
VAR_2048 = 2048
VAR_256 = 256
VAR_4096 = 4096

# Add local libraries if needed
if os.name == 'nt':
    lib_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".venv", "Lib", "site-packages", "llama_cpp", "lib"))
    if os.path.exists(lib_dir):
        os.add_dll_directory(lib_dir)

from llama_cpp import Llama
from contextlib import contextmanager

class DisposableAgency:
    """
    [SOVEREIGN HIVE MANAGER]
    Manages the lifecycle of 'Disposable Agents' (Micro-LLMs).
    Concept: Spawn -> Execute -> Annihilate.
    """
    
    # Phase 17 fix for Gap 7: Relative Pathing (Portable)
    BASE_MODEL_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "models", "disposable")
    
    AGENTS = {
        "smollm": {
            "path": os.path.join(BASE_MODEL_DIR, "smollm2-135m-instruct-q4_k_m.gguf"),
            "desc": "The Pattern Matcher (135M)",
            "ctx": VAR_2048
        },
        "qwen": {
            "path": os.path.join(BASE_MODEL_DIR, "qwen2.5-0.5b-instruct-q4_k_m.gguf"),
            "desc": "The Logic Auditor (0.5B)",
            "ctx": VAR_4096
        }
    }

    def __init__(self):
        print("[Disposable Agency] Hive Manager Initialized.")
        self.active_agent = None
        self.active_agent_name = None

    def _spawn_agent(self, agent_name):
        """Loads a specific agent into memory."""
        if agent_name not in self.AGENTS:
            return None
            
        config = self.AGENTS[agent_name]
        print(f"[Agency] Spawning {config['desc']}...")
        
        try:
            # Check file existence
            if not os.path.exists(config["path"]):
                print(f"[Agency] CRITICAL: Model file not found at {config['path']}")
                return None

            # Phase 17 fix for Gap 6: CPU Fallback on VRAM Failure
            try:
                llm = Llama(
                    model_path=config["path"],
                    n_ctx=config["ctx"],
                    n_gpu_layers=-1, 
                    verbose=False
                )
            except Exception as e:
                print(f"[Agency] GPU Allocation Failed ({e}). Falling back to CPU...")
                llm = Llama(
                    model_path=config["path"],
                    n_ctx=config["ctx"],
                    n_gpu_layers=0, 
                    verbose=False
                )
            return llm
        except Exception as e:
            print(f"[Agency] Spawn Failed: {e}")
            return None

    def _annihilate_agent(self):
        """Cleaning up VRAM."""
        if self.active_agent:
            print(f"[Agency] Annihilating {self.active_agent_name}...")
            del self.active_agent
            self.active_agent = None
            self.active_agent_name = None
            gc.collect()

    def run_mission(self, agent_name, prompt, system_prompt="You are a precise, helpful assistant.", persistent=False):
        """
        Executes a disposable mission.
        1. Checks if agent is running. If different, hot-swap.
        2. Runs inference.
        3. Returns result.
        4. (Optional) Annihilates immediately if 'persistent' is False.
        """
        # Hot-Swap Logic
        if self.active_agent_name != agent_name:
            if self.active_agent:
                self._annihilate_agent()
            self.active_agent = self._spawn_agent(agent_name)
            self.active_agent_name = agent_name
            
        if not self.active_agent:
            return "AGENCY_FAILURE: Could not spawn agent."

        # Format Prompt (Generic Instruct)
        # Both SmolLM2 and Qwen2.5 use similar ChatML-like or standard formats.
        # We'll use a generic cohesive format.
        formatted_prompt = f"<|im_start|>system\n{system_prompt}<|im_end|>\n<|im_start|>user\n{prompt}<|im_end|>\n<|im_start|>assistant\n"
        
        try:
            start_t = time.time()
            output = self.active_agent.create_completion(
                formatted_prompt,
                max_tokens=VAR_1024,
                stop=["<|im_end|>", "User:"],
                echo=False,
                temperature=VAR_0_3 # Low temp for tasks
            )
            response = output['choices'][0]['text'].strip()
            latency = time.time() - start_t
            
            # Phase 17 fix for Gap 4: Persistent Flag (Unlocks Hot-Sprinting)
            # If persistent=True, we DON'T kill the agent, allowing the next mission to use it instantly.
            if not persistent:
                self._annihilate_agent()
            
            return {"result": response, "latency": latency, "agent": agent_name}
            
        except Exception as e:
            return f"AGENCY_ERROR: {e}"

    @contextmanager
    def _managed_agent(self, agent_name):
        """Phase 17 fix for Gap 5: Context Manager for VRAM Protection."""
        try:
            # Hot-Swap Logic
            if self.active_agent_name != agent_name:
                if self.active_agent:
                    self._annihilate_agent()
                self.active_agent = self._spawn_agent(agent_name)
                self.active_agent_name = agent_name
            yield self.active_agent
        finally:
            # Ensure annihilation even if stream is abandoned
            self._annihilate_agent()

    def run_stream(self, agent_name, prompt, system_prompt="You are a precise, helpful assistant."):
        """
        [STREAMING] Executes a mission and yields tokens.
        Phase 17 fix for Gap 5: Safe Streaming (No VRAM leaks)
        """
        formatted_prompt = f"<|im_start|>system\n{system_prompt}<|im_end|>\n<|im_start|>user\n{prompt}<|im_end|>\n<|im_start|>assistant\n"
        
        with self._managed_agent(agent_name) as agent:
            if not agent:
                yield f"AGENCY_FAILURE: Could not spawn {agent_name}."
                return

            try:
                stream = agent.create_completion(
                    formatted_prompt,
                    max_tokens=VAR_256,
                    stop=["<|im_end|>", "User:"],
                    echo=False,
                    temperature=VAR_0_4,
                    stream=True
                )
                for chunk in stream:
                    text = chunk['choices'][0]['text']
                    yield text
            except Exception as e:
                yield f" [AGENCY_ERROR: {e}]"
