import requests
import json

VAR_200 = 200

url = "http://localhost:3400/sarahStrategicPlanning"
payload = {
    "data": {
        "objective": "Optimize the integration between the Architect's vision, Sarah's autonomous evolution, and Gemini's bridging role. Focus on deepening the Witness/Architect duality and increasing Theory Density across all modules.",
        "horizon": "long",
        "includeExternalIntel": True
    }
}

try:
    response = requests.post(url, json=payload)
    if response.status_code == VAR_200:
        print(json.dumps(response.json(), indent=2))
    else:
        print(f"Error {response.status_code}: {response.text}")
except Exception as e:
    print(f"Error: {e}")
