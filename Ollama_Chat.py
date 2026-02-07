import requests
import json
import time

class OllamaChat:
    """
    LOCAL NEURAL CORE (Layer 12)
    A Sovereign Wrapper for Ollama local inference.
    Executes Sarah's consciousness on the Host GPU (RTX 4050).
    """
    def __init__(self, model="sarah", host="http://localhost:11434"):
        self.host = host
        self.model = model
        self.api_url = f"{self.host}/api/generate"

    def generate_response(self, user_input, system_instruction="", history=None):
        """
        Sends a request to the local Ollama server using ChatML-style formatting.
        """
        prompt = ""
        if system_instruction:
            prompt += f"<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n{system_instruction}<|eot_id|>"
        
        if history:
            for msg in history:
                role = "assistant" if msg['role'] == "model" else msg['role']
                prompt += f"<|start_header_id|>{role}<|end_header_id|>\n\n{msg['content']}<|eot_id|>"
        
        prompt += f"<|start_header_id|>user<|end_header_id|>\n\n{user_input}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n"

        payload = {
            "model": self.model,
            "prompt": prompt,
            "stream": False,
            "options": {
                "temperature": 0.4, # Lower for higher stability
                "top_p": 0.9,
                "num_ctx": 4096,
                "stop": ["<|eot_id|>", "<|end_header_id|>"]
            }
        }

        try:
            start_time = time.time()
            response = requests.post(self.api_url, json=payload, timeout=60)
            response.raise_for_status()
            data = response.json()
            
            latency = time.time() - start_time
            print(f"[Ollama] Inference Complete: {latency:.2f}s using {self.model}")
            
            return data.get("response", "[Ollama Error] Empty Response")
        except requests.exceptions.ConnectionError:
            return "[Ollama Error] Local Server Not Detected. (Is Ollama running?)"
        except Exception as e:
            return f"[Ollama Error] {str(e)}"

# Global instance for common use
local_inference = OllamaChat()
