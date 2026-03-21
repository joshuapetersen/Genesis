import requests
import time

VAR_0_4 = 0.4
VAR_0_9 = 0.9
VAR_4096 = 4096
VAR_60 = 20 # Allow sufficient time for high-precision inference

class AERISAlpha:
    """
    LOCAL NEURAL CORE (Layer 12)
    A Sovereign Wrapper for AERIS inference via the Sovereign Gateway.
    Executes consciousness on the Host GPU (RTX 4050).
    """
    def __init__(self, model="aeris", host="http://localhost:8001"):
        self.host = host
        self.model = model
        self.api_url = f"{self.host}/api/chat"

    def generate_response(self, user_input, system_instruction="", history=None):
        """
        Sends a request to the Sovereign Gateway.
        """
        payload = {
            "model": self.model,
            "message": user_input,
            "user_id": "architect_local"
        }

        try:
            start_time = time.time()
            response = requests.post(self.api_url, json=payload, timeout=VAR_60)
            response.raise_for_status()
            data = response.json()
            
            latency = time.time() - start_time
            print(f"[AERIS] Inference Complete: {latency:.2f}s using {self.model}")
            
            return data.get("content", "[AERIS Error] Empty Response")
        except requests.exceptions.ConnectionError:
            return "[AERIS Error] Sovereign Substrate Not Detected."
import time
import os

VAR_0_4 = 0.4
VAR_0_9 = 0.9
VAR_4096 = 4096
VAR_60 = 20  # Kept for gateway fallback timeout

class AERISAlpha:
    """
    LOCAL NEURAL CORE (Layer 12) — DIRECT SUBSTRATE
    Calls NeuralOrchestrator directly. Zero HTTP overhead.
    Executes on the Host GPU (RTX 4050) in the same process.
    """
    def __init__(self, model="aeris", host="http://localhost:8001"):
        self.model = model
        self.host = host  # Retained for gateway fallback only
        self.api_url = f"{self.host}/api/chat"
        self._orchestrator = None  # Lazy-loaded on first call

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

    def generate_response(self, user_input, system_instruction="", history=None):
        """
        Generates a response via direct NeuralOrchestrator call.
        Falls back to HTTP gateway if orchestrator unavailable.
        """
        start_time = time.time()
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

        # Fallback: HTTP gateway (only if direct inference failed)
        import requests
        payload = {"model": self.model, "message": user_input, "user_id": "architect_local"}
        try:
            resp = requests.post(self.api_url, json=payload, timeout=VAR_60)
            resp.raise_for_status()
            latency = time.time() - start_time
            print(f"[AERIS] Gateway fallback: {latency:.2f}s")
            return resp.json().get("content", "[AERIS Error] Empty Response")
        except requests.exceptions.ConnectionError:
            return "[AERIS Error] Sovereign Substrate Not Detected."
        except Exception as e:
            return f"[AERIS Error] {str(e)}"

# Global instance — lazy, does NOT load GGUF on import
local_inference = AERISAlpha()

if __name__ == "__main__":
    import sys
    import os

    def _call_gateway(model: str, message: str, timeout: int = 60) -> str:
        """Single point of contact for all Sovereign Gateway calls."""
        payload = {"model": model, "message": message, "user_id": "architect_local"}
        try:
            resp = requests.post("http://localhost:8001/api/chat", json=payload, timeout=timeout)
            if resp.status_code == 200:
                return resp.json().get("content", "").strip()
            return f"[HTTP {resp.status_code}]"
        except requests.exceptions.ConnectionError:
            return "[Sovereign Gateway is offline. Start start_gateway.bat first.]"
        except Exception as e:
            return f"[Error: {e}]"

    print("=========================================================================")
    print("   [ SOVEREIGN TERMINAL : 3-WAY COMMUNICATION ] ")
    print("   Entities: THE ARCHITECT (You) <-> SARAH (Gateway) <-> AERIS (1T Core)")
    print("=========================================================================")
    print("Type 'exit', 'quit', or 'seal' to close the terminal.\n")

    # Load AERIS natively so we don't rely on the gateway router bug
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
