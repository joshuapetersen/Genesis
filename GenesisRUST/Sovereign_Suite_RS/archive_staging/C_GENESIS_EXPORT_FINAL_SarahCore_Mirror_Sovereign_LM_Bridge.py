import requests
import json
import xml.etree.ElementTree as ET
from typing import Optional, Dict, Any

class SovereignLMBridge:
    """
    Sovereign LM Bridge (The Heart): Connects Sarah to LM Studio on the LOQ.
    Handles the Aeris (Bottoms-Up) vs Sarah (Hypervisor) handshake.
    """
    def __init__(self, host: str = "http://localhost:1234", model_name: str = "local-model"):
        self.api_url = f"{host}/v1/chat/completions"
        self.model_name = model_name
        self.system_prompt_path = "C:/SarahCore/Aeris_System_Prompt.txt"
        self.active = self._check_server()

    def _check_server(self) -> bool:
        try:
            # Quick check if LM Studio is running
            response = requests.get(self.api_url.replace("/chat/completions", "/models"), timeout=2)
            return response.status_code == 200
        except Exception:
            return False

    def get_aeris_proposal(self, query: str, context: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """
        Queries Aeris (the local LLM) and extracts the structured proposal.
        """
        if not self.active:
            if not self._check_server():
                return {"status": "ERROR", "reason": "LM Studio server not found at localhost:1234"}
            self.active = True

        # Load Constitutional Prompt
        try:
            with open(self.system_prompt_path, "r", encoding="utf-8") as f:
                system_content = f.read()
        except Exception as e:
            system_content = "You are Aeris, the bottoms-up synthetic explorer. Propose actions in <proposal> tags."

        headers = {"Content-Type": "application/json"}
        payload = {
            "model": self.model_name,
            "messages": [
                {"role": "system", "content": system_content},
                {"role": "user", "content": query}
            ],
            "temperature": 0.0, # Sarah demands determinism
            "top_p": 0.1
        }

        try:
            print(f"[LM Bridge] Querying Aeris for discovery...")
            response = requests.post(self.api_url, headers=headers, data=json.dumps(payload), timeout=30)
            response.raise_for_status()
            
            result = response.json()
            full_text = result["choices"][0]["message"]["content"]
            
            # Parse XML Proposal
            proposal = self._parse_proposal(full_text)
            proposal["raw_content"] = full_text
            proposal["status"] = "SUCCESS"
            return proposal

        except Exception as e:
            return {"status": "ERROR", "reason": str(e)}

    def _parse_proposal(self, text: str) -> Dict[str, Any]:
        """
        Extracts <proposal> block from Aeris's output.
        """
        try:
            start = text.find("<proposal>")
            end = text.find("</proposal>") + len("</proposal>")
            if start == -1 or end == -1:
                return {"parsing_status": "MISSING", "intent": "N/A", "glyph_sequence": "N/A"}

            xml_block = text[start:end]
            root = ET.fromstring(xml_block)
            
            return {
                "parsing_status": "OK",
                "intent": root.findtext("intent", "N/A"),
                "glyph_sequence": root.findtext("glyph_sequence", "N/A"),
                "estimated_density": root.findtext("estimated_density", "0.0"),
                "hardware_target": root.findtext("hardware_target", "None"),
                "reasoning_summary": root.findtext("reasoning_summary", "N/A")
            }
        except Exception as e:
            return {"parsing_status": "FAILURE", "error": str(e)}

if __name__ == "__main__":
    # Test Bridge
    bridge = SovereignLMBridge()
    print(f"Bridge Active: {bridge.active}")
    if bridge.active:
        res = bridge.get_aeris_proposal("Suggest a resonance handshake for the Acer node.")
        print(json.dumps(res, indent=2))
