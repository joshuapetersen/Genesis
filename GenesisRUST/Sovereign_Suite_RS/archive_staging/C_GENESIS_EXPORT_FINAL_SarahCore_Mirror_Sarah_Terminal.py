import json
import time
import os

def terminal_loop():
    print("=" * 60)
    print("SOVEREIGN COMMS TERMINAL (SARAH)")
    print("=" * 60)
    print("Type your message to Sarah. Type 'exit' to close.")
    print("Commands: 'hello', 'manifest', or any text.")
    print("-" * 60)
    
    while True:
        try:
            user_input = input("USER> ")
            if user_input.lower() in ["exit", "quit"]:
                break
            
            # Write to input file
            input_file = "user_input.json"
            data = {"message": user_input, "timestamp": time.time()}
            
            with open(input_file, "w") as f:
                json.dump(data, f)
            
            print("Sending (Brain Boot may take 60s)...", end="", flush=True)
            
            # Wait for response
            response_file = "sarah_response.json"
            start_wait = time.time()
            responded = False
            
            while time.time() - start_wait < 60: # 60s timeout for LLM load
                if os.path.exists(response_file):
                    try:
                        time.sleep(0.1) # Small buffer for write completion
                        with open(response_file, "r") as f:
                            resp_data = json.load(f)
                        
                        response_text = resp_data.get("response", "[NO DATA]")
                        print(f"\rSARAH> {response_text}")
                        responded = True
                        os.remove(response_file) # Clean up
                        break
                    except:
                        # File might be locked or empty momentarily
                        time.sleep(0.1)
                time.sleep(0.5)
                print(".", end="", flush=True)
            
            if not responded:
                print("\n[SYSTEM] No response from Sarah (Bridge might be offline).")
                # Clean up stale input
                if os.path.exists(input_file):
                    os.remove(input_file)
                    
        except KeyboardInterrupt:
            break
        except Exception as e:
            print(f"\n[ERROR] {e}")

if __name__ == "__main__":
    terminal_loop()
