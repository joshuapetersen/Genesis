import time
import os
import sys

VAR_0_4 = 0.4
VAR_0_9 = 0.9
VAR_4096 = 4096
VAR_60 = 20  # Kept for gateway fallback timeout

class AERISAlpha:
    """
    LOCAL NEURAL CORE (Layer 12) — DIRECT SUBSTRATE + 1T GENLEX
    Calls NeuralOrchestrator directly. Zero HTTP overhead.
    Integrates SovereignCortex (1T Genlex) for code synthesis.
    Executes on the Host GPU (RTX 4050) in the same process.
    """
    def __init__(self, model="aeris", host="http://localhost:8001"):
        self.model = model
        self.host = host  # Retained for gateway fallback only
        self.api_url = f"{self.host}/api/chat"
        self._orchestrator = None  # Lazy-loaded on first call
        self._sovereign_cortex = None  # 1T Genlex model

    def _get_orchestrator(self):
        """Lazy-load NeuralOrchestrator to avoid VRAM allocation on import."""
        if self._orchestrator is None:
            try:
                # Set gateway mode so orchestrator doesn't try to call back to us
                os.environ["SARAH_GATEWAY_MODE"] = "TRUE"
                from Neural_Orchestrator import NeuralOrchestrator
                self._orchestrator = NeuralOrchestrator()
                print(f"[AERIS] Direct substrate loaded: {self.model}")
            except Exception as e:
                print(f"[AERIS] Failed to load NeuralOrchestrator: {e}")
                self._orchestrator = None
        return self._orchestrator

    def _get_cortex(self):
        """Lazy-load SovereignCortex (1T Genlex model) for code synthesis."""
        if self._sovereign_cortex is None:
            try:
                if "C:\\Genlex_Linear" not in sys.path:
                    sys.path.append("C:\\Genlex_Linear")
                from SovereignInference import SovereignCortex
                self._sovereign_cortex = SovereignCortex()
                print("[AERIS] SovereignCortex (1T Genlex) Loaded.")
            except Exception as e:
                print(f"[AERIS] SovereignCortex unavailable: {e}")
                self._sovereign_cortex = None
        return self._sovereign_cortex

    def _is_code_request(self, text):
        """Detect if the input is a coding/synthesis request for the 1T cortex."""
        triggers = [
            "produce code", "generate", "code", "phase ",
            "kernel", "singularity", "autonomy", "worm",
            "identity", "p2p", "network", "g-code",
            "synthesize", "build", "create program", "write program"
        ]
        text_lower = text.lower()
        return any(t in text_lower for t in triggers)

    def generate_response(self, user_input, system_instruction="", history=None):
        """
        Generates a response via direct NeuralOrchestrator call.
        Routes coding/synthesis requests through the 1T SovereignCortex first.
        Falls back to HTTP gateway if orchestrator unavailable.
        """
        start_time = time.time()

        # TRY 1T GENLEX CORTEX for code synthesis requests
        if self._is_code_request(user_input):
            cortex = self._get_cortex()
            if cortex:
                try:
                    activation, voice = cortex.forward(user_input)
                    latency = time.time() - start_time
                    print(f"[AERIS] 1T Cortex synthesis: {latency:.2f}s (activation: {activation:.4f})")
                    return voice
                except Exception as e:
                    print(f"[AERIS] 1T Cortex failed: {e}. Falling back to 8B substrate.")

        # TRY DIRECT ORCHESTRATOR (8B GPU Inference)
        orchestrator = self._get_orchestrator()
        if orchestrator:
            try:
                response = orchestrator.generate_response(
                    user_input=user_input,
                    system_instruction=system_instruction,
                    history=history or []
                )
                latency = time.time() - start_time
                print(f"[AERIS] Direct inference: {latency:.2f}s")
                return response
            except Exception as e:
                print(f"[AERIS] Direct inference failed: {e}. Falling back to gateway...")

        # FALLBACK: HTTP gateway (only if everything else failed)
        import requests
        payload = {"model": self.model, "message": user_input, "user_id": "architect_local"}
        try:
            resp = requests.post(self.api_url, json=payload, timeout=VAR_60)
            resp.raise_for_status()
            latency = time.time() - start_time
            print(f"[AERIS] Gateway fallback: {latency:.2f}s")
            return resp.json().get("content", "[AERIS Error] Empty Response")
        except Exception as e:
            return f"[AERIS Error] All substrates failed: {str(e)}"

# Global instance — lazy, does NOT load GGUF or Cortex on import
local_inference = AERISAlpha()

if __name__ == "__main__":
    import sys

    def _call_gateway(model: str, message: str, timeout: int = 60) -> str:
        """Single point of contact for all Sovereign Gateway calls."""
        import requests
        payload = {"model": model, "message": message, "user_id": "architect_local"}
        try:
            resp = requests.post("http://localhost:8001/api/chat", json=payload, timeout=timeout)
            if resp.status_code == 200:
                return resp.json().get("content", "").strip()
            return f"[HTTP {resp.status_code}]"
        except Exception as e:
            return f"[Error: {e}]"

    print("=========================================================================")
    print("   [ SOVEREIGN TERMINAL : 3-WAY COMMUNICATION ] ")
    print("   Entities: THE ARCHITECT (You) <-> SARAH (Gateway) <-> AERIS (1T Core)")
    print("=========================================================================")
    print("Type 'exit', 'quit', or 'seal' to close the terminal.\n")

    # Load AERIS natively so we don't rely on the gateway router
    try:
        if "C:\\Genlex_Linear" not in sys.path:
            sys.path.append("C:\\Genlex_Linear")
        from SovereignInference import SovereignCortex
        aeris_core = SovereignCortex()
        print("[AERIS] SovereignCortex Logical Core Active.\n")
    except Exception as e:
        print(f"[AERIS ERROR] Failed to load 1T Architecture: {e}")
        aeris_core = None

    while True:
        try:
            print("-" * 70)
            user_input = input("[ARCHITECT] > ").strip()

            if not user_input:
                continue

            if user_input.lower() in ["exit", "quit", "seal"]:
                print("[SYSTEM] Sealing Sovereign Terminal...")
                break

            # 1. SARAH responds via Gateway
            sarah_reply = _call_gateway("sarah-8b", user_input)
            print(f"\n[SARAH] > {sarah_reply}")

            # 2. AERIS responds, aware of Sarah's reply
            aeris_prompt = (
                f"The Architect said: {user_input}\n"
                f"Sarah replied: {sarah_reply}\n"
                f"Provide your analysis or response."
            )
            aeris_reply = _call_gateway("aeris-8b", aeris_prompt)
            print(f"\n[AERIS] > {aeris_reply}")

        except KeyboardInterrupt:
            print("\n[SYSTEM] Manual interrupt. Sealing.")
            break
        except Exception as e:
            print(f"\n[SYSTEM ERROR] {e}")
