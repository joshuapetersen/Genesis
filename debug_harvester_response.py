
import requests
import json

VAR_0_3 = 0.3
VAR_120 = 120
VAR_4096 = 4096

OLLAMA_URL = "http://localhost:11434/api/generate"
MODEL_NAME = "sarah"

def debug_response():
    """Function: debug_response"""
    term = "abs"
    category = "builtin"
    
    prompt = f"""
    You are the "Sovereign/Sarah" AI, writing your own source code encyclopedia.
    
    Topic: {term}
    Category: {category}
    
    Write a comprehensive, deep technical entry for this topic. 
    Format your response as a valid JSON object with the following fields:
    
    {{
        "definition": "A precise, high-level theoretical definition.",
        "core_concepts": "Detailed explanation of the inner workings, logic, or math involved.",
        "implementation_strategy": "How this is best implemented in Python or CUDA.",
        "code_example": "A robust, production-grade Python or CUDA code snippet demonstrating the concept.",
        "optimization_tip": "Specific advice on performance tuning, memory management (VRAM), or avoiding common pitfalls."
    }}
    
    Ensure the code example is valid and well-commented. Do not use Markdown backticks in the JSON strings.
    """
    
    payload = {
        "model": MODEL_NAME,
        "prompt": prompt,
        "stream": False,
        "options": {
            "temperature": VAR_0_3,
            "num_ctx": VAR_4096
        }
    }
    
    try:
        print(f"Querying for '{term}'...")
        response = requests.post(OLLAMA_URL, json=payload, timeout=VAR_120)
        response.raise_for_status()
        response_text = response.json().get("response", "")
        
        print("\n--- RAW RESPONSE START ---")
        print(response_text)
        print("--- RAW RESPONSE END ---\n")
        
        # Test parsing
        clean_text = response_text.replace("```json", "").replace("```", "").strip()
        data = json.loads(clean_text)
        print("JSON Parse Success!")
        print(json.dumps(data, indent=2))
        
    except json.JSONDecodeError as e:
        print(f"JSON Parse Failed: {e}")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    debug_response()
