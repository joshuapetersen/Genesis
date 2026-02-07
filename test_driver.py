import requests
import json
import time

url = "http://localhost:8001/v1/chat/completions"

# Dense input designed to trigger Sovereign Math Density Check > 1.0
prompt = "Analyze the volumetric density of the Sovereign Anchor constant 1.0927 in relation to 68D lattice stability."

payload = {
    "messages": [
        {"role": "user", "content": prompt}
    ],
    "stream": True 
}

print(f"--- SOVEREIGN DRIVER TEST ---")
print(f"Target: {url}")
print(f"Prompt: {prompt}")
print("Sending request to trigger HEMODYNAMICS calculation...")

try:
    start = time.time()
    response = requests.post(url, json=payload, stream=True)
    
    print("Response Stream Started:")
    for line in response.iter_lines():
        if line:
            decoded_line = line.decode('utf-8')
            if decoded_line.startswith("data: "):
                try:
                    data = json.loads(decoded_line[6:])
                    if 'choices' in data:
                        content = data['choices'][0]['delta'].get('content', '')
                        print(content, end="", flush=True)
                except:
                    pass
    print(f"\n\n[DONE] Latency: {time.time() - start:.2f}s")
    print("CHECK TERMINAL LOGS FOR: [Neural Driver] Adjusted Modulators")
    
except Exception as e:
    print(f"Connection Error: {e}")
