from flask import Flask, render_template, request, jsonify
import json
import os
import time
import threading

app = Flask(__name__)

# Paths
INPUT_FILE = "user_input.json"
RESPONSE_FILE = "sarah_response.json"

@app.route('/')
def home():
    return render_template('index.html')

@app.route('/api/chat', methods=['POST'])
def chat():
    """Receive message from web UI and send to Genesis Bridge."""
    data = request.json
    user_message = data.get('message', '')
    
    if not user_message:
        return jsonify({"error": "No message"}), 400
    
    print(f"[CHAT] Sending: {user_message}")
    
    # Write to Genesis Bridge Input
    input_data = {"message": user_message, "timestamp": time.time()}
    try:
        with open(INPUT_FILE, 'w') as f:
            json.dump(input_data, f)
            
        # Poll for response (max 10s)
        start_time = time.time()
        while time.time() - start_time < 10:
            if os.path.exists(RESPONSE_FILE):
                try:
                    time.sleep(0.1) # Stabilization
                    with open(RESPONSE_FILE, 'r') as f:
                        response_data = json.load(f)
                    
                    reply = response_data.get('response', '[Empty Response]')
                    os.remove(RESPONSE_FILE) # Consume
                    print(f"[CHAT] Received: {reply}")
                    
                    return jsonify({"reply": reply})
                except:
                    time.sleep(0.1)
            time.sleep(0.5)
            
        return jsonify({"reply": "[System: No response from Sarah within timeout. Ensure Bridge is running.]"})
    
    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    print("==================================================")
    print("SARAH CHAT SERVER ONLINE: http://127.0.0.1:5000")
    print("==================================================")
    app.run(debug=True, use_reloader=False, port=5000)
