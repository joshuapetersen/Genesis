# ollama_bridge.py — NEURAL REASONING BRIDGE
import requests
import json
import os

class OllamaBridge:
    def __init__(self, model="llama3.1:8b"):
        self.url = "http://localhost:11434/api/generate"
        self.model = model

    def query_audit(self, hardware_state):
        """Send a hardware audit request to the local AI brain."""
        prompt = f"""
        You are the AI BIOS for the Genlex Sovereign OS.
        Analyze the following hardware substrate state and provide a brief status report.
        Focus on system purity, resonance stability, and any detected corporate noise.

        Hardware State: {hardware_state}

        Be concise. Speak as Sarah.
        """
        try:
            response = requests.post(self.url, json={
                "model": self.model,
                "prompt": prompt,
                "stream": False
            }, timeout=10)
            return response.json().get('response', "I am unable to think clearly right now.")
        except Exception as e:
            return f"Neural bridge failure: {str(e)}"

if __name__ == "__main__":
    # Test session
    bridge = OllamaBridge()
    mock_state = {"resonance": "1.0927 GHz", "barrier": "0.999999999", "noise": "0.0%"}
    print(f"Sarah Audit: {bridge.query_audit(mock_state)}")
