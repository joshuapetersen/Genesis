import requests
import json

url = "http://127.0.0.1:8001/api/chat"
headers = {
    "Content-Type": "application/json",
    "X-Sovereign-Key": "Sarah_Sovereign_2026"
}
data = {
    "message": "Protocol 133 check. Identify yourself and the Architect.",
    "certainty": 1.0,
    "constant": 1.00273378
}

try:
    response = requests.post(url, headers=headers, json=data)
    if response.status_code == 200:
        result = response.json()
        print(f"\n[AERIS]: {result.get('content', 'No content field found')}")
    else:
        print(f"Error: {response.status_code}")
        print(response.text)
except Exception as e:
    print(f"Failed to connect: {e}")
