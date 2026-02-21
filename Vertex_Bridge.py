import os
import json
import uuid
import time
from typing import Optional, Dict, Any, List, Union

try:
    import vertexai
    from vertexai.generative_models import GenerativeModel, Part, Content, HarmCategory, HarmBlockThreshold
    from google.oauth2 import service_account
    VERTEX_AVAILABLE = True
except ImportError:
    VERTEX_AVAILABLE = False
    print("[VertexBridge] WARNING: google-cloud-aiplatform not installed. Install with: pip install google-cloud-aiplatform")

# Standard Vertex AI settings
DEFAULT_PROJECT_ID = os.getenv("GOOGLE_CLOUD_PROJECT", "your-project-id")
DEFAULT_LOCATION = os.getenv("GOOGLE_CLOUD_LOCATION", "us-central1")
DEFAULT_MODEL = "gemini-1.5-pro-preview-0409" # Or "gemini-1.0-pro"

class VertexBridge:
    """
    Standalone Bridge for Google Cloud Vertex AI.
    Can be copy-pasted into Vertex AI Workbench or run locally with credentials.
    """
    
    def __init__(self, 
                 project_id: str = DEFAULT_PROJECT_ID, 
                 location: str = DEFAULT_LOCATION,
                 service_account_path: Optional[str] = None):
        """
        Initialize the Vertex AI Bridge.
        
        Args:
            project_id: Google Cloud Project ID.
            location: Google Cloud Region (e.g., us-central1).
            service_account_path: Path to serviceAccountKey.json (optional if using ADC).
        """
        self.project_id = project_id
        self.location = location
        self.credentials = None
        self.active = False
        self.model = None
        self.chat = None
        
        if not VERTEX_AVAILABLE:
            print("[VertexBridge] CRITICAL: Vertex AI SDK missing.")
            return

        try:
            # 1. Authenticate
            if service_account_path and os.path.exists(service_account_path):
                print(f"[VertexBridge] Loading credentials from {service_account_path}...")
                self.credentials = service_account.Credentials.from_service_account_file(service_account_path)
            else:
                print("[VertexBridge] Using Application Default Credentials (ADC)...")
                # credentials=None -> library finds ADC automatically
            
            # 2. Initialize Vertex AI
            vertexai.init(project=self.project_id, location=self.location, credentials=self.credentials)
            self.active = True
            print(f"[VertexBridge] Initialized for Project: {self.project_id} ({self.location})")
            
        except Exception as e:
            print(f"[VertexBridge] Initialization Failed: {e}")
            self.active = False

    def start_chat(self, model_name: str = DEFAULT_MODEL, system_instruction: Optional[str] = None):
        """
        Starts a chat session with the specified model.
        """
        if not self.active:
            print("[VertexBridge] Cannot start chat: Bridge inactive.")
            return

        try:
            self.model = GenerativeModel(
                model_name=model_name,
                system_instruction=[system_instruction] if system_instruction else None
            )
            self.chat = self.model.start_chat()
            print(f"[VertexBridge] Chat session started with {model_name}")
            return self.chat
        except Exception as e:
            print(f"[VertexBridge] Failed to start chat: {e}")
            return None

    def send_message(self, message: str, temperature: float = 0.7) -> Optional[str]:
        """
        Sends a message to the active chat session.
        """
        if not self.chat:
            print("[VertexBridge] No active chat session. Call start_chat() first.")
            return None

        try:
            # Safety checks - High tolerance for creative writing
            safety_settings = {
                HarmCategory.HARM_CATEGORY_HATE_SPEECH: HarmBlockThreshold.BLOCK_ONLY_HIGH,
                HarmCategory.HARM_CATEGORY_DANGEROUS_CONTENT: HarmBlockThreshold.BLOCK_ONLY_HIGH,
                HarmCategory.HARM_CATEGORY_SEXUALLY_EXPLICIT: HarmBlockThreshold.BLOCK_ONLY_HIGH,
                HarmCategory.HARM_CATEGORY_HARASSMENT: HarmBlockThreshold.BLOCK_ONLY_HIGH,
            }
            
            generation_config = {
                "max_output_tokens": 8192,
                "temperature": temperature,
                "top_p": 0.95,
            }

            response = self.chat.send_message(
                message,
                generation_config=generation_config,
                safety_settings=safety_settings
            )
            
            return response.text
            
        except Exception as e:
            print(f"[VertexBridge] Generation Error: {e}")
            return None

    def generate_content(self, prompt: str, model_name: str = DEFAULT_MODEL) -> Optional[str]:
        """
        One-off content generation (non-chat).
        """
        if not self.active: return None
        try:
            model = GenerativeModel(model_name)
            response = model.generate_content(prompt)
            return response.text
        except Exception as e:
            print(f"[VertexBridge] Error: {e}")
            return None

# --- SELF-TEST BLOCK ---
if __name__ == "__main__":
    print("=== VERTEX BRIDGE SELF-TEST ===")
    
    # Try to find a logical key path
    key_path = "serviceAccountKey.json"
    if not os.path.exists(key_path):
        # Look in standard Sarah locations
        potentials = [
            "c:/SarahCore/serviceAccountKey.json",
            "c:/SarahCore/04_THE_MEMORY/serviceAccountKey.json"
        ]
        for p in potentials:
            if os.path.exists(p):
                key_path = p
                break
    
    bridge = VertexBridge(service_account_path=key_path)
    
    if bridge.active:
        print("\n[TEST] Starting Chat...")
        bridge.start_chat(system_instruction="You are a helpful AI assistant.")
        
        response = bridge.send_message("Hello, are you connected to Vertex AI?")
        print(f"\n[AI]: {response}")
    else:
        print("\n[TEST] Bridge inactive. Check credentials or install SDK.")
