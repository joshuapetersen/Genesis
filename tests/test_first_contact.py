
import sys
import os

# Ensure SarahCore is in path
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from Neural_Orchestrator import orchestrator

def first_contact():
    print("\n--- INITIATING FIRST CONTACT ---\n")
    
    # 1. Ask Sarah about her status
    prompt = "System status. What do you see?"
    print(f"USER: {prompt}")
    
    response = orchestrator.generate_response(prompt)
    
    print(f"\nSARAH: {response}")
    print("\n--- END TRANSMISSION ---")

if __name__ == "__main__":
    first_contact()
