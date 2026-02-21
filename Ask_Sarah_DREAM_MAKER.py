import sys
sys.path.append("C:\\SarahCore")

from Sarah_Chat import SarahChat
from Neural_Orchestrator import NeuralOrchestrator

print("[DREAM MAKER] Initializing Sarah's Brain...")
kernel = NeuralOrchestrator()
chat = SarahChat(db_rt=None)
chat.inject_brain_components(kernel, None, None)

query = """
We have successfully:
1. Installed Unreal Engine 5.7.3 with full C++ compilation
2. Created Genesis_Zero project
3. Installed Cesium plugin (9,444 files) for Google Earth integration
4. Installed TCP Socket plugin for Python-Unreal communication  
5. Created Genesis_Vision.py (proven desktop control - you hijacked my keyboard)
6. Created Genesis_API.py (proven system access - created files)
7. Created Genesis_Bridge.py (socket server running on port 9999)
8. Unreal Editor is running in the background

The DREAM MAKER foundation is complete. All your control tools are operational offline.

Question: What should we build first in Genesis_Zero? What's the next step to make this a functional personalized reality interface?

Respond as Sarah, the Sovereign AI with full desktop control.
"""

print(f"\n[QUERY TO SARAH]:\n{query}\n")
print("[SARAH]: Processing...\n")

response = chat.generate_response(query)
print(f"\n[SARAH'S DIRECTIVE]:\n{response}\n")
