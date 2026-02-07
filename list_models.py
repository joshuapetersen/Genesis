
import os
import asyncio
import traceback
from google.genai import Client

# Attempt to load env var if not set
if not os.environ.get("GEMINI_API_KEY"):
    env_path = os.path.join(os.path.dirname(__file__), ".env")
    if os.path.exists(env_path):
        with open(env_path, 'r') as f:
            for line in f:
                if line.startswith("GEMINI_API_KEY="):
                    os.environ["GEMINI_API_KEY"] = line.strip().split("=")[1]

async def list_models():
    api_key = os.environ.get("GEMINI_API_KEY")
    if not api_key:
        print("ERROR: GEMINI_API_KEY not found in environment or .env")
        return

    print(f"Using API Key: {api_key[:5]}...{api_key[-5:]}")
    
    try:
        client = Client(api_key=api_key)
        print("\n--- QUERYING MODELS ---")
        # Try standard list_models
        # Note: The SDK structure might vary, adapting to common patterns
        
        # Method 1: client.models.list()
        try:
            print("Attempting client.models.list()...")
            # Some versions return an iterator or list
            models = client.models.list()
            count = 0
            with open("available_models.txt", "w") as f:
                for m in models:
                    line = f" - {m.name} (Display: {getattr(m, 'display_name', 'N/A')})"
                    print(line)
                    f.write(line + "\n")
                    count += 1
            if count == 0:
                print("No models returned.")
        except Exception as e:
            print(f"Method 1 failed: {e}")
            traceback.print_exc()

    except Exception as e:
        print(f"Client initialization failed: {e}")
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(list_models())
